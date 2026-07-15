# Contributing

Do not work on `main`. Use an isolated worktree and a focused conventional commit for each approved task.

Before using an external library API, complete the Context7 evidence gate and record the selected version and contract in `docs/evidence/context7-ledger.md`. Keep changes within the approved task scope, use TDD for executable behavior, and obtain a fresh review after the change.

## Task 4 validation and commit

Run these commands from the repository root. Implementers do not stage or commit; the parent orchestrator stages reviewed paths before receipt validation.

```sh
cargo fmt --all --check
cargo check --workspace --locked
git diff --cached --check
git status --short
```

Task 4 is limited to `crates/arsenalero-core/src/domain.rs`, `crates/arsenalero-core/src/error.rs`, and required `crates/arsenalero-core/src/lib.rs` wiring. Inspect the diff to confirm no other path, MCP handler, filesystem or scanner behavior, classification implementation, receipt, UUID generation, hashing, journal, reconciliation, fixture, or dependency was added.

For executable behavior, follow RED → GREEN → REFACTOR, record Context7 evidence before using an external library API, run the focused validation, then obtain a fresh scope-aware review. Implementers do not stage or commit; the parent orchestrator stages the reviewed Task 4 paths, validates the content-bound receipt, and commits the approved Task 4 Conventional Commit. Bootstrap Commit 4 artifacts are complete historical record and must not be revised here. Task 5 is permitted only after the Task 4 commit.
