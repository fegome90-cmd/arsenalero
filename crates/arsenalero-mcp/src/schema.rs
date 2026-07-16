use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct InitInput {
    pub skill_root: String,
    pub task_summary: String,
    pub operation: String,
}

#[derive(Clone, Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct StageInput {
    pub case_id: String,
    pub stage: String,
    pub current_intent: String,
}

#[derive(Clone, Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct IssueInput {
    pub case_id: String,
    #[schemars(length(min = 1, max = 4))]
    pub resource_ids: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AttestInput {
    pub case_id: String,
    #[schemars(length(max = 16))]
    pub attestations: Vec<AttestationInput>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AttestationInput {
    pub receipt_id: String,
    #[schemars(length(min = 1))]
    pub usage: String,
    #[serde(default)]
    pub evidence: Vec<EvidenceInput>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct EvidenceInput {
    #[serde(rename = "type")]
    pub kind: String,
    #[schemars(length(min = 1))]
    pub reference: String,
}

#[derive(Clone, Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ReconcileInput {
    pub case_id: String,
}

#[derive(Clone, Debug, Serialize, JsonSchema)]
pub struct InitOutput {
    pub case_id: String,
    pub skill: SkillOutput,
    pub required_resource_ids: Vec<String>,
    pub resources: Vec<ResourceOutput>,
    pub unresolved: Vec<String>,
    pub orphan_files: Vec<String>,
    pub warnings: Vec<String>,
    pub status: String,
}

#[derive(Clone, Debug, Serialize, JsonSchema)]
pub struct SkillOutput {
    pub name: String,
    pub digest: String,
}

#[derive(Clone, Debug, Serialize, JsonSchema)]
pub struct ResourceOutput {
    pub resource_id: String,
    pub path: String,
    pub purpose: String,
    pub obligation: String,
    pub evidence_contract: EvidenceContractOutput,
}

#[derive(Clone, Debug, Serialize, JsonSchema)]
pub struct EvidenceContractOutput {
    pub minimum: String,
    pub supported_levels: Vec<String>,
}

#[derive(Clone, Debug, Serialize, JsonSchema)]
pub struct StageOutput {
    pub required_now: Vec<String>,
    pub recommended_now: Vec<String>,
    pub already_issued: Vec<String>,
    pub unresolved_relevant: Vec<String>,
}

#[derive(Clone, Debug, Serialize, JsonSchema)]
pub struct IssueOutput {
    pub resources: Vec<IssuedResourceOutput>,
}

#[derive(Clone, Debug, Serialize, JsonSchema)]
pub struct IssuedResourceOutput {
    pub receipt_id: String,
    pub resource_id: String,
    pub digest: String,
    pub purpose: String,
    pub content: String,
    pub evidence_contract: EvidenceContractOutput,
}

#[derive(Clone, Debug, Serialize, JsonSchema)]
pub struct AttestOutput {
    pub attestations: Vec<AttestationOutput>,
}

#[derive(Clone, Debug, Serialize, JsonSchema)]
pub struct AttestationOutput {
    pub receipt_id: String,
    pub attained_evidence_level: String,
}

#[derive(Clone, Debug, Serialize, JsonSchema)]
pub struct ReconcileOutput {
    pub status: String,
    pub protocol_completion: ProtocolCompletionOutput,
    pub evidence_coverage: EvidenceCoverageOutput,
    pub attestation_breakdown: AttestationBreakdownOutput,
    pub verification: VerificationOutput,
    pub missing_attestations: Vec<String>,
    pub required_but_never_issued: Vec<String>,
    pub stale_receipts: Vec<String>,
    pub resource_modifications_post_attestation: Vec<String>,
    pub unresolved_resources: Vec<String>,
    pub per_resource_evidence: Vec<PerResourceEvidenceOutput>,
    pub disclaimer: String,
}

#[derive(Clone, Debug, Serialize, JsonSchema)]
pub struct ProtocolCompletionOutput {
    pub required: usize,
    pub issued: usize,
    pub attested: usize,
    pub ratio: f64,
}
#[derive(Clone, Debug, Serialize, JsonSchema)]
pub struct EvidenceCoverageOutput {
    pub expected_artifact_references: usize,
    pub artifact_referenced: usize,
    pub ratio: f64,
}
#[derive(Clone, Debug, Serialize, JsonSchema)]
pub struct AttestationBreakdownOutput {
    pub self_report_only: usize,
    pub artifact_referenced: usize,
    pub externally_verified: usize,
}
#[derive(Clone, Debug, Serialize, JsonSchema)]
pub struct VerificationOutput {
    pub status: String,
    pub verified_resources: usize,
}
#[derive(Clone, Debug, Serialize, JsonSchema)]
pub struct PerResourceEvidenceOutput {
    pub resource_id: String,
    pub attained_evidence_level: Option<String>,
    pub verification_status: String,
}
