# Arsenalero

Arsenalero is a local MCP server project. Bootstrap is complete through **Bootstrap Commit 4** (`479700012a7b20dbcfead01b1af0ec25ffa06308`), Task 4 is complete at `bbc3cc9`, and Task 5 is complete at `4b7e953`. The current work is **Task 6: Markdown scanner and metadata parser**.

## Architecture

```text
.
├── crates/arsenalero-core/  # Domain contracts, Task 5 path policy, and Task 6 scanner
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

Task 6 defines pure source-string, event-based Markdown scanning only, using `pulldown-cmark =0.13.4` with `default-features=false`. It preserves source byte ranges; extracts relative resource links; covers inline resource and reference code paths; emits free-filename warnings; retains heading, list, and adjacent context; and optionally parses Arsenal frontmatter metadata.

Permitted support is limited to the existing Task 6 implementation paths, required `lib.rs` wiring, approved parser dependency and `Cargo.lock`, focused Task 6 fixtures already present in the worktree, and truthful Context7 ledger/manifest updates. It adds no classification, digests/UUIDs, receipts, journal/reconciliation, MCP handlers or tools, filesystem access, execution, network access, or HTML/script execution. The MCP boundary remains zero-domain-tool, and the eventual implementation exposes exactly five tools. Task 7 deterministic classification is permitted only after Task 6 is reviewed and committed.

## Security

The bootstrap accepts protocol messages only over stdio. Task 6 scans supplied source strings without filesystem access or writes. It performs no network access, shell execution, HTML/script execution, secret handling, or persistent writes.

## Authority

Repository authority documents and dependency evidence live under `docs/`. Preserve Bootstrap, Task 4, and Task 5 history and do not rewrite copied authority documents. Record only truthful Task 6 dependency evidence in the owned ledger and manifest.
