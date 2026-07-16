use crate::{
    ArsenalError, AttainedEvidenceLevel, JournalEventType, JournalWriter, Obligation,
    ReconciliationInput, ReconciliationStatus, ResourceId, ResourceReconciliation, reconcile,
    validate_reconciliation_invariants,
};

fn resource(id: &str, obligation: Obligation) -> ResourceReconciliation {
    ResourceReconciliation {
        resource_id: ResourceId::new(id),
        obligation,
        issued: false,
        attained_evidence_level: None,
        expects_artifact_reference: false,
        modified_post_attestation: false,
        unresolved: false,
    }
}

#[test]
fn journal_hash_links_append_only_event_order() {
    let mut journal = JournalWriter::default();
    journal.append(
        JournalEventType::CaseInitialized,
        "2026-07-15T00:00:00Z",
        "{}",
    );
    journal.append(
        JournalEventType::ResourceIssued,
        "2026-07-15T00:00:01Z",
        "{resource}",
    );
    assert_eq!(journal.events()[0].sequence(), 1);
    assert_eq!(
        journal.events()[1].previous_digest(),
        Some(journal.events()[0].event_digest())
    );
    journal.close();
    assert!(journal.is_valid());
}

#[test]
fn reconciliation_is_complete_only_after_required_issue_and_attestation() {
    let mut required = resource("resources::required", Obligation::Required);
    required.issued = true;
    required.attained_evidence_level = Some(AttainedEvidenceLevel::Attestation);
    let report = reconcile(&ReconciliationInput {
        resources: vec![required],
        stale_receipts: vec![],
        skill_digest_changed: false,
    })
    .unwrap();
    assert_eq!(report.status, ReconciliationStatus::Complete);
    assert_eq!(report.verification.status, "not_supported_in_v1");
    assert_eq!(report.attestation_breakdown.self_report_only, 1);
}

#[test]
fn reconciliation_fails_closed_for_missing_issue_attestation_drift_and_unresolved() {
    let missing_issue = reconcile(&ReconciliationInput {
        resources: vec![resource("resources::missing-issue", Obligation::Required)],
        stale_receipts: vec![],
        skill_digest_changed: false,
    })
    .unwrap();
    assert_eq!(missing_issue.status, ReconciliationStatus::Incomplete);
    let mut missing_attestation = resource("resources::missing-attestation", Obligation::Required);
    missing_attestation.issued = true;
    assert_eq!(
        reconcile(&ReconciliationInput {
            resources: vec![missing_attestation],
            stale_receipts: vec![],
            skill_digest_changed: false
        })
        .unwrap()
        .status,
        ReconciliationStatus::Incomplete
    );
    let mut changed = resource("resources::changed", Obligation::Required);
    changed.modified_post_attestation = true;
    assert_eq!(
        reconcile(&ReconciliationInput {
            resources: vec![changed],
            stale_receipts: vec![],
            skill_digest_changed: false
        })
        .unwrap()
        .status,
        ReconciliationStatus::NeedsReview
    );
    assert_eq!(
        reconcile(&ReconciliationInput {
            resources: vec![resource("resources::unknown", Obligation::Unknown)],
            stale_receipts: vec![],
            skill_digest_changed: false
        })
        .unwrap()
        .status,
        ReconciliationStatus::NeedsReview
    );
    assert_eq!(
        reconcile(&ReconciliationInput {
            resources: vec![],
            stale_receipts: vec![],
            skill_digest_changed: true
        })
        .unwrap()
        .status,
        ReconciliationStatus::Invalidated
    );
}

#[test]
fn reconciliation_invariants_and_zero_ratios_are_safe() {
    let report = reconcile(&ReconciliationInput {
        resources: vec![],
        stale_receipts: vec![],
        skill_digest_changed: false,
    })
    .unwrap();
    assert_eq!(report.protocol_completion.ratio, 1.0);
    assert_eq!(report.evidence_coverage.ratio, 1.0);
    let mut invalid = report.clone();
    invalid.protocol_completion.attested = 1;
    assert_eq!(
        validate_reconciliation_invariants(&invalid),
        Err(ArsenalError::ReconciliationInvariantViolation)
    );
}

#[test]
fn reconciliation_keeps_artifact_coverage_equal_to_attestation_breakdown() {
    let mut required = resource("resources::artifact", Obligation::Required);
    required.issued = true;
    required.expects_artifact_reference = true;
    required.attained_evidence_level = Some(AttainedEvidenceLevel::ArtifactReference);

    let report = reconcile(&ReconciliationInput {
        resources: vec![required],
        stale_receipts: vec![],
        skill_digest_changed: false,
    })
    .unwrap();

    assert_eq!(report.evidence_coverage.artifact_referenced, 1);
    assert_eq!(report.attestation_breakdown.artifact_referenced, 1);
    assert_eq!(report.attestation_breakdown.externally_verified, 0);
}

#[test]
fn journal_hash_framing_distinguishes_newline_ambiguous_fields() {
    let mut first = JournalWriter::default();
    first.append(JournalEventType::CaseInitialized, "a\nb", "c");
    let mut second = JournalWriter::default();
    second.append(JournalEventType::CaseInitialized, "a", "b\nc");

    assert_ne!(
        first.events()[0].event_digest(),
        second.events()[0].event_digest()
    );
}

#[test]
fn stale_receipts_are_reported_and_require_review() {
    let stale = ResourceId::new("resources::stale");
    let report = reconcile(&ReconciliationInput {
        resources: vec![],
        stale_receipts: vec![stale.clone()],
        skill_digest_changed: false,
    })
    .unwrap();

    assert_eq!(report.stale_receipts, vec![stale]);
    assert_eq!(report.status, ReconciliationStatus::NeedsReview);
}

#[test]
fn coverage_without_expected_artifacts_fails_closed() {
    let mut required = resource("resources::unexpected-artifact", Obligation::Required);
    required.issued = true;
    required.attained_evidence_level = Some(AttainedEvidenceLevel::ArtifactReference);

    assert_eq!(
        reconcile(&ReconciliationInput {
            resources: vec![required],
            stale_receipts: vec![],
            skill_digest_changed: false,
        }),
        Err(ArsenalError::ReconciliationInvariantViolation)
    );
}

#[test]
fn reconciliation_invariants_reject_impossible_counts_and_statuses() {
    let report = reconcile(&ReconciliationInput {
        resources: vec![],
        stale_receipts: vec![],
        skill_digest_changed: false,
    })
    .unwrap();

    let mut attested_over_issued = report.clone();
    attested_over_issued.protocol_completion.required = 1;
    attested_over_issued.protocol_completion.issued = 0;
    attested_over_issued.protocol_completion.attested = 1;
    assert_eq!(
        validate_reconciliation_invariants(&attested_over_issued),
        Err(ArsenalError::ReconciliationInvariantViolation)
    );

    let mut issued_over_required = report.clone();
    issued_over_required.protocol_completion.issued = 1;
    assert_eq!(
        validate_reconciliation_invariants(&issued_over_required),
        Err(ArsenalError::ReconciliationInvariantViolation)
    );

    let mut complete_with_stale = report;
    complete_with_stale
        .stale_receipts
        .push(ResourceId::new("resources::stale"));
    assert_eq!(
        validate_reconciliation_invariants(&complete_with_stale),
        Err(ArsenalError::ReconciliationInvariantViolation)
    );
}
