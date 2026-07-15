# Context7 Evidence Ledger — Arsenalero MCP v1.3

**Generated:** 2026-07-15
**Gate:** `CONTEXT7_EVIDENCE_PROTOCOL.md`
**Scope:** Bootstrap-only dependencies and tooling. Future implementation dependencies remain deferred.
**Status:** The Bootstrap Commit 3 Rust workspace and zero-domain-tool stdio MCP scaffold are present. `cargo fmt --all --check` and `cargo check --workspace --locked` passed. Runtime MCP protocol integration tests are not run and remain deferred to Commit 4.

## Evidence client provenance

- Client: `/tmp/context7_client.js`
- SHA-256: `843ac91b80beda4ecb683b43a40823816717aaba9edb271a393bcfe57ccdc630`
- Client contract: performs `initialize`, captures `Mcp-Session-Id`, sends `notifications/initialized`, then calls the requested Context7 tool.
- Live wrapper verification: `node /tmp/context7_client.js search 'sha2 RustCrypto SHA-2 streaming digest' 'sha2'` completed and returned unrelated libraries, including `/shadcn-ui/ui`.
- Bundled-script verification: `node /Users/felipe_gonzalez/.pi/agent/skills/context7/search.js 'sha2 RustCrypto SHA-2 streaming digest'` failed with `No valid session ID provided`; source inspection confirms the bundled script calls `tools/call` without first calling `initialize`.
- Limitation: `/tmp/context7_client.js` is an external, non-repository artifact. Its hash and verification commands are recorded here, but it must be revalidated or copied into an approved evidence location before release.

## Bootstrap dependency summary

| Dependency/tooling | Context7 ID | Version status | Bootstrap disposition |
| --- | --- | --- | --- |
| `rmcp` | `/websites/rs_rmcp_rmcp` | exact requirement `=2.2.0`; lockfile selection `2.2.0` | Zero-tool stdio server scaffold |
| `tokio` | `/websites/rs_tokio_1_49_0` | Context7 versioned docs, exact requirement, and lockfile selection: `1.49.0` | Scaffold runtime; no network runtime |
| `serde` | `/websites/serde_rs` | `1.0.x`; exact lockfile pin pending | Conditional; add only if the bootstrap contract requires it |
| `schemars` | `/gresau/schemars` | Exact version and direct-use need pending confirmation | Conditional; add only if the SDK/bootstrap requires it |
| `cargo-deny` | `/websites/embarkstudios_github_io_cargo-deny` | Tool version pending `cargo deny --version`/CI pin | Bootstrap tooling, not a product dependency |

IDs are evidence of resolver results, not proof that a dependency is version-pinned or implementation-ready.

## 2026-07-15 — `rmcp` Rust MCP SDK

- Requested package: `rmcp` (official Rust MCP SDK)
- Resolved Context7 library ID: `/websites/rs_rmcp_rmcp`
- Requested version: `2.2.0`, selected from official current rmcp documentation; exact requirement `=2.2.0` and lockfile selection `2.2.0`.
- Query: zero-tool MCP server over `stdio`; lifecycle, `initialize`, `tools/list == []`, no resources/prompts/sampling/roots/HTTP.
- Contract learned: `rmcp::transport::io::stdio()` is available with `transport-io` plus `server` or `client`; `RoleServer::serve_with_ct` supports cancellation-token-driven serving. Tool macros are not needed for the bootstrap because no tools are registered.
- Chosen API: `rmcp = 2.2.0` with `default-features = false` and `server`, `transport-io`; `ServerHandler`, `ServerInfo::new`, `Implementation::new`, `transport::stdio`, and `ServiceExt::serve` provide the zero-tool stdio server and transport-close shutdown.
- Rejected alternatives: HTTP transport, dynamic registration, domain tool macros, resources, prompts, sampling, and roots.
- Security implications: stdio only; no network listener or arbitrary execution.
- Files affected: `crates/arsenalero-mcp/src/{main.rs,server.rs}`.
- Verification command: `cargo test --workspace --all-features --locked` after the integration test exists.
- Result: Context7 evidence plus official rmcp 2.2.0 documentation verified the selected API; `cargo fmt --all --check` and `cargo check --workspace --locked` passed. Protocol integration tests are deferred to Commit 4 and were not run.

## 2026-07-15 — Tokio

- Requested package: `tokio`
- Resolved Context7 library ID: `/websites/rs_tokio_1_49_0`
- Requested documentation version: `1.49.0` in Context7's versioned documentation. The selected dependency is exact requirement `=1.49.0` and lockfile selection `1.49.0`.
- Query: minimal async main and graceful shutdown for the selected `rmcp` stdio transport; exclude network runtimes.
- Contract learned: `#[tokio::main]` requires the runtime/macros features; the exact shutdown integration must be checked against the selected `rmcp` version.
- Chosen API: `tokio = =1.49.0` with only `macros` and `rt-multi-thread`; no `TcpListener`, `signal`, process, or network runtime features.
- Rejected alternatives: network transports and unnecessary runtime features.
- Security implications: local stdio only; shutdown must not spawn external processes.
- Files affected: `crates/arsenalero-mcp/src/main.rs`.
- Verification command: `cargo test --workspace --all-features --locked` after the integration test exists.
- Result: Context7 evidence verified the documented runtime macro/features; `cargo fmt --all --check` and `cargo check --workspace --locked` passed. An initialized stdio session exited successfully on EOF; EOF before `initialize` is an expected nonzero `ConnectionClosed(initialize request)` degraded state. These are recorded transport observations, not runtime protocol integration tests; those remain deferred to Commit 4 and were not run.

## 2026-07-15 — Serde

- Requested package: `serde`
- Resolved Context7 library ID: `/websites/serde_rs`
- Requested version: `1.0.x`; exact lockfile pin pending.
- Query: strict serialization/deserialization only if required by the zero-tool bootstrap contract.
- Contract learned: `#[serde(deny_unknown_fields)]` is a documented container attribute; derive support uses the `derive` feature.
- Chosen API: **pending**; do not add a direct dependency unless the selected `rmcp` integration or bootstrap test requires it.
- Rejected alternatives: accepting unbounded unknown input fields.
- Security implications: strict input handling where external structs are introduced.
- Files affected: none yet.
- Verification command: implementation-specific Cargo test after a direct use is justified.
- Result: Context7 documentation query verified; direct bootstrap use not yet established.

## 2026-07-15 — Schemars

- Requested package: `schemars`
- Resolved Context7 library ID: `/gresau/schemars`
- Requested version: **pending**; confirm the version compatible with the selected `rmcp` release before adding it directly.
- Query: JSON Schema generation only if required by the bootstrap server contract.
- Contract learned: `schema_for!` and `SchemaSettings::draft2020_12()` are documented for JSON Schema 2020-12 generation.
- Chosen API: **pending**; no domain input/output schemas are introduced during the zero-tool bootstrap.
- Rejected alternatives: hand-written schemas and future domain schemas.
- Security implications: schemas must be generated from trusted Rust types, not accepted from clients.
- Files affected: none yet.
- Verification command: implementation-specific Cargo test after a direct use is justified.
- Result: Context7 documentation query verified; direct bootstrap use not yet established.

## 2026-07-15 — `cargo-deny` tooling

- Requested package/tool: `cargo-deny`
- Resolved Context7 library ID: `/websites/embarkstudios_github_io_cargo-deny`
- Requested version: **pending**; record the installed CLI version and pin the CI action/tooling before the CI commit.
- Query: advisories, bans, licenses, and sources configuration for a small Rust workspace.
- Contract learned: the policy covers advisories, bans, licenses, and sources and is executed with `cargo deny check`.
- Chosen API: **pending local version verification**; this is CI tooling, not a runtime crate.
- Rejected alternatives: no dependency policy.
- Security implications: supply-chain checks remain fail-closed in CI.
- Files affected: future `deny.toml` and `.github/workflows/ci.yml`.
- Verification command: `cargo deny check` after the workspace and policy exist.
- Result: Context7 documentation query verified; local CLI/CI version not yet recorded.

## Deferred until implementation task

The bootstrap prompt explicitly forbids resolving or adding these future implementation dependencies now. Their presence here is a deferred scope record, not a resolution:

| Future dependency | Context7 status | Deferred reason |
| --- | --- | --- |
| `pulldown-cmark` | Candidate ID: `/pulldown-cmark/pulldown-cmark` | Markdown scanner is excluded from bootstrap |
| `sha2` | **UNRESOLVED**; resolver returned unrelated `/shadcn-ui/ui` | SHA-256 is excluded from bootstrap; stop before any task that needs it |
| `uuid` | Candidate ID: `/uuid-rs/uuid` | UUIDv7 domain identifiers are excluded from bootstrap |
| `directories` | Candidate ID: `/git_codeberg_org/dirs_directories-rs` | Runtime data directories are excluded from bootstrap |
| `proptest` | Candidates include `/proptest-rs/proptest` and `/websites/altsysrq_github_io_proptest-book` | Property testing is future implementation scope |

The full ten-query protocol remains applicable when the corresponding implementation tasks begin. It does not authorize resolving future dependencies during this bootstrap.

## Verification state

- Rust scaffold: present; it provides a stdio MCP server with tools capability and an empty tool list, with no domain handlers or domain tools.
- Cargo manifest/lockfile: present with exact direct requirements `rmcp = =2.2.0` and `tokio = =1.49.0`; lockfile selections are `rmcp 2.2.0` and `tokio 1.49.0`.
- Formatting: `cargo fmt --all --check` passed.
- Dependency compilation: `cargo check --workspace --locked` passed.
- Runtime MCP protocol integration tests: not run; deferred to Commit 4.
- Context7 evidence: resolver/API queries recorded above; no unrun runtime validation is claimed.
