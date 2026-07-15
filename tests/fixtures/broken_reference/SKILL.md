---
arsenal:
  id: rollback-procedure
  purpose: Restore the repository after a failed migration.
  stages:
    - implementation
    - recovery
  requirement: required
  resource_kind: procedure
  evidence_contract:
    minimum: attestation
    supported:
      - attestation
      - artifact_reference
---

# Recovery stage

- Required: read [rollback](resources/missing.md) before finalizing.
