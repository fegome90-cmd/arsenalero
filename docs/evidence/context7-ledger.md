# Context7 Evidence Ledger — Arsenalero MCP v1.3

**Generated:** 2026-07-16
**Gate:** `CONTEXT7_EVIDENCE_PROTOCOL.md`
**Scope:** Bootstrap Commit 4, Tasks 4-9 evidence, and Task 10 MCP schemas, handlers, strict inputs, stale-receipt reconciliation, and output contracts.
**Status:** Task 10 current official Context7 provenance is recorded below. Historical deferred entries remain historical records and are superseded only where the Task 10 implementation now directly uses the dependency.

## Evidence client provenance

- Client: `/tmp/context7_client.js`
- SHA-256: `843ac91b80beda4ecb683b43a40823816717aaba9edb271a393bcfe57ccdc630`
- Client contract: performs `initialize`, captures `Mcp-Session-Id`, sends `notifications/initialized`, then calls the requested Context7 tool.
- Live wrapper verification: `node /tmp/context7_client.js search 'sha2 RustCrypto SHA-2 streaming digest' 'sha2'` completed and returned unrelated libraries, including `/shadcn-ui/ui`.
- Bundled-script verification: `node ~/.pi/agent/skills/context7/search.js 'sha2 RustCrypto SHA-2 streaming digest'` failed with `No valid session ID provided`; source inspection confirms the bundled script calls `tools/call` without first calling `initialize`.
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

## 2026-07-15 — Task 8 SHA-256 and UUIDv7

- `sha2` Context7 status remains **UNRESOLVED**. Multiple exact/upstream resolver queries returned unrelated libraries (including `/shadcn-ui/ui`), so that result is not authority for RustCrypto `sha2`. The historical unresolved record above is retained intentionally.
- Replacement decision: Task 8 uses `ring = "=0.17.14"`, not `sha2`. Context7 resolved the official Ring documentation as `/websites/rs_ring` and documented the selected streaming API: `ring::digest::Context::new(&SHA256)`, repeated `Context::update`, and final `Context::finish`.
- Cargo provenance is separate from Context7 API provenance: an elevated `cargo check --workspace` on 2026-07-15 updated the registry index and locked `ring 0.17.14`; Cargo compiled it successfully with Rust `1.97.0`.
- UUID provenance is also split: Context7 resolved UUID documentation as `/uuid-rs/uuid`, while Cargo locked `uuid 1.24.0`. Task 8 declares `uuid = { version = "=1.24.0", default-features = false, features = ["serde", "std", "v7"] }`; the `v7` and `std` features make `Uuid::now_v7()` available and `serde` preserves the documented domain-serialization contract.
- Tradeoff: Ring provides the necessary audited streaming SHA-256 context without adding RustCrypto after the sha2 resolver failure. The implementation owns lowercase hexadecimal rendering to keep the `sha256:` digest format stable without another dependency.
- Stop condition rationale: this substitution is limited to core Task 8 receipts and local file digesting. It does not authorize a hash abstraction, cryptographic policy expansion, server/MCP changes, or any additional unpinned dependency. Revisit sha2 only if its official Context7 authority becomes resolvable and a later scoped task needs its API.
- Files affected: `crates/arsenalero-core/Cargo.toml`, `Cargo.lock`, `crates/arsenalero-core/src/{case.rs,receipt.rs,domain.rs}`, and Task 8 drift fixtures/tests.
- Verification: `cargo test -p arsenalero-core --locked` passed with 47 tests; the receipt tests cover case, skill, and resource binding; cross-case rejection; batch limits; empty usage; pre-attest resource/skill drift; UUIDv7; and lowercase streaming digests.

## Verification state

- Rust scaffold: present; it provides a stdio MCP server with tools capability and an empty tool list, with no domain handlers or domain tools.
- Cargo manifest/lockfile: present with exact direct requirements `rmcp = =2.2.0` and `tokio = =1.49.0`; lockfile selections are `rmcp 2.2.0` and `tokio 1.49.0`.
- Formatting: `cargo fmt --all --check` passed.
- Dependency compilation: `cargo check --workspace --locked` passed.
- Runtime MCP protocol integration tests: Bootstrap Commit 4 completed the stdio integration test; `cargo test --workspace --all-features --locked` passed.
- Evals, CI, `deny.toml`, and the final bootstrap report: completed in Bootstrap Commit 4 (`479700012a7b20dbcfead01b1af0ec25ffa06308`).
- Context7 evidence: resolver/API queries recorded above; all reported runtime validation was executed.


## 2026-07-16 — Task 10 MCP adapter dependencies and APIs

Task 10 obtained current official Context7 resolver and documentation evidence before relying on the external APIs below. Context7 library IDs identify documentation sources; Cargo manifest and lockfile entries provide the exact selected versions.

| Direct dependency | Context7 library ID | Exact Cargo selection | Task 10 use |
| --- | --- | --- | --- |
| rmcp | /websites/rs_rmcp_rmcp | =2.2.0 | ServerHandler, Tool, list_tools, get_tool, call_tool, CallToolResult, stdio service |
| schemars | /gresau/schemars | =1.2.1 | JsonSchema derives and input/output JSON Schema generation |
| serde | /websites/serde_rs | =1.0.228 | Serialize/Deserialize derives, field rename/default, deny_unknown_fields |
| serde_json | /websites/rs_serde_json | =1.0.150 | request decoding, structured values, serialized JSON TextContent |
| ring | /websites/rs_ring_0_17_14 | =0.17.14 | streaming SHA-256 for post-issue resource drift |
| uuid | /uuid-rs/uuid | =1.24.0 | case/receipt UUID parsing and stable formatting |
| tokio | /websites/rs_tokio_1_49_0 | =1.49.0 | #[tokio::main] stdio server runtime |

### rmcp

- Resolver result: official Rust MCP SDK, /websites/rs_rmcp_rmcp, high source reputation.
- Queries: ServerHandler list/call lifecycle and Tool schema registration.
- Official API evidence:
  - ServerHandler::list_tools returns ListToolsResult; call_tool returns CallToolResult for tool execution.
  - Tool::new creates a named tool with a description and input schema; with_input_schema<T> and with_output_schema<T> register JsonSchema-derived schemas.
  - CallToolResult::structured(value) emits both structuredContent and a JSON-string TextContent, with isError: false.
  - CallToolResult::structured_error(value) emits both structured error content and JSON-string TextContent, with isError: true.
- Official sources: https://docs.rs/rmcp/latest/rmcp/handler/server/trait.ServerHandler.html, https://docs.rs/rmcp/latest/rmcp/model/struct.Tool.html, https://docs.rs/rmcp/latest/rmcp/model/struct.CallToolResult.html.
- Chosen API: direct ServerHandler implementation with exactly five static Tool definitions and structured/structured_error; no tool router, resources, prompts, sampling, roots, HTTP, or network capability.
- Files: crates/arsenalero-mcp/src/server.rs, crates/arsenalero-mcp/src/main.rs.

### schemars

- Resolver result: /gresau/schemars, high source reputation.
- Query: JsonSchema derive and length constraints.
- Official API evidence: derive(JsonSchema) generates a schema matching the Serde representation; schemars(length(min = 1, max = 10)) maps to minLength/maxLength for strings and minItems/maxItems for arrays.
- Chosen API: JsonSchema derives for every Task 10 input/output, with max = 4 for issue batches, max = 16 for attestations, and minimum non-empty usage/evidence references.
- Files: crates/arsenalero-mcp/src/schema.rs, crates/arsenalero-mcp/src/server.rs.

### serde and serde_json

- Resolver results: /websites/serde_rs and /websites/rs_serde_json, both high source reputation.
- Query: derive serialization/deserialization, strict unknown-field handling, serde(default), field rename, serde_json::to_value, and JSON text serialization.
- Official API evidence: Serde documents derive(Serialize, Deserialize), serde(deny_unknown_fields), serde(default), and serde(rename = "..."); Serde JSON documents to_value for Value conversion and to_string for JSON text.
- Chosen API: all external Task 10 input structs deny unknown fields; optional evidence defaults to an empty list; the Rust type field is serialized/deserialized as JSON type; serde_json::from_value decodes requests and rmcp structured constructors provide the compatibility text.
- Files: crates/arsenalero-mcp/src/schema.rs, crates/arsenalero-mcp/src/server.rs, crates/arsenalero-mcp/src/tools.rs.

### ring and uuid

- Resolver results: /websites/rs_ring_0_17_14 and /uuid-rs/uuid, both high source reputation.
- Query: streaming SHA-256 and UUID parsing/formatting.
- Official API evidence: Ring documents digest::Context::new(&SHA256), repeated update, and consuming finish; UUID documents Uuid::parse_str and standard hyphenated formatting.
- Chosen API: digest_file reads bounded chunks through Ring's streaming context; Uuid::parse_str maps malformed case/receipt IDs to CASE_UNKNOWN, while UUID values are formatted in the standard hyphenated representation.
- Files: crates/arsenalero-mcp/src/tools.rs.

### tokio

- Resolver result: versioned official documentation /websites/rs_tokio_1_49_0, high source reputation.
- Query: #[tokio::main] runtime and required features.
- Official API evidence: Tokio documents #[tokio::main] for async entrypoints, requiring rt plus macros; the multi-threaded flavor requires rt-multi-thread.
- Chosen API: the existing binary keeps #[tokio::main] with macros and rt-multi-thread, and serves only rmcp stdio.
- Files: crates/arsenalero-mcp/src/main.rs, crates/arsenalero-mcp/Cargo.toml.

### Task 10 verification provenance

- Context7 resolver/query calls completed on 2026-07-16 through the configured Context7 MCP server.
- Cargo exact pins were checked against the changed manifest and lockfile.
- The exact requested cargo test -p arsenalero-mcp was attempted in the worktree and was blocked before compilation by Operation not permitted opening target/debug/.cargo-lock; a supplemental CARGO_TARGET_DIR=/tmp/arsenalero-task-10-target cargo test -p arsenalero-mcp passed after implementation.
- The known core -D warnings issue remains outside Task 10 scope: cargo clippy -p arsenalero-mcp --all-targets -- -D warnings reports pre-existing arsenalero-core dead-code errors for CaseId::new and ReceiptId::new.

## 2026-07-17 — Post-Task-14 release state

**Slice**: S1 — report package version from Cargo metadata.
**Authority**: `docs/governance/POST_RELEASE_STABILIZATION_S1_AUTHORITY.md` (item 10, binding-operative for S1 only).
**Change**: `crates/arsenalero-mcp/src/main.rs` line 10 prints `arsenalero {CARGO_PKG_VERSION}` (matching `server.rs:47` `serverInfo.name = "arsenalero"`) for `--version` and `-V`, then exits 0. New workspace test `tests/integration/version_flag.rs` covers the long flag, short flag, and a no-flags still-alive contract (process alive after 3s, then reaped). README gap text removed (lines ~75 and ~110). AGENTS.md and CONTRIBUTING.md Task 6 references rewritten to S1. No new dependency, no Cargo.toml change beyond the new `[[test]]` block, `bootstrap-manifest.json` byte-identical.
**Result**: `cargo test --package arsenalero-mcp --test version_flag --locked` ran 3 tests; all passed (exit code 0): `long_version_flag_prints_package_version_and_exits_zero`, `short_version_flag_prints_package_version_and_exits_zero`, `no_flags_keeps_the_stdio_server_alive`; finished in 3.01s. `cargo test --workspace --locked` passed with no regressions. Note: the build invoked `cargo` with `SDKROOT=/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk` and `RUSTFLAGS="-L /Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/lib"` to work around the known environmental `-liconv` linker gap (out of S1 scope per the addendum); no repository change was required for that environment workaround.
