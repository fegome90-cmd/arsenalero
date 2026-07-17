# Authority Addendum — Post-Release Stabilization Slice S1

> Status: **PROPOSED** (not yet operative). This document becomes operative only when committed as part of `chore/bootstrap-arsenalero-mcp-v1` and listed as item 10 in the authority hierarchy of `AGENTS.md` and `CONTRIBUTING.md`.

## 1. Purpose and slice identity

This addendum authorizes a single, narrow post-release stabilization slice, **S1: report package version from Cargo metadata**, for the Arsenalero MCP repository. S1 supersedes no prior authority; it narrows code/test scope and reconciles stale documentation scope (see section 2). It is NOT a generic stabilization phase and does NOT reopen any prior task, authority copy, or committed history.

S1 closes the "Known gap — no `--version` flag" documented in `README.md` by implementing `--version` and `-V` flags on the `arsenalero-mcp` binary that print `arsenalero {CARGO_PKG_VERSION}` and exit 0, plus a workspace-level integration test and the documentation updates required to keep the repository truthful.

## 2. Authority hierarchy insertion

This addendum enters the authority hierarchy as **item 10**, after "Current official library documentation" (item 9):

10. `docs/governance/POST_RELEASE_STABILIZATION_S1_AUTHORITY.md` (this document) — binding-operative for slice S1 only.

Items 1-9 retain their existing order and content.

**M1 — Scope distinction (dual scope):**
Item 10 narrows CODE/TEST scope (the S1 change set is a strict subset of permitted repo operations) and RECONCILES DOCUMENTATION scope (rewrites Task 6 references that were already stale before S1 — Tasks 6-10 were committed at `f9b6650`/`ff4a9e1` but AGENTS.md/CONTRIBUTING.md still described Task 6 as active). The reconciliation of stale documentation is a correctness fix to evidence that pre-existed S1; it is NOT a widening of authority.

Where item 10 is silent, items 1-9 govern unchanged.

## 3. S1 scope (binding-operative)

### In scope (exhaustive)
- `crates/arsenalero-mcp/src/main.rs`: change the `--version`/`-V` branch to print `arsenalero {CARGO_PKG_VERSION}` via `env!("CARGO_PKG_VERSION")` and return `Ok(())`. The printed name MUST be `arsenalero` (no `-mcp` suffix) to align with `crates/arsenalero-mcp/src/server.rs:47` `Implementation::new("arsenalero", env!("CARGO_PKG_VERSION"))`. **ONLY line 10 changes.** Lines 9, 11, 12, 13 are unchanged. **server.rs is NOT touched.**
- `crates/arsenalero-mcp/Cargo.toml`: add exactly one new `[[test]]` block for `version_flag` mirroring the existing `mcp_bootstrap_stdio` convention. No dependency, feature, version, or other change.
- `tests/integration/version_flag.rs`: new workspace-level integration test with three functions per section 6 (version_test_spec).
- `README.md`: remove the "Known gap" paragraph (line 75) and the "No `--version` flag" limitations bullet (line 110). Do not restate the gap elsewhere.
- `AGENTS.md` and `CONTRIBUTING.md`: rewrite all "Task 6" references to reflect S1 as the current closed slice and Task 6 as historical record. Add item 10 to the authority hierarchy.
- `docs/evidence/context7-ledger.md`: append one dated entry per the ledger_entry_draft (section 8).
- This document (`docs/governance/POST_RELEASE_STABILIZATION_S1_AUTHORITY.md`): newly created.

### Out of scope (unchanged from repository invariants)
- `ring`/`sha2` not in scope. `-liconv` out of scope (environmental). No `.cargo/config.toml`. No new dependency. Five-tool set fixed. `bootstrap-manifest.json` byte-identical (historical snapshot, NOT superseded). No Cargo.toml dependency/feature/version change. **server.rs NOT touched.** No filesystem access, network, classification, receipts, journal/reconciliation, or new MCP handlers/tools.

## 4. Ordering constraint (V5)

**C1 MUST precede C2; C2 MUST precede C3.** The orchestrator enforces this ordering. There is no technical enforcement in git; the ordering is documented as a human/orchestrator invariant. Specifically:
- Commit 1 (governance authorization) lands FIRST, so the operative authority for the code change exists before the code change.
- Commit 2 (the code fix) lands AFTER C1.
- Commit 3 (the ledger evidence entry) lands AFTER C2 passes validation, with its Result block filled from actual test output.

### M3 — Known limitation

This ordering is orchestrator-enforced only; there is no git hook or CI check that mechanically prevents C2 from landing before C1. This is consistent with the repository's existing convention (AGENTS.md:42 "Implementers do not stage or commit. The parent orchestrator stages reviewed paths"). S1 does NOT introduce git hooks or pre-commit enforcement; doing so would be a broader repo-wide governance change outside S1's scope. If the orchestrator fails or a manual commit violates ordering, the recovery is to reset to the last valid commit in the sequence and redo.

### M5 — Revert ordering

Commits MUST be reverted in LIFO order. If C2 (code+test) is reverted, C3 (ledger entry) MUST also be reverted in the same operation. Leaving C3 in tree after C2 is reverted leaves the ledger claiming a `--version` flag that no longer exists — violates the evidence convention ("never claim a command, test, build, plugin check, or commit that was not run"). Out-of-order reverts leaving the ledger inconsistent are a governance violation and must be corrected immediately by reverting C3.

## 5. Retirement clause (V4)

Item 10 is **binding-operative for slice S1 only**. Upon S1 close, item 10 is moved to `docs/governance/archive/POST_RELEASE_STABILIZATION_S1_AUTHORITY.md` by the S1 close step — either as a fourth commit `chore(governance): archive slice S1 addendum` or by folding archival into commit 3 (commit 3 becomes `docs: close slice S1 and archive addendum`). After archival:
- The item 10 reference in `AGENTS.md` and `CONTRIBUTING.md` is updated to point to the archive path.
- Item 10 is marked as **no longer binding-operative** (historical record only).
- Items 1-9 resume full authority over the repository.

Until archival, item 10 remains binding-operative and the S1 change set is the only permitted mutation.

## 5a. Governing principle for self-retiring addenda (M2)

Self-retiring addenda (items that enter the authority hierarchy and are later archived at slice close) are appropriate ONLY when ALL of the following hold:
- The slice touches fewer than 10 files
- The slice completes within 5 commits or fewer
- The slice does NOT introduce a new capability, new tool, new dependency, or new authority decision (only fixes debt, reconciles evidence, or aligns documentation)
- The slice's authority is fully captured by its own addendum (no cross-references to items 1-9 needed for operative interpretation)

For slices that DO introduce new capabilities, multi-file features, or enduring authority decisions, use a PERMANENT authority entry that lives indefinitely in the hierarchy, not a self-retiring addendum.

This principle prevents `docs/governance/archive/` from accumulating unboundedly many self-retiring addenda that obscure the active authority surface. S1 meets all four criteria (3 commits, ~8 files, no new capability, self-contained); future slices must verify the same.
