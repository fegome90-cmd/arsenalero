use uuid::Uuid;

/// UUIDv7 identifier for an Arsenalero case.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CaseId(Uuid);

/// Opaque resource identifier.
///
/// Validation and identifier generation are deferred to a later task.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ResourceId(String);

/// UUIDv7 identifier for an issued receipt.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReceiptId(Uuid);

macro_rules! opaque_string_id {
    ($id:ident) => {
        impl $id {
            /// Wraps an identifier without imposing validation or generation rules.
            pub fn new(value: impl Into<String>) -> Self {
                Self(value.into())
            }

            /// Returns the identifier exactly as supplied to [`Self::new`].
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
    };
}

opaque_string_id!(ResourceId);

macro_rules! uuid_v7_id {
    ($id:ident) => {
        impl $id {
            /// Creates a UUIDv7 identifier using the system clock.
            pub fn now_v7() -> Self {
                Self(Uuid::now_v7())
            }

            /// Returns the underlying UUID value.
            pub const fn as_uuid(&self) -> Uuid {
                self.0
            }
        }
    };
}

uuid_v7_id!(CaseId);
uuid_v7_id!(ReceiptId);

/// How a resource classification was obtained.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClassificationSource {
    Declared,
    Derived,
    Unresolved,
}

/// The normative obligation assigned independently from classification source.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Obligation {
    Required,
    Recommended,
    Optional,
    Unknown,
}

/// The resource's evidence capability, independent of its obligation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResourceKind {
    Consultative,
    Procedure,
    VerifiableContract,
}

/// Evidence levels that Arsenalero v1 can attain.
///
/// External verification is intentionally not represented because v1 does not support it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AttainedEvidenceLevel {
    Attestation,
    ArtifactReference,
}

/// Evidence expected or supported by a resource.
///
/// This is a direct representation of the SDD's `minimum` and `supported_levels`
/// vocabulary. Validation of consistency between those fields is deferred.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EvidenceContract {
    pub minimum: AttainedEvidenceLevel,
    pub supported_levels: Vec<AttainedEvidenceLevel>,
}

/// Lifecycle state for an inventoried resource.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResourceState {
    Discovered,
    Classified,
    Required,
    Recommended,
    Optional,
    Unknown,
    Issued,
    Attested,
    Reconciled,
}

impl ResourceState {
    /// Returns whether the SDD explicitly permits this lifecycle transition.
    ///
    /// All unlisted transitions, including self-transitions, are rejected
    /// fail-closed because the authority does not define them.
    pub const fn can_transition_to(self, next: Self) -> bool {
        matches!(
            (self, next),
            (Self::Discovered, Self::Classified)
                | (Self::Classified, Self::Required)
                | (Self::Classified, Self::Recommended)
                | (Self::Classified, Self::Optional)
                | (Self::Classified, Self::Unknown)
                | (Self::Required, Self::Issued)
                | (Self::Recommended, Self::Issued)
                | (Self::Optional, Self::Issued)
                | (Self::Unknown, Self::Issued)
                | (Self::Issued, Self::Attested)
                | (Self::Attested, Self::Reconciled)
        )
    }
}

/// Final reconciliation outcome for a case.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReconciliationStatus {
    Complete,
    Incomplete,
    NeedsReview,
    Invalidated,
}

#[cfg(test)]
mod tests {
    use super::{
        AttainedEvidenceLevel, CaseId, ClassificationSource, EvidenceContract, Obligation,
        ReceiptId, ResourceId, ResourceState,
    };

    #[test]
    fn permits_only_explicitly_documented_transitions() {
        let allowed = [
            (ResourceState::Discovered, ResourceState::Classified),
            (ResourceState::Classified, ResourceState::Required),
            (ResourceState::Classified, ResourceState::Recommended),
            (ResourceState::Classified, ResourceState::Optional),
            (ResourceState::Classified, ResourceState::Unknown),
            (ResourceState::Required, ResourceState::Issued),
            (ResourceState::Recommended, ResourceState::Issued),
            (ResourceState::Optional, ResourceState::Issued),
            (ResourceState::Unknown, ResourceState::Issued),
            (ResourceState::Issued, ResourceState::Attested),
            (ResourceState::Attested, ResourceState::Reconciled),
        ];

        for (from, to) in allowed {
            assert!(from.can_transition_to(to));
        }

        assert!(!ResourceState::Discovered.can_transition_to(ResourceState::Attested));
        assert!(!ResourceState::Attested.can_transition_to(ResourceState::Issued));
        assert!(!ResourceState::Issued.can_transition_to(ResourceState::Issued));
    }

    #[test]
    fn ids_preserve_uuid_value_semantics() {
        let case = CaseId::now_v7();
        let receipt = ReceiptId::now_v7();
        assert_eq!(case.as_uuid().get_version_num(), 7);
        assert_eq!(receipt.as_uuid().get_version_num(), 7);
        assert_eq!(
            ResourceId::new("resources::check").as_str(),
            "resources::check"
        );
    }

    #[test]
    fn classification_and_evidence_dimensions_remain_independent() {
        let source = ClassificationSource::Derived;
        let obligation = Obligation::Recommended;
        let contract = EvidenceContract {
            minimum: AttainedEvidenceLevel::Attestation,
            supported_levels: vec![
                AttainedEvidenceLevel::Attestation,
                AttainedEvidenceLevel::ArtifactReference,
            ],
        };

        assert_eq!(source, ClassificationSource::Derived);
        assert_eq!(obligation, Obligation::Recommended);
        assert_eq!(contract.minimum, AttainedEvidenceLevel::Attestation);
    }
}
