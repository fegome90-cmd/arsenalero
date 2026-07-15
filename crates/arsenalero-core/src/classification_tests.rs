use std::collections::BTreeSet;

use crate::{
    ArsenalMetadata, ClassificationInput, ClassificationSource, MetadataEvidenceContract,
    Obligation, ResourceId, ResourceKind, classify_resource, required_set,
};

fn input(id: &str, heading: Option<&str>, context: &str) -> ClassificationInput {
    ClassificationInput {
        resource_id: ResourceId::new(id),
        metadata: None,
        heading: heading.map(str::to_owned),
        prose_context: Some(context.to_owned()),
        list_context: Vec::new(),
    }
}

#[test]
fn classify_recognizes_every_closed_stage_alias_with_a_closed_purpose_verb() {
    let aliases = [
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

    for alias in aliases {
        let resource = classify_resource(input("resources/check.md", Some(alias), "consult this"));
        assert_eq!(resource.source, ClassificationSource::Derived, "{alias}");
        assert_eq!(resource.obligation, Obligation::Recommended, "{alias}");
    }
}

#[test]
fn classify_derived_does_not_imply_required() {
    let resource = classify_resource(input(
        "resources/architecture.md",
        Some("Architecture exploration"),
        "review this reference before implementation",
    ));

    assert_eq!(resource.source, ClassificationSource::Derived);
    assert_eq!(resource.obligation, Obligation::Recommended);
}

#[test]
fn classify_declared_requirement_as_required() {
    let resource = classify_resource(ClassificationInput {
        resource_id: ResourceId::new("resources/validation.md"),
        metadata: Some(ArsenalMetadata {
            requirement: Some("required".into()),
            ..ArsenalMetadata::default()
        }),
        heading: None,
        prose_context: None,
        list_context: Vec::new(),
    });

    assert_eq!(resource.source, ClassificationSource::Declared);
    assert_eq!(resource.obligation, Obligation::Required);
}

#[test]
fn classify_closed_english_and_spanish_normative_markers_as_required() {
    for context in [
        "must consult this",
        "this is required; consult this",
        "consult this before completing",
        "consult this before finalizing",
        "consult this; do not proceed without it",
        "debe consultar esto",
        "esto es obligatorio; consultar esto",
        "consultar esto antes de completar",
        "consultar esto antes de finalizar",
        "no continúe sin consultar esto",
        "no proceda sin consultar esto",
    ] {
        let resource = classify_resource(input("resources/required.md", Some("testing"), context));
        assert_eq!(resource.source, ClassificationSource::Derived, "{context}");
        assert_eq!(resource.obligation, Obligation::Required, "{context}");
    }
}

#[test]
fn classify_closed_optional_markers_as_optional() {
    for context in [
        "use this optional reference",
        "use esta referencia opcional",
        "use this example",
        "use este ejemplo",
        "use this illustrative reference",
        "use esta referencia ilustrativo",
    ] {
        let resource = classify_resource(input("resources/example.md", Some("testing"), context));
        assert_eq!(resource.source, ClassificationSource::Derived, "{context}");
        assert_eq!(resource.obligation, Obligation::Optional, "{context}");
    }
}

#[test]
fn classify_declared_resources_without_markers_as_recommended() {
    let resource = classify_resource(ClassificationInput {
        resource_id: ResourceId::new("resources/declared.md"),
        metadata: Some(ArsenalMetadata::default()),
        heading: None,
        prose_context: None,
        list_context: Vec::new(),
    });

    assert_eq!(resource.source, ClassificationSource::Declared);
    assert_eq!(resource.obligation, Obligation::Recommended);
}

#[test]
fn classify_resource_metadata_alone_does_not_declare_or_oblige() {
    let resource = classify_resource(ClassificationInput {
        resource_id: ResourceId::new("resources/metadata-only.md"),
        metadata: Some(ArsenalMetadata {
            resource_kind: Some(ResourceKind::Procedure),
            evidence_contract: Some(MetadataEvidenceContract::default()),
            ..ArsenalMetadata::default()
        }),
        heading: None,
        prose_context: None,
        list_context: Vec::new(),
    });

    assert_eq!(resource.source, ClassificationSource::Unresolved);
    assert_eq!(resource.obligation, Obligation::Unknown);
}

#[test]
fn classify_requirement_metadata_uses_exact_normalized_values() {
    for requirement in ["required", " REQUIRED "] {
        let resource = classify_resource(ClassificationInput {
            resource_id: ResourceId::new("resources/required-metadata.md"),
            metadata: Some(ArsenalMetadata {
                requirement: Some(requirement.into()),
                ..ArsenalMetadata::default()
            }),
            heading: None,
            prose_context: None,
            list_context: Vec::new(),
        });
        assert_eq!(resource.obligation, Obligation::Required, "{requirement}");
    }

    for requirement in ["not required", "required eventually"] {
        let resource = classify_resource(ClassificationInput {
            resource_id: ResourceId::new("resources/non-required-metadata.md"),
            metadata: Some(ArsenalMetadata {
                requirement: Some(requirement.into()),
                ..ArsenalMetadata::default()
            }),
            heading: None,
            prose_context: None,
            list_context: Vec::new(),
        });
        assert_ne!(resource.obligation, Obligation::Required, "{requirement}");
    }
}

#[test]
fn classify_ignores_negated_normative_contexts() {
    for context in ["not required", "no obligatorio", "must not consult this"] {
        let resource = classify_resource(input("resources/negated.md", Some("testing"), context));
        assert_ne!(resource.obligation, Obligation::Required, "{context}");
    }
}

#[test]
fn classify_uses_heading_markers_and_list_purpose_context() {
    let resource = classify_resource(ClassificationInput {
        resource_id: ResourceId::new("resources/heading-required.md"),
        metadata: None,
        heading: Some("testing required".into()),
        prose_context: None,
        list_context: vec!["consult this".into()],
    });

    assert_eq!(resource.source, ClassificationSource::Derived);
    assert_eq!(resource.obligation, Obligation::Required);
}

#[test]
fn classify_leaves_unresolved_references_unknown() {
    let resource = classify_resource(input("resources/ambiguous.md", None, "see this document"));

    assert_eq!(resource.source, ClassificationSource::Unresolved);
    assert_eq!(resource.obligation, Obligation::Unknown);
}

#[test]
fn classify_rejects_conflicting_normative_markers() {
    let resource = classify_resource(input(
        "resources/conflicting.md",
        Some("testing"),
        "must consult this optional example",
    ));

    assert_eq!(resource.source, ClassificationSource::Unresolved);
    assert_eq!(resource.obligation, Obligation::Unknown);
}

#[test]
fn classify_required_set_contains_only_required_resources_in_sorted_order() {
    let required = classify_resource(input("resources/z.md", Some("testing"), "must use this"));
    let recommended = classify_resource(input("resources/a.md", Some("testing"), "use this"));
    let optional = classify_resource(input("resources/b.md", Some("testing"), "optional example"));

    assert_eq!(
        required_set(&[recommended, optional, required]),
        BTreeSet::from([ResourceId::new("resources/z.md")])
    );
}

#[test]
fn classify_uses_exact_normalized_matching_without_fuzzy_terms() {
    let resource = classify_resource(input(
        "resources/fuzzy.md",
        Some("testingish"),
        "mustard consultation",
    ));

    assert_eq!(resource.source, ClassificationSource::Unresolved);
    assert_eq!(resource.obligation, Obligation::Unknown);
}
