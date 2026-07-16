use crate::{ArsenalError, AttainedEvidenceLevel, Obligation, ReconciliationStatus, ResourceId};

pub const NOT_SUPPORTED_IN_V1: &str = "not_supported_in_v1";
pub const RECONCILIATION_DISCLAIMER: &str = "Protocol completion records issue and attestation events. Artifact references are agent-supplied attributions and are not externally verified in Arsenalero v1.";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResourceReconciliation {
    pub resource_id: ResourceId,
    pub obligation: Obligation,
    pub issued: bool,
    pub attained_evidence_level: Option<AttainedEvidenceLevel>,
    pub expects_artifact_reference: bool,
    pub modified_post_attestation: bool,
    pub unresolved: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReconciliationInput {
    pub resources: Vec<ResourceReconciliation>,
    pub stale_receipts: Vec<ResourceId>,
    pub skill_digest_changed: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProtocolCompletion {
    pub required: usize,
    pub issued: usize,
    pub attested: usize,
    pub ratio: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EvidenceCoverage {
    pub expected_artifact_references: usize,
    pub artifact_referenced: usize,
    pub ratio: f64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AttestationBreakdown {
    pub self_report_only: usize,
    pub artifact_referenced: usize,
    pub externally_verified: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Verification {
    pub status: &'static str,
    pub verified_resources: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PerResourceEvidence {
    pub resource_id: ResourceId,
    pub attained_evidence_level: Option<AttainedEvidenceLevel>,
    pub verification_status: &'static str,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReconciliationReport {
    pub status: ReconciliationStatus,
    pub protocol_completion: ProtocolCompletion,
    pub evidence_coverage: EvidenceCoverage,
    pub attestation_breakdown: AttestationBreakdown,
    pub verification: Verification,
    pub missing_attestations: Vec<ResourceId>,
    pub required_but_never_issued: Vec<ResourceId>,
    pub stale_receipts: Vec<ResourceId>,
    pub resource_modifications_post_attestation: Vec<ResourceId>,
    pub unresolved_resources: Vec<ResourceId>,
    pub per_resource_evidence: Vec<PerResourceEvidence>,
    pub disclaimer: &'static str,
}

/// Reconciles supplied in-memory facts without inferring evidence or external verification.
pub fn reconcile(input: &ReconciliationInput) -> Result<ReconciliationReport, ArsenalError> {
    let required = input
        .resources
        .iter()
        .filter(|resource| resource.obligation == Obligation::Required)
        .collect::<Vec<_>>();
    let required_but_never_issued = required
        .iter()
        .filter(|resource| !resource.issued)
        .map(|resource| resource.resource_id.clone())
        .collect::<Vec<_>>();
    let missing_attestations = required
        .iter()
        .filter(|resource| resource.issued && resource.attained_evidence_level.is_none())
        .map(|resource| resource.resource_id.clone())
        .collect::<Vec<_>>();
    let resource_modifications_post_attestation = input
        .resources
        .iter()
        .filter(|resource| resource.modified_post_attestation)
        .map(|resource| resource.resource_id.clone())
        .collect::<Vec<_>>();
    let unresolved_resources = input
        .resources
        .iter()
        .filter(|resource| resource.unresolved || resource.obligation == Obligation::Unknown)
        .map(|resource| resource.resource_id.clone())
        .collect::<Vec<_>>();
    let issued = required.iter().filter(|resource| resource.issued).count();
    let attested_required = required
        .iter()
        .filter_map(|resource| resource.attained_evidence_level)
        .collect::<Vec<_>>();
    let self_report_only = attested_required
        .iter()
        .filter(|level| **level == AttainedEvidenceLevel::Attestation)
        .count();
    let artifact_referenced = attested_required
        .iter()
        .filter(|level| **level == AttainedEvidenceLevel::ArtifactReference)
        .count();
    let expected_artifact_references = required
        .iter()
        .filter(|resource| resource.expects_artifact_reference)
        .count();
    let evidence_artifact_referenced = artifact_referenced;
    let status = if input.skill_digest_changed {
        ReconciliationStatus::Invalidated
    } else if !resource_modifications_post_attestation.is_empty()
        || !input.stale_receipts.is_empty()
        || !unresolved_resources.is_empty()
    {
        ReconciliationStatus::NeedsReview
    } else if !required_but_never_issued.is_empty() || !missing_attestations.is_empty() {
        ReconciliationStatus::Incomplete
    } else {
        ReconciliationStatus::Complete
    };
    let report = ReconciliationReport {
        status,
        protocol_completion: ProtocolCompletion {
            required: required.len(),
            issued,
            attested: attested_required.len(),
            ratio: ratio(attested_required.len(), required.len()),
        },
        evidence_coverage: EvidenceCoverage {
            expected_artifact_references,
            artifact_referenced: evidence_artifact_referenced,
            ratio: ratio(evidence_artifact_referenced, expected_artifact_references),
        },
        attestation_breakdown: AttestationBreakdown {
            self_report_only,
            artifact_referenced,
            externally_verified: 0,
        },
        verification: Verification {
            status: NOT_SUPPORTED_IN_V1,
            verified_resources: 0,
        },
        missing_attestations,
        required_but_never_issued,
        stale_receipts: input.stale_receipts.clone(),
        resource_modifications_post_attestation,
        unresolved_resources,
        per_resource_evidence: input
            .resources
            .iter()
            .map(|resource| PerResourceEvidence {
                resource_id: resource.resource_id.clone(),
                attained_evidence_level: resource.attained_evidence_level,
                verification_status: NOT_SUPPORTED_IN_V1,
            })
            .collect(),
        disclaimer: RECONCILIATION_DISCLAIMER,
    };
    validate_reconciliation_invariants(&report)?;
    Ok(report)
}

pub fn validate_reconciliation_invariants(
    report: &ReconciliationReport,
) -> Result<(), ArsenalError> {
    let breakdown = &report.attestation_breakdown;
    if breakdown.self_report_only + breakdown.artifact_referenced
        != report.protocol_completion.attested
        || report.protocol_completion.attested > report.protocol_completion.issued
        || report.protocol_completion.issued > report.protocol_completion.required
        || breakdown.externally_verified > breakdown.artifact_referenced
        || report.evidence_coverage.artifact_referenced != breakdown.artifact_referenced
        || report.evidence_coverage.artifact_referenced
            > report.evidence_coverage.expected_artifact_references
        || !report.protocol_completion.ratio.is_finite()
        || !report.evidence_coverage.ratio.is_finite()
        || report.protocol_completion.ratio
            != ratio(
                report.protocol_completion.attested,
                report.protocol_completion.required,
            )
        || report.evidence_coverage.ratio
            != ratio(
                report.evidence_coverage.artifact_referenced,
                report.evidence_coverage.expected_artifact_references,
            )
        || !status_is_possible(report)
    {
        return Err(ArsenalError::ReconciliationInvariantViolation);
    }
    Ok(())
}

fn ratio(numerator: usize, denominator: usize) -> f64 {
    if denominator == 0 {
        if numerator == 0 { 1.0 } else { 0.0 }
    } else {
        numerator as f64 / denominator as f64
    }
}

fn status_is_possible(report: &ReconciliationReport) -> bool {
    let completion = &report.protocol_completion;
    let has_review_blocker = !report.stale_receipts.is_empty()
        || !report.resource_modifications_post_attestation.is_empty()
        || !report.unresolved_resources.is_empty();
    let is_incomplete =
        completion.issued < completion.required || completion.attested < completion.issued;

    match report.status {
        ReconciliationStatus::Invalidated => true,
        ReconciliationStatus::Complete => !has_review_blocker && !is_incomplete,
        ReconciliationStatus::Incomplete => !has_review_blocker && is_incomplete,
        ReconciliationStatus::NeedsReview => has_review_blocker,
    }
}
