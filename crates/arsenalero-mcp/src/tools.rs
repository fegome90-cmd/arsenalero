use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Read,
    path::{Path, PathBuf},
    sync::Mutex,
    time::{SystemTime, UNIX_EPOCH},
};

use arsenalero_core::{
    ArsenalCase, ArsenalError, AttainedEvidenceLevel, AttestationRequest, CanonicalResourcePath,
    CanonicalSkillRoot, ClassificationInput, EvidenceContract, EvidenceReference, JournalEventType,
    JournalWriter, Obligation, PathPolicy, ReconciliationInput, ResourceId, ResourceIssueRequest,
    ResourceKind, ResourceReceipt, ResourceReconciliation, attest_resources, classify_resource,
    issue_resources, parse_resource_metadata, reconcile,
};
use ring::digest::{Context, SHA256};
use rmcp::model::CallToolResult;
use serde::Serialize;
use uuid::Uuid;

use crate::schema::*;

#[derive(Debug)]
pub(crate) struct ServerState {
    pub(crate) cases: Mutex<BTreeMap<Uuid, ActiveCase>>,
    pub(crate) policy: PathPolicy,
}

impl ServerState {
    pub(crate) fn empty() -> Self {
        Self {
            cases: Mutex::new(BTreeMap::new()),
            policy: PathPolicy::new(std::iter::empty::<PathBuf>())
                .expect("an empty root allowlist is valid"),
        }
    }

    pub(crate) fn with_allowed_roots<I, P>(allowed_roots: I) -> Result<Self, ArsenalError>
    where
        I: IntoIterator<Item = P>,
        P: AsRef<Path>,
    {
        Ok(Self {
            cases: Mutex::new(BTreeMap::new()),
            policy: PathPolicy::new(allowed_roots)?,
        })
    }

    pub(crate) fn from_environment() -> Result<Self, ArsenalError> {
        let allowed_roots = std::env::var_os("ARSENALERO_ALLOWED_ROOTS")
            .map(|value| std::env::split_paths(&value).collect::<Vec<_>>())
            .unwrap_or_default();
        Self::with_allowed_roots(allowed_roots)
    }
}

#[derive(Debug)]
pub(crate) struct ActiveCase {
    pub(crate) case: ArsenalCase,
    pub(crate) policy: PathPolicy,
    pub(crate) root: PathBuf,
    pub(crate) resources: BTreeMap<String, ResourceRecord>,
    pub(crate) receipts: Vec<ResourceReceipt>,
    pub(crate) attestations: BTreeMap<String, AttainedEvidenceLevel>,
    pub(crate) journal: JournalWriter,
}

#[derive(Debug)]
pub(crate) struct ResourceRecord {
    pub(crate) path: String,
    pub(crate) purpose: String,
    pub(crate) obligation: Obligation,
    pub(crate) evidence_contract: EvidenceContract,
    pub(crate) stages: Vec<String>,
}

pub(crate) fn init(state: &ServerState, input: InitInput) -> CallToolResult {
    let result = (|| -> Result<InitOutput, ArsenalError> {
        let root = PathBuf::from(&input.skill_root);
        let policy = state.policy.clone();
        let canonical = policy.canonical_skill_root(&root)?;
        let skill = resolve_skill_md(&policy, &canonical)?;
        let source = read_retained_file(&skill).map_err(|_| ArsenalError::SkillMdMissing)?;
        let document = arsenalero_core::scan_skill(&source)?;
        let referenced_paths = document
            .references
            .iter()
            .map(|reference| reference.path.clone())
            .collect::<BTreeSet<_>>();
        let orphan_files = find_orphan_files(&policy, &canonical, &referenced_paths)?;
        let case = ArsenalCase::open(skill.as_path())?;
        let mut resources = BTreeMap::new();
        for reference in document.references {
            let handle = policy.resolve_resource(&canonical, &reference.path)?;
            let contents = read_retained_file(&handle)?;
            let metadata = parse_resource_metadata(&contents)?;
            let id = metadata
                .as_ref()
                .and_then(|metadata| metadata.id.clone())
                .unwrap_or_else(|| {
                    reference
                        .path
                        .replace('/', "::")
                        .rsplit_once('.')
                        .map_or(reference.path.clone(), |(path, _)| path.to_owned())
                });
            if resources.contains_key(&id) {
                return Err(ArsenalError::ResourceIdCollision);
            }
            let classified = classify_resource(ClassificationInput {
                resource_id: ResourceId::new(id.clone()),
                metadata: metadata.clone(),
                heading: reference.heading,
                prose_context: reference.prose_context,
                list_context: reference.list_context,
            });
            let kind = metadata
                .as_ref()
                .and_then(|metadata| metadata.resource_kind)
                .unwrap_or(ResourceKind::Consultative);
            let evidence_contract = evidence_contract(metadata.as_ref(), kind);
            let purpose = metadata
                .as_ref()
                .and_then(|metadata| metadata.purpose.clone())
                .unwrap_or_else(|| format!("Referenced resource at {}", reference.path));
            let stages = metadata
                .as_ref()
                .map_or_else(Vec::new, |metadata| metadata.stages.clone());
            resources.insert(
                id,
                ResourceRecord {
                    path: reference.path,
                    purpose,
                    obligation: classified.obligation,
                    evidence_contract,
                    stages,
                },
            );
        }
        let id = case.id().as_uuid();
        let required_resource_ids = resources
            .iter()
            .filter(|(_, resource)| resource.obligation == Obligation::Required)
            .map(|(id, _)| id.clone())
            .collect();
        let unresolved = resources
            .iter()
            .filter(|(_, resource)| resource.obligation == Obligation::Unknown)
            .map(|(id, _)| id.clone())
            .collect();
        let output_resources = resources
            .iter()
            .map(|(id, resource)| resource_output(id, resource))
            .collect();
        let mut journal = JournalWriter::default();
        journal.append(
            JournalEventType::CaseInitialized,
            now(),
            format!("{}:{}", input.operation, input.task_summary),
        );
        state
            .cases
            .lock()
            .map_err(|_| ArsenalError::CaseUnknown)?
            .insert(
                id,
                ActiveCase {
                    case,
                    policy,
                    root: canonical.as_path().to_owned(),
                    resources,
                    receipts: Vec::new(),
                    attestations: BTreeMap::new(),
                    journal,
                },
            );
        Ok(InitOutput {
            case_id: id.to_string(),
            skill: SkillOutput {
                name: skill_name(&source),
                digest: state
                    .cases
                    .lock()
                    .map_err(|_| ArsenalError::CaseUnknown)?
                    .get(&id)
                    .expect("case inserted")
                    .case
                    .skill_digest()
                    .to_owned(),
            },
            required_resource_ids,
            resources: output_resources,
            unresolved,
            orphan_files,
            warnings: document
                .warnings
                .into_iter()
                .map(|warning| warning.candidate)
                .collect(),
            status: "ready".to_owned(),
        })
    })();
    result_response(result)
}

pub(crate) fn stage(state: &ServerState, input: StageInput) -> CallToolResult {
    let result = with_case(state, &input.case_id, |active| {
        let stage = normalize_stage(&input.stage).ok_or(ArsenalError::StageUnknown)?;
        active.journal.append(
            JournalEventType::StageEntered,
            now(),
            format!("{}:{}", stage, input.current_intent),
        );
        let matches_stage = |resource: &ResourceRecord| {
            resource.stages.is_empty()
                || resource.stages.iter().any(|resource_stage| {
                    normalize_stage(resource_stage).as_deref() == Some(&stage)
                })
        };
        let issued = |id: &str| {
            active
                .receipts
                .iter()
                .any(|receipt| receipt.resource().resource_id.as_str() == id)
        };
        Ok(StageOutput {
            required_now: active
                .resources
                .iter()
                .filter(|(id, resource)| {
                    resource.obligation == Obligation::Required
                        && matches_stage(resource)
                        && !issued(id)
                })
                .map(|(id, _)| id.clone())
                .collect(),
            recommended_now: active
                .resources
                .iter()
                .filter(|(id, resource)| {
                    resource.obligation == Obligation::Recommended
                        && matches_stage(resource)
                        && !issued(id)
                })
                .map(|(id, _)| id.clone())
                .collect(),
            already_issued: active
                .resources
                .keys()
                .filter(|id| issued(id))
                .cloned()
                .collect(),
            unresolved_relevant: active
                .resources
                .iter()
                .filter(|(_, resource)| {
                    resource.obligation == Obligation::Unknown && matches_stage(resource)
                })
                .map(|(id, _)| id.clone())
                .collect(),
        })
    });
    result_response(result)
}

pub(crate) fn issue(state: &ServerState, input: IssueInput) -> CallToolResult {
    let result = with_case(state, &input.case_id, |active| {
        if input.resource_ids.is_empty() || input.resource_ids.len() > 4 {
            return Err(ArsenalError::ResourceBatchLimit);
        }
        let root = active.policy.canonical_skill_root(&active.root)?;
        let requests = input
            .resource_ids
            .iter()
            .map(|id| {
                let resource = active
                    .resources
                    .get(id)
                    .ok_or(ArsenalError::ResourceUnknown)?;
                Ok(ResourceIssueRequest {
                    resource_id: ResourceId::new(id.clone()),
                    resource: active.policy.resolve_resource(&root, &resource.path)?,
                    purpose: resource.purpose.clone(),
                    evidence_contract: resource.evidence_contract.clone(),
                })
            })
            .collect::<Result<Vec<_>, ArsenalError>>()?;
        let receipts = issue_resources(&active.case, &requests)?;
        let output = receipts
            .iter()
            .map(|receipt| {
                active.journal.append(
                    JournalEventType::ResourceIssued,
                    now(),
                    receipt.id().as_uuid().to_string(),
                );
                IssuedResourceOutput {
                    receipt_id: receipt.id().as_uuid().to_string(),
                    resource_id: receipt.resource().resource_id.as_str().to_owned(),
                    digest: receipt.resource().digest.clone(),
                    purpose: receipt.resource().purpose.clone(),
                    content: receipt.resource().content.clone(),
                    evidence_contract: contract_output(receipt.resource().evidence_contract()),
                }
            })
            .collect();
        active.receipts.extend(receipts);
        Ok(IssueOutput { resources: output })
    });
    result_response(result)
}

pub(crate) fn attest(state: &ServerState, input: AttestInput) -> CallToolResult {
    let result = with_case(state, &input.case_id, |active| {
        let requests = input
            .attestations
            .iter()
            .map(|attestation| {
                let receipt_id = parse_receipt_uuid(&attestation.receipt_id)?;
                if attestation
                    .evidence
                    .iter()
                    .any(|evidence| evidence.kind != "artifact_reference")
                {
                    return Err(ArsenalError::EvidenceReferenceInvalid);
                }
                let receipt = active
                    .receipts
                    .iter()
                    .find(|receipt| receipt.id().as_uuid() == receipt_id)
                    .ok_or(ArsenalError::ReceiptUnknown)?;
                Ok(AttestationRequest {
                    receipt_id: receipt.id().clone(),
                    usage: attestation.usage.clone(),
                    evidence: attestation
                        .evidence
                        .iter()
                        .map(|evidence| EvidenceReference::artifact(evidence.reference.clone()))
                        .collect(),
                })
            })
            .collect::<Result<Vec<_>, ArsenalError>>()?;
        let values = attest_resources(&active.case, &active.receipts, &requests)?;
        let output = values
            .into_iter()
            .map(|attestation| {
                let receipt_id = attestation.receipt_id.as_uuid().to_string();
                active
                    .attestations
                    .insert(receipt_id.clone(), attestation.attained_evidence_level);
                active.journal.append(
                    JournalEventType::ResourceAttested,
                    now(),
                    receipt_id.clone(),
                );
                AttestationOutput {
                    receipt_id,
                    attained_evidence_level: evidence_level(attestation.attained_evidence_level)
                        .to_owned(),
                }
            })
            .collect();
        Ok(AttestOutput {
            attestations: output,
        })
    });
    result_response(result)
}

pub(crate) fn reconcile_case(state: &ServerState, input: ReconcileInput) -> CallToolResult {
    let result = with_case(state, &input.case_id, |active| {
        let root = active.policy.canonical_skill_root(&active.root)?;
        let skill_digest_changed =
            digest_resource(&active.policy, &root, "SKILL.md")? != active.case.skill_digest();
        let mut stale_receipts = Vec::new();
        let mut post_attestation_drift = Vec::new();
        if !skill_digest_changed {
            for receipt in &active.receipts {
                let resource = active
                    .resources
                    .get(receipt.resource().resource_id.as_str())
                    .ok_or(ArsenalError::ResourceUnknown)?;
                if digest_resource(&active.policy, &root, &resource.path)?
                    != receipt.resource_digest()
                    && !active
                        .attestations
                        .contains_key(&receipt.id().as_uuid().to_string())
                {
                    stale_receipts.push(receipt.resource().resource_id.clone());
                }
            }
        }
        let mut resources = Vec::with_capacity(active.resources.len());
        for (id, resource) in &active.resources {
            let receipt = active
                .receipts
                .iter()
                .find(|receipt| receipt.resource().resource_id.as_str() == id);
            let attained_evidence_level = receipt.and_then(|receipt| {
                active
                    .attestations
                    .get(&receipt.id().as_uuid().to_string())
                    .copied()
            });
            let mut modified_post_attestation = false;
            if !skill_digest_changed {
                let current_digest = digest_resource(&active.policy, &root, &resource.path)?;
                if let Some(receipt) = receipt
                    && current_digest != receipt.resource_digest()
                    && attained_evidence_level.is_some()
                {
                    modified_post_attestation = true;
                    post_attestation_drift.push(ResourceId::new(id.clone()));
                }
            }
            resources.push(ResourceReconciliation {
                resource_id: ResourceId::new(id.clone()),
                obligation: resource.obligation,
                issued: receipt.is_some(),
                attained_evidence_level,
                expects_artifact_reference: resource
                    .evidence_contract
                    .supported_levels
                    .contains(&AttainedEvidenceLevel::ArtifactReference),
                modified_post_attestation,
                unresolved: resource.obligation == Obligation::Unknown,
            });
        }
        for resource_id in &stale_receipts {
            active.journal.append(
                JournalEventType::ReceiptStale,
                now(),
                resource_id.as_str().to_owned(),
            );
        }
        for resource_id in &post_attestation_drift {
            active.journal.append(
                JournalEventType::ResourceModifiedPostAttestation,
                now(),
                resource_id.as_str().to_owned(),
            );
        }
        if skill_digest_changed {
            active
                .journal
                .append(JournalEventType::SkillDigestChanged, now(), "SKILL.md");
        }
        let report = reconcile(&ReconciliationInput {
            resources,
            stale_receipts,
            skill_digest_changed,
        })?;
        active.journal.append(
            JournalEventType::CaseReconciled,
            now(),
            status(report.status).to_owned(),
        );
        active.journal.close();
        if !active.journal.is_valid() {
            return Err(ArsenalError::ReconciliationInvariantViolation);
        }
        Ok(ReconcileOutput {
            status: status(report.status).to_owned(),
            protocol_completion: ProtocolCompletionOutput {
                required: report.protocol_completion.required,
                issued: report.protocol_completion.issued,
                attested: report.protocol_completion.attested,
                ratio: report.protocol_completion.ratio,
            },
            evidence_coverage: EvidenceCoverageOutput {
                expected_artifact_references: report.evidence_coverage.expected_artifact_references,
                artifact_referenced: report.evidence_coverage.artifact_referenced,
                ratio: report.evidence_coverage.ratio,
            },
            attestation_breakdown: AttestationBreakdownOutput {
                self_report_only: report.attestation_breakdown.self_report_only,
                artifact_referenced: report.attestation_breakdown.artifact_referenced,
                externally_verified: report.attestation_breakdown.externally_verified,
            },
            verification: VerificationOutput {
                status: report.verification.status.to_owned(),
                verified_resources: report.verification.verified_resources,
            },
            missing_attestations: ids(report.missing_attestations),
            required_but_never_issued: ids(report.required_but_never_issued),
            stale_receipts: ids(report.stale_receipts),
            resource_modifications_post_attestation: ids(
                report.resource_modifications_post_attestation
            ),
            unresolved_resources: ids(report.unresolved_resources),
            per_resource_evidence: report
                .per_resource_evidence
                .into_iter()
                .map(|item| PerResourceEvidenceOutput {
                    resource_id: item.resource_id.as_str().to_owned(),
                    attained_evidence_level: item
                        .attained_evidence_level
                        .map(evidence_level)
                        .map(str::to_owned),
                    verification_status: item.verification_status.to_owned(),
                })
                .collect(),
            disclaimer: report.disclaimer.to_owned(),
        })
    });
    result_response(result)
}

fn find_orphan_files(
    policy: &PathPolicy,
    root: &CanonicalSkillRoot,
    referenced_paths: &BTreeSet<String>,
) -> Result<Vec<String>, ArsenalError> {
    let mut candidates = Vec::new();
    for directory_name in ["resources", "references"] {
        let directory = root.as_path().join(directory_name);
        if fs::symlink_metadata(&directory)
            .map(|metadata| metadata.file_type().is_dir())
            .unwrap_or(false)
        {
            collect_resource_files(&directory, &mut candidates)?;
        }
    }

    let mut orphan_files = BTreeSet::new();
    for candidate in candidates {
        let relative = candidate
            .strip_prefix(root.as_path())
            .map_err(|_| ArsenalError::ResourcePathEscape)?;
        let relative = relative
            .to_str()
            .ok_or(ArsenalError::ResourceReferenceBroken)?;
        if referenced_paths.contains(relative) {
            continue;
        }
        if policy.resolve_resource(root, relative).is_ok() {
            orphan_files.insert(relative.to_owned());
        }
    }
    Ok(orphan_files.into_iter().collect())
}

fn collect_resource_files(directory: &Path, files: &mut Vec<PathBuf>) -> Result<(), ArsenalError> {
    for entry in fs::read_dir(directory).map_err(|_| ArsenalError::ResourceReferenceBroken)? {
        let entry = entry.map_err(|_| ArsenalError::ResourceReferenceBroken)?;
        let path = entry.path();
        let metadata =
            fs::symlink_metadata(&path).map_err(|_| ArsenalError::ResourceReferenceBroken)?;
        if metadata.file_type().is_dir() {
            collect_resource_files(&path, files)?;
        } else if metadata.file_type().is_file() {
            files.push(path);
        }
    }
    Ok(())
}

fn with_case<T>(
    state: &ServerState,
    case_id: &str,
    operation: impl FnOnce(&mut ActiveCase) -> Result<T, ArsenalError>,
) -> Result<T, ArsenalError> {
    let id = parse_case_uuid(case_id)?;
    let mut cases = state.cases.lock().map_err(|_| ArsenalError::CaseUnknown)?;
    operation(cases.get_mut(&id).ok_or(ArsenalError::CaseUnknown)?)
}
fn parse_case_uuid(value: &str) -> Result<Uuid, ArsenalError> {
    Uuid::parse_str(value).map_err(|_| ArsenalError::CaseUnknown)
}
fn parse_receipt_uuid(value: &str) -> Result<Uuid, ArsenalError> {
    Uuid::parse_str(value).map_err(|_| ArsenalError::ReceiptUnknown)
}
fn result_response<T: Serialize>(result: Result<T, ArsenalError>) -> CallToolResult {
    match result {
        Ok(value) => CallToolResult::structured(
            serde_json::to_value(value).expect("tool output is serializable"),
        ),
        Err(error) => CallToolResult::structured_error(
            serde_json::json!({"error_code": error.code(), "message": "Arsenalero domain operation failed."}),
        ),
    }
}
fn evidence_contract(
    metadata: Option<&arsenalero_core::ArsenalMetadata>,
    kind: ResourceKind,
) -> EvidenceContract {
    if let Some(contract) = metadata.and_then(|metadata| metadata.evidence_contract.clone()) {
        return EvidenceContract {
            minimum: contract
                .minimum
                .unwrap_or(AttainedEvidenceLevel::Attestation),
            supported_levels: if contract.supported.is_empty() {
                vec![AttainedEvidenceLevel::Attestation]
            } else {
                contract.supported
            },
        };
    }
    match kind {
        ResourceKind::Consultative => EvidenceContract {
            minimum: AttainedEvidenceLevel::Attestation,
            supported_levels: vec![AttainedEvidenceLevel::Attestation],
        },
        ResourceKind::Procedure | ResourceKind::VerifiableContract => EvidenceContract {
            minimum: AttainedEvidenceLevel::Attestation,
            supported_levels: vec![
                AttainedEvidenceLevel::Attestation,
                AttainedEvidenceLevel::ArtifactReference,
            ],
        },
    }
}
fn resource_output(id: &str, resource: &ResourceRecord) -> ResourceOutput {
    ResourceOutput {
        resource_id: id.to_owned(),
        path: resource.path.clone(),
        purpose: resource.purpose.clone(),
        obligation: obligation(resource.obligation).to_owned(),
        evidence_contract: contract_output(&resource.evidence_contract),
    }
}
fn contract_output(contract: &EvidenceContract) -> EvidenceContractOutput {
    EvidenceContractOutput {
        minimum: evidence_level(contract.minimum).to_owned(),
        supported_levels: contract
            .supported_levels
            .iter()
            .map(|level| evidence_level(*level).to_owned())
            .collect(),
    }
}
fn evidence_level(level: AttainedEvidenceLevel) -> &'static str {
    match level {
        AttainedEvidenceLevel::Attestation => "attestation",
        AttainedEvidenceLevel::ArtifactReference => "artifact_reference",
    }
}
fn obligation(value: Obligation) -> &'static str {
    match value {
        Obligation::Required => "required",
        Obligation::Recommended => "recommended",
        Obligation::Optional => "optional",
        Obligation::Unknown => "unknown",
    }
}
fn status(value: arsenalero_core::ReconciliationStatus) -> &'static str {
    match value {
        arsenalero_core::ReconciliationStatus::Complete => "complete",
        arsenalero_core::ReconciliationStatus::Incomplete => "incomplete",
        arsenalero_core::ReconciliationStatus::NeedsReview => "needs_review",
        arsenalero_core::ReconciliationStatus::Invalidated => "invalidated",
    }
}
fn ids(values: Vec<ResourceId>) -> Vec<String> {
    values
        .into_iter()
        .map(|value| value.as_str().to_owned())
        .collect()
}
fn skill_name(source: &str) -> String {
    source
        .lines()
        .find_map(|line| {
            line.strip_prefix("name:")
                .map(|name| name.trim().to_owned())
        })
        .unwrap_or_else(|| "skill".to_owned())
}

const STAGE_ALIASES: &[&str] = &[
    "exploration",
    "discovery",
    "research",
    "context",
    "exploración",
    "descubrimiento",
    "investigación",
    "contexto",
    "implementation",
    "build",
    "coding",
    "desarrollo",
    "implementación",
    "construcción",
    "código",
    "testing",
    "tests",
    "test",
    "pruebas",
    "prueba",
    "verification",
    "validation",
    "review",
    "verificación",
    "validación",
    "revisión",
    "recovery",
    "rollback",
    "error handling",
    "recuperación",
    "reversión",
    "manejo de errores",
    "deployment",
    "release",
    "promotion",
    "despliegue",
    "liberación",
    "promoción",
];

fn normalize_stage(value: &str) -> Option<String> {
    let normalized = value
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .chars()
        .flat_map(char::to_lowercase)
        .collect::<String>();
    STAGE_ALIASES
        .contains(&normalized.as_str())
        .then_some(normalized)
}

fn digest_resource(
    policy: &PathPolicy,
    root: &CanonicalSkillRoot,
    relative_path: impl AsRef<Path>,
) -> Result<String, ArsenalError> {
    let resource = policy.resolve_resource(root, relative_path)?;
    let mut file = resource
        .try_clone_file()
        .map_err(|_| ArsenalError::ResourceReferenceBroken)?;
    digest_reader(&mut file).map_err(|_| ArsenalError::ResourceReferenceBroken)
}
fn resolve_skill_md(
    policy: &PathPolicy,
    root: &CanonicalSkillRoot,
) -> Result<CanonicalResourcePath, ArsenalError> {
    policy
        .resolve_resource(root, "SKILL.md")
        .map_err(|error| match error {
            ArsenalError::ResourceReferenceBroken | ArsenalError::ResourceTypeUnsupported => {
                ArsenalError::SkillMdMissing
            }
            ArsenalError::ResourceTooLarge => ArsenalError::SkillMdTooLarge,
            error => error,
        })
}

fn read_retained_file(resource: &CanonicalResourcePath) -> Result<String, ArsenalError> {
    let mut file = resource
        .try_clone_file()
        .map_err(|_| ArsenalError::ResourceReferenceBroken)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|_| ArsenalError::ResourceReferenceBroken)?;
    Ok(contents)
}
fn digest_reader(reader: &mut impl Read) -> std::io::Result<String> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0_u8; 8192];
    loop {
        let read = reader.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        context.update(&buffer[..read]);
    }
    Ok(format!(
        "sha256:{}",
        context
            .finish()
            .as_ref()
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>()
    ))
}
fn now() -> String {
    SystemTime::now().duration_since(UNIX_EPOCH).map_or_else(
        |_| "0".to_owned(),
        |duration| duration.as_secs().to_string(),
    )
}
#[cfg(test)]
mod tests {
    use std::{
        fs,
        sync::atomic::{AtomicUsize, Ordering},
    };

    use super::{ServerState, attest, init, issue, reconcile_case, stage};
    use crate::schema::{
        AttestInput, AttestationInput, InitInput, IssueInput, ReconcileInput, StageInput,
    };
    use arsenalero_core::JournalEventType;

    static NEXT_TEMP_DIR: AtomicUsize = AtomicUsize::new(0);

    #[test]
    fn init_reports_unreferenced_supported_resource_files() {
        let root = std::env::temp_dir().join(format!(
            "arsenalero-mcp-task10-orphan-{}-{}",
            std::process::id(),
            NEXT_TEMP_DIR.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(root.join("resources")).expect("resource directory");
        fs::write(
            root.join("SKILL.md"),
            "# demo

Use [the guide](resources/guide.md).
",
        )
        .expect("skill document");
        fs::write(root.join("resources/guide.md"), "guide").expect("referenced resource");
        fs::write(root.join("resources/orphan.md"), "orphan").expect("orphan resource");

        let state = ServerState::with_allowed_roots([root.clone()]).expect("configured root");
        let result = init(
            &state,
            InitInput {
                skill_root: root.to_string_lossy().into_owned(),
                task_summary: "task".to_owned(),
                operation: "implementation".to_owned(),
            },
        );

        assert_eq!(
            result
                .structured_content
                .as_ref()
                .and_then(|value| value.get("orphan_files")),
            Some(&serde_json::json!(["resources/orphan.md"]))
        );
        fs::remove_dir_all(root).expect("temporary fixture cleanup");
    }
    #[test]
    fn reconcile_reports_receipts_stale_after_resource_drift() {
        let root = std::env::temp_dir().join(format!(
            "arsenalero-mcp-task10-stale-{}-{}",
            std::process::id(),
            NEXT_TEMP_DIR.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(root.join("resources")).expect("resource directory");
        fs::write(
            root.join("SKILL.md"),
            "# demo

Use [the guide](resources/guide.md).
",
        )
        .expect("skill document");
        fs::write(
            root.join("resources/guide.md"),
            "---
arsenal:
  requirement: required
---
original
",
        )
        .expect("resource");

        let state = ServerState::with_allowed_roots([root.clone()]).expect("configured root");
        let init_result = init(
            &state,
            InitInput {
                skill_root: root.to_string_lossy().into_owned(),
                task_summary: "task".to_owned(),
                operation: "implementation".to_owned(),
            },
        );
        let case_id = init_result
            .structured_content
            .as_ref()
            .and_then(|value| value.get("case_id"))
            .and_then(serde_json::Value::as_str)
            .expect("case id")
            .to_owned();

        let issue_result = issue(
            &state,
            IssueInput {
                case_id: case_id.clone(),
                resource_ids: vec!["resources::guide".to_owned()],
            },
        );
        assert_eq!(issue_result.is_error, Some(false));

        fs::write(
            root.join("resources/guide.md"),
            "changed
",
        )
        .expect("resource drift");
        let reconcile_result = reconcile_case(&state, ReconcileInput { case_id });

        assert_eq!(
            reconcile_result
                .structured_content
                .as_ref()
                .and_then(|value| value.get("stale_receipts")),
            Some(&serde_json::json!(["resources::guide"]))
        );
        fs::remove_dir_all(root).expect("temporary fixture cleanup");
    }
    #[test]
    fn reconcile_separates_post_attestation_drift_and_skill_invalidation() {
        let root = fixture_root("post-attestation-drift");
        let state = state_for_root(&root);
        let case_id = init_case(&state, &root);
        issue_and_attest(&state, &case_id);
        fs::write(root.join("resources/guide.md"), "changed\n").expect("resource drift");
        let output = reconcile_output(&state, &case_id);
        assert_eq!(output["status"], serde_json::json!("needs_review"));
        assert_eq!(output["stale_receipts"], serde_json::json!([]));
        assert_eq!(
            output["resource_modifications_post_attestation"],
            serde_json::json!(["resources::guide"]),
        );
        assert!(journal_has(
            &state,
            &case_id,
            JournalEventType::ResourceModifiedPostAttestation
        ));
        fs::write(
            root.join("SKILL.md"),
            "# changed\n\nUse [the guide](resources/guide.md).\n",
        )
        .expect("skill drift");
        fs::remove_file(root.join("resources/guide.md")).expect("broken resource");
        let output = reconcile_output(&state, &case_id);
        assert_eq!(output["status"], serde_json::json!("invalidated"));
        assert!(journal_has(
            &state,
            &case_id,
            JournalEventType::SkillDigestChanged
        ));
        cleanup(root);
    }

    #[test]
    fn rejects_unknown_stage_aliases() {
        let root = fixture_root("stage");
        let state = state_for_root(&root);
        let case_id = init_case(&state, &root);

        let result = stage(
            &state,
            StageInput {
                case_id,
                stage: "not-a-stage".to_owned(),
                current_intent: "test".to_owned(),
            },
        );

        assert_domain_error(result, "STAGE_UNKNOWN");
        cleanup(root);
    }

    #[test]
    fn rejects_empty_issue_batches() {
        let root = fixture_root("empty-issue");
        let state = state_for_root(&root);
        let case_id = init_case(&state, &root);

        let result = issue(
            &state,
            IssueInput {
                case_id,
                resource_ids: Vec::new(),
            },
        );

        assert_domain_error(result, "RESOURCE_BATCH_LIMIT");
        cleanup(root);
    }

    #[test]
    fn rejects_malformed_receipt_uuid_as_receipt_unknown() {
        let root = fixture_root("malformed-receipt");
        let state = state_for_root(&root);
        let case_id = init_case(&state, &root);

        let result = attest(
            &state,
            AttestInput {
                case_id,
                attestations: vec![AttestationInput {
                    receipt_id: "not-a-uuid".to_owned(),
                    usage: "used".to_owned(),
                    evidence: Vec::new(),
                }],
            },
        );

        assert_domain_error(result, "RECEIPT_UNKNOWN");
        cleanup(root);
    }

    #[cfg(unix)]
    #[test]
    fn reconcile_revalidates_resource_path_policy_after_issue() {
        use std::os::unix::fs::symlink;

        let root = fixture_root("reconcile-path-policy");
        let outside = std::env::temp_dir().join(format!(
            "arsenalero-mcp-task10-outside-{}-{}",
            std::process::id(),
            NEXT_TEMP_DIR.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&outside).expect("outside directory");
        fs::write(outside.join("outside.md"), "outside").expect("outside resource");

        let state = state_for_root(&root);
        let case_id = init_case(&state, &root);
        let result = issue(
            &state,
            IssueInput {
                case_id: case_id.clone(),
                resource_ids: vec!["resources::guide".to_owned()],
            },
        );
        assert_eq!(result.is_error, Some(false));

        fs::remove_file(root.join("resources/guide.md")).expect("original resource");
        symlink(outside.join("outside.md"), root.join("resources/guide.md"))
            .expect("escaping replacement symlink");

        let result = reconcile_case(&state, ReconcileInput { case_id });
        assert_domain_error(result, "RESOURCE_SYMLINK_ESCAPE");
        cleanup(root);
        cleanup(outside);
    }

    #[test]
    fn rejects_skill_root_without_external_allowlist_entry() {
        let root = fixture_root("root-policy");
        let state = empty_state();

        let result = init(
            &state,
            InitInput {
                skill_root: root.to_string_lossy().into_owned(),
                task_summary: "task".to_owned(),
                operation: "implementation".to_owned(),
            },
        );

        assert_domain_error(result, "SKILL_ROOT_NOT_ALLOWED");
        cleanup(root);
    }

    fn empty_state() -> ServerState {
        ServerState::empty()
    }

    fn state_for_root(root: &std::path::Path) -> ServerState {
        ServerState::with_allowed_roots([root]).expect("configured root")
    }

    fn fixture_root(name: &str) -> std::path::PathBuf {
        let root = std::env::temp_dir().join(format!(
            "arsenalero-mcp-task10-{name}-{}-{}",
            std::process::id(),
            NEXT_TEMP_DIR.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(root.join("resources")).expect("resource directory");
        fs::write(
            root.join("SKILL.md"),
            "# demo\n\nUse [the guide](resources/guide.md).\n",
        )
        .expect("skill document");
        fs::write(
            root.join("resources/guide.md"),
            "---\narsenal:\n  requirement: required\n---\noriginal\n",
        )
        .expect("resource");
        root
    }

    fn init_case(state: &ServerState, root: &std::path::Path) -> String {
        let result = init(
            state,
            InitInput {
                skill_root: root.to_string_lossy().into_owned(),
                task_summary: "task".to_owned(),
                operation: "implementation".to_owned(),
            },
        );
        result
            .structured_content
            .as_ref()
            .and_then(|value| value.get("case_id"))
            .and_then(serde_json::Value::as_str)
            .expect("case id")
            .to_owned()
    }

    fn issue_and_attest(state: &ServerState, case_id: &str) {
        let result = issue(
            state,
            IssueInput {
                case_id: case_id.to_owned(),
                resource_ids: vec!["resources::guide".to_owned()],
            },
        );
        let receipt_id =
            result.structured_content.expect("issue output")["resources"][0]["receipt_id"]
                .as_str()
                .expect("receipt id")
                .to_owned();
        assert_eq!(
            attest(
                state,
                AttestInput {
                    case_id: case_id.to_owned(),
                    attestations: vec![AttestationInput {
                        receipt_id,
                        usage: "used".to_owned(),
                        evidence: Vec::new(),
                    }],
                },
            )
            .is_error,
            Some(false)
        );
    }
    fn assert_domain_error(result: rmcp::model::CallToolResult, expected: &str) {
        assert_eq!(result.is_error, Some(true));
        assert_eq!(
            result
                .structured_content
                .as_ref()
                .and_then(|value| value.get("error_code"))
                .and_then(serde_json::Value::as_str),
            Some(expected)
        );
    }

    fn reconcile_output(state: &ServerState, case_id: &str) -> serde_json::Value {
        let result = reconcile_case(
            state,
            ReconcileInput {
                case_id: case_id.to_owned(),
            },
        );
        assert_eq!(result.is_error, Some(false));
        result.structured_content.expect("reconcile output")
    }
    fn journal_has(state: &ServerState, case_id: &str, expected: JournalEventType) -> bool {
        let id = uuid::Uuid::parse_str(case_id).expect("case id should be a UUID");
        let cases = state.cases.lock().expect("case lock");
        let journal = &cases.get(&id).expect("case should exist").journal;
        journal
            .events()
            .iter()
            .any(|event| event.event_type() == expected)
    }

    fn cleanup(path: std::path::PathBuf) {
        fs::remove_dir_all(path).expect("temporary fixture cleanup");
    }
}
