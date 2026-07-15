# Arsenalero MCP Threat Model

## Purpose and current status

This document describes the security boundary for the approved Arsenalero MCP v1.3 design and explicitly separates **planned mitigations** from controls present in Bootstrap Commit 3. The current tree contains a compiling Rust workspace, local Codex plugin/MCP metadata, and a zero-domain-tool local stdio rmcp scaffold. It contains no domain tools, handlers, state, network/HTTP transport, shell or arbitrary-process capability, secrets, or persistent state.

## Assets

- The Agentic Constitution and approved Arsenalero authority documents.
- Skill instructions and referenced resource content.
- The identity, digest, and lifecycle of a future case.
- Future receipts, attestations, reconciliation results, and the append-only journal.
- The separation between read-only skill roots and plugin-owned runtime data.
- User/agent trust in scope, evidence, and status reports.
- Local MCP configuration and plugin execution boundaries.
- Availability and bounded resource consumption of the local server.

## Trust boundaries

1. **User and host ↔ Codex/plugin:** the host selects/activates skills and decides how tools are approved. Arsenalero must not assume control of this boundary.
2. **MCP client ↔ server:** requests and arguments are untrusted inputs; the future server must validate schemas, case ownership, limits, and errors.
3. **Server ↔ skill root:** the future server may read only an explicitly allowed skill root and must never write into it.
4. **Server ↔ plugin data:** future journal/state writes belong outside skill roots and require bounded, append-oriented handling.
5. **Resource content ↔ server logic:** resource text is data, not instructions to the server. It must never expand permissions or trigger execution.
6. **Authority documents ↔ generated artifacts:** copied authority is not runtime evidence, and later evidence must not silently rewrite authority.

## Hostile inputs

- Missing, malformed, or misleading `skill_root` paths.
- `..` traversal, absolute paths, alternate path forms, and symlinks escaping an allowed root.
- Broken references, duplicate resource identifiers, unsupported file types, oversized files, and oversized batches.
- Resource content containing prompt injection, shell commands, URLs, secrets, or instructions aimed at the server.
- `task_summary` crafted to influence classification or expand scope.
- Malformed MCP JSON, unknown fields, unknown tools, wrong case IDs, stale receipts, and cross-case receipt reuse.
- Resource or `SKILL.md` changes between issue, attest, and reconcile.
- Corrupted or replayed journal events.
- Host/plugin configuration that enables capabilities outside the local no-network/no-execution boundary.

## Threats

| Threat | Impact | Planned mitigation | Present in bootstrap |
|---|---|---|---|
| Path traversal or symlink escape | Read outside the authorized skill root; possible data exposure | Canonicalize paths, enforce explicit allowed roots, reject traversal and symlink escapes, keep roots read-only | No runtime path handling exists; boundary is documented only |
| Resource prompt injection | Server follows hostile content or changes classification/permissions | Treat resources as data; never execute, follow URLs, obey embedded instructions, or use an LLM | No runtime consumer exists; the threat and prohibition are documented |
| Arbitrary execution or network egress | Host compromise, exfiltration, non-reproducible behavior | No shell, process execution, network, HTTP, hooks, listener, or dynamic loading | The compiling local stdio scaffold exposes no such capability; this is scope/configuration evidence, not a proof against host compromise |
| Resource/skill drift | False attestation or incorrect close status | Bind future receipts/cases to digests; reject stale pre-attestation; mark post-attestation changes for review; invalidate on `SKILL.md` drift | No receipts or digest checks exist; the future contract is preserved in the SDD |
| Cross-case confusion or replay | Evidence from one skill/case is attributed to another | Enforce case ownership, unique receipts, digest checks, and explicit reason codes | No case state exists; the isolation requirement is documented |
| Evidence inflation | User mistakes a reference or self-report for external verification | Separate delivery, attestation, artifact reference, and verification; report verification as unsupported until a trusted validator exists | Authored docs explicitly state the distinction; no evidence system exists |
| Resource exhaustion | Denial of service or excessive context/latency | Enforce file, batch, case, timeout, and aggregate byte limits; measure overhead | No runtime limits exist; the planned limits are documented |
| Journal tampering/corruption | Loss of auditability or misleading reconciliation | Keep journal outside skill roots, use append-only events and chained digests, and state that this is not an attacker-proof signature | No journal exists; the limitation is documented |
| Authority drift | Implementation follows stale or rewritten design claims | Preserve verbatim authority copies, use the stated hierarchy, and record provenance separately from runtime evidence | Authority copies and hierarchy exist in this slice |

## Mitigations planned for later slices

The future implementation must provide, and verify with tests, at least:

- explicit allowed-root configuration and fail-closed path canonicalization;
- read-only skill access and plugin data outside skill roots;
- no network, shell, arbitrary execution, hooks, database, or internal LLM;
- strict request schemas, bounded file/batch/case/time budgets, and actionable domain errors;
- closed deterministic vocabularies for classification and obligation;
- case-scoped, digest-bound receipts and stale/drift transitions;
- separation of protocol completion, evidence coverage, and unsupported verification;
- append-only journal events with corruption detection, without claiming cryptographic signatures;
- adversarial fixtures for traversal, symlink, malformed, oversized, hostile-content, drift, and cross-case inputs.

## Mitigations present in Bootstrap Commit 3

- The canonical Constitution is copied from the explicitly authorized commit rather than reconstructed.
- Approved SDD, plan, audit, review findings, and changelog are preserved as authority copies; the input report and Context7 protocol remain external authority inputs for later slices.
- `AGENTS.md` and the charter impose fail-closed scope, no domain artifacts, no network/shell/arbitrary execution, no fake tools, and no untruthful evidence claims.
- ADRs make the global-server, deterministic-classification, and observer-not-enforcer boundaries explicit.
- The Rust workspace and rmcp local-stdio server scaffold compile with an empty tools list; no domain tools, handlers, state, resources, prompts, sampling, roots, network/HTTP, shell, arbitrary process, secrets, or persistence are implemented.
- The dependency evidence ledger and bootstrap manifest record the present scaffold and exact locked dependencies. Runtime protocol integration tests and the final report remain deferred to Commit 4.

## Residual risks

- Documentation does not enforce filesystem, process, network, or MCP boundaries.
- The scaffold has no runtime protocol integration tests, dependency policy, or CI yet; plugin metadata and compile evidence exist but do not prove runtime protocol behavior.
- The target has no implementation-level protection against hostile inputs until later tasks are completed and tested.
- Authority copies can be misused if an implementer ignores the hierarchy or treats examples as guarantees.
- A future local process may still be exposed to host-level compromise outside Arsenalero's control.
- The final commit SHA remains external-only; runtime protocol integration evidence is deferred to Commit 4.

## Non-guarantees

Bootstrap Commit 3 does **not** guarantee:

- that the server completes the MCP runtime protocol under all client scenarios;
- that MCP `initialize` succeeds or `tools/list` is empty;
- that roots, symlinks, resource contents, receipts, or journals are secured at runtime;
- that network, shell, arbitrary execution, hooks, or secrets are technically impossible;
- that a resource was delivered, used, attested, verified, or understood;
- that any task succeeded or that any future plugin configuration is valid.

These are design targets or later validation obligations, not results of this slice.
