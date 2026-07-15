# Arsenalero MCP Bootstrap Charter

## Intent and current stage

The four-commit bootstrap establishes a truthful, reviewable foundation before domain implementation. Bootstrap Commit 1 (constitutional baseline) and Bootstrap Commit 2 (dependency evidence) are complete. The current, uncommitted slice is Bootstrap Commit 3: a Rust MCP plugin workspace scaffold. It establishes a local stdio protocol boundary with zero domain tools; it does not implement Arsenalero domain behavior.

## Risk

**MEDIUM.** The scaffold compiles and provides a zero-tool MCP boundary, but runtime protocol integration tests remain deferred. Primary risks are authority drift, premature capability, false evidence, and treating planned safety controls as implemented.

## Scope

Commit 3 may update only the isolated worktree `/Users/felipe_gonzalez/Developer/arsenalero-bootstrap-worktree` and may contain:

- the governed Rust workspace and its `arsenalero-core` and `arsenalero-mcp` crates;
- zero-domain-tool stdio MCP server scaffolding;
- Codex plugin metadata and local MCP configuration;
- the pinned Rust toolchain, license, and repository metadata;
- truthful updates to the Context7 ledger and bootstrap manifest needed to reflect this existing scaffold.

Implementers do not stage or commit in this slice. After reviewed changes are complete, the parent orchestrator stages the reviewed paths, validates the content-bound receipt, and commits the exact Conventional Commit message for Commit 3.

## Non-goals and Commit 4 boundary

This slice does not add domain tools, domain handlers, domain state, resources, prompts, sampling, roots, HTTP, persistence, network access, shell execution, or arbitrary process execution. It also must not create Bootstrap Commit 4 artifacts: runtime MCP integration tests, fixtures, evaluation contracts, CI, `deny.toml`, or a final report. Scanner, classification, path-policy implementation, receipts, cases, journal, attestations, reconciliation, validation adapters, hooks, UI, and future implementation dependencies remain out of scope.

No domain tools or domain code may be created before Task 4 of the approved implementation plan. The server may advertise MCP tools capability only with an empty list; it must not advertise or fake future domain tool names.

## Authority hierarchy

When authority conflicts, use:

1. Agentic Constitution v1.0;
2. Arsenalero MCP SDD v1.3;
3. review findings v1.3;
4. changelog v1.3;
5. implementation plan v1.3;
6. Context7 evidence protocol;
7. AI Engineering audit v1.3;
8. historical input report v1.1;
9. current official library documentation only for API and version facts.

Library documentation cannot change product scope or domain contracts defined by higher authorities.

## Validation plan

For the current scaffold, validation is limited to what exists:

1. verify the isolated target worktree and its scoped diff;
2. run `cargo fmt --all --check` and `cargo check --workspace --locked`;
3. parse plugin/MCP JSON and run the official plugin validator and scope check;
4. verify the ledger SHA-256 and byte count recorded in `bootstrap-manifest.json`;
5. verify that no Commit 4 artifacts or domain tool names exist;
6. after the parent orchestrator stages the reviewed paths, run `git diff --cached --check` against the staged candidate.

Runtime MCP protocol integration tests, eval contracts, fixtures, CI, and dependency-policy validation are not run here and must be reported as deferred to Commit 4.

## Evidence convention

The Commit 2 manifest retains copied authority hashes and byte counts. It records the current ledger provenance and the **pre-finalization commit SHA** for this slice: `a8cede82aca0dc41740b924ce13d4d565b5a0171`. The final Commit 3 SHA belongs **only** in an external post-commit report and must never be written into the manifest or another in-tree artifact.

Evidence must distinguish a present scaffold and passed formatting/compilation checks from deferred runtime integration validation. It must not claim protocol tests, CI, evaluation, domain behavior, or a final SHA that does not exist.

## Commit plan

The bootstrap has exactly four Conventional Commits, with no squash:

1. `docs: establish constitutional bootstrap baseline` — Constitution, AGENTS, charter, approved authority copies, ADRs, and initial threat model.
2. `docs: record bootstrap dependency evidence` — Context7 evidence, dependency decisions, deferred dependencies, and the initial manifest.
3. `chore: scaffold Rust MCP plugin workspace` — workspace, crates, plugin metadata, MCP configuration, toolchain, license, and repository metadata.
4. `test: verify bootstrap MCP and eval contracts` — integration test, fixtures, eval contracts, CI, dependency policy, and final report.

## Rollback

Rollback is a path-scoped removal of the Commit 3 scaffold artifacts. It must not delete, reset, rewrite, or stage authority sources, the shared `arsenalero` checkout, or another worktree. Preserve Commit 1/2 authority evidence unless an explicitly scoped correction is required.

## Stop conditions

Stop before mutation if an authority source cannot be verified; the target is not the isolated worktree; a requested change belongs to Commit 4 or another worktree; a copied authority hash/byte count would be altered; a change introduces domain behavior, future tool names, unsafe capability, or unverified API; or truthful evidence would require a runtime test, commit, or final SHA that does not exist.

## Definition of done

Commit 3 is ready for its fresh re-audit when the scoped workspace/plugin scaffold exists, exact dependency requirements agree with the locked selections, the declared formatting and workspace compilation checks pass, plugin JSON/validation passes, the manifest matches the current ledger hash and byte count, Commit 4 artifacts and domain tool names remain absent, and the parent orchestrator can stage reviewed paths, validate the receipt, and commit the exact Conventional Commit message. Commit 4 verification is the only next bootstrap slice.
