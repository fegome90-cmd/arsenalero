# Branching Model

Adopted 2026-07-18 by Felipe Gonzalez (human approval materialized in `docs/governance/BRANCH_CONSOLIDATION_B1_AUTHORITY.md`). Effective upon PR 1 merge.

## Branch roles

- **`main`**: canonical protected branch. All changes enter via PR.
- **`feature/*`, `fix/*`, `chore/*`, `eval/*`**: short-lived branches. Merge to `main` via PR.
- **`docs/governance/archive/`**: closed governance addenda (historical, no longer binding-operative).

## Rules

- Never commit directly to `main`. Use PRs from feature branches.
- New slices require a governance addendum approved as a durable artifact (per the S1 close statement, section 5b of the archived S1 addendum: an agent may research, draft, review, or recommend an addendum, but may not approve, activate, or represent it as operative authority).
- Force-push to `main` is forbidden.
- Tags `archive/*` are immutable historical markers of legitimate product snapshots.
- Tags `incident/*` are immutable historical markers of process incidents (NOT product snapshots; they preserve evidence of deviations).
- **Two-phase push barrier**: every agent with push capability MUST follow prepare → report exact diff and refs → explicit human approval → push. An agent MUST NOT prepare AND publish in the same delegation without an explicit approval gate.

## PR target convention

- All PRs target `main`.
- Merge strategy: prefer merge commits for history connection; squash only for trivial single-commit PRs; rebase only when explicitly approved.

## Rationale

Adopted 2026-07-18 via Branch Consolidation B1 (Recovery), after the unauthorized M2 gitflow incident of 2026-07-17. The repository consolidates two diverged branches:
- `main` (3df8742): pre-MCP public polish, 5 commits after common base `ff4a9e1`
- bootstrap MCP line (5ae4de0): MCP implementation + closed S1

Decisions:
- Bootstrap is canonical (ADRs in README, --version flag closed by S1, full MCP implementation).
- README of bootstrap is canonical for the consolidated main.
- SVG visual polish from main (WCAG AA, cross-platform fonts, honest viewBox) enters the consolidated main via merge `--theirs`.
- `main` pre-MCP state preserved via tag `archive/pre-mcp-public-2026-07-17`.
- No `develop` branch, no gitflow — single canonical line for a single-developer repository.
- The unauthorized M2 commits (539d2e3, 68e3e60) are quarantined via `incident/*` tags and excluded from canonical history. Their branches (`origin/develop`, `origin/chore/bootstrap-arsenalero-mcp-v1`) will be deleted after B1 closes; the tags preserve the evidence indefinitely.
