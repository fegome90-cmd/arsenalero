# Arsenalero

Arsenalero is a local MCP server project. Its current status is **Bootstrap Commit 3 scaffold**: a governed Rust workspace and stdio protocol boundary with no Arsenalero domain behavior.

## Architecture

```text
.
├── crates/arsenalero-core/  # Reserved shared-library boundary; no domain behavior
├── crates/arsenalero-mcp/   # Zero-domain-tool stdio MCP server scaffold
├── docs/                    # Governance, authority, and dependency evidence
├── .codex-plugin/           # Codex plugin metadata
└── .mcp.json                # Local cargo-run MCP configuration
```

## Development

```sh
cargo fmt --all --check
cargo check --workspace --locked
cargo run --locked --package arsenalero-mcp
```

The server uses standard input/output and advertises an empty tool list. The plugin runs the same reproducible Cargo command from the repository root. Development mode requires a local Rust toolchain and an available dependency cache; it is not a packaged standalone binary.

## Quality and evaluation status

- `cargo fmt --all --check`: passed.
- `cargo check --workspace --locked`: passed.
- Runtime MCP protocol integration tests: deferred to Bootstrap Commit 4 and not run.
- Evaluation contracts, fixtures, and CI: deferred to Bootstrap Commit 4 and not present.

## Scope

This bootstrap contains no domain tools, resources, prompts, sampling, roots, HTTP transport, persistent state, or domain handlers. It is a scaffold only: do not use it for production workflows, filesystem operations, or domain decisions. The next permitted work is Bootstrap Commit 4 verification, followed by Task 4 in the approved implementation plan.

## Security

The bootstrap accepts protocol messages only over stdio. It performs no network access, shell execution, secret handling, or persistent writes.

## Authority

The repository authority documents and dependency evidence are under `docs/`. Evidence governs dependency and API choices; implementation verification is recorded separately when tests are introduced in the permitted future task.
