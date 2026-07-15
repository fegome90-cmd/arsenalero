# AGENTS.md

Repository-level operating rules for the Arsenalero MCP repository.

## Bootstrap boundary

This repository is in **bootstrap-only** scope until the four authorized bootstrap commits are complete. Bootstrap Commit 1 and Bootstrap Commit 2 are complete. The current slice is **Bootstrap Commit 3: Rust MCP plugin workspace scaffold**.

- Commit 3 may contain only the Rust workspace/crates, plugin metadata, MCP stdio scaffold, toolchain, license, and repository metadata required by its exact commit plan entry.
- Do not create Commit 4 artifacts: integration tests, fixtures, eval contracts, CI, `deny.toml`, or a final report.
- Do not create domain tools, domain handlers, or domain code before Task 4 of the approved v1.3 implementation plan.
- Do not create fake handlers or advertise future domain tools before their real contracts and implementations exist.
- Update owned dependency evidence only to keep the Commit 3 ledger and manifest truthful about this scaffold; preserve copied authority hashes and byte counts.
- Implementers do not stage or commit and do not modify another worktree. The sole target for implementation is `/Users/felipe_gonzalez/Developer/arsenalero-bootstrap-worktree`.
- Work risk is `MEDIUM`; use at most one implementer per task and perform a fresh review after every task.

The bootstrap establishes governance, dependency evidence, and a zero-domain-tool stdio scaffold. It may truthfully claim completed formatting and compilation checks, but not unrun runtime MCP integration tests or domain behavior.

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

The Constitution is copied verbatim from the authorized `constitucion-ai` commit. The SDD, plan, audit, review findings, changelog, Context7 protocol, and input report are copied authority inputs from their authorized sources. The Commit 3-owned Context7 ledger and manifest record current scaffold evidence and must preserve the copied-source provenance; do not silently reinterpret or rewrite authority copies.

## Required workflow

1. Work only in the isolated branch/worktree authorized for the current slice; never work on `main`.
2. Read applicable instructions and authority documents before mutation.
3. Keep changes minimal, attributable, reversible, and limited to the declared slice.
4. Preserve a truthful relationship between authority, diff, validation, and evidence.
5. Stop rather than inventing an API, schema, dependency, security control, or authority decision.

## TDD and Context7 gates

- TDD is mandatory for every future executable behavior: RED (write a failing test), GREEN (minimal implementation), REFACTOR (keep tests green), then run the relevant quality gates.
- Commit 3 contains only the zero-domain-tool scaffold; runtime protocol integration tests remain deferred to Commit 4.
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

This slice performs only the third scope above. Implementers do not stage or commit, do not modify another worktree, and do not create any later-commit artifact here. After reviewed changes are complete, the parent orchestrator stages the reviewed paths, validates the content-bound receipt, and commits the exact Conventional Commit message for this slice.

The eventual implementation exposes exactly five tools: `arsenal_init`, `arsenal_stage`, `arsenal_issue`, `arsenal_attest`, and `arsenal_reconcile`. The bootstrap server exposes zero tools; it must not create fake handlers or reserve names as advertised capabilities.

## Evidence convention

The authorized evidence convention is explicit:

- `bootstrap-manifest.json` records the **pre-finalization commit SHA** and the byte/hash provenance of authority copies. Its Commit 2 evidence may be corrected in this slice only to reflect the current ledger and known pre-Commit-3 SHA.
- The **final commit SHA** is recorded only in an external post-commit report. Do not place the final commit SHA in `bootstrap-manifest.json` or any in-tree artifact.
- Evidence must distinguish planned controls from controls present in the current tree. Never claim a validation command, test, build, plugin check, or commit that was not actually run or created.

## Prohibited attribution

Commit messages and repository artifacts must not contain `Co-Authored-By` trailers or AI attribution. Use only the exact Conventional Commit messages above when the authorized commit phases are executed.

## Stop conditions

Stop and report a blocker before mutation when:

- an authority source is missing, cannot be verified, or differs from its required verbatim copy;
- the target worktree is not the declared isolated branch or contains unexplained prior changes;
- a requested artifact belongs to Commit 4 or another worktree;
- completing the slice requires changing the SDD, plan, copied authority inputs, or any evidence claim beyond truthful Commit 3 status;
- a generated artifact would introduce runtime behavior, domain code, fake tools, unsafe capability, or an unrecorded dependency;
- evidence would require claiming a test, build, commit, or final SHA that does not exist.

## Validation expectations

For this slice, validate the workspace with `cargo fmt --all --check` and `cargo check --workspace --locked`; validate plugin JSON with the official plugin validator and scope checks; verify manifest ledger bytes/hash; inspect forbidden-path and zero-domain-tool absence; and run `git diff --cached --check` for the staged candidate. Implementers do not stage; the parent orchestrator stages the reviewed paths before this candidate validation. Do not claim runtime MCP integration, CI, eval, or domain behavior results.

## References

- `docs/governance/BOOTSTRAP_CHARTER.md`
- `docs/architecture/ARSENALERO_MCP_SDD_v1.3.md`
- `docs/plans/ARSENALERO_MCP_IMPLEMENTATION_PLAN_v1.3.md`
- `docs/governance/CONTEXT7_EVIDENCE_PROTOCOL.md`
