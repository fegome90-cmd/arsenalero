use crate::{ArsenalError, AttainedEvidenceLevel, ResourceKind};

/// Explicit `arsenal` frontmatter metadata for a resource.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ArsenalMetadata {
    pub id: Option<String>,
    pub purpose: Option<String>,
    pub stages: Vec<String>,
    pub requirement: Option<String>,
    pub resource_kind: Option<ResourceKind>,
    pub evidence_contract: Option<MetadataEvidenceContract>,
}

/// Explicit evidence capability declared in an `arsenal` frontmatter block.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MetadataEvidenceContract {
    pub minimum: Option<AttainedEvidenceLevel>,
    pub supported: Vec<AttainedEvidenceLevel>,
}

/// Parses only the optional `arsenal` mapping in a leading YAML frontmatter block.
pub fn parse_resource_metadata(source: &str) -> Result<Option<ArsenalMetadata>, ArsenalError> {
    let normalized = source.replace("\r\n", "\n");
    let Some(frontmatter) = frontmatter(&normalized) else {
        return Ok(None);
    };

    let mut metadata = ArsenalMetadata::default();
    let mut in_arsenal = false;
    let mut in_stages = false;
    let mut in_evidence_contract = false;
    let mut in_supported = false;
    let mut evidence_contract = MetadataEvidenceContract::default();
    let mut saw_arsenal = false;

    for line in frontmatter.lines() {
        let indentation = line.len() - line.trim_start().len();
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if indentation == 0 {
            in_arsenal = trimmed == "arsenal:";
            in_stages = false;
            in_evidence_contract = false;
            in_supported = false;
            saw_arsenal |= in_arsenal;
            continue;
        }
        if !in_arsenal {
            continue;
        }
        if indentation == 2 {
            in_stages = trimmed == "stages:";
            in_evidence_contract = trimmed == "evidence_contract:";
            in_supported = false;
            if in_evidence_contract {
                continue;
            }
            if let Some((key, value)) = key_value(trimmed) {
                match key {
                    "id" => metadata.id = Some(value.to_owned()),
                    "purpose" => metadata.purpose = Some(value.to_owned()),
                    "requirement" => metadata.requirement = Some(value.to_owned()),
                    "resource_kind" => metadata.resource_kind = Some(resource_kind(value)?),
                    _ => {}
                }
            }
            continue;
        }
        if indentation == 4 && in_stages && trimmed.starts_with("- ") {
            metadata.stages.push(trimmed[2..].trim().to_owned());
            continue;
        }
        if indentation == 4 && in_evidence_contract && trimmed == "supported:" {
            in_supported = true;
            in_stages = false;
            continue;
        }
        if indentation == 4 && in_evidence_contract {
            in_supported = false;
            if let Some(("minimum", value)) = key_value(trimmed) {
                evidence_contract.minimum = Some(evidence_level(value)?);
            }
            continue;
        }
        if indentation == 6 && in_supported && trimmed.starts_with("- ") {
            evidence_contract
                .supported
                .push(evidence_level(trimmed[2..].trim())?);
        }
    }

    if !saw_arsenal {
        return Ok(None);
    }
    if evidence_contract.minimum.is_some() || !evidence_contract.supported.is_empty() {
        metadata.evidence_contract = Some(evidence_contract);
    }
    Ok(Some(metadata))
}

fn frontmatter(source: &str) -> Option<&str> {
    let rest = source.strip_prefix("---\n")?;
    rest.split_once("\n---\n")
        .map(|(frontmatter, _)| frontmatter)
}

fn key_value(value: &str) -> Option<(&str, &str)> {
    let (key, value) = value.split_once(':')?;
    Some((key.trim(), value.trim().trim_matches('"')))
}

fn resource_kind(value: &str) -> Result<ResourceKind, ArsenalError> {
    match value {
        "consultative" => Ok(ResourceKind::Consultative),
        "procedure" => Ok(ResourceKind::Procedure),
        "verifiable_contract" => Ok(ResourceKind::VerifiableContract),
        _ => Err(ArsenalError::ResourceUnresolved),
    }
}

fn evidence_level(value: &str) -> Result<AttainedEvidenceLevel, ArsenalError> {
    match value {
        "attestation" => Ok(AttainedEvidenceLevel::Attestation),
        "artifact_reference" => Ok(AttainedEvidenceLevel::ArtifactReference),
        _ => Err(ArsenalError::ResourceUnresolved),
    }
}
