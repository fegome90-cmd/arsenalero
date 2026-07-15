# ADR-0001: Use One Global MCP Server

## Status

Accepted for the Arsenalero MCP v1.3 design baseline; implementation is deferred to later bootstrap commits.

## Context

Arsenalero manages resources for an already-active skill. The SDD requires one installation to operate across multiple compatible skills, without selecting or activating skills. A per-skill server would duplicate lifecycle, inventory, receipt, and reconciliation behavior and would make the source of truth harder to maintain. The server is local and communicates with the host through MCP `stdio`.

## Decision

Provide one reusable global Arsenalero MCP server distributed through a Codex plugin. Keep the domain core transport-neutral and place the MCP lifecycle, schemas, tool adapter, and `stdio` transport at the edge. The server may inspect an explicitly authorized skill root as read-only and keep runtime journal data outside skill roots in plugin-owned data.

This decision does not authorize runtime implementation in Bootstrap Commit 1. It also does not authorize routing, activation, network transport, dynamic tools, or future domain handlers before their approved task.

## Consequences

- Multiple skills can share one implementation and one evidence model.
- The core can be tested independently of MCP transport.
- Plugin installation/configuration becomes a deployment boundary.
- Explicit root authorization and strict path handling are required.
- A single global server increases the importance of case isolation, resource identifiers, and cross-skill state separation.
- The server remains dependent on the host to activate a skill and invoke the protocol.

## Rejected alternatives

- One MCP server per skill: duplicates behavior and fragments the authority model.
- A network listener or remote MCP service: expands the attack surface and violates the local, no-network MVP boundary.
- A server that selects or activates skills: exceeds Arsenalero's responsibility and conflicts with the SDD.

## Evidence

- `docs/architecture/ARSENALERO_MCP_SDD_v1.3.md`, Sections 4, 5, 6, and 8.
- `docs/audit/ARSENALERO_MCP_AUDIT_AI_ENGINEERING_v1.3.md`, Sections 2.2 and 3.
- `docs/architecture/INPUT_REPORT_v1.1.md`, Sections 1 and 2.
