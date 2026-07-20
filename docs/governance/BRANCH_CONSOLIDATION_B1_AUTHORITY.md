# Authority Addendum — Branch Consolidation B1 (Recovery)

> Status: **APPROVED**
> Lifecycle: **ACTIVE** (until PR 1 merges to main and PR 2 closes B1)
> Recovery of: B1 originally planned 2026-07-17, restarted 2026-07-18 after an unauthorized M2 gitflow execution

## Approval

Approved by: Felipe Gonzalez
Approval date: 2026-07-18 (recovery plan); original B1 plan approved 2026-07-17 with 6 corrections
Scope: Branch Consolidation B1 — Recovery D (cuarentena no destructiva + reinicio limpio desde 5ae4de0)

This approval was given in the orchestrator session on 2026-07-18 and
is materialized in this document (a durable artifact in the repository).
Per the S1 close statement (section 5b of the archived S1 addendum): an
agent may research, draft, review, or recommend an addendum, but may
not approve, activate, or represent it as operative authority.

The local worktree and branch creation prior to C1 was non-operative
preparation. No remote mutation, content mutation, tag creation, or
merge occurred before B1 was activated by C1. The branch was created
from the last authorized state (5ae4de0, S1 closed), NOT from
origin/chore/bootstrap-arsenalero-mcp-v1 (which contains unauthorized
commits — see section "Incident context" below).

## Incident context (mandatory reading)

On 2026-07-17, the orchestrator delegated the M2 gitflow plan
(develop-canonical) to a sub-agent BEFORE receiving the human's final
approval. The human rejected M2 and sent 6 corrections resulting in
plan B1 (main-canonical PR-only). The orchestrator cancelled the M2
sub-agent, but the sub-agent had already published:

- `539d2e3`: "docs(governance): adopt gitflow with develop as canonical branch"
- `68e3e60`: "chore(design): absorb svg polish from main"
- tag `archive/pre-mcp-public-2026-07-17` → `3df8742`

The commits `539d2e3` and `68e3e60` cite "Approved by: Felipe Gonzalez"
in their commit messages. That approval was retracted before the
sub-agent published. The attribution in those commits is therefore
NOT valid as operative approval (it represents a briefly-considered
option, not a final decision).

### What was NOT touched by the incident

- `origin/main` (still `3df8742`)
- GitHub default branch (still `main`, verified via `gh repo view`)
- The S1 closed history (`5ae4de0` and ancestors unchanged)

### Quarantine decision

The unauthorized commits are preserved as incident evidence via
immutable `incident/*` tags (created after C1). They MUST NOT enter
the canonical main history. They are not `archive/*` tags (those are
legitimate product snapshots); they are process-incident evidence.

The `archive/pre-mcp-public-2026-07-17` tag is RETAINED despite its
premature creation. Its content and purpose are correct (points to
`3df8742`); only its creation moment was improper. Deleting and
recreating it would be process theater, not a substantive correction.

### Recovery baseline

Recovery begins from the last authorized S1 commit:
`5ae4de0ef32964f30d0faa920d4bd2fbc8d9d453`. This is the parent of
the unauthorized `539d2e3`. The recovery branch
`chore/branch-consolidation-b1-recovery` is created directly from
`5ae4de0`, bypassing the unauthorized commits.

### Prohibition

NO force-push or history rewrite is authorized. The accidental commits
will be quarantined through immutable incident tags and their branch
references will be deleted only after the canonical integration and B1
closure are complete (in a future session).

## 1. Purpose

Authorize the consolidation of two diverged branches into a single
canonical history on `main` via a normal merge commit:
- `main` (3df8742): pre-MCP public polish, 5 commits after common base
- Recovery baseline `5ae4de0` (= bootstrap at S1 close): MCP implementation + S1

Both share common ancestor `ff4a9e1`. After B1, `main` becomes the
canonical PR-only branch.

## 2. Authority hierarchy insertion

Item 11 in the AGENTS.md authority hierarchy. Item 10 (archived S1
addendum) remains unchanged.

## 3. Scope (binding-operative)

### In scope (exhaustive)
- Worktree `/Users/felipe_gonzalez/Developer/arsenalero-integration-worktree` (temporary, non-operative prep before C1)
- Branch `chore/branch-consolidation-b1-recovery` (created from `5ae4de0`, non-operative prep before C1)
- This document: `docs/governance/BRANCH_CONSOLIDATION_B1_AUTHORITY.md` (NEW)
- `docs/governance/BRANCHING_MODEL.md` (NEW, in C2)
- `AGENTS.md` (scoped updates in C1 and C2)
- `CONTRIBUTING.md` (scoped updates in C2)
- Tag `incident/unauthorized-m2-bootstrap-2026-07-17` → `539d2e3` (created AFTER C1)
- Tag `incident/unauthorized-m2-develop-2026-07-17` → `68e3e60` (created AFTER C1)
- Tag `archive/pre-mcp-public-2026-07-17` is RETAINED (already exists, do not delete)
- Merge of `origin/main` into `chore/branch-consolidation-b1-recovery` with history preservation
- Conflict resolution: README bootstrap canonical (`--ours`), SVGs from main (`--theirs`)
- Push `chore/branch-consolidation-b1-recovery` to origin (TWO-PHASE BARRIER: prepare, then explicit approval, then push)
- PR 1 via `gh pr create` (NOT auto-merged)

### Out of scope (forbidden during B1 recovery)
- Force-push to ANY branch (`main`, `develop`, `chore/bootstrap-arsenalero-mcp-v1`, `chore/branch-consolidation-b1-recovery`)
- Squash or rebase merge of PR 1 (must be a real merge commit to connect histories)
- NO new edits to `crates/**` relative to the recovery baseline `5ae4de0`
- Rewriting, amending, squashing, or otherwise altering the closed S1 commit history
- Touching `bootstrap-manifest.json` or `server.rs`
- Modifying or deleting `origin/develop`, `origin/chore/bootstrap-arsenalero-mcp-v1`, or the `archive/pre-mcp-public-2026-07-17` tag (these remain until cleanup phase in a future session)
- Work on `feat/arsenalero-eval-framework` or `feat/arsenalero-mcp-v1` (UNRELATED, excluded)
- Auto-merging PR 1 in the same execution sequence that creates it
- Push without explicit two-phase approval

## 4. Conflict resolution policy

Expected conflicts (verify before resolving):
- `README.md`: resolve `--ours` (bootstrap README is canonical: ADRs, --version flag, S1 state)
- `assets/readme/hero.svg`: resolve `--theirs` (main's WCAG AA + cross-platform fonts + honest viewBox)
- `assets/readme/pipeline.svg`: resolve `--theirs` (same design polish)

Any conflict in `crates/**`, `docs/`, `tests/`, or other paths NOT in the expected set is a STOP CONDITION.

## 5. Stop conditions

Stop before or during mutation when:
- Conflict set is larger than {README.md, assets/readme/hero.svg, assets/readme/pipeline.svg}
- A file under `crates/**` shows unexpected conflict or diff
- Force-push would be required at any point
- Validation fails (cargo fmt --all --check, cargo check --workspace --locked, cargo test --workspace --locked)
- The merge would require `--allow-unrelated-histories` (would indicate wrong base)
- The recovery branch was created from anything other than `5ae4de0`
- Any commit accidentally includes Co-Authored-By or AI attribution
- PR 1 auto-merge is attempted in the same execution that creates it
- Push is attempted without the two-phase approval (prepare → human approval → push)

## 6. Lifecycle and retirement

B1 is a **short-lived, single-purpose addendum with an explicit
retirement procedure**. (Do NOT invoke the M2 self-retiring addendum
principle's commit-count threshold; this addendum is declared
short-lived by its single-purpose scope, not by meeting a numeric
threshold.)

Upon successful merge of PR 1 to main:
1. Open PR 2 (`docs/close-branch-consolidation-b1` → `main`) containing:
   - Move this document to `docs/governance/archive/BRANCH_CONSOLIDATION_B1_AUTHORITY.md`
   - Update banner to `Lifecycle: APPROVED/CLOSED`
   - Record PR 1 merge SHA
   - Withdraw B1 from the active item 11 slot in AGENTS.md
   - Declare no active slice
   - Record that bootstrap history is now integrated into main
   - Record the incident quarantine outcome (incident tags retained, unauthorized branches deleted)
2. After PR 2 merges:
   - Verify `origin/chore/branch-consolidation-b1-recovery` is an ancestor of `origin/main`
   - Verify `5ae4de0` is an ancestor of `origin/main`
   - Delete `chore/branch-consolidation-b1-recovery` from origin
   - Delete `origin/develop` from origin (quarantined via incident tag)
   - Delete `origin/chore/bootstrap-arsenalero-mcp-v1` from origin (quarantined via incident tag)
   - Remove the integration worktree and the bootstrap worktree (its content is now in main)
   - Decide on `feat/arsenalero-eval-framework` and `feat/arsenalero-mcp-v1` (UNRELATED, separate decision)

PR 2 and the cleanup steps happen in a future session. PR 1 of this session is NOT auto-merged.

## 7. Authorization for PR 2

This addendum explicitly pre-authorizes PR 2 (the close PR) as part of
B1's retirement. PR 2 may be drafted, pushed, and opened by an agent
after PR 1 is merged and main's SHA is verified. PR 2 itself still
requires human review before merge.

## 8. Operational control: two-phase push barrier

To prevent recurrence of the M2 incident, every agent with push
capability MUST follow the two-phase barrier:

1. **Prepare locally**: create commits, tags, branches, merge results
   in the local worktree.
2. **Report exact diff and refs**: report to the orchestrator the
   exact set of refs to be pushed, the diff content, and the target
   remote refs.
3. **Explicit parent approval**: the orchestrator relays the report
   to the human; the human explicitly approves the push (or rejects
   and requests changes).
4. **Push**: only after explicit approval.

An agent MUST NOT prepare AND publish in the same delegation without
an explicit approval gate between them.

## 9. Enmienda 2026-07-18: revisión de PR #1

Approved by: Felipe Gonzalez
Approval date: 2026-07-18
Scope: PR #1 review remediation

After PR #1 was opened, Copilot review (status COMMENTED, not
REQUEST_CHANGES) identified 6 documentary findings:

1. README.md declares "70 tests" (mutable counter, will go stale).
2. AGENTS.md has 2 broken references to the non-archived S1 addendum path.
3. CONTRIBUTING.md says S1 is the current active slice (contradicts AGENTS.md).
4. Ledger entry has a broken reference to the non-archived S1 path.
5. Archived S1 addendum has internal references to its own non-archived path.
6. AGENTS.md is autocontradictory: declares B1 active but still has
   stale S1 operative blocks (scope, TDD, commits, safety, stop
   conditions, validations).

This enmienda authorizes a single corrective commit on the existing
recovery branch `chore/branch-consolidation-b1-recovery` to remediate
all 6 findings. The commit touches exactly these 6 files (5 target files plus this addendum itself):

- `README.md` (remove mutable counter, replace with "CI verified")
- `AGENTS.md` (fix 2 broken paths + replace 6 stale S1 operative blocks with B1 references)
- `CONTRIBUTING.md` (replace "S1 active" with "B1 active" references)
- `docs/evidence/context7-ledger.md` (additive path correction note)
- `docs/governance/archive/POST_RELEASE_STABILIZATION_S1_AUTHORITY.md` (fix internal broken paths)
- This document (`docs/governance/BRANCH_CONSOLIDATION_B1_AUTHORITY.md`) — adds this enmienda section itself

(The enmienda adds this section to itself; the corrective changes to
the other 5 target files happen in the same commit; this addendum is the 6th file.)

### Additional stop conditions for this enmienda

- The corrective commit MUST NOT introduce new code or test changes.
- The corrective commit MUST NOT touch any file outside the 6 listed above
  (5 target files plus this addendum itself).
- The corrective commit MUST NOT alter the merge commit or any prior
  commit on the recovery branch (it is an additive commit on top).
- Push MUST be a fast-forward (no force-push needed; the branch just
  gains one new commit).

### Three-gate workflow applies

The corrective commit is prepared (Encargo A), validated, and reported
with exact SHA. The human reviews the exact SHA and diff. Only after
explicit human approval does Encargo B push to the existing branch.

### Cardinality reconciliation (2026-07-19)

The enmienda authorized "a single corrective commit" (section 9 main
text) but the actual remediation required three commits:

1. `30fe4cc` — initial remediation of all 6 review findings across
   5 target files plus this addendum.
2. `ee118e5` — residual fix for CONTRIBUTING.md lines 11 and 23
   (same Hallazgo 6 pattern: sub-agent updated the active-slice
   pointer but left operative blocks stale; detected in human review
   of `30fe4cc`, not in the initial sub-agent pass).
3. this reconciliation commit: corrects the file-count description
   (was "5 files", actually 6 including this addendum), acknowledges
   the cardinality deviation from "single corrective commit", and
   fixes residual wording issues in AGENTS.md (autocontradictory
   line 39) and CONTRIBUTING.md (stale heading line 7) detected in
   second human review. (The exact SHA is available via git log;
   the prior commit's pre-amend SHA was previously inlined here but
   removed because it is not durable: a commit cannot contain its
   own SHA, and the pre-amend SHA was not the actual final SHA of
   the prior commit.)

Cardinality deviation acknowledged, not normalized. Future enmiendas
must declare exact commit count upfront, or accept that additional
commits require explicit reconciliation like this one. The single-commit
assumption was wrong: review remediation in a slice that touches
interdependent governance documents predictably produces additional
commits as each review surfaces related defects.

Additionally corrected in this commit: section 9 originally said
"touches exactly these 5 files" while the same sentence enumerated
6 files (5 target files plus this addendum). The stop condition
"outside the 5 listed above (plus this addendum itself)" was
internally inconsistent. Corrected to "6 listed above" — the 6 files
were always 6 (5 target + this addendum); only the count was wrong.

## 10. Amendment: PR #1 CI merge-gate remediation

Status: APPROVED/ACTIVE
Approved by: Felipe Gonzalez
Approval date: 2026-07-19
Approval source: exact human approval in the orchestration thread (Gate A1)

Purpose: authorize the minimum behavior-neutral source-documentation
change required to make the existing GitHub Actions Clippy gate pass.

Permitted files:
- `tests/integration/version_flag.rs`
- `docs/governance/BRANCH_CONSOLIDATION_B1_AUTHORITY.md`

Permitted changes:
- correct only the rustdoc indentation that triggers
  `clippy::doc_overindented_list_items`;
- materialize this human-approved amendment.

Forbidden:
- changing executable behavior or test assertions;
- adding lint allowances;
- weakening or disabling `-D warnings`;
- modifying `.github/workflows/ci.yml`;
- changing any other source, test, governance, or evidence file;
- rewriting existing commits or force-pushing.

Validation:
- `cargo fmt --all --check`
- `cargo check --workspace --locked`
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`
- `cargo test --workspace --all-features --locked`
- `cargo deny check`
- `git diff --check`
- explicit permitted-path scope check for the section 10 amendment
  snapshot: its recorded base SHA is
  `a653e67346e153074bb9c92a27fff005fa4dbeb9`, and its recorded end SHA is
  `9fc97df2bbea5911c1555d69887f9dbce881108`. Run
  `git diff --name-only a653e67346e153074bb9c92a27fff005fa4dbeb9..9fc97df2bbea5911c1555d69887f9dbce881108 | sort` and compare the
  output exactly with this sorted permitted-file list:
  ```text
  docs/governance/BRANCH_CONSOLIDATION_B1_AUTHORITY.md
  tests/integration/version_flag.rs
  ```
  Fail closed if the comparison is not an exact match, including when any
  third path appears, and record the actual command output before approval
  or push.

## 11. Amendment: PR #1 CI toolchain remediation

Status: APPROVED/ACTIVE
Approved by: Felipe Gonzalez
Approval date: 2026-07-19
Approval source: exact human approval in the orchestration thread (Gate A1)

Purpose: authorize the minimum change to make the existing GitHub Actions
Clippy step pass — currently it fails because `cargo-clippy` is not
installed in the toolchain defined by `rust-toolchain.toml`.

Permitted files:
- `rust-toolchain.toml`
- `docs/governance/BRANCH_CONSOLIDATION_B1_AUTHORITY.md`

Permitted changes:
- add `"clippy"` to the `components` array in `rust-toolchain.toml`;
- materialize this human-approved amendment.

Forbidden:
- changing the toolchain `channel` (must remain `1.97.0`);
- changing the `profile` (must remain `minimal`);
- removing `rustfmt` from `components`;
- modifying `.github/workflows/ci.yml`;
- changing any other source, test, governance, or evidence file;
- rewriting existing commits or force-pushing.

Validation:
- `cargo fmt --all --check`
- `cargo check --workspace --locked`
- `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`
- `cargo test --workspace --all-features --locked`
- `cargo deny check`
- `git diff --check`
