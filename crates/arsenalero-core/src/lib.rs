#![forbid(unsafe_code)]
#![doc = "Shared library boundary for the Arsenalero bootstrap workspace."]
#![doc = "\n"]
#![doc = "Minimal domain contracts for the Arsenalero bootstrap workspace."]

pub mod case;
pub mod classify;
pub mod domain;
pub mod error;
pub mod inventory;
pub mod journal;
pub mod markdown;
pub mod path_policy;
pub mod receipt;
pub mod reconcile;

pub use case::ArsenalCase;
pub use classify::{ClassificationInput, ClassifiedResource, classify_resource, required_set};
pub use domain::{
    AttainedEvidenceLevel, CaseId, ClassificationSource, EvidenceContract, Obligation, ReceiptId,
    ReconciliationStatus, ResourceId, ResourceKind, ResourceState,
};
pub use error::ArsenalError;
pub use inventory::{ArsenalMetadata, MetadataEvidenceContract, parse_resource_metadata};
pub use journal::{JournalEvent, JournalEventType, JournalWriter};
pub use markdown::{
    Heading, ListItem, ReferenceKind, ResourceReference, SkillDocument, Warning, scan_skill,
};
pub use path_policy::{CanonicalResourcePath, CanonicalSkillRoot, PathPolicy};
pub use receipt::{
    Attestation, AttestationRequest, EvidenceReference, ResourceIssueRequest, ResourceReceipt,
    attest_resources, issue_resources,
};
pub use reconcile::{
    AttestationBreakdown, EvidenceCoverage, PerResourceEvidence, ProtocolCompletion,
    ReconciliationInput, ReconciliationReport, ResourceReconciliation, Verification, reconcile,
    validate_reconciliation_invariants,
};

#[cfg(test)]
mod classification_tests;
#[cfg(test)]
mod task6_tests;
#[cfg(test)]
mod task8_tests;
#[cfg(test)]
mod task9_tests;
