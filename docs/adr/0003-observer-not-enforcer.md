# ADR-0003: Keep Arsenalero an Observer, Dispenser, and Reconciler

## Status

Accepted for the Arsenalero MCP v1.3 design baseline; enforcement and verification capabilities remain out of scope.

## Context

The host/user decides whether a skill is active, and the agent performs the primary work. Arsenalero is responsible for resource inventory, delivery, evidence attribution, and reconciliation after explicit tool calls. It cannot reliably prove that an agent understood a resource, obeyed it, or completed the main task. The SDD also excludes hooks, interception, tool masking, arbitrary execution, workspace artifact validation, and semantic verification from v1.

The protocol must distinguish resource delivery, attestation, artifact reference, and external verification. A referenced artifact is an agent-supplied attribution, not proof that the artifact exists, has the claimed content, or passed validation.

## Decision

Treat Arsenalero as an **observer + dispenser + reconciler**, not an enforcer. It may report missing calls, stale receipts, digest drift, unresolved resources, and reconciliation status. It must not activate skills, intercept tools, force calls, execute validators, follow resource instructions, or claim semantic verification. The skill must explicitly request the protocol after activation, and the agent must surface the reconciliation summary.

Bootstrap Commit 1 documents this boundary only. It does not create handlers, hooks, runtime enforcement, or verification adapters.

## Consequences

- The system can provide honest, reproducible protocol evidence without overstating agent cognition or task correctness.
- A user can inspect what was issued, what use was declared, and whether the count reconciled.
- An agent can omit or bypass the protocol; v1 reports that gap rather than preventing it.
- Stronger enforcement or artifact verification requires a separate SDD and approval.
- Product messaging must preserve the distinction between protocol completion, evidence coverage, and unsupported external verification.

## Rejected alternatives

- Host hooks or interception to force every call: out of scope, host-dependent, and likely to create hidden control flow.
- Tool masking or skill activation control: violates the ownership boundary between routing, skills, agents, and Arsenalero.
- Automatic artifact validation or semantic grading: the bootstrap/MVP has no authorized workspace read scope, executor, or trustworthy verifier.
- A final auto-rendered UI panel: the MCP can return a report, but the host does not guarantee automatic presentation.

## Evidence

- `docs/architecture/ARSENALERO_MCP_SDD_v1.3.md`, Sections 1, 4, 5, 7, 13, 18, 20, 24, and 26.
- `docs/audit/ARSENALERO_MCP_AUDIT_AI_ENGINEERING_v1.3.md`, Sections 2.4, 4.2, 4.5, and 6.
- `docs/audit/REVIEW_FINDINGS_v1.3.md`, including the rejected validation/obligation interpretations.
