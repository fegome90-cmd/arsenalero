# Contributing

Do not work on `main`. Work only in `/Users/felipe_gonzalez/Developer/arsenalero-bootstrap-worktree` and use a focused Conventional Commit for each approved task. Do not modify `/Users/felipe_gonzalez/Developer/arsenalero`.

Before using an external library API, complete the Context7 evidence gate and record the selected version and contract in `docs/evidence/context7-ledger.md`. Keep changes within the approved task scope, use TDD for executable behavior, and obtain a fresh review after the change.

## Task 5 validation and commit

Task 4 is complete at `bbc3cc9`. Task 5 is the active slice: read-only filesystem path policy. It may implement only `PathPolicy`, `CanonicalSkillRoot`, and `CanonicalResourcePath`, with fail-closed canonical containment and traversal, symlink, type, size, extension, and path-length checks covered by focused tests and fixtures.

Permitted paths are `crates/arsenalero-core/src/path_policy.rs`, required `crates/arsenalero-core/src/lib.rs` wiring, focused core path-policy tests and fixtures, `crates/arsenalero-core/Cargo.toml`, `Cargo.lock` for the approved `proptest` dev-dependency, and truthful Context7 ledger/manifest updates. Do not alter Bootstrap or Task 4 history, copied authority documents, scanner or metadata parsing, classification, receipts/UUID/digest behavior, journal/reconciliation, MCP handlers, or domain tools.

Run these commands from the isolated worktree:

```sh
cargo fmt --all --check
cargo test --package arsenalero-core --locked
cargo check --workspace --locked
git diff --check
git status --short
```

Inspect the diff and status to confirm only permitted Task 5 paths changed. Implementers do not stage or commit. The parent orchestrator stages the reviewed paths, validates the content-bound receipt, and makes the planned commit:

```text
feat: enforce read-only skill path policy
```

The MCP server remains a zero-domain-tool stdio boundary. Task 6 is permitted only after Task 5 is reviewed and committed.
