# Contributing

Do not work on `main`. Work only in focused feature branches (`feature/*`, `fix/*`, `chore/*`, `eval/*`) and merge to `main` via PR. See `docs/governance/BRANCHING_MODEL.md`. Every push requires the two-phase barrier: prepare locally → report exact diff and refs → explicit human approval → push.

Before using an external library API, complete the Context7 evidence gate and record the selected version and contract in `docs/evidence/context7-ledger.md`. Keep changes within the approved task scope, use TDD for executable behavior, and obtain a fresh review after the change.

## Validation and commit workflow

Task 4 is complete at `bbc3cc9`, Task 5 at `4b7e953`, and Task 6 (Markdown scanner and metadata parser) is historical record. S1 is closed (see `docs/governance/archive/POST_RELEASE_STABILIZATION_S1_AUTHORITY.md`). PR #1 merged at `9375f83efe715a799089ddaaedc9075415e0e519`; PR #2 prepares the archive and closure of Branch Consolidation B1 (Recovery). B1 remains operative only for that PR #2 closure scope until PR #2 merges; see `docs/governance/archive/BRANCH_CONSOLIDATION_B1_AUTHORITY.md`. After that merge, no slice remains active and any future work requires a new durable addendum approved by a human.

Permitted paths are defined by a current durable human-approved addendum. Until PR #2 merges, B1 permits only its PR #2 closure scope; after that merge, no addendum is active. S1's historical permitted paths (`crates/arsenalero-mcp/src/main.rs` line 10 only, the `[[test]]` block in `Cargo.toml`, the new `tests/integration/version_flag.rs`, `README.md` gap-text removal, etc.) are historical record. Do not alter Bootstrap, Task 4, Task 5, or Task 6 history; copied authority documents; classification; digests/UUIDs; receipts; journal/reconciliation; MCP handlers or tools; filesystem access; execution; network access; or HTML/script execution.

Run these commands from the isolated worktree:

```sh
cargo fmt --all --check
cargo test --package arsenalero-mcp --test version_flag --locked
cargo test --workspace --locked
cargo check --workspace --locked
git diff --check
git status --short
```

Inspect the diff and status to confirm only paths permitted by the current durable human-approved addendum changed. Implementers do not stage or commit. The parent orchestrator executes the applicable commit plan under the three-gate workflow (see `docs/governance/BRANCHING_MODEL.md`).

The MCP server remains a zero-domain-tool stdio boundary (S1 adds no tools), and the implementation exposes exactly five tools. Task 7 remains deferred historical work and is not authorized by this closure. Future slices require a new durable human-approved addendum.
