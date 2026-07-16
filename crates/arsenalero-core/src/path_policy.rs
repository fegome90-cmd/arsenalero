//! Read-only, fail-closed filesystem containment for skill resources.

use std::{
    collections::BTreeSet,
    fs::{self, File},
    io,
    path::{Component, Path, PathBuf},
};

use crate::ArsenalError;

const MAX_RESOURCE_BYTES: u64 = 256 * 1024;
const MAX_NORMALIZED_RELATIVE_PATH_BYTES: usize = 256;

/// A configured, canonical allowlist for read-only skill roots.
///
/// Task 5 does not prescribe constructors or call signatures. `new` therefore accepts the
/// smallest useful configuration: an explicit collection of existing absolute directories. Each
/// entry is canonicalized once, so later callers can prove that their requested root belongs to
/// this allowlist without performing any write, network, shell, or digest operation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PathPolicy {
    allowed_roots: BTreeSet<PathBuf>,
}

/// An absolute skill root that was canonicalized and accepted by a [`PathPolicy`].
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct CanonicalSkillRoot(PathBuf);

impl CanonicalSkillRoot {
    /// Returns the canonical filesystem path retained by this proof object.
    pub fn as_path(&self) -> &Path {
        &self.0
    }
}

/// An absolute, regular resource file resolved within a [`CanonicalSkillRoot`].
///
/// `as_path` is retained only for diagnostics. Consumers must read through a cloned retained
/// handle so a later replacement at this path cannot alter the bytes they receive.
#[derive(Debug)]
pub struct CanonicalResourcePath {
    path: PathBuf,
    file: File,
}

impl PartialEq for CanonicalResourcePath {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl Eq for CanonicalResourcePath {}

impl CanonicalResourcePath {
    pub(crate) fn from_retained(path: PathBuf, file: File) -> Self {
        Self { path, file }
    }
    /// Returns the canonical filesystem path retained by this proof object.
    pub fn as_path(&self) -> &Path {
        &self.path
    }

    /// Clones the retained, validated read-only handle for a consumer read.
    pub fn try_clone_file(&self) -> io::Result<File> {
        self.file.try_clone()
    }
}

impl PathPolicy {
    /// Builds an explicit allowlist from existing, absolute skill-root directories.
    ///
    /// Invalid or non-absolute configured entries map to [`ArsenalError::SkillRootInvalid`].
    /// This is a Task 5 API choice rather than a prescribed authority signature.
    pub fn new<I, P>(allowed_roots: I) -> Result<Self, ArsenalError>
    where
        I: IntoIterator<Item = P>,
        P: AsRef<Path>,
    {
        let mut canonical_roots = BTreeSet::new();

        for root in allowed_roots {
            let root = root.as_ref();
            if !root.is_absolute() {
                return Err(ArsenalError::SkillRootInvalid);
            }

            let canonical = fs::canonicalize(root).map_err(|_| ArsenalError::SkillRootInvalid)?;
            if !fs::metadata(&canonical)
                .map_err(|_| ArsenalError::SkillRootInvalid)?
                .is_dir()
            {
                return Err(ArsenalError::SkillRootInvalid);
            }
            canonical_roots.insert(canonical);
        }

        Ok(Self {
            allowed_roots: canonical_roots,
        })
    }

    /// Canonicalizes an absolute root and proves it is in this policy's explicit allowlist.
    ///
    /// Non-absolute or non-directory input maps to [`ArsenalError::SkillRootInvalid`]; a valid
    /// canonical directory absent from the allowlist maps to the narrower
    /// [`ArsenalError::SkillRootNotAllowed`].
    pub fn canonical_skill_root(
        &self,
        root: impl AsRef<Path>,
    ) -> Result<CanonicalSkillRoot, ArsenalError> {
        let root = root.as_ref();
        if !root.is_absolute() {
            return Err(ArsenalError::SkillRootInvalid);
        }

        let canonical = fs::canonicalize(root).map_err(|_| ArsenalError::SkillRootInvalid)?;
        if !fs::metadata(&canonical)
            .map_err(|_| ArsenalError::SkillRootInvalid)?
            .is_dir()
        {
            return Err(ArsenalError::SkillRootInvalid);
        }
        if !self.allowed_roots.contains(&canonical) {
            return Err(ArsenalError::SkillRootNotAllowed);
        }

        Ok(CanonicalSkillRoot(canonical))
    }

    /// Resolves one relative resource path under an accepted skill root and retains its handle.
    ///
    /// Absolute paths, every `..` component, and normalized relative paths longer than 256
    /// **bytes** map to [`ArsenalError::ResourcePathEscape`]. The byte limit is inclusive: a
    /// 256-byte normalized relative path is allowed. Missing paths use
    /// [`ArsenalError::ResourceReferenceBroken`], symlink/canonical containment escapes use
    /// [`ArsenalError::ResourceSymlinkEscape`], non-regular files and unsupported extensions use
    /// [`ArsenalError::ResourceTypeUnsupported`], and files larger than 256 KiB use
    /// [`ArsenalError::ResourceTooLarge`]. These mappings choose the narrowest existing reason
    /// code where the authority does not define one.
    pub fn resolve_resource(
        &self,
        skill_root: &CanonicalSkillRoot,
        relative_path: impl AsRef<Path>,
    ) -> Result<CanonicalResourcePath, ArsenalError> {
        if !self.allowed_roots.contains(skill_root.as_path()) {
            return Err(ArsenalError::SkillRootNotAllowed);
        }

        let normalized = normalize_relative_path(relative_path.as_ref())?;
        let candidate = skill_root.as_path().join(normalized);
        let canonical =
            fs::canonicalize(candidate).map_err(|_| ArsenalError::ResourceReferenceBroken)?;

        if !canonical.starts_with(skill_root.as_path()) {
            return Err(ArsenalError::ResourceSymlinkEscape);
        }

        let file = File::open(&canonical).map_err(|_| ArsenalError::ResourceReferenceBroken)?;
        let metadata = file
            .metadata()
            .map_err(|_| ArsenalError::ResourceReferenceBroken)?;
        if !metadata.is_file() || !has_supported_extension(&canonical) {
            return Err(ArsenalError::ResourceTypeUnsupported);
        }
        if metadata.len() > MAX_RESOURCE_BYTES {
            return Err(ArsenalError::ResourceTooLarge);
        }

        Ok(CanonicalResourcePath {
            path: canonical,
            file,
        })
    }
}

fn normalize_relative_path(path: &Path) -> Result<PathBuf, ArsenalError> {
    if path.is_absolute() {
        return Err(ArsenalError::ResourcePathEscape);
    }

    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::Normal(segment) => normalized.push(segment),
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => {
                return Err(ArsenalError::ResourcePathEscape);
            }
        }
    }

    if normalized.as_os_str().is_empty()
        || normalized.as_os_str().len() > MAX_NORMALIZED_RELATIVE_PATH_BYTES
    {
        return Err(ArsenalError::ResourcePathEscape);
    }

    Ok(normalized)
}

fn has_supported_extension(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|extension| extension.to_str()),
        Some("md" | "txt" | "json" | "yaml" | "yml" | "toml")
    )
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        io::Read,
        path::{Path, PathBuf},
        sync::atomic::{AtomicUsize, Ordering},
    };

    use proptest::{prelude::*, prop_oneof};

    use super::{ArsenalError, PathPolicy};

    static NEXT_TEMP_DIR: AtomicUsize = AtomicUsize::new(0);

    struct TempDir {
        path: PathBuf,
    }

    impl TempDir {
        fn new() -> Self {
            let suffix = NEXT_TEMP_DIR.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "arsenalero-path-policy-{}-{suffix}",
                std::process::id()
            ));
            fs::create_dir_all(&path).expect("test temp directory should be created");
            Self { path }
        }

        fn path(&self) -> &Path {
            &self.path
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    fn write_file(root: &Path, relative: &str, contents: &[u8]) {
        let path = root.join(relative);
        fs::create_dir_all(path.parent().expect("test resource should have a parent"))
            .expect("test resource parent should be created");
        fs::write(path, contents).expect("test resource should be written");
    }

    fn policy_for(root: &Path) -> (PathPolicy, super::CanonicalSkillRoot) {
        let policy = PathPolicy::new([root]).expect("absolute test root should be valid");
        let canonical_root = policy
            .canonical_skill_root(root)
            .expect("configured test root should be allowed");
        (policy, canonical_root)
    }

    #[test]
    fn accepts_a_resource_within_an_allowed_root() {
        let temp = TempDir::new();
        write_file(temp.path(), "resources/guide.md", b"guide");
        let (policy, root) = policy_for(temp.path());

        let resource = policy
            .resolve_resource(&root, "resources/guide.md")
            .expect("supported regular resource should resolve");

        assert!(resource.as_path().starts_with(root.as_path()));
    }

    #[test]
    fn rejects_a_root_outside_the_explicit_allowlist() {
        let allowed = TempDir::new();
        let outside = TempDir::new();
        let policy = PathPolicy::new([allowed.path()]).expect("allowed root should be valid");

        assert_eq!(
            policy.canonical_skill_root(outside.path()),
            Err(ArsenalError::SkillRootNotAllowed)
        );
    }

    #[test]
    fn rejects_a_root_proof_created_by_another_policy() {
        let first = TempDir::new();
        let second = TempDir::new();
        write_file(first.path(), "safe.md", b"safe");
        let (_, first_root) = policy_for(first.path());
        let second_policy = PathPolicy::new([second.path()]).expect("second root should be valid");

        assert_eq!(
            second_policy.resolve_resource(&first_root, "safe.md"),
            Err(ArsenalError::SkillRootNotAllowed)
        );
    }

    #[test]
    fn rejects_non_absolute_configured_roots() {
        assert_eq!(
            PathPolicy::new([Path::new("relative-root")]),
            Err(ArsenalError::SkillRootInvalid)
        );
    }

    #[test]
    fn rejects_an_absolute_resource_path() {
        let temp = TempDir::new();
        write_file(temp.path(), "resources/guide.md", b"guide");
        let (policy, root) = policy_for(temp.path());

        assert_eq!(
            policy.resolve_resource(&root, temp.path().join("resources/guide.md")),
            Err(ArsenalError::ResourcePathEscape)
        );
    }

    #[test]
    fn rejects_parent_traversal_even_when_it_would_normalize_inside() {
        let temp = TempDir::new();
        write_file(temp.path(), "resources/guide.md", b"guide");
        let (policy, root) = policy_for(temp.path());

        assert_eq!(
            policy.resolve_resource(&root, "resources/../resources/guide.md"),
            Err(ArsenalError::ResourcePathEscape)
        );
    }

    #[cfg(unix)]
    #[test]
    fn rejects_a_symlink_that_escapes_the_skill_root() {
        use std::os::unix::fs::symlink;

        let root = TempDir::new();
        let outside = TempDir::new();
        write_file(outside.path(), "outside.md", b"outside");
        symlink(
            outside.path().join("outside.md"),
            root.path().join("escape.md"),
        )
        .expect("test symlink should be created");
        let (policy, canonical_root) = policy_for(root.path());

        assert_eq!(
            policy.resolve_resource(&canonical_root, "escape.md"),
            Err(ArsenalError::ResourceSymlinkEscape)
        );
    }

    #[test]
    fn rejects_a_directory_as_a_resource() {
        let temp = TempDir::new();
        fs::create_dir_all(temp.path().join("resources"))
            .expect("test directory should be created");
        let (policy, root) = policy_for(temp.path());

        assert_eq!(
            policy.resolve_resource(&root, "resources"),
            Err(ArsenalError::ResourceTypeUnsupported)
        );
    }

    #[test]
    fn rejects_an_unsupported_extension() {
        let temp = TempDir::new();
        write_file(temp.path(), "resources/tool.rs", b"fn main() {}");
        let (policy, root) = policy_for(temp.path());

        assert_eq!(
            policy.resolve_resource(&root, "resources/tool.rs"),
            Err(ArsenalError::ResourceTypeUnsupported)
        );
    }

    #[test]
    fn rejects_a_missing_resource_reference() {
        let temp = TempDir::new();
        let (policy, root) = policy_for(temp.path());

        assert_eq!(
            policy.resolve_resource(&root, "missing.md"),
            Err(ArsenalError::ResourceReferenceBroken)
        );
    }

    #[test]
    fn rejects_a_resource_larger_than_256_kib() {
        let temp = TempDir::new();
        write_file(temp.path(), "resources/large.md", &vec![0; 256 * 1024 + 1]);
        let (policy, root) = policy_for(temp.path());

        assert_eq!(
            policy.resolve_resource(&root, "resources/large.md"),
            Err(ArsenalError::ResourceTooLarge)
        );
    }

    #[test]
    fn accepts_a_resource_of_exactly_256_kib() {
        let temp = TempDir::new();
        write_file(temp.path(), "resources/exact.md", &vec![0; 256 * 1024]);
        let (policy, root) = policy_for(temp.path());

        assert!(policy.resolve_resource(&root, "resources/exact.md").is_ok());
    }

    #[cfg(unix)]
    #[test]
    fn retained_handle_reads_original_bytes_after_path_replacement() {
        let temp = TempDir::new();
        write_file(temp.path(), "resources/guide.md", b"original");
        let (policy, root) = policy_for(temp.path());
        let resource = policy
            .resolve_resource(&root, "resources/guide.md")
            .expect("resource should resolve");
        let replacement = temp.path().join("resources/replacement.md");
        fs::write(&replacement, b"replacement").expect("replacement should be written");
        fs::rename(&replacement, resource.as_path()).expect("path replacement should succeed");

        let mut reader = resource
            .try_clone_file()
            .expect("retained handle should clone for reading");
        let mut bytes = Vec::new();
        reader
            .read_to_end(&mut bytes)
            .expect("retained handle should remain readable");

        assert_eq!(bytes, b"original");
    }

    #[test]
    fn accepts_a_256_byte_normalized_relative_path() {
        let temp = TempDir::new();
        let directories = ["a".repeat(100), "b".repeat(100), "c".repeat(46)];
        let relative = directories
            .iter()
            .fold(PathBuf::new(), |path, directory| path.join(directory))
            .join("file.md");
        assert_eq!(relative.as_os_str().len(), 256);
        write_file(
            temp.path(),
            relative.to_str().expect("test path is UTF-8"),
            b"guide",
        );
        let (policy, root) = policy_for(temp.path());

        assert!(policy.resolve_resource(&root, &relative).is_ok());
    }

    #[test]
    fn rejects_a_relative_path_longer_than_256_bytes() {
        let temp = TempDir::new();
        let relative = PathBuf::from("a".repeat(249)).join("file.md");
        assert!(relative.as_os_str().len() > 256);
        let (policy, root) = policy_for(temp.path());

        assert_eq!(
            policy.resolve_resource(&root, relative),
            Err(ArsenalError::ResourcePathEscape)
        );
    }

    proptest! {
        #[test]
        fn every_accepted_generated_path_starts_with_the_canonical_skill_root(
            segments in prop::collection::vec(
                prop_oneof![
                    Just(".".to_owned()),
                    Just("..".to_owned()),
                    Just("safe.md".to_owned()),
                    Just("resources".to_owned()),
                    "[a-z]{1,8}".prop_map(|segment| segment),
                ],
                0..12,
            ),
        ) {
            let temp = TempDir::new();
            write_file(temp.path(), "safe.md", b"safe");
            let (policy, root) = policy_for(temp.path());
            let known_safe = policy
                .resolve_resource(&root, "safe.md")
                .expect("known accepted path should resolve");
            prop_assert!(known_safe.as_path().starts_with(root.as_path()));
            let relative = segments.iter().fold(PathBuf::new(), |path, segment| path.join(segment));

            if let Ok(resource) = policy.resolve_resource(&root, &relative) {
                prop_assert!(resource.as_path().starts_with(root.as_path()));
            }
        }
    }
}
