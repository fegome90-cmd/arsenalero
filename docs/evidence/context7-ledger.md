# Context7 Evidence Ledger — Arsenalero MCP v1.3

**Generated:** 2026-07-15
**Gate:** `CONTEXT7_EVIDENCE_PROTOCOL.md`
**Scope:** Completed Bootstrap Commit 4, Tasks 4–5 evidence, and the active Task 6 Markdown scanner dependency. Other future implementation dependencies remain deferred.
**Status:** Bootstrap Commit 4 is complete. Tasks 4 (`bbc3cc9`) and 5 (`4b7e953`) are complete. Task 6 is the active pure core Markdown scanner and metadata-parser slice; it adds no MCP domain tools or handlers.

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
| `cargo-deny` | `/websites/embarkstudios_github_io_cargo-deny` | `cargo-deny 0.20.2` verified; CI action configured | Bootstrap tooling, not a product dependency |

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
- Result: Context7 evidence plus official rmcp 2.2.0 documentation verified the selected API. Bootstrap Commit 4 completed the stdio protocol integration test and final workspace verification.

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
- Result: Context7 evidence verified the documented runtime macro/features. Bootstrap Commit 4 completed the stdio protocol integration test and final workspace verification. An initialized stdio session exited successfully on EOF; EOF before `initialize` is an expected nonzero `ConnectionClosed(initialize request)` degraded state.

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
- Requested version: `cargo-deny 0.20.2` verified during Bootstrap Commit 4; CI uses `EmbarkStudios/cargo-deny-action@v2`.
- Query: advisories, bans, licenses, and sources configuration for a small Rust workspace.
- Contract learned: the policy covers advisories, bans, licenses, and sources and is executed with `cargo deny check`.
- Chosen API: `cargo deny check` with the committed `deny.toml`; this is CI tooling, not a runtime crate.
- Rejected alternatives: no dependency policy.
- Security implications: supply-chain checks remain fail-closed in CI.
- Files affected: `deny.toml` and `.github/workflows/ci.yml`.
- Verification command: `cargo deny check`.
- Result: Context7 documentation query verified; Bootstrap Commit 4 recorded `cargo deny check` passing with `cargo-deny 0.20.2` and configured the CI action.

## 2026-07-15 — `proptest` Task 5 property testing

- Requested package: `proptest`
- Resolved Context7 library ID: `/proptest-rs/proptest`
- Requested and selected exact dev requirement: `=1.10.0`.
- Query: generate relative resource paths for the Task 5 canonical-containment property without adding a production dependency.
- Contract learned: the documented `proptest!` macro defines property tests; `any::<T>()`, `prop_assert!`, and collection/strategy combinators support generated inputs and assertions.
- Chosen API: `proptest = "=1.10.0"` under `arsenalero-core` `dev-dependencies` only. The Task 5 property first proves `safe.md` resolves, then asserts every accepted generated relative path resolves under its canonical skill root. Resource extensions are intentionally lowercase UTF-8 only (`md`, `txt`, `json`, `yaml`, `yml`, `toml`).
- Rejected alternatives: runtime property-test dependency, scanner/parser integration, digesting, UUIDs, metadata parsing, or MCP exposure.
- Security implications: test-only generation; production path policy remains dependency-free and read-only.
- Files affected: `crates/arsenalero-core/Cargo.toml`, `Cargo.lock`, and `crates/arsenalero-core/src/path_policy.rs`.
- Verification command: `cargo test -p arsenalero-core path_policy`.
- Result: Context7 API/version evidence was resolved before use; the lockfile selected `proptest 1.10.0`. `cargo test -p arsenalero-core path_policy`, `cargo test -p arsenalero-core`, `cargo clippy -p arsenalero-core --all-targets -- -D warnings`, `cargo check --workspace --locked`, and `cargo deny check` passed on 2026-07-15.

## 2026-07-15 — `pulldown-cmark` Task 6 Markdown scanning

- Requested package: `pulldown-cmark` for CommonMark event parsing with source byte offsets; rendering, HTML output, script execution, link following, and filesystem access are out of scope.
- Resolved Context7 library ID: `/pulldown-cmark/pulldown-cmark`.
- Context7 query: stream headings, list items, inline code, and relative links while preserving source ranges without rendering.
- Contract learned from Context7: `Parser::into_offset_iter` yields `(Event, Range<usize>)` byte offsets; `Event` and `Tag` support structural parsing of headings, list items, links, and inline code; the pull-parser model retains source mapping without executing Markdown content.
- Registry verification was separate from Context7: `cargo search pulldown-cmark --limit 5` and `cargo info pulldown-cmark` verified release `0.13.4` and `rust-version: 1.71.1`, compatible with workspace Rust `1.97.0`.
- Chosen API: exact direct requirement `pulldown-cmark = { version = "=0.13.4", default-features = false }`. `default-features = false` excludes `getopts` and HTML rendering because Task 6 only needs parser events and source offsets.
- Version provenance: Context7 supplied API behavior, not the release version; Cargo registry commands supplied the version and Rust-version evidence.
- Files affected: `crates/arsenalero-core/Cargo.toml`, `Cargo.lock`, `crates/arsenalero-core/src/markdown.rs`, and Task 6 fixtures/tests.
- Verification command: `cargo test -p arsenalero-core markdown --locked`.
- Result: the scanner uses `Parser::into_offset_iter` for structural discovery and byte ranges; it is pure over the input string and does not access files, execute content, follow URLs, hash, classify, journal, reconcile, or expose MCP tools.

## Deferred until implementation task

The bootstrap prompt explicitly forbids resolving or adding these future implementation dependencies now. Their presence here is a deferred scope record, not a resolution:

| Future dependency | Context7 status | Deferred reason |
| --- | --- | --- |
| `sha2` | **UNRESOLVED**; resolver returned unrelated `/shadcn-ui/ui` | SHA-256 is excluded from bootstrap; stop before any task that needs it |
| `uuid` | Candidate ID: `/uuid-rs/uuid` | UUIDv7 domain identifiers are excluded from bootstrap |
| `directories` | Candidate ID: `/git_codeberg_org/dirs_directories-rs` | Runtime data directories are excluded from bootstrap |

The full ten-query protocol remains applicable when the corresponding implementation tasks begin. It does not authorize resolving future dependencies during this bootstrap.

## Verification state

- Rust scaffold: present; it provides a stdio MCP server with tools capability and an empty tool list, with no domain handlers or domain tools.
- Cargo manifest/lockfile: present with exact direct requirements `rmcp = =2.2.0` and `tokio = =1.49.0`; lockfile selections are `rmcp 2.2.0` and `tokio 1.49.0`.
- Formatting: `cargo fmt --all --check` passed.
- Dependency compilation: `cargo check --workspace --locked` passed.
- Runtime MCP protocol integration tests: Bootstrap Commit 4 completed the stdio integration test; `cargo test --workspace --all-features --locked` passed.
- Evals, CI, `deny.toml`, and the final bootstrap report: completed in Bootstrap Commit 4 (`479700012a7b20dbcfead01b1af0ec25ffa06308`).
- Context7 evidence: resolver/API queries recorded above; all reported runtime validation was executed.
