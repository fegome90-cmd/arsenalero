use std::{
    collections::BTreeSet,
    path::{Path, PathBuf},
    sync::{Mutex, MutexGuard},
};

use crate::{ArsenalError, CaseId, ReceiptId, ResourceId, receipt::digest_skill_file};

/// An active case anchored to the exact `SKILL.md` bytes it was opened with.
#[derive(Debug)]
pub struct ArsenalCase {
    id: CaseId,
    skill_path: PathBuf,
    skill_digest: String,
    issued_resources: Mutex<IssuedResources>,
    consumed_receipts: Mutex<BTreeSet<ReceiptId>>,
}

#[derive(Debug, Default)]
pub(crate) struct IssuedResources {
    pub(crate) ids: BTreeSet<ResourceId>,
    pub(crate) content_bytes: usize,
}

impl ArsenalCase {
    /// Opens a case and records the initial `SKILL.md` digest for drift detection.
    pub fn open(skill_path: impl Into<PathBuf>) -> Result<Self, ArsenalError> {
        let skill_path =
            std::fs::canonicalize(skill_path.into()).map_err(|_| ArsenalError::SkillMdMissing)?;
        let skill_digest = digest_skill_file(&skill_path)?;
        Ok(Self {
            id: CaseId::now_v7(),
            skill_path,
            skill_digest,
            issued_resources: Mutex::new(IssuedResources::default()),
            consumed_receipts: Mutex::new(BTreeSet::new()),
        })
    }

    /// Returns the UUIDv7 case identifier.
    pub fn id(&self) -> &CaseId {
        &self.id
    }

    /// Returns the digest recorded when the case was opened.
    pub fn skill_digest(&self) -> &str {
        &self.skill_digest
    }

    pub(crate) fn skill_path(&self) -> &Path {
        &self.skill_path
    }

    pub(crate) fn issued_resources(&self) -> Result<MutexGuard<'_, IssuedResources>, ArsenalError> {
        self.issued_resources
            .lock()
            .map_err(|_| ArsenalError::ResourceAlreadyIssued)
    }

    pub(crate) fn consumed_receipts(
        &self,
    ) -> Result<MutexGuard<'_, BTreeSet<ReceiptId>>, ArsenalError> {
        self.consumed_receipts
            .lock()
            .map_err(|_| ArsenalError::ReceiptUnknown)
    }
}
