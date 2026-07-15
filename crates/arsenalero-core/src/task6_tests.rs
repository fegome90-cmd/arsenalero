use std::fs;

use crate::{
    ArsenalMetadata, AttainedEvidenceLevel, ResourceKind, parse_resource_metadata, scan_skill,
};

#[test]
fn markdown_scanner_matches_bilingual_golden() {
    let source = fs::read_to_string("../../tests/fixtures/valid_bilingual/SKILL.md").unwrap();
    let expected =
        fs::read_to_string("../../tests/fixtures/valid_bilingual/task6-golden.json").unwrap();

    assert_eq!(render_scan(&scan_skill(&source).unwrap()), expected);
}

#[test]
fn markdown_scanner_preserves_broken_relative_link_for_later_resolution() {
    let source = fs::read_to_string("../../tests/fixtures/broken_reference/SKILL.md").unwrap();
    let expected =
        fs::read_to_string("../../tests/fixtures/broken_reference/task6-golden.json").unwrap();

    assert_eq!(render_scan(&scan_skill(&source).unwrap()), expected);
}

#[test]
fn parses_arsenal_frontmatter_metadata() {
    let source = fs::read_to_string("../../tests/fixtures/broken_reference/SKILL.md").unwrap();

    assert_eq!(
        parse_resource_metadata(&source).unwrap(),
        Some(ArsenalMetadata {
            id: Some("rollback-procedure".into()),
            purpose: Some("Restore the repository after a failed migration.".into()),
            stages: vec!["implementation".into(), "recovery".into()],
            requirement: Some("required".into()),
            resource_kind: Some(ResourceKind::Procedure),
            evidence_contract: Some(crate::MetadataEvidenceContract {
                minimum: Some(AttainedEvidenceLevel::Attestation),
                supported: vec![
                    AttainedEvidenceLevel::Attestation,
                    AttainedEvidenceLevel::ArtifactReference,
                ],
            }),
        })
    );
}

#[test]
fn markdown_metadata_scopes_evidence_contract_with_crlf_frontmatter() {
    let source = "---\r\nminimum: artifact_reference\r\nsupported:\r\n  - artifact_reference\r\narsenal:\r\n  evidence_contract:\r\n    minimum: attestation\r\n    supported:\r\n      - attestation\r\n      - artifact_reference\r\n---\r\n";
    assert_eq!(
        parse_resource_metadata(source)
            .unwrap()
            .unwrap()
            .evidence_contract,
        Some(crate::MetadataEvidenceContract {
            minimum: Some(AttainedEvidenceLevel::Attestation),
            supported: vec![
                AttainedEvidenceLevel::Attestation,
                AttainedEvidenceLevel::ArtifactReference,
            ],
        })
    );
}
fn render_scan(document: &crate::SkillDocument) -> String {
    let references = document
        .references
        .iter()
        .map(|reference| {
            format!(
                "    {{\"path\":\"{}\",\"kind\":\"{}\",\"range\":[{},{}],\"heading\":{},\"prose\":{},\"list\":{}}}",
                reference.path,
                reference.kind.as_str(),
                reference.range.start,
                reference.range.end,
                json_option(&reference.heading),
                json_option(&reference.prose_context),
                json_strings(&reference.list_context),
            )
        })
        .collect::<Vec<_>>()
        .join(",\n");
    let warnings = document
        .warnings
        .iter()
        .map(|warning| {
            format!(
                "    {{\"candidate\":\"{}\",\"range\":[{},{}],\"heading\":{},\"prose\":{},\"list\":{}}}",
                warning.candidate,
                warning.range.start,
                warning.range.end,
                json_option(&warning.heading),
                json_option(&warning.prose_context),
                json_strings(&warning.list_context),
            )
        })
        .collect::<Vec<_>>()
        .join(",\n");

    let references = if references.is_empty() {
        "[]".to_owned()
    } else {
        format!("[\n{references}\n  ]")
    };
    let warnings = if warnings.is_empty() {
        "[]".to_owned()
    } else {
        format!("[\n{warnings}\n  ]")
    };

    format!("{{\n  \"references\": {references},\n  \"warnings\": {warnings}\n}}\n")
}

fn json_option(value: &Option<String>) -> String {
    value
        .as_deref()
        .map(|value| format!("{value:?}"))
        .unwrap_or_else(|| "null".into())
}

fn json_strings(values: &[String]) -> String {
    format!(
        "[{}]",
        values
            .iter()
            .map(|value| format!("{value:?}"))
            .collect::<Vec<_>>()
            .join(",")
    )
}
