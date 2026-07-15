use std::ops::Range;

use pulldown_cmark::{Event, Parser, Tag, TagEnd};

use crate::ArsenalError;

/// Parsed, source-local view of a skill document.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SkillDocument {
    pub headings: Vec<Heading>,
    pub list_items: Vec<ListItem>,
    pub references: Vec<ResourceReference>,
    pub warnings: Vec<Warning>,
}

/// A Markdown heading and its source range.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Heading {
    pub text: String,
    pub range: Range<usize>,
}

/// A Markdown list item and its source range.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ListItem {
    pub text: String,
    pub range: Range<usize>,
}

/// The syntactic form that exposed a confirmed resource reference.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReferenceKind {
    MarkdownLink,
    InlineCode,
}

impl ReferenceKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MarkdownLink => "markdown_link",
            Self::InlineCode => "inline_code",
        }
    }
}

/// A confirmed, relative resource reference with parser-provided byte offsets.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResourceReference {
    pub path: String,
    pub kind: ReferenceKind,
    pub range: Range<usize>,
    pub heading: Option<String>,
    pub prose_context: Option<String>,
    pub list_context: Vec<String>,
}

/// A free filename mention that is not a confirmed resource reference.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Warning {
    pub candidate: String,
    pub range: Range<usize>,
    pub heading: Option<String>,
    pub prose_context: Option<String>,
    pub list_context: Vec<String>,
}

/// Scans Markdown source without reading files, resolving paths, or executing content.
pub fn scan_skill(source: &str) -> Result<SkillDocument, ArsenalError> {
    let mut document = SkillDocument::default();
    let mut heading = None;
    let mut heading_start = 0;
    let mut heading_text = String::new();
    let mut item_stack = Vec::new();
    let mut paragraph = None;
    let mut paragraphs = Vec::new();

    for (event, range) in Parser::new(source).into_offset_iter() {
        match event {
            Event::Start(Tag::Heading { .. }) => {
                heading_start = range.start;
                heading_text.clear();
            }
            Event::End(TagEnd::Heading(_)) => {
                let text = heading_text.trim().to_owned();
                document.headings.push(Heading {
                    text: text.clone(),
                    range: heading_start..range.end,
                });
                heading = Some(text);
            }
            Event::Start(Tag::Item) => item_stack.push(ListItem {
                text: String::new(),
                range: range.start..range.end,
            }),
            Event::End(TagEnd::Item) => {
                if let Some(mut completed) = item_stack.pop() {
                    completed.range.end = range.end;
                    document.list_items.push(completed);
                }
            }
            Event::Start(Tag::Paragraph) => {
                paragraph = Some(ListItem {
                    text: String::new(),
                    range: range.start..range.end,
                });
            }
            Event::End(TagEnd::Paragraph) => {
                if let Some(mut completed) = paragraph.take() {
                    completed.range.end = range.end;
                    completed.text = completed.text.trim().to_owned();
                    paragraphs.push(completed);
                }
            }
            Event::Start(Tag::Link { dest_url, .. }) => {
                let path = dest_url.into_string();
                if is_authoritative_resource_path(&path) {
                    document.references.push(ResourceReference {
                        path,
                        kind: ReferenceKind::MarkdownLink,
                        range,
                        heading: heading.clone(),
                        prose_context: None,
                        list_context: Vec::new(),
                    });
                }
            }
            Event::Code(code) => {
                let value = code.into_string();
                append_text(
                    &mut heading_text,
                    &mut item_stack,
                    paragraph.as_mut(),
                    &value,
                );
                if is_authoritative_resource_path(&value) {
                    document.references.push(ResourceReference {
                        path: value,
                        kind: ReferenceKind::InlineCode,
                        range,
                        heading: heading.clone(),
                        prose_context: None,
                        list_context: Vec::new(),
                    });
                }
            }
            Event::Text(text) => {
                let value = text.into_string();
                append_text(
                    &mut heading_text,
                    &mut item_stack,
                    paragraph.as_mut(),
                    &value,
                );
                for (candidate, candidate_range) in filename_candidates(&value, range.start) {
                    document.warnings.push(Warning {
                        candidate,
                        range: candidate_range,
                        heading: heading.clone(),
                        prose_context: None,
                        list_context: Vec::new(),
                    });
                }
            }
            Event::SoftBreak | Event::HardBreak => {
                append_text(&mut heading_text, &mut item_stack, paragraph.as_mut(), " ")
            }
            _ => {}
        }
    }

    document.list_items.sort_by_key(|item| item.range.start);
    for item in &mut document.list_items {
        item.text = source[item.range.clone()].trim().to_owned();
    }
    for reference in &mut document.references {
        reference.list_context = list_context(&document.list_items, &reference.range);
        reference.prose_context = prose_context(&paragraphs, &reference.range)
            .or_else(|| reference.list_context.last().cloned());
    }
    for warning in &mut document.warnings {
        warning.list_context = list_context(&document.list_items, &warning.range);
        warning.prose_context = prose_context(&paragraphs, &warning.range)
            .or_else(|| warning.list_context.last().cloned());
    }

    Ok(document)
}

fn append_text(
    heading: &mut String,
    item_stack: &mut [ListItem],
    paragraph: Option<&mut ListItem>,
    value: &str,
) {
    heading.push_str(value);
    for item in item_stack {
        item.text.push_str(value);
    }
    if let Some(paragraph) = paragraph {
        paragraph.text.push_str(value);
    }
}

fn prose_context(paragraphs: &[ListItem], range: &Range<usize>) -> Option<String> {
    paragraphs
        .iter()
        .find(|paragraph| paragraph.range.start <= range.start && range.end <= paragraph.range.end)
        .map(|paragraph| paragraph.text.clone())
}

fn list_context(items: &[ListItem], range: &Range<usize>) -> Vec<String> {
    items
        .iter()
        .filter(|item| item.range.start <= range.start && range.end <= item.range.end)
        .map(|item| item.text.clone())
        .collect()
}

fn is_authoritative_resource_path(path: &str) -> bool {
    let Some(relative) = path
        .strip_prefix("resources/")
        .or_else(|| path.strip_prefix("references/"))
    else {
        return false;
    };

    !relative.is_empty()
        && !path.contains("://")
        && !path.contains(['#', '?', '\\'])
        && relative
            .split('/')
            .all(|component| !component.is_empty() && component != "." && component != "..")
}

fn filename_candidates(value: &str, offset: usize) -> Vec<(String, Range<usize>)> {
    let mut candidates = Vec::new();
    let mut cursor = 0;

    for token in value.split_whitespace() {
        let start = value[cursor..]
            .find(token)
            .map(|index| cursor + index)
            .unwrap_or(cursor);
        cursor = start + token.len();
        let candidate = token.trim_matches(|character: char| {
            matches!(
                character,
                '.' | ',' | ':' | ';' | '!' | '?' | ')' | ']' | '}' | '"' | '\''
            )
        });

        if is_free_filename(candidate) {
            let candidate_start = start + token.find(candidate).unwrap_or_default();
            candidates.push((
                candidate.to_owned(),
                offset + candidate_start..offset + candidate_start + candidate.len(),
            ));
        }
    }

    candidates
}

fn is_free_filename(value: &str) -> bool {
    !value.contains('/')
        && !value.contains("://")
        && value
            .rsplit_once('.')
            .is_some_and(|(stem, extension)| !stem.is_empty() && is_supported_extension(extension))
}

fn is_supported_extension(extension: &str) -> bool {
    matches!(extension, "md" | "txt" | "json" | "yaml" | "yml" | "toml")
}
