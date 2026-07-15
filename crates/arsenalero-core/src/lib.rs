#![forbid(unsafe_code)]
#![doc = "Shared library boundary for the Arsenalero bootstrap workspace."]
#![doc = "\n"]
#![doc = "Minimal domain contracts for the Arsenalero bootstrap workspace."]

pub mod classify;
pub mod domain;
pub mod error;
pub mod inventory;
pub mod markdown;
pub mod path_policy;

pub use classify::{ClassificationInput, ClassifiedResource, classify_resource, required_set};
pub use domain::{
    AttainedEvidenceLevel, CaseId, ClassificationSource, EvidenceContract, Obligation, ReceiptId,
    ReconciliationStatus, ResourceId, ResourceKind, ResourceState,
};
pub use error::ArsenalError;
pub use inventory::{ArsenalMetadata, MetadataEvidenceContract, parse_resource_metadata};
pub use markdown::{
    Heading, ListItem, ReferenceKind, ResourceReference, SkillDocument, Warning, scan_skill,
};
pub use path_policy::{CanonicalResourcePath, CanonicalSkillRoot, PathPolicy};

#[cfg(test)]
mod classification_tests;
#[cfg(test)]
mod task6_tests;
