# Bootstrap Report

## Status

PASS — the stdio integration test and all required local quality gates passed. The selected Rust
toolchain has the required `clippy` component and `cargo-deny` 0.20.2 was installed for the final
verification.

## Branch / worktree

`chore/bootstrap-arsenalero-mcp-v1` in
`<arsenalero-root>`.

## Authority documents

The copied authority documents, source paths, byte counts, and SHA-256 digests remain recorded in
`docs/evidence/bootstrap-manifest.json`. Commit 4 does not alter those authority copies.

## Context7 evidence

- `rmcp`: `/websites/rs_rmcp_rmcp`, direct requirement `=2.2.0`.
- Tokio: `/websites/rs_tokio_1_49_0`, direct requirement `=1.49.0`.
- `cargo-deny`: `/websites/embarkstudios_github_io_cargo-deny`, tooling policy only.
- Deferred dependencies remain deferred; Commit 4 registers the stdio integration test target and
  uses the existing rmcp server/transport-I/O and Tokio macros/multi-thread runtime selections.

## Files created

- `tests/integration/mcp_bootstrap_stdio.rs`
- `tests/fixtures/{valid_bilingual,unresolved,path_escape}/...`
- `evals/{README.md,cases.jsonl,labels.jsonl}`
- `.github/workflows/ci.yml`
- `deny.toml`

## Commits

Commit 4 uses the exact message `test: verify bootstrap MCP and eval contracts`. Its final SHA is
recorded in the external post-commit report because a commit cannot contain its own final SHA.
Commits 1–3 remain unchanged.

## Verification

| Command | Result | Evidence |
| --- | --- | --- |
| `cargo fmt --all --check` | PASS | Completed after formatting the integration test. |
| `cargo check --workspace --locked` | PASS | Completed with an isolated temporary target directory. |
| `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | PASS | Completed with the `clippy` component for toolchain `1.97.0-aarch64-apple-darwin`. |
| `cargo test --workspace --all-features --locked` | PASS | One stdio integration test passed; workspace and doc tests also passed. |
| `cargo deny check` | PASS | Completed with `cargo-deny` 0.20.2. |
| Plugin validator | PASS | `validate_plugin.py .` passed. |
| Placeholder scan and `git diff --check` | PASS | No matches and no whitespace errors. |

## Plugin

The plugin remains local stdio-only. The test verifies `initialize`, server name `arsenalero`, the
Cargo package version, a tools capability with `tools/list == []`, and no advertised resources or
prompts. The official plugin validator passed during final verification.

## AI Engineering readiness

Three evaluation arms, locked regression separation, raw-trace separation, metrics, concrete
fixtures, and future labels are defined. No case evaluation is claimed: no domain implementation
exists to classify resources, determine obligations, or reject path escape.

## Security

Implemented: local stdio transport and zero domain tools. Deferred: path policy, resource handling,
attestation, persistence, reconciliation, and all domain behavior. Residual risk: the bootstrap
does not yet enforce fixture labels or filesystem policy.

## Deviations

None from Commit 4 scope. Installing the verification tools changed the host environment only;
no unverified product dependency or repository scope was added.

## Known limitations

The MCP is a protocol scaffold only. It provides no resources, prompts, domain tools, network,
shell execution, persistent state, scanner, classifiers, receipts, cases, journal, attestations,
reconciliation, URIs, validators, or domain dependencies.

## Next permitted task

Task 4: domain model and reason codes.

## Final reconciliation

The Commit 4 diff adds verification and future evaluation contracts only. It does not add domain
behavior or advertise any domain capability.
