use std::collections::BTreeSet;

use crate::{ArsenalMetadata, ClassificationSource, Obligation, ResourceId};

/// Source-local facts used to classify one discovered resource reference.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClassificationInput {
    pub resource_id: ResourceId,
    pub metadata: Option<ArsenalMetadata>,
    pub heading: Option<String>,
    pub prose_context: Option<String>,
    pub list_context: Vec<String>,
}

/// A resource's independent classification source and normative obligation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClassifiedResource {
    pub resource_id: ResourceId,
    pub source: ClassificationSource,
    pub obligation: Obligation,
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

const PURPOSE_VERBS: &[&str] = &[
    "use",
    "follow",
    "consult",
    "check",
    "apply",
    "validate",
    "review",
    "run",
    "usar",
    "utilizar",
    "seguir",
    "consultar",
    "comprobar",
    "aplicar",
    "validar",
    "revisar",
    "ejecutar",
];

const REQUIRED_MARKERS: &[&str] = &[
    "must",
    "required",
    "before completing",
    "before finalizing",
    "do not proceed without",
    "debe",
    "obligatorio",
    "antes de completar",
    "antes de finalizar",
    "no continúe sin",
    "no proceda sin",
];

const OPTIONAL_MARKERS: &[&str] = &[
    "optional",
    "opcional",
    "example",
    "ejemplo",
    "illustrative",
    "ilustrativo",
];

const NEGATED_REQUIRED_MARKERS: &[&str] = &["not required", "no obligatorio", "must not consult"];

/// Classifies one resource using only closed, normalized vocabularies.
pub fn classify_resource(input: ClassificationInput) -> ClassifiedResource {
    let contexts = contexts(&input);
    let obligation_contexts = obligation_contexts(&input, &contexts);
    let required = contains_required(&obligation_contexts)
        || metadata_requirement_is(input.metadata.as_ref(), "required");
    let optional = contains_any(&obligation_contexts, OPTIONAL_MARKERS)
        || metadata_requirement_is(input.metadata.as_ref(), "optional");

    let source = if required && optional {
        ClassificationSource::Unresolved
    } else if metadata_declares(input.metadata.as_ref()) {
        ClassificationSource::Declared
    } else if input
        .heading
        .as_deref()
        .is_some_and(|heading| contains_any(&[heading], STAGE_ALIASES))
        && contains_any(&contexts, PURPOSE_VERBS)
    {
        ClassificationSource::Derived
    } else {
        ClassificationSource::Unresolved
    };

    let obligation = match source {
        ClassificationSource::Unresolved => Obligation::Unknown,
        ClassificationSource::Declared | ClassificationSource::Derived if required => {
            Obligation::Required
        }
        ClassificationSource::Declared | ClassificationSource::Derived if optional => {
            Obligation::Optional
        }
        ClassificationSource::Declared | ClassificationSource::Derived => Obligation::Recommended,
    };

    ClassifiedResource {
        resource_id: input.resource_id,
        source,
        obligation,
    }
}

/// Returns the unique IDs whose obligation is explicitly required.
pub fn required_set(resources: &[ClassifiedResource]) -> BTreeSet<ResourceId> {
    resources
        .iter()
        .filter(|resource| resource.obligation == Obligation::Required)
        .map(|resource| resource.resource_id.clone())
        .collect()
}

fn contexts(input: &ClassificationInput) -> Vec<&str> {
    input
        .prose_context
        .iter()
        .map(String::as_str)
        .chain(input.list_context.iter().map(String::as_str))
        .collect()
}

fn obligation_contexts<'a>(input: &'a ClassificationInput, contexts: &[&'a str]) -> Vec<&'a str> {
    input
        .heading
        .as_deref()
        .into_iter()
        .chain(contexts.iter().copied())
        .collect()
}

fn metadata_declares(metadata: Option<&ArsenalMetadata>) -> bool {
    metadata.is_some_and(|metadata| {
        metadata.resource_kind.is_none() && metadata.evidence_contract.is_none()
            || metadata.id.is_some()
            || metadata.purpose.is_some()
            || !metadata.stages.is_empty()
            || metadata.requirement.is_some()
    })
}

fn metadata_requirement_is(metadata: Option<&ArsenalMetadata>, value: &str) -> bool {
    metadata.is_some_and(|metadata| {
        metadata
            .requirement
            .as_deref()
            .is_some_and(|requirement| normalize(requirement) == value)
    })
}

fn contains_required(values: &[&str]) -> bool {
    !contains_any(values, NEGATED_REQUIRED_MARKERS) && contains_any(values, REQUIRED_MARKERS)
}

fn contains_any(values: &[&str], markers: &[&str]) -> bool {
    values.iter().any(|value| {
        let normalized = normalize(value);
        markers
            .iter()
            .any(|marker| contains_phrase(&normalized, &normalize(marker)))
    })
}

fn contains_phrase(value: &str, phrase: &str) -> bool {
    value == phrase
        || value
            .split_whitespace()
            .collect::<Vec<_>>()
            .windows(phrase.split_whitespace().count())
            .any(|window| window.join(" ") == phrase)
}

fn normalize(value: &str) -> String {
    value
        .chars()
        .flat_map(char::to_lowercase)
        .map(|character| {
            if character.is_alphanumeric() {
                character
            } else {
                ' '
            }
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}
