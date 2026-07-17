# Branching Model

Adopted 2026-07-17 by Felipe Gonzalez (human approval materialized in this commit). Effective immediately.

## Branch roles

- **`develop`**: canonical integration branch. All feature work merges here first via PR.
- **`main`**: archived snapshot of pre-MCP public state (3df8742). Tagged `archive/pre-mcp-public-2026-07-17` for permanent reference. Do not commit here.
- **`feature/*`, `fix/*`, `chore/*`, `eval/*`**: isolated work branches. Merge to `develop` via PR.
- **`docs/governance/archive/`**: closed governance addenda (historical, no longer binding-operative).

## Rules

- Never commit directly to `main` (archived).
- Never commit directly to `develop` (use PRs from feature branches).
- New slices require a governance addendum approved as a durable artifact (per the S1 close statement, section 5b of the archived S1 addendum: an agent may research, draft, review, or recommend an addendum, but may not approve, activate, or represent it as operative authority).
- Force-push to `main` or `develop` is forbidden.
- Tags `archive/*` are immutable historical markers.
- Human approval for branch restructuring or other governance changes must be recorded as a durable artifact (commit, signed file, or empty approval commit). Ephemeral orchestrator chat approval is insufficient (lesson from S1 lifecycle reconciliation).

## PR target convention

- All PRs target `develop`.
- Promoting `develop` to `main` would require a separate explicit decision + durable approval artifact. Not currently planned.
- `main` stays as archive reference; if a visitor lands on the repo, they see `develop` (default branch) as of the GitHub UI configuration change made alongside this adoption.

## Rationale

Adopted 2026-07-17 after the post-S1 verification that `main` (3df8742, pre-MCP public polish) and `chore/bootstrap-arsenalero-mcp-v1` (5ae4de0, MCP implementation + S1) had diverged from common base `ff4a9e1`. The two branches carried different lines of work (design polish vs MCP implementation + S1) and could not be cleanly merged without choosing one as canonical.

Decisions:
- `chore/bootstrap-arsenalero-mcp-v1` is canonical (has ADRs in README, --version flag closed by S1, full MCP implementation).
- README of `main` is discarded (less technical, missing S1 state, "70 tests" outdated).
- SVG visual polish from `main` (WCAG AA #767676, cross-platform fonts, honest viewBox) is absorbed into `develop` as a separate commit.
- `main` is preserved as archive reference via tag `archive/pre-mcp-public-2026-07-17` → 3df8742.

## Pending cleanup (Phase 2, deferred)

The following cleanup is pending and will be completed in a subsequent session:
- Move `chore/bootstrap-arsenalero-mcp-v1` to `archive/chore-bootstrap-arsenalero-mcp-v1` (preserve history).
- Remove the physical worktree `arsenalero-bootstrap-worktree` after the move (no longer needed).
- Decide on the future of `feat/arsenalero-eval-framework` and `feat/arsenalero-mcp-v1` (UNRELATED histories; either reconstruct, archive, or integrate via classified tree decision).
