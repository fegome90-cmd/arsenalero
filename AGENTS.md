# AGENTS.md

Repository-level operating rules for the Arsenalero MCP repository.

## Bootstrap and active-slice boundary

Bootstrap is complete through **Bootstrap Commit 4** (`479700012a7b20dbcfead01b1af0ec25ffa06308`). **Task 4: domain model and reason codes** is complete at `bbc3cc9a3bc4bca4090c1cfce4b451374d212646`, and **Task 5: read-only filesystem path policy** is complete at `4b7e953`. The active slice is **Task 6: Markdown scanner and metadata parser**.

Task 6 establishes only pure source-string, event-based Markdown scanning with `pulldown-cmark =0.13.4` and `default-features=false`. It may preserve source byte ranges; extract relative resource links; cover inline resource and reference code paths; emit free-filename warnings; retain heading, list, and adjacent context; and optionally parse Arsenal frontmatter metadata.

Task 6 may change only:

- existing Task 6 implementation paths, including required `crates/arsenalero-core/src/lib.rs` wiring;
- focused Task 6 tests and fixtures already present in the worktree;
- `crates/arsenalero-core/Cargo.toml` and `Cargo.lock` only for the approved Markdown parser dependency;
- truthful Task 6 updates to the owned Context7 ledger and bootstrap manifest.

Do not modify Bootstrap, Task 4, or Task 5 history, copied authority documents, or another worktree. Do not add classification, digests/UUIDs, receipts, journal/reconciliation, MCP handlers or tools, filesystem access, execution, network access, or HTML/script execution. The MCP server remains a zero-domain-tool stdio boundary. Task 7 deterministic classification is the next permitted task only after Task 6 is reviewed and committed.

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

The Constitution, SDD, plan, audits, Context7 protocol, and input report are copied authority inputs. Preserve their provenance and do not rewrite authority copies. The owned Context7 ledger and bootstrap manifest may receive only truthful Task 6 evidence updates.

## Required workflow

1. Work only in `<arsenalero-root>`; never work on `main` or `<arsenalero-root>`.
2. Read applicable instructions and authority documents before mutation.
3. Use TDD for Task 6 behavior: RED, GREEN, REFACTOR; resolve and record the approved `pulldown-cmark =0.13.4` contract in Context7 before use.
4. Keep changes minimal, attributable, reversible, and limited to the declared paths.
5. Implementers do not stage or commit. The parent orchestrator stages reviewed paths, validates the content-bound receipt, and makes the planned Task 6 commit.
6. Stop rather than inventing an API, schema, dependency, security control, or authority decision.

## Safety restrictions

Fail closed. Do not add or enable network access, HTTP listeners, remote MCP, shell or arbitrary process execution, hooks, databases, secrets, writes inside skill roots, internal LLMs, embeddings, RAG, AST/LSP/graph machinery, UI, dynamic tools, or semantic verification claims.

Task 6 may scan supplied source strings only. It must not access the filesystem, classify skills, create digests/UUIDs or receipts, journal or reconcile state, expose MCP handlers or tools, execute processes, use the network, or execute HTML or scripts.

## Exact bootstrap commit plan

The complete bootstrap is exactly these four Conventional Commits, with no squash:

1. `docs: establish constitutional bootstrap baseline`
2. `docs: record bootstrap dependency evidence`
3. `chore: scaffold Rust MCP plugin workspace`
4. `test: verify bootstrap MCP and eval contracts`

Those commits and Task 4 commit `bbc3cc9` and Task 5 commit `4b7e953` are historical record. Do not rewrite them. The eventual implementation exposes exactly five tools: `arsenal_init`, `arsenal_stage`, `arsenal_issue`, `arsenal_attest`, and `arsenal_reconcile`; Task 6 exposes none.

## Evidence convention

`bootstrap-manifest.json` records the pre-finalization commit SHA and copied-authority provenance. The final commit SHA belongs only in an external post-commit report. Evidence must distinguish planned controls from controls present in the current tree; never claim a command, test, build, plugin check, or commit that was not run.

## Prohibited attribution

Commit messages and repository artifacts must not contain `Co-Authored-By` trailers or AI attribution.

## Stop conditions

Stop and report a blocker before mutation when an authority source is missing or differs from its required verbatim copy; the target is not the declared worktree or has unexplained changes; a requested artifact is outside Task 6 paths; the slice requires a prohibited behavior; or an evidence claim cannot be made truthfully.

## Validation expectations

For Task 6, run focused Markdown scanner and metadata-parser tests, `cargo fmt --all --check`, `cargo check --workspace --locked`, `git diff --check`, and a scope inspection confirming only permitted Task 6 paths changed. Implementers do not stage. The parent orchestrator stages the reviewed paths before receipt validation and the planned Task 6 commit. Do not claim MCP runtime, filesystem access, classification, digest/UUID, receipt, journal/reconciliation, or handler/tool results.

## References

- `docs/governance/BOOTSTRAP_CHARTER.md`
- `docs/architecture/ARSENALERO_MCP_SDD_v1.3.md`
- `docs/plans/ARSENALERO_MCP_IMPLEMENTATION_PLAN_v1.3.md`
- `docs/governance/CONTEXT7_EVIDENCE_PROTOCOL.md`
