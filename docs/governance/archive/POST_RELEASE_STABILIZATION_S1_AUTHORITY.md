# Authority Addendum — Post-Release Stabilization Slice S1

> Status: **APPROVED**
> Lifecycle: **CLOSED** (as of commit 4, the S1 close commit)
> Archived: this document lives under `docs/governance/archive/`. It is no longer binding-operative; it is retained as the historical authority record for slice S1. Items 1-9 of the AGENTS.md authority hierarchy resume full authority.

## 1. Purpose and slice identity

This addendum authorizes a single, narrow post-release stabilization slice, **S1: report package version from Cargo metadata**, for the Arsenalero MCP repository. S1 supersedes no prior authority; it narrows code/test scope and reconciles stale documentation scope (see section 2). It is NOT a generic stabilization phase and does NOT reopen any prior task, authority copy, or committed history.

S1 closes the "Known gap — no `--version` flag" documented in `README.md` by implementing `--version` and `-V` flags on the `arsenalero-mcp` binary that print `arsenalero {CARGO_PKG_VERSION}` and exit 0, plus a workspace-level integration test and the documentation updates required to keep the repository truthful.

## 1a. Lifecycle reconciliation

The original PROPOSED banner was an incorrect documentary classification. The human user approved S1 in the orchestrating session before C1 was executed; that approval was recorded in the orchestrator's conversation context but was NOT persisted as an independent approval artifact (signed file, ticket, empty commit) in the repository before C1.

This is a process deviation: the addendum should have been marked APPROVED/ACTIVE in C1 itself, with the approval recorded as a durable artifact, not left as PROPOSED. C2 and C3 were executed under the operative intent of S1 (the user-approved slice), but the documentary record lagged behind the operational reality.

This reconciliation:
- records the prior approval and its ephemeral location (orchestrator chat context);
- corrects the lifecycle representation to APPROVED/CLOSED as of the S1 close commit, without retroactively creating evidence that did not exist;
- does not change the scope, permissions, acceptance criteria, or outcomes under which C1, C2, C3 were made;
- establishes (in the "Addendum authorization rule" section below) that future slices MUST persist approval as a durable artifact, not as ephemeral chat approval, to prevent this deviation from recurring.

Future slices must not cite this reconciliation as precedent for skipping durable approval recording. The deviation is acknowledged, not normalized.

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

## 5b. Addendum authorization rule

A governance or stabilization addendum has no operative authority merely because it has been drafted, added to the repository, referenced by an agent, or included in the authority hierarchy.

An addendum becomes operative only after explicit human approval is recorded either:

1. in the addendum itself; OR
2. in an accompanying approval artifact that unambiguously identifies the approved addendum revision.

An agent may research, draft, review, or recommend an addendum, but may not approve, activate, or represent it as operative authority.

Before approval, agents must treat the addendum as PROPOSED and must not perform mutations whose authorization depends on it.

An addendum may narrow or extend an implementation slice within the authority delegated to it, but it may not:

- override the Constitution;
- modify or supersede copied authority documents;
- retroactively authorize previously unauthorized mutations;
- weaken repository safety restrictions without explicit higher-level authority;
- approve its own creation or activation.

### Lifecycle states

Lifecycle states are:

- **PROPOSED**: drafted but not operative;
- **APPROVED / ACTIVE**: human-approved and currently operative;
- **APPROVED / CLOSED**: completed or retired and no longer authorizes new mutations.

Closing an addendum does not erase or invalidate work truthfully performed while it was active.

## 5c. V7 coverage clarification

The no-flags version-flag test (`no_flags_keeps_the_stdio_server_alive` in `tests/integration/version_flag.rs`) is a bounded smoke test. It verifies only that an invocation without `--version` or `-V` does not take the version early-exit branch and that the child remains alive until deliberately reaped.

It does not independently prove MCP request serving, MCP loop entry, or preservation of the complete stdio protocol behavior. MCP `initialize` and `tools/list` behavior remain covered by the existing `tests/integration/mcp_bootstrap_stdio.rs` integration test. The two tests provide complementary, non-duplicative coverage.

### Corrected acceptance criterion wording

The original acceptance criterion 5 said: "V7: the no-flags test asserts `try_wait() == Ok(None)` after the 3s sleep (process still alive = MCP loop entered)". The phrase "MCP loop entered" over-claims. The correct wording is:

> V7: the no-flags test asserts `try_wait() == Ok(None)` after the 3s sleep, proving that a no-flags invocation does not take the version early-exit path.

The implementation already matches this corrected wording (see the M4 coverage note in `tests/integration/version_flag.rs`); only the acceptance criterion text in this addendum is corrected.

## 5d. Implementation narrative reconciliation

Three narrative imprecisions in the S1 proposal record are acknowledged here, without rewriting the prior commits:

1. **C2 mutation shape**: The proposal section 7 described the change as "OLD line 10: `println!("arsenalero-mcp 0.1.0");` → NEW line 10". Against the actual baseline `ff4a9e1`, the version early-return branch was a four-line ADDITION (the `if std::env::args()...` block) rather than a one-line modification of a pre-existing println. The final state of `crates/arsenalero-mcp/src/main.rs` matches acceptance criterion 1 exactly; only the narrative of "replace existing line 10" was inaccurate.
2. **Line-number references**: References to specific source line numbers (e.g., "line 10 prints...") in the proposal and in the C3 ledger entry are non-durable narrative imprecision. Source line numbers shift across edits; durable evidence should describe behavior (e.g., "the version early-return branch prints...") not line numbers. The implemented behavior and acceptance results were not affected.
3. **`-liconv` evidence scope**: The note in the C3 ledger Result field records a validation-host workaround only (`SDKROOT` and `RUSTFLAGS` overrides for this host's Nix/GCC linker). It is NOT a repository dependency, portability requirement, or general build instruction. It is environmental evidence; the addendum section 3 "Out of scope" correctly excludes `-liconv` from S1.

## 6. S1 close statement

| Item | Status |
|---|---|
| C1 governance activation | complete (`0f81ff3`) |
| C2 version implementation and tests | complete (`c7a907f`) |
| C3 release evidence reconciliation | complete (`5345103`) |
| B1 lifecycle defect | reconciled (this commit, lifecycle now APPROVED/CLOSED) |
| B2 anti-self-authorization rule | added (section 5b) |
| B3 V7 claim boundary | clarified (section 5c) |
| Implementation narrative reconciliation | recorded (section 5d) |
| Remaining authorized mutations | **none** |

**After this commit, S1 authorizes no further repository mutations.** Any additional stabilization work requires a new human-approved slice. Per section 5b, that new slice's approval MUST be persisted as a durable artifact (signed file, ticket, or empty approval commit), not as ephemeral orchestrator chat context — this is the lesson from the B1 deviation.
