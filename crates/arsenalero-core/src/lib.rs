#![forbid(unsafe_code)]
#![doc = "Shared library boundary for the Arsenalero bootstrap workspace."]
#![doc = "\n"]
#![doc = "Minimal domain contracts for the Arsenalero bootstrap workspace."]

pub mod domain;
pub mod error;
pub mod path_policy;

pub use domain::{
    AttainedEvidenceLevel, CaseId, ClassificationSource, EvidenceContract, Obligation, ReceiptId,
    ReconciliationStatus, ResourceId, ResourceKind, ResourceState,
};
pub use error::ArsenalError;
pub use path_policy::{CanonicalResourcePath, CanonicalSkillRoot, PathPolicy};
