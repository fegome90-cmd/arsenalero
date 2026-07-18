# AGENTS.md

Repository-level operating rules for the Arsenalero MCP repository.

## Bootstrap and active-slice boundary

Bootstrap is complete through **Bootstrap Commit 4** (`479700012a7b20dbcfead01b1af0ec25ffa06308`). **Task 4: domain model and reason codes** is complete at `bbc3cc9a3bc4bca4090c1cfce4b451374d212646`, and **Task 5: read-only filesystem path policy** is complete at `4b7e953`. Task 6 (Markdown scanner and metadata parser) is complete in the historical record. Slice S1 is closed. The current active slice is **Branch Consolidation B1 (Recovery)** (consolidate `main` and the bootstrap MCP line from `5ae4de0` into a single canonical history, after quarantining unauthorized M2 gitflow commits), authorized by `docs/governance/BRANCH_CONSOLIDATION_B1_AUTHORITY.md` (authority item 11, APPROVED/ACTIVE).

**Task 6 historical record:** Task 6 established pure source-string, event-based Markdown scanning with `pulldown-cmark =0.13.4` and `default-features=false`, preserving source byte ranges, extracting relative resource links, covering inline resource and reference code paths, emitting free-filename warnings, retaining heading/list/adjacent context, and optionally parsing Arsenal frontmatter metadata. That work is committed; nothing in S1 reopens it.

S1 may change only:

- `crates/arsenalero-mcp/src/main.rs` line 10 only (the `--version`/`-V` branch prints `arsenalero {CARGO_PKG_VERSION}` and returns `Ok(())`); lines 9, 11, 12, 13 are unchanged;
- one new `[[test]]` block in `crates/arsenalero-mcp/Cargo.toml` mirroring the existing `mcp_bootstrap_stdio` convention (no dependency, feature, or version change);
- the new file `tests/integration/version_flag.rs` with three tests per the S1 test spec;
- `README.md` gap-text removal (the "Known gap — no `--version` flag" paragraph near line 75 and the "No `--version` flag" bullet near line 110);
- `AGENTS.md` and `CONTRIBUTING.md` Task 6 → S1 rewrites plus the insertion of item 10 in the authority hierarchy;
- `docs/governance/POST_RELEASE_STABILIZATION_S1_AUTHORITY.md` (new, this slice's operative authority);
- one appended entry in `docs/evidence/context7-ledger.md` with a Result block filled from actual test output.

Do not modify Bootstrap, Task 4, Task 5, or Task 6 history, copied authority documents, or another worktree. Do not add classification, digests/UUIDs, receipts, journal/reconciliation, MCP handlers or tools, filesystem access, execution, network access, or HTML/script execution. The MCP server remains a zero-domain-tool stdio boundary. Task 7 deterministic classification is the next permitted task only after S1 is reviewed, committed, and closed.

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
10. `docs/governance/archive/POST_RELEASE_STABILIZATION_S1_AUTHORITY.md` — historical authority record for the closed slice S1; NO LONGER binding-operative. Items 1-9 resume full authority. New slices must follow the Addendum authorization rule recorded in the archived document (section 5b): an agent may draft but must not approve or activate.
11. `docs/governance/BRANCH_CONSOLIDATION_B1_AUTHORITY.md` — binding-operative for Branch Consolidation B1 (Recovery) only; declared APPROVED/ACTIVE on 2026-07-18 by human approval recorded inside the addendum.

The Constitution, SDD, plan, audits, Context7 protocol, and input report are copied authority inputs. Preserve their provenance and do not rewrite authority copies. The owned Context7 ledger and bootstrap manifest may receive only truthful S1 evidence updates; `bootstrap-manifest.json` is a byte-identical historical snapshot and S1 does not supersede it.

## Required workflow

1. Work only in `<arsenalero-root>`; never work on `main` or `<arsenalero-root>`.
2. Read applicable instructions and authority documents before mutation.
3. Use TDD for S1 behavior: RED, GREEN, REFACTOR for the `--version`/`-V` flag and the no-flags still-alive contract. S1 adds no new dependency, so no Context7 contract resolution is required.
4. Keep changes minimal, attributable, reversible, and limited to the declared paths.
5. Implementers do not stage or commit. The parent orchestrator stages reviewed paths, validates the content-bound receipt, and makes the planned S1 commits in order C1 → C2 → C3 (orchestrator-enforced per the addendum's section 4).
6. Stop rather than inventing an API, schema, dependency, security control, or authority decision.

## Safety restrictions

Fail closed. Do not add or enable network access, HTTP listeners, remote MCP, shell or arbitrary process execution, hooks, databases, secrets, writes inside skill roots, internal LLMs, embeddings, RAG, AST/LSP/graph machinery, UI, dynamic tools, or semantic verification claims.

Historical Task 6 scanned supplied source strings only. S1 inherits the same fail-closed restrictions: it changes only the `--version`/`-V` branch of `main.rs`, adds the test file, and updates documentation. S1 adds no MCP handler, tool, filesystem access, network, execution, or new dependency.

## Exact bootstrap commit plan

The complete bootstrap is exactly these four Conventional Commits, with no squash:

1. `docs: establish constitutional bootstrap baseline`
2. `docs: record bootstrap dependency evidence`
3. `chore: scaffold Rust MCP plugin workspace`
4. `test: verify bootstrap MCP and eval contracts`

Those commits and Task 4 commit `bbc3cc9` and Task 5 commit `4b7e953` are historical record. Do not rewrite them. The eventual implementation exposes exactly five tools: `arsenal_init`, `arsenal_stage`, `arsenal_issue`, `arsenal_attest`, and `arsenal_reconcile`; Task 6 exposes none. S1 exposes no new tools either; it only adds the `--version`/`-V` flag and its test.

## Evidence convention

`bootstrap-manifest.json` records the pre-finalization commit SHA and copied-authority provenance. The final commit SHA belongs only in an external post-commit report. Evidence must distinguish planned controls from controls present in the current tree; never claim a command, test, build, plugin check, or commit that was not run.

## Prohibited attribution

Commit messages and repository artifacts must not contain `Co-Authored-By` trailers or AI attribution.

## Stop conditions

Stop and report a blocker before mutation when an authority source is missing or differs from its required verbatim copy; the target is not the declared worktree or has unexplained changes; a requested artifact is outside S1 paths (or historical Task 6 paths); the slice requires a prohibited behavior; or an evidence claim cannot be made truthfully.

## Validation expectations

For S1, run `cargo test --package arsenalero-mcp --test version_flag --locked`, `cargo test --workspace --locked`, `cargo fmt --all --check`, `cargo check --workspace --locked`, `git diff --check`, and a scope inspection confirming only permitted S1 paths changed. Implementers do not stage. The parent orchestrator stages and commits in order C1 → C2 → C3. Do not claim MCP runtime, filesystem access, classification, digest/UUID, receipt, journal/reconciliation, or handler/tool results beyond what S1 actually executes.

## References

- `docs/governance/BOOTSTRAP_CHARTER.md`
- `docs/architecture/ARSENALERO_MCP_SDD_v1.3.md`
- `docs/plans/ARSENALERO_MCP_IMPLEMENTATION_PLAN_v1.3.md`
- `docs/governance/CONTEXT7_EVIDENCE_PROTOCOL.md`
- `docs/governance/POST_RELEASE_STABILIZATION_S1_AUTHORITY.md`
- `docs/governance/BRANCHING_MODEL.md`
