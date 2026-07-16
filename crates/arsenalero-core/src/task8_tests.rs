use std::{
    fs,
    path::Path,
    sync::{Mutex, OnceLock},
};

use crate::{
    ArsenalCase, ArsenalError, AttainedEvidenceLevel, EvidenceContract, EvidenceReference,
    PathPolicy, ResourceId, ResourceIssueRequest, attest_resources, issue_resources,
};

const FIXTURE_ROOT: &str = "../../tests/fixtures/digest_drift";

fn fixture_lock() -> std::sync::MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(())).lock().unwrap()
}

fn case() -> ArsenalCase {
    ArsenalCase::open(format!("{FIXTURE_ROOT}/SKILL.md")).unwrap()
}

fn resource() -> ResourceIssueRequest {
    let fixture_root = fs::canonicalize(FIXTURE_ROOT).unwrap();
    let policy = PathPolicy::new([&fixture_root]).unwrap();
    let root = policy.canonical_skill_root(&fixture_root).unwrap();
    ResourceIssueRequest {
        resource_id: ResourceId::new("resources::validation"),
        resource: policy
            .resolve_resource(&root, Path::new("validation.md"))
            .unwrap(),
        purpose: "Define final acceptance checks".into(),
        evidence_contract: EvidenceContract {
            minimum: AttainedEvidenceLevel::Attestation,
            supported_levels: vec![
                AttainedEvidenceLevel::Attestation,
                AttainedEvidenceLevel::ArtifactReference,
            ],
        },
    }
}

#[test]
fn receipt_is_bound_to_case_skill_and_resource_digests() {
    let _guard = fixture_lock();
    let case = case();
    let receipt = issue_resources(&case, &[resource()])
        .unwrap()
        .pop()
        .unwrap();

    assert_eq!(receipt.case_id(), case.id());
    assert_eq!(receipt.skill_digest(), case.skill_digest());
    assert_eq!(receipt.resource_digest(), receipt.resource().digest);
    assert!(receipt.resource_digest().starts_with("sha256:"));
}

#[test]
fn issue_exposes_capability_but_never_attained_evidence() {
    let _guard = fixture_lock();
    let receipt = issue_resources(&case(), &[resource()])
        .unwrap()
        .pop()
        .unwrap();

    assert_eq!(
        receipt.resource().evidence_contract().supported_levels,
        vec![
            AttainedEvidenceLevel::Attestation,
            AttainedEvidenceLevel::ArtifactReference,
        ]
    );
}

#[test]
fn attest_computes_attained_evidence_level() {
    let _guard = fixture_lock();
    let case = case();
    let receipt = issue_resources(&case, &[resource()])
        .unwrap()
        .pop()
        .unwrap();

    let attestation = attest_resources(
        &case,
        &[receipt.clone()],
        &[crate::AttestationRequest {
            receipt_id: receipt.id().clone(),
            usage: "Used to choose the final test commands.".into(),
            evidence: vec![EvidenceReference::artifact("verification-report.json")],
        }],
    )
    .unwrap()
    .pop()
    .unwrap();

    assert_eq!(
        attestation.attained_evidence_level,
        AttainedEvidenceLevel::ArtifactReference
    );
}

#[test]
fn rejects_cross_case_receipts() {
    let _guard = fixture_lock();
    let receipt = issue_resources(&case(), &[resource()])
        .unwrap()
        .pop()
        .unwrap();
    let other_case = case();

    assert_eq!(
        attest_resources(
            &other_case,
            &[receipt.clone()],
            &[crate::AttestationRequest::used(
                receipt.id().clone(),
                "Used it"
            )],
        ),
        Err(ArsenalError::ReceiptCaseMismatch)
    );
}

#[test]
fn enforces_batch_and_usage_limits() {
    let _guard = fixture_lock();
    let case = case();
    assert_eq!(
        issue_resources(
            &case,
            &[resource(), resource(), resource(), resource(), resource()]
        ),
        Err(ArsenalError::ResourceBatchLimit)
    );

    let receipt = issue_resources(&case, &[resource()])
        .unwrap()
        .pop()
        .unwrap();
    assert_eq!(
        attest_resources(
            &case,
            &[receipt.clone()],
            &[crate::AttestationRequest::used(receipt.id().clone(), "   ")],
        ),
        Err(ArsenalError::AttestationEmpty)
    );

    let requests = (0..17)
        .map(|_| crate::AttestationRequest::used(receipt.id().clone(), "Used it"))
        .collect::<Vec<_>>();
    assert_eq!(
        attest_resources(&case, &[receipt], &requests),
        Err(ArsenalError::ResourceBatchLimit)
    );
}

#[test]
fn rejects_duplicate_and_replayed_receipts() {
    let _guard = fixture_lock();
    let case = case();
    let receipt = issue_resources(&case, &[resource()])
        .unwrap()
        .pop()
        .unwrap();
    let request = crate::AttestationRequest::used(receipt.id().clone(), "Used it");
    let duplicate = [request.clone(), request.clone()];
    let once = [request];

    assert_eq!(
        attest_resources(&case, &[receipt.clone()], &duplicate),
        Err(ArsenalError::ReceiptUnknown)
    );
    assert!(attest_resources(&case, &[receipt.clone()], &once).is_ok());
    assert_eq!(
        attest_resources(&case, &[receipt], &once),
        Err(ArsenalError::ReceiptUnknown)
    );
}

#[test]
fn rejects_issue_content_above_the_aggregate_limit() {
    let _guard = fixture_lock();
    let path = format!("{FIXTURE_ROOT}/validation.md");
    let original = fs::read_to_string(&path).unwrap();
    fs::write(&path, "x".repeat(70 * 1024)).unwrap();

    let mut resources = [resource(), resource(), resource(), resource()];
    for (index, resource) in resources.iter_mut().enumerate() {
        resource.resource_id = ResourceId::new(format!("resources::validation-{index}"));
    }
    let result = issue_resources(&case(), &resources);

    fs::write(path, original).unwrap();
    assert_eq!(result, Err(ArsenalError::ResourceLimitExceeded));
}

#[test]
fn detects_resource_and_skill_digest_drift_before_attestation() {
    let _guard = fixture_lock();
    let case = case();
    let receipt = issue_resources(&case, &[resource()])
        .unwrap()
        .pop()
        .unwrap();
    let resource_path = format!("{FIXTURE_ROOT}/validation.md");
    let original_resource = fs::read_to_string(&resource_path).unwrap();
    let replacement_path = format!("{resource_path}.replacement");
    fs::write(&replacement_path, "changed resource\n").unwrap();
    fs::rename(&replacement_path, &resource_path).unwrap();

    assert_eq!(
        attest_resources(
            &case,
            &[receipt.clone()],
            &[crate::AttestationRequest::used(
                receipt.id().clone(),
                "Used it"
            )],
        ),
        Err(ArsenalError::ReceiptStale)
    );
    fs::write(&resource_path, original_resource).unwrap();

    let skill_path = format!("{FIXTURE_ROOT}/SKILL.md");
    let original_skill = fs::read_to_string(&skill_path).unwrap();
    fs::write(&skill_path, "changed skill\n").unwrap();
    assert_eq!(
        attest_resources(
            &case,
            &[receipt.clone()],
            &[crate::AttestationRequest::used(
                receipt.id().clone(),
                "Used it"
            )],
        ),
        Err(ArsenalError::SkillDigestChanged)
    );
    fs::write(&skill_path, original_skill).unwrap();
}

#[test]
fn identifiers_are_uuidv7_and_digest_is_streamed_lowercase_hex() {
    let _guard = fixture_lock();
    let case = case();
    let receipt = issue_resources(&case, &[resource()])
        .unwrap()
        .pop()
        .unwrap();

    assert_eq!(case.id().as_uuid().get_version_num(), 7);
    assert_eq!(receipt.id().as_uuid().get_version_num(), 7);
    assert!(
        receipt.resource_digest()["sha256:".len()..]
            .chars()
            .all(|character| character.is_ascii_lowercase() || character.is_ascii_digit())
    );
}
