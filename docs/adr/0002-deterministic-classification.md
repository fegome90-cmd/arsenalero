# ADR-0002: Use Deterministic Classification

## Status

Accepted for the Arsenalero MCP v1.3 design baseline; implementation is deferred to the domain tasks.

## Context

Resource classification must be reproducible, auditable, and safe under hostile or ambiguous input. The SDD defines closed stage aliases and purpose verbs and separates the source of classification from obligation. The audit rejected an internal LLM, open-ended semantic inference, and the rule that every `DECLARED` or `DERIVED` resource is automatically `REQUIRED`.

The following dimensions are independent and must not be collapsed:

- `classification_source`: `DECLARED`, `DERIVED`, or `UNRESOLVED`;
- `obligation`: `REQUIRED`, `RECOMMENDED`, `OPTIONAL`, or `UNKNOWN`;
- `resource_kind`: the kind of resource/evidence contract;
- `evidence_contract`: the evidence expected or supported by the resource.

## Decision

Use deterministic, closed-vocabulary classification with no internal LLM. A resource is `DERIVED` only when the SDD's recognized stage alias and purpose verb conditions are met. A resource is `REQUIRED` only when explicit requirement metadata or a closed normative marker establishes the obligation. Ambiguity remains `UNRESOLVED`/`UNKNOWN`; the classifier must not invent purpose, obligation, or "reasonable equivalents" outside the approved vocabularies.

Bootstrap Commit 1 records this contract only. It does not create a scanner, classifier, obligation calculator, domain tool, or test fixture.

## Consequences

- The same input yields the same classification and is suitable for regression tests.
- Ambiguous content fails closed instead of silently becoming a requirement.
- The model is simpler to audit and does not introduce runtime inference or prompt-injection concerns.
- Closed vocabularies may miss natural-language intent and require explicit SDD updates for expansion.
- Classification remains distinct from evidence attainment and external verification.

## Rejected alternatives

- Internal LLM classification: nondeterministic, costly, and vulnerable to instruction-bearing resource content.
- Semantic similarity or open-ended "reasonable equivalent" matching: not reproducible and difficult to audit.
- `DERIVED` or `DECLARED` automatically implies `REQUIRED`: conflates discovery with normative obligation and creates false incompletes.
- Resource kind or evidence contract determines obligation: the SDD explicitly keeps these dimensions orthogonal.

## Evidence

- `docs/architecture/ARSENALERO_MCP_SDD_v1.3.md`, Sections 10, 11, 13, and 21.
- `docs/audit/ARSENALERO_MCP_AUDIT_AI_ENGINEERING_v1.3.md`, Sections 4.1, 4.2, and 5.
- `docs/audit/REVIEW_FINDINGS_v1.3.md`, especially the final obligation rule.
