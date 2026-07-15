# Arsenalero MCP Bootstrap Charter

## Intent

Establish a truthful, reviewable governance and authority baseline for Arsenalero MCP v1.3 before any executable implementation begins. Bootstrap Commit 1 preserves approved design material and defines the boundaries for later dependency, workspace, and test slices.

## Risk

**MEDIUM.** The current slice is documentation-only, but it governs a future local MCP server that will read skill resources, issue content, record attestations, and reconcile evidence. The primary risks are authority drift, premature capability, false evidence, and confusion between planned and implemented controls.

## Scope

This slice includes:

- the canonical Agentic Constitution copied verbatim;
- the approved SDD, implementation plan, audit, review findings, and changelog copied as authority documents;
- the historical input report and Context7 protocol retained as authoritative external inputs for their later owned slices;
- repository operating rules in `AGENTS.md`;
- this bootstrap charter;
- ADRs for the global MCP, deterministic classification, and observer-not-enforcer boundary;
- the initial threat model.

The work is limited to `/Users/felipe_gonzalez/Developer/arsenalero-bootstrap-worktree` and must not stage or commit.

## Non-goals

This slice does not create or modify:

- Rust or Cargo files, plugin metadata, MCP configuration, MCP server code, domain tools, handlers, tests, fixtures, evals, CI, `deny.toml`, or runtime persistence;
- `docs/evidence/context7-ledger.md`, which belongs to the dependency-evidence slice;
- `bootstrap-manifest.json` or a final report;
- scanner, classification, REQUIRED calculation, path policy implementation, receipts, cases, journal, attestations, reconciliation, validation adapters, hooks, UI, network, shell, or arbitrary execution.

No domain tools or domain code may be created before Task 4 of the approved implementation plan.

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

Library documentation cannot change the product scope or domain contracts defined by the higher authorities.

## Expected artifacts

- `AGENTIC-CONSTITUTION-v1.0.md`
- `AGENTS.md`
- `docs/architecture/ARSENALERO_MCP_SDD_v1.3.md`
- `docs/plans/ARSENALERO_MCP_IMPLEMENTATION_PLAN_v1.3.md`
- `docs/audit/ARSENALERO_MCP_AUDIT_AI_ENGINEERING_v1.3.md`
- `docs/audit/REVIEW_FINDINGS_v1.3.md`
- `docs/audit/CHANGELOG_v1.3.md`
- `docs/governance/BOOTSTRAP_CHARTER.md`
- `docs/adr/0001-global-mcp.md`
- `docs/adr/0002-deterministic-classification.md`
- `docs/adr/0003-observer-not-enforcer.md`
- `docs/security/threat-model.md`

## Validation plan

For Commit 1, validation is limited to documentation and provenance:

1. verify the target branch/worktree and absence of unexplained prior files;
2. compare every verbatim copy byte-for-byte with its authorized source;
3. calculate source and target byte counts and SHA-256 digests for review evidence;
4. inspect authored documents for the required sections and scope boundaries;
5. verify that no later-commit artifact or owned evidence ledger was created;
6. run `git diff --check` only if it can be run without staging or committing.

Rust, Cargo, MCP, plugin, Context7, CI, test, and eval validation belongs to later commits and must not be claimed here.

## Evidence plan

Authority-copy provenance will be recorded in the dependency-evidence slice. Per the authorized convention, `bootstrap-manifest.json` will record the **pre-finalization commit SHA**, source/target hashes, and byte counts. The final commit SHA is recorded only in an external post-commit report; it must not be inserted into `bootstrap-manifest.json` or another in-tree artifact.

This slice does not create the manifest, the final report, or `docs/evidence/context7-ledger.md`. The Context7 protocol remains an authoritative external input until the dependency-evidence slice copies and records it; it is not represented as completed dependency evidence here.

## Commit plan

The bootstrap has exactly four commits, with no squash:

1. `docs: establish constitutional bootstrap baseline` — this charter, Constitution, AGENTS, approved authority copies, ADRs, and threat model.
2. `docs: record bootstrap dependency evidence` — Context7 evidence, dependency decisions, deferred dependencies, and the initial manifest.
3. `chore: scaffold Rust MCP plugin workspace` — workspace, crates, plugin metadata, MCP configuration, toolchain, license, and repository metadata.
4. `test: verify bootstrap MCP and eval contracts` — integration test, fixtures, eval contracts, CI, dependency policy, and final report.

Bootstrap Commit 1 is intentionally left uncommitted in this task. No staging or commit is authorized.

## Rollback

Rollback is a path-scoped removal of the Commit 1 artifacts listed in **Expected artifacts**. It must not delete, reset, or rewrite the authority source repository, the shared `arsenalero` checkout, or any other worktree. If any later slice has begun, rollback requires an explicit review of dependent paths before removal.

## Stop conditions

Stop before further mutation if:

- the canonical Constitution or any required authority source cannot be verified;
- a required verbatim copy is not byte-identical;
- the target contains unexplained prior changes or is not the isolated bootstrap worktree;
- a requested file belongs to a later commit or the dependency-evidence owner;
- a source/target path boundary is ambiguous;
- the change would introduce executable behavior, a future tool, an unverified API, or a security capability;
- truthful evidence would require a test, commit, final SHA, or runtime claim that does not exist.

## Definition of done

Commit 1 is complete when all expected artifacts exist in the target worktree, the authority copies are byte-identical to their approved sources, the authored documents contain the required governance/decision/security content, no forbidden later-commit artifact exists, `docs/evidence/context7-ledger.md` is unchanged/absent, and the resulting status can be reported without claiming tests or commits. The next permitted work is Bootstrap Commit 2, subject to a fresh scope check.
