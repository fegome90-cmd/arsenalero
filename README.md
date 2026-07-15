# Arsenalero

Arsenalero is a local MCP server project. Bootstrap is complete through **Bootstrap Commit 4** (`479700012a7b20dbcfead01b1af0ec25ffa06308`). The current work is **Task 4: domain model and reason codes**.

## Architecture

```text
.
├── crates/arsenalero-core/  # Shared-library boundary; Task 4 domain contracts and reason codes
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
- Bootstrap Commit 4 verification artifacts are complete historical bootstrap record.
- Task 4 does not add or revise runtime MCP integration, evaluation contracts, fixtures, CI, `deny.toml`, or the final report.

## Scope

Task 4 may add only `crates/arsenalero-core/src/domain.rs`, `crates/arsenalero-core/src/error.rs`, and required `crates/arsenalero-core/src/lib.rs` wiring. It adds no MCP handlers, filesystem, scanner, classification implementation, receipts, UUID generation, hashing, journal, reconciliation, fixtures, or dependencies. The MCP server remains a zero-domain-tool stdio boundary. The next permitted task is Task 5 after the Task 4 commit.

## Security

The bootstrap accepts protocol messages only over stdio. It performs no network access, shell execution, secret handling, or persistent writes.

## Authority

The repository authority documents and dependency evidence are under `docs/`. Evidence governs dependency and API choices; implementation verification is recorded separately when tests are introduced in the permitted future task.
