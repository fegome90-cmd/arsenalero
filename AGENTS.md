# AGENTS.md

Repository-level operating rules for the Arsenalero MCP repository.

## Bootstrap boundary

This repository is in **bootstrap-only** scope until the four authorized bootstrap commits are complete. This slice is **Bootstrap Commit 1 only** and is documentation/governance-only.

- Do not create Rust, Cargo, plugin, MCP, test, eval, CI, `deny.toml`, bootstrap-manifest, or final-report artifacts in this slice.
- Do not create domain tools, domain handlers, or domain code before Task 4 of the approved v1.3 implementation plan.
- Do not create fake handlers or advertise future domain tools before their real contracts and implementations exist.
- Do not change `docs/evidence/context7-ledger.md`; that file is owned by the dependency-evidence slice.
- Do not stage, commit, or modify another worktree. The sole target for this slice is `/Users/felipe_gonzalez/Developer/arsenalero-bootstrap-worktree`.
- Work risk is `MEDIUM`; use at most one implementer per task and perform a fresh review after every task.

The bootstrap establishes governance and authority. It does not claim that Arsenalero runs, compiles, exposes MCP tools, or passes tests.

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

The Constitution is copied verbatim from the authorized `constitucion-ai` commit. The SDD, plan, audit, review findings, and changelog are copied in this slice. The input report and Context7 protocol remain authoritative external inputs until their owned later slices; do not silently reinterpret or rewrite any of them.

## Required workflow

1. Work only in the isolated branch/worktree authorized for the current slice; never work on `main`.
2. Read applicable instructions and authority documents before mutation.
3. Keep changes minimal, attributable, reversible, and limited to the declared slice.
4. Preserve a truthful relationship between authority, diff, validation, and evidence.
5. Stop rather than inventing an API, schema, dependency, security control, or authority decision.

## TDD and Context7 gates

- TDD is mandatory for every future executable behavior: RED (write a failing test), GREEN (minimal implementation), REFACTOR (keep tests green), then run the relevant quality gates.
- This documentation-only slice has no executable behavior and therefore has no test claim.
- Before writing code against an external library, resolve the official library with Context7, query the version-specific contract, record the result in the owned evidence ledger, write the contract test, implement, and compile. Do not use remembered APIs.
- Dependencies not used by the bootstrap remain deferred. The Context7 protocol copy is documentation authority, not runtime evidence.

## Safety restrictions

Fail closed. Do not add or enable:

- network access, HTTP listeners, remote MCP, shell execution, arbitrary process execution, hooks, databases, secrets, or writes inside skill roots;
- internal LLMs, embeddings, RAG, AST/LSP/graph machinery, UI, dynamic tools, or semantic verification claims;
- runtime behavior that treats resource content as instructions, follows URLs, expands roots, or uses `task_summary` for classification.

Future runtime work must keep skill roots read-only, keep plugin data outside skill roots, enforce explicit allowed roots, reject traversal and symlink escapes, apply size/batch/time limits, and distinguish delivery, attestation, artifact references, and external verification.

## Exact bootstrap commit plan

The complete bootstrap is exactly these four Conventional Commits, with no squash:

1. `docs: establish constitutional bootstrap baseline` — Constitution, AGENTS, charter, approved authority copies, ADRs, and initial threat model.
2. `docs: record bootstrap dependency evidence` — Context7 ledger, dependency decisions, deferred dependencies, and the initial bootstrap manifest.
3. `chore: scaffold Rust MCP plugin workspace` — Rust workspace, crates, plugin metadata, MCP configuration, toolchain, license, and repository metadata.
4. `test: verify bootstrap MCP and eval contracts` — MCP integration test, fixtures, evaluation contracts, CI, dependency policy, and final report.

This slice performs only the first scope above. Do not stage or create any later-commit artifact here.

The eventual implementation exposes exactly five tools: `arsenal_init`, `arsenal_stage`, `arsenal_issue`, `arsenal_attest`, and `arsenal_reconcile`. The bootstrap server exposes zero tools; it must not create fake handlers or reserve names as advertised capabilities.

## Evidence convention

The authorized evidence convention is explicit:

- `bootstrap-manifest.json` records the **pre-finalization commit SHA** and the byte/hash provenance of authority copies. It is created only in the dependency-evidence slice, not here.
- The **final commit SHA** is recorded only in an external post-commit report. Do not place the final commit SHA in `bootstrap-manifest.json` or any in-tree artifact.
- Evidence must distinguish planned controls from controls present in the current tree. Never claim a validation command, test, build, plugin check, or commit that was not actually run or created.

## Prohibited attribution

Commit messages and repository artifacts must not contain `Co-Authored-By` trailers or AI attribution. Use only the exact Conventional Commit messages above when the authorized commit phases are executed.

## Stop conditions

Stop and report a blocker before mutation when:

- an authority source is missing, cannot be verified, or differs from its required verbatim copy;
- the target worktree is not the declared isolated branch or contains unexplained prior changes;
- a requested artifact belongs to a later bootstrap commit or to another worktree;
- completing the slice requires changing the SDD, plan, or owned evidence ledger;
- a generated artifact would introduce runtime behavior, domain code, fake tools, unsafe capability, or an unrecorded dependency;
- evidence would require claiming a test, build, commit, or final SHA that does not exist.

## Validation expectations

For this slice, validate only what exists: source-to-target byte identity for verbatim copies, required section/path presence for authored documents, forbidden-path absence, and `git diff --check` if the working tree can be inspected without staging or committing. Do not report Rust, Cargo, MCP, plugin, CI, test, or eval results for this slice.

## References

- `docs/governance/BOOTSTRAP_CHARTER.md`
- `docs/architecture/ARSENALERO_MCP_SDD_v1.3.md`
- `docs/plans/ARSENALERO_MCP_IMPLEMENTATION_PLAN_v1.3.md`
- `docs/governance/CONTEXT7_EVIDENCE_PROTOCOL.md`
