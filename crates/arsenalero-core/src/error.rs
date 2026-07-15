/// Closed set of SDD domain reason codes.
///
/// Payloads and standard error trait implementations are intentionally deferred
/// until a later task defines their contracts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArsenalError {
    SkillRootInvalid,
    SkillRootNotAllowed,
    SkillMdMissing,
    SkillMdTooLarge,
    SkillDigestChanged,
    ResourceReferenceBroken,
    ResourcePathEscape,
    ResourceSymlinkEscape,
    ResourceTypeUnsupported,
    ResourceTooLarge,
    ResourceLimitExceeded,
    ResourceIdCollision,
    ResourceUnresolved,
    ResourceClassificationConflict,
    StageUnknown,
    CaseUnknown,
    CaseInvalidated,
    ResourceUnknown,
    ResourceBatchLimit,
    ResourceAlreadyIssued,
    ReceiptUnknown,
    ReceiptCaseMismatch,
    ReceiptStale,
    ResourceModifiedPostAttestation,
    AttestationEmpty,
    EvidenceReferenceInvalid,
    ReconciliationComplete,
    ReconciliationIncomplete,
    ReconciliationNeedsReview,
    ReconciliationInvalidated,
    ReconciliationInvariantViolation,
}

impl ArsenalError {
    /// Returns the stable, uppercase SDD reason code for this domain error.
    pub const fn code(&self) -> &'static str {
        match self {
            Self::SkillRootInvalid => "SKILL_ROOT_INVALID",
            Self::SkillRootNotAllowed => "SKILL_ROOT_NOT_ALLOWED",
            Self::SkillMdMissing => "SKILL_MD_MISSING",
            Self::SkillMdTooLarge => "SKILL_MD_TOO_LARGE",
            Self::SkillDigestChanged => "SKILL_DIGEST_CHANGED",
            Self::ResourceReferenceBroken => "RESOURCE_REFERENCE_BROKEN",
            Self::ResourcePathEscape => "RESOURCE_PATH_ESCAPE",
            Self::ResourceSymlinkEscape => "RESOURCE_SYMLINK_ESCAPE",
            Self::ResourceTypeUnsupported => "RESOURCE_TYPE_UNSUPPORTED",
            Self::ResourceTooLarge => "RESOURCE_TOO_LARGE",
            Self::ResourceLimitExceeded => "RESOURCE_LIMIT_EXCEEDED",
            Self::ResourceIdCollision => "RESOURCE_ID_COLLISION",
            Self::ResourceUnresolved => "RESOURCE_UNRESOLVED",
            Self::ResourceClassificationConflict => "RESOURCE_CLASSIFICATION_CONFLICT",
            Self::StageUnknown => "STAGE_UNKNOWN",
            Self::CaseUnknown => "CASE_UNKNOWN",
            Self::CaseInvalidated => "CASE_INVALIDATED",
            Self::ResourceUnknown => "RESOURCE_UNKNOWN",
            Self::ResourceBatchLimit => "RESOURCE_BATCH_LIMIT",
            Self::ResourceAlreadyIssued => "RESOURCE_ALREADY_ISSUED",
            Self::ReceiptUnknown => "RECEIPT_UNKNOWN",
            Self::ReceiptCaseMismatch => "RECEIPT_CASE_MISMATCH",
            Self::ReceiptStale => "RECEIPT_STALE",
            Self::ResourceModifiedPostAttestation => "RESOURCE_MODIFIED_POST_ATTESTATION",
            Self::AttestationEmpty => "ATTESTATION_EMPTY",
            Self::EvidenceReferenceInvalid => "EVIDENCE_REFERENCE_INVALID",
            Self::ReconciliationComplete => "RECONCILIATION_COMPLETE",
            Self::ReconciliationIncomplete => "RECONCILIATION_INCOMPLETE",
            Self::ReconciliationNeedsReview => "RECONCILIATION_NEEDS_REVIEW",
            Self::ReconciliationInvalidated => "RECONCILIATION_INVALIDATED",
            Self::ReconciliationInvariantViolation => "RECONCILIATION_INVARIANT_VIOLATION",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ArsenalError;

    #[test]
    fn reason_codes_are_stable() {
        let cases = [
            (ArsenalError::SkillRootInvalid, "SKILL_ROOT_INVALID"),
            (ArsenalError::SkillRootNotAllowed, "SKILL_ROOT_NOT_ALLOWED"),
            (ArsenalError::SkillMdMissing, "SKILL_MD_MISSING"),
            (ArsenalError::SkillMdTooLarge, "SKILL_MD_TOO_LARGE"),
            (ArsenalError::SkillDigestChanged, "SKILL_DIGEST_CHANGED"),
            (
                ArsenalError::ResourceReferenceBroken,
                "RESOURCE_REFERENCE_BROKEN",
            ),
            (ArsenalError::ResourcePathEscape, "RESOURCE_PATH_ESCAPE"),
            (
                ArsenalError::ResourceSymlinkEscape,
                "RESOURCE_SYMLINK_ESCAPE",
            ),
            (
                ArsenalError::ResourceTypeUnsupported,
                "RESOURCE_TYPE_UNSUPPORTED",
            ),
            (ArsenalError::ResourceTooLarge, "RESOURCE_TOO_LARGE"),
            (
                ArsenalError::ResourceLimitExceeded,
                "RESOURCE_LIMIT_EXCEEDED",
            ),
            (ArsenalError::ResourceIdCollision, "RESOURCE_ID_COLLISION"),
            (ArsenalError::ResourceUnresolved, "RESOURCE_UNRESOLVED"),
            (
                ArsenalError::ResourceClassificationConflict,
                "RESOURCE_CLASSIFICATION_CONFLICT",
            ),
            (ArsenalError::StageUnknown, "STAGE_UNKNOWN"),
            (ArsenalError::CaseUnknown, "CASE_UNKNOWN"),
            (ArsenalError::CaseInvalidated, "CASE_INVALIDATED"),
            (ArsenalError::ResourceUnknown, "RESOURCE_UNKNOWN"),
            (ArsenalError::ResourceBatchLimit, "RESOURCE_BATCH_LIMIT"),
            (
                ArsenalError::ResourceAlreadyIssued,
                "RESOURCE_ALREADY_ISSUED",
            ),
            (ArsenalError::ReceiptUnknown, "RECEIPT_UNKNOWN"),
            (ArsenalError::ReceiptCaseMismatch, "RECEIPT_CASE_MISMATCH"),
            (ArsenalError::ReceiptStale, "RECEIPT_STALE"),
            (
                ArsenalError::ResourceModifiedPostAttestation,
                "RESOURCE_MODIFIED_POST_ATTESTATION",
            ),
            (ArsenalError::AttestationEmpty, "ATTESTATION_EMPTY"),
            (
                ArsenalError::EvidenceReferenceInvalid,
                "EVIDENCE_REFERENCE_INVALID",
            ),
            (
                ArsenalError::ReconciliationComplete,
                "RECONCILIATION_COMPLETE",
            ),
            (
                ArsenalError::ReconciliationIncomplete,
                "RECONCILIATION_INCOMPLETE",
            ),
            (
                ArsenalError::ReconciliationNeedsReview,
                "RECONCILIATION_NEEDS_REVIEW",
            ),
            (
                ArsenalError::ReconciliationInvalidated,
                "RECONCILIATION_INVALIDATED",
            ),
            (
                ArsenalError::ReconciliationInvariantViolation,
                "RECONCILIATION_INVARIANT_VIOLATION",
            ),
        ];

        for (error, code) in cases {
            assert_eq!(error.code(), code);
        }
    }
}
