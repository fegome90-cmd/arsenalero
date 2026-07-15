# AGENTS.md

Repository-level operating rules for the Arsenalero MCP repository.

## Bootstrap and active-slice boundary

Bootstrap is complete through **Bootstrap Commit 4** (`479700012a7b20dbcfead01b1af0ec25ffa06308`). **Task 4: domain model and reason codes** is complete at `bbc3cc9a3bc4bca4090c1cfce4b451374d212646`. The active slice is **Task 5: read-only filesystem path policy**.

Task 5 establishes only `PathPolicy`, `CanonicalSkillRoot`, and `CanonicalResourcePath`: fail-closed canonical containment plus traversal, symlink, type, size, extension, and path-length checks, with focused tests and fixtures. It must preserve read-only skill roots.

Task 5 may change only:

- `crates/arsenalero-core/src/path_policy.rs` and required `crates/arsenalero-core/src/lib.rs` wiring;
- focused core path-policy tests and path-policy fixtures;
- `crates/arsenalero-core/Cargo.toml` and `Cargo.lock` only for the approved `proptest` core dev-dependency;
- truthful Task 5 updates to the owned Context7 ledger and bootstrap manifest.

Do not modify Bootstrap or Task 4 history, copied authority documents, or another worktree. Do not add scanner or metadata parsing, classification, receipts/UUID/digest behavior, journal/reconciliation, MCP handlers, or domain tools. The MCP server remains a zero-domain-tool stdio boundary. Task 6 is the next permitted task only after Task 5 is reviewed and committed.

## Authority hierarchy

When sources conflict, apply this order:

1. `AGENTIC-CONSTITUTION-v1.0.md`
2. `docs/architecture/ARSENALERO_MCP_SDD_v1.3.md`
3. `docs/audit/REVIEW_FINDINGS_v1.3.md`
4. `docs/audit/CHANGELOG_v1.3.md`
5. `docs/plans/ARSENALERO_MCP_IMPLEMENTATION_PLAN_v1.3.md`
6. The approved `CONTEXT7_EVIDENCE_PROTOCOL.md` authority copy, owned by the dependency-evidence slice
7. `docs/audit/ARSENALERO_MCP_AUDIT_AI_ENGINEERING_v1.3.md`
8. The approved `INPUT_REPORT_v1.1.md` authority input, owned by the later authority-archive slice
9. Current official library documentation, only for API/version details and never for domain scope.

The Constitution, SDD, plan, audits, Context7 protocol, and input report are copied authority inputs. Preserve their provenance and do not rewrite authority copies. The owned Context7 ledger and bootstrap manifest may receive only truthful Task 5 evidence updates.

## Required workflow

1. Work only in `/Users/felipe_gonzalez/Developer/arsenalero-bootstrap-worktree`; never work on `main` or `/Users/felipe_gonzalez/Developer/arsenalero`.
2. Read applicable instructions and authority documents before mutation.
3. Use TDD for Task 5 behavior: RED, GREEN, REFACTOR; resolve and record the approved `proptest` contract in Context7 before use.
4. Keep changes minimal, attributable, reversible, and limited to the declared paths.
5. Implementers do not stage or commit. The parent orchestrator stages reviewed paths, validates the content-bound receipt, and makes the planned commit: `feat: enforce read-only skill path policy`.
6. Stop rather than inventing an API, schema, dependency, security control, or authority decision.

## Safety restrictions

Fail closed. Do not add or enable network access, HTTP listeners, remote MCP, shell or arbitrary process execution, hooks, databases, secrets, writes inside skill roots, internal LLMs, embeddings, RAG, AST/LSP/graph machinery, UI, dynamic tools, or semantic verification claims.

Task 5 may validate resource paths only. It must not scan or parse resource metadata/content, classify skills, create receipts/UUIDs/digests, journal or reconcile state, or expose MCP handlers.

## Exact bootstrap commit plan

The complete bootstrap is exactly these four Conventional Commits, with no squash:

1. `docs: establish constitutional bootstrap baseline`
2. `docs: record bootstrap dependency evidence`
3. `chore: scaffold Rust MCP plugin workspace`
4. `test: verify bootstrap MCP and eval contracts`

Those commits and Task 4 commit `bbc3cc9` are historical record. Do not rewrite them. The eventual implementation exposes exactly five tools: `arsenal_init`, `arsenal_stage`, `arsenal_issue`, `arsenal_attest`, and `arsenal_reconcile`; Task 5 exposes none.

## Evidence convention

`bootstrap-manifest.json` records the pre-finalization commit SHA and copied-authority provenance. The final commit SHA belongs only in an external post-commit report. Evidence must distinguish planned controls from controls present in the current tree; never claim a command, test, build, plugin check, or commit that was not run.

## Prohibited attribution

Commit messages and repository artifacts must not contain `Co-Authored-By` trailers or AI attribution.

## Stop conditions

Stop and report a blocker before mutation when an authority source is missing or differs from its required verbatim copy; the target is not the declared worktree or has unexplained changes; a requested artifact is outside Task 5 paths; the slice requires a prohibited behavior; or an evidence claim cannot be made truthfully.

## Validation expectations

For Task 5, run focused path-policy tests, `cargo fmt --all --check`, `cargo check --workspace --locked`, `git diff --check`, and a scope inspection confirming only permitted Task 5 paths changed. Implementers do not stage. The parent orchestrator stages the reviewed paths before receipt validation and the planned Task 5 commit. Do not claim MCP runtime, scanner, metadata parsing, classification, receipt/UUID/digest, journal/reconciliation, or handler results.

## References

- `docs/governance/BOOTSTRAP_CHARTER.md`
- `docs/architecture/ARSENALERO_MCP_SDD_v1.3.md`
- `docs/plans/ARSENALERO_MCP_IMPLEMENTATION_PLAN_v1.3.md`
- `docs/governance/CONTEXT7_EVIDENCE_PROTOCOL.md`
