# Contributing

Do not work on `main`. Work only in `/Users/felipe_gonzalez/Developer/arsenalero-bootstrap-worktree` and use a focused Conventional Commit for each approved task. Do not modify `/Users/felipe_gonzalez/Developer/arsenalero`.

Before using an external library API, complete the Context7 evidence gate and record the selected version and contract in `docs/evidence/context7-ledger.md`. Keep changes within the approved task scope, use TDD for executable behavior, and obtain a fresh review after the change.

## Task 6 validation and commit

Task 4 is complete at `bbc3cc9`, and Task 5 is complete at `4b7e953`. Task 6 is the active slice: Markdown scanner and metadata parser. It may implement only pure source-string, event-based Markdown scanning with `pulldown-cmark =0.13.4` and `default-features=false`, including source byte ranges, relative resource links, inline resource and reference code paths, free-filename warnings, heading/list/adjacent context, and optional Arsenal frontmatter metadata parsing.

Permitted paths are the existing Task 6 implementation paths, required `crates/arsenalero-core/src/lib.rs` wiring, focused Task 6 tests and fixtures already present in the worktree, `crates/arsenalero-core/Cargo.toml`, `Cargo.lock` for the approved parser dependency, and truthful Context7 ledger/manifest updates. Do not alter Bootstrap, Task 4, or Task 5 history; copied authority documents; classification; digests/UUIDs; receipts; journal/reconciliation; MCP handlers or tools; filesystem access; execution; network access; or HTML/script execution.

Run these commands from the isolated worktree:

```sh
cargo fmt --all --check
cargo test --package arsenalero-core --locked
cargo check --workspace --locked
git diff --check
git status --short
```

Inspect the diff and status to confirm only permitted Task 6 paths changed. Implementers do not stage or commit. The parent orchestrator stages the reviewed paths, validates the content-bound receipt, and makes the planned Task 6 commit.

The MCP server remains a zero-domain-tool stdio boundary, and the eventual implementation exposes exactly five tools. Task 7 deterministic classification is permitted only after Task 6 is reviewed and committed.
