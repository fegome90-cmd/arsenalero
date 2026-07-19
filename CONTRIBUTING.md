# Contributing

Do not work on `main`. Work only in focused feature branches (`feature/*`, `fix/*`, `chore/*`, `eval/*`) and merge to `main` via PR. See `docs/governance/BRANCHING_MODEL.md`. Every push requires the two-phase barrier: prepare locally → report exact diff and refs → explicit human approval → push.

Before using an external library API, complete the Context7 evidence gate and record the selected version and contract in `docs/evidence/context7-ledger.md`. Keep changes within the approved task scope, use TDD for executable behavior, and obtain a fresh review after the change.

## S1 validation and commit

Task 4 is complete at `bbc3cc9`, Task 5 at `4b7e953`, and Task 6 (Markdown scanner and metadata parser) is historical record. S1 is closed (see docs/governance/archive/POST_RELEASE_STABILIZATION_S1_AUTHORITY.md). The current active slice is Branch Consolidation B1 (Recovery), authorized by docs/governance/BRANCH_CONSOLIDATION_B1_AUTHORITY.md (authority item 11). See that addendum for permitted paths, validations, and stop conditions.

Permitted paths are defined by the active slice. The current active slice is Branch Consolidation B1 (Recovery); see `docs/governance/BRANCH_CONSOLIDATION_B1_AUTHORITY.md` section 3 for the exhaustive permitted-paths list. S1's historical permitted paths (`crates/arsenalero-mcp/src/main.rs` line 10 only, the `[[test]]` block in `Cargo.toml`, the new `tests/integration/version_flag.rs`, `README.md` gap-text removal, etc.) are historical record. Do not alter Bootstrap, Task 4, Task 5, or Task 6 history; copied authority documents; classification; digests/UUIDs; receipts; journal/reconciliation; MCP handlers or tools; filesystem access; execution; network access; or HTML/script execution.

Run these commands from the isolated worktree:

```sh
cargo fmt --all --check
cargo test --package arsenalero-mcp --test version_flag --locked
cargo test --workspace --locked
cargo check --workspace --locked
git diff --check
git status --short
```

Inspect the diff and status to confirm only permitted paths for the active slice changed. Implementers do not stage or commit. The parent orchestrator executes the active slice's commit plan under the three-gate workflow (see `docs/governance/BRANCH_CONSOLIDATION_B1_AUTHORITY.md` and `docs/governance/BRANCHING_MODEL.md`).

The MCP server remains a zero-domain-tool stdio boundary (S1 adds no tools), and the implementation exposes exactly five tools. Historical Task 7 deterministic classification was permitted after Task 6; B1 Recovery is the currently permitted slice. Future slices require a new human-approved slice.
