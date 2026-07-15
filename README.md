# Arsenalero

Arsenalero is a local MCP server project. Bootstrap is complete through **Bootstrap Commit 4** (`479700012a7b20dbcfead01b1af0ec25ffa06308`), and Task 4 is complete at `bbc3cc9`. The current work is **Task 5: read-only filesystem path policy**.

## Architecture

```text
.
├── crates/arsenalero-core/  # Domain contracts and Task 5 path policy
├── crates/arsenalero-mcp/   # Zero-domain-tool stdio MCP server scaffold
├── docs/                    # Governance, authority, and dependency evidence
├── .codex-plugin/           # Codex plugin metadata
└── .mcp.json                # Local cargo-run MCP configuration
```

## Development

```sh
cargo fmt --all --check
cargo test --package arsenalero-core --locked
cargo check --workspace --locked
```

The server uses standard input/output and advertises an empty tool list. The plugin runs the same reproducible Cargo command from the repository root. Development mode requires a local Rust toolchain and an available dependency cache; it is not a packaged standalone binary.

## Scope

Task 5 defines `PathPolicy`, `CanonicalSkillRoot`, and `CanonicalResourcePath` only. It enforces fail-closed canonical containment and validates traversal, symlinks, file type, size, extension, and path length with focused tests and fixtures. Skill roots remain read-only.

Permitted support is limited to `path_policy.rs`, required `lib.rs` wiring, focused core tests and path-policy fixtures, the approved `proptest` core dev-dependency plus `Cargo.lock`, and truthful Context7 ledger/manifest updates. It adds no scanner or metadata parsing, classification, receipts/UUID/digest behavior, journal/reconciliation, MCP handlers, or domain tools. The MCP boundary remains zero-domain-tool. Task 6 is permitted only after Task 5 is reviewed and committed.

## Security

The bootstrap accepts protocol messages only over stdio. Task 5 validates paths without reading resource content, expanding roots, following symlink escapes, or writing inside skill roots. It performs no network access, shell execution, secret handling, or persistent writes.

## Authority

Repository authority documents and dependency evidence live under `docs/`. Preserve Bootstrap and Task 4 history and do not rewrite copied authority documents. Record only truthful Task 5 dependency evidence in the owned ledger and manifest.
