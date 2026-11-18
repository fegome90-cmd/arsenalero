# AGENTS.md

Repository-level operating rules for the Arsenalero MCP repository.

## Bootstrap and active-slice boundary

Bootstrap is complete through **Bootstrap Commit 4** (`479700012a7b20dbcfead01b1af0ec25ffa06308`). **Task 4: domain model and reason codes** is complete at `bbc3cc9a3bc4bca4090c1cfce4b451374d212646`, and **Task 5: read-only filesystem path policy** is complete at `4b7e953`. Task 6 (Markdown scanner and metadata parser) and Slice S1 are historical record. PR #1 merged to `main` at `9375f83efe715a799089ddaaedc9075415e0e519`; PR #2 prepares the archive and closure of Branch Consolidation B1 (Recovery). Until PR #2 merges, B1 remains operative only for that PR #2 closure scope. After PR #2 merges, no slice remains active and future work requires a new durable addendum approved by a human.

**Task 6 historical record:** Task 6 established pure source-string, event-based Markdown scanning with `pulldown-cmark =0.13.4` and `default-features=false`, preserving source byte ranges, extracting relative resource links, covering inline resource and reference code paths, emitting free-filename warnings, retaining heading/list/adjacent context, and optionally parsing Arsenal frontmatter metadata. That work is committed; nothing in S1 reopens it.

S1 is closed (see `docs/governance/archive/POST_RELEASE_STABILIZATION_S1_AUTHORITY.md`). B1 remains operative only for the PR #2 closure scope until PR #2 merges (see `docs/governance/archive/BRANCH_CONSOLIDATION_B1_AUTHORITY.md`). After that merge, no active slice defines permitted paths or operations.

Do not modify Bootstrap, Task 4, Task 5, or Task 6 history, copied authority documents, or another worktree. Do not add classification, digests/UUIDs, receipts, journal/reconciliation, MCP handlers or tools, filesystem access, execution, network access, or HTML/script execution. The MCP server remains a zero-domain-tool stdio boundary. Task 7 remains deferred historical work and is not authorized by this closure. Future implementation work requires a new durable human-approved governance addendum.

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
11. `docs/governance/archive/BRANCH_CONSOLIDATION_B1_AUTHORITY.md` — closure-prepared historical authority record for Branch Consolidation B1 (Recovery). It remains binding-operative only for PR #2's closure scope until PR #2 merges; its incident evidence and retained incident tags remain historical record. After that merge, new slices require a new durable human-approved addendum.

The Constitution, SDD, plan, audits, Context7 protocol, and input report are copied authority inputs. Preserve their provenance and do not rewrite authority copies. The owned Context7 ledger and bootstrap manifest may receive only truthful S1 evidence updates; `bootstrap-manifest.json` is a byte-identical historical snapshot and S1 does not supersede it.

## Required workflow

1. Work only in the isolated worktree declared by B1's remaining PR #2 closure scope or by a future active slice. Never work directly on `main` or in another worktree.
2. Read applicable instructions and authority documents before mutation.
3. Use TDD for any executable behavior introduced by future slices.
   B1 permits only its remaining PR #2 closure scope until that PR merges.
   Future addenda define their own behavior and validation requirements.
4. Keep changes minimal, attributable, reversible, and limited to the declared paths.
5. Implementers do not stage or commit. The parent orchestrator stages
   reviewed paths, validates the content-bound receipt, and executes the
   active slice's commit plan under the three-gate workflow (see
   `docs/governance/BRANCHING_MODEL.md`).
6. Stop rather than inventing an API, schema, dependency, security control, or authority decision.

## Safety restrictions

Fail closed. Do not add or enable network access, HTTP listeners, remote MCP, shell or arbitrary process execution, hooks, databases, secrets, writes inside skill roots, internal LLMs, embeddings, RAG, AST/LSP/graph machinery, UI, dynamic tools, or semantic verification claims.

Historical Task 6 scanned supplied source strings only; historical S1
changed only the --version/-V branch of main.rs and added the test file.
The B1 Recovery addendum is governance-only and remains operative only for
PR #2 closure until that PR merges. Future slices inherit the fail-closed
restrictions.

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

Stop and report a blocker before mutation when an authority source is
missing or differs from its required verbatim copy; the target is not
the declared worktree or has unexplained changes; no current durable
human-approved addendum authorizes the requested artifact; the slice
requires a prohibited behavior; or an evidence claim cannot be made
truthfully.

## Validation expectations

Future slices define their own validation requirements in their durable
human-approved addenda. Implementers do not stage or commit. The parent
orchestrator executes the applicable three-gate workflow.

## References

- `docs/governance/BOOTSTRAP_CHARTER.md`
- `docs/architecture/ARSENALERO_MCP_SDD_v1.3.md`
- `docs/plans/ARSENALERO_MCP_IMPLEMENTATION_PLAN_v1.3.md`
- `docs/governance/CONTEXT7_EVIDENCE_PROTOCOL.md`
- `docs/governance/archive/POST_RELEASE_STABILIZATION_S1_AUTHORITY.md`
- `docs/governance/archive/BRANCH_CONSOLIDATION_B1_AUTHORITY.md`
- `docs/governance/BRANCHING_MODEL.md`
