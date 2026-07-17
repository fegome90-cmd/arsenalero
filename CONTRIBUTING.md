# Contributing

Do not work on `main` (archived) or `develop`. Work only in focused feature branches (`feature/*`, `fix/*`, `chore/*`, `eval/*`) and merge to `develop` via PR. See `docs/governance/BRANCHING_MODEL.md`.

Before using an external library API, complete the Context7 evidence gate and record the selected version and contract in `docs/evidence/context7-ledger.md`. Keep changes within the approved task scope, use TDD for executable behavior, and obtain a fresh review after the change.

## S1 validation and commit

Task 4 is complete at `bbc3cc9`, Task 5 at `4b7e953`, and Task 6 (Markdown scanner and metadata parser) is historical record. S1 is the current active slice: report package version from Cargo metadata. S1 implements only the `--version`/`-V` flag on the `arsenalero-mcp` binary (printing `arsenalero {CARGO_PKG_VERSION}`, matching the MCP `serverInfo.name`), plus its integration test and documentation updates. S1 adds no dependency.

Permitted S1 paths are `crates/arsenalero-mcp/src/main.rs` line 10 only, one new `[[test]]` block in `crates/arsenalero-mcp/Cargo.toml`, the new file `tests/integration/version_flag.rs`, `README.md` gap-text removal, `AGENTS.md` and `CONTRIBUTING.md` Task 6 → S1 rewrites plus item 10 in the authority hierarchy, `docs/governance/archive/POST_RELEASE_STABILIZATION_S1_AUTHORITY.md` (historical authority record for the closed slice S1), and one appended entry in `docs/evidence/context7-ledger.md`. `bootstrap-manifest.json` is a historical snapshot, byte-identical. Do not alter Bootstrap, Task 4, Task 5, or Task 6 history; copied authority documents; classification; digests/UUIDs; receipts; journal/reconciliation; MCP handlers or tools; filesystem access; execution; network access; or HTML/script execution.

Run these commands from the isolated worktree:

```sh
cargo fmt --all --check
cargo test --package arsenalero-mcp --test version_flag --locked
cargo test --workspace --locked
cargo check --workspace --locked
git diff --check
git status --short
```

Inspect the diff and status to confirm only permitted S1 paths changed. Implementers do not stage or commit. The parent orchestrator stages the reviewed paths, validates the content-bound receipt, and makes the planned S1 commits in order C1 → C2 → C3 (orchestrator-enforced).

The MCP server remains a zero-domain-tool stdio boundary (S1 adds no tools), and the implementation exposes exactly five tools. Historical Task 7 deterministic classification was permitted after Task 6; S1 is the currently permitted slice and Task 7 remains deferred until S1 closes.
