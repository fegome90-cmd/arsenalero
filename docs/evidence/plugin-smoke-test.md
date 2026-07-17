# Task 12 Plugin Installation and Smoke Test

## Scope and evidence boundary

- Target worktree: `<arsenalero-root>`
- Base commit: `dea1f8486a8d281bf6c0ffd9a030e12878f93e82` (`test: verify arsenalero MCP over stdio`)
- Evidence timestamps are UTC on 2026-07-16.
- Repository files were not modified during verification. The temporary Codex marketplace and `CODEX_HOME` were under `/tmp`.
- No fresh Codex session result or five-tool discovery is claimed below without direct evidence.

## PASS

### Locked installation

**Timestamp:** `2026-07-16T16:51:07Z`–`2026-07-16T16:51:07Z`

**Command:**

```bash
env CARGO_TARGET_DIR=/tmp/arsenalero-task12-cargo-target-v1 cargo install --path <arsenalero-root>/crates/arsenalero-mcp --locked --root /tmp/arsenalero-task12-install-v1
```

**Result:** PASS, exit `0`.

**Exact result output:**

```text
  Installing arsenalero-mcp v0.1.0 (<arsenalero-root>/crates/arsenalero-mcp)
    Updating crates.io index
    Finished `release` profile [optimized] target(s) in 0.18s
    Replaced package `arsenalero-mcp v0.1.0 (<arsenalero-root>/crates/arsenalero-mcp)` with `arsenalero-mcp v0.1.0 (<arsenalero-root>/crates/arsenalero-mcp)` (executable `arsenalero-mcp`)
warning: be sure to add `/tmp/arsenalero-task12-install-v1/bin` to your PATH to be able to run the installed binaries
END 2026-07-16T16:51:07Z
EXIT 0
```

The command was first run against an empty install root and completed a full locked release build. The timestamped rerun above confirms the same command and install root remain usable.

### Plugin manifest validation

**Timestamp:** `2026-07-16T16:51:18Z`–`2026-07-16T16:51:18Z`

**Command:**

```bash
python3 ~/.codex/skills/.system/plugin-creator/scripts/validate_plugin.py <arsenalero-root>
```

**Result:** PASS, exit `0`.

```text
Plugin validation passed: <arsenalero-root>
END 2026-07-16T16:51:18Z
EXIT 0
```

### Current Codex plugin tooling: temporary local marketplace install

The local harness used a temporary marketplace root with the manifest at `/tmp/arsenalero-task12-marketplace-v1/.agents/plugins/marketplace.json`, which is the manifest location accepted by the installed Codex CLI. The repository plugin files were copied without modification to the temporary source plugin.

**Timestamp:** `2026-07-16T16:51:25Z`–`2026-07-16T16:51:25Z`

**Commands:**

```bash
CODEX_HOME=/tmp/arsenalero-task12-codex-home-v3 codex plugin marketplace add --json /tmp/arsenalero-task12-marketplace-v1
CODEX_HOME=/tmp/arsenalero-task12-codex-home-v3 codex plugin add --json arsenalero@task12-local
CODEX_HOME=/tmp/arsenalero-task12-codex-home-v3 codex plugin list --json
```

**Result:** PASS, exit `0`; the plugin installed and was enabled at version `0.1.0`.

**Exact result output:**

```json
{
  "marketplaceName": "task12-local",
  "installedRoot": "/private/tmp/arsenalero-task12-marketplace-v1",
  "alreadyAdded": false
}
{
  "pluginId": "arsenalero@task12-local",
  "name": "arsenalero",
  "marketplaceName": "task12-local",
  "version": "0.1.0",
  "installedPath": "/private/tmp/arsenalero-task12-codex-home-v3/plugins/cache/task12-local/arsenalero/0.1.0",
  "authPolicy": "ON_INSTALL"
}
{
  "installed": [
    {
      "pluginId": "arsenalero@task12-local",
      "name": "arsenalero",
      "marketplaceName": "task12-local",
      "version": "0.1.0",
      "installed": true,
      "enabled": true,
      "source": {
        "source": "local",
        "path": "/private/tmp/arsenalero-task12-marketplace-v1/plugins/arsenalero"
      },
      "marketplaceSource": {
        "sourceType": "local",
        "source": "/private/tmp/arsenalero-task12-marketplace-v1"
      },
      "installPolicy": "AVAILABLE",
      "authPolicy": "ON_INSTALL"
    }
  ],
  "available": []
}
END 2026-07-16T16:51:25Z
EXIT 0
```

### Direct installed-binary MCP evidence

**Timestamp:** `2026-07-16T16:52:55Z`–`2026-07-16T16:52:55Z`

The direct stdio harness started `/tmp/arsenalero-task12-install-v1/bin/arsenalero-mcp`, sent `initialize`, `notifications/initialized`, and `tools/list`, then ran the complete fixture flow (`arsenal_init`, `arsenal_stage`, `arsenal_issue`, `arsenal_attest`, `arsenal_reconcile`).

**Exact result output:**

```json
{"initialize_server":{"name":"arsenalero","version":"0.1.0"},"tool_count":5,"tool_names":["arsenal_init","arsenal_stage","arsenal_issue","arsenal_attest","arsenal_reconcile"],"fixture":{"skill_status":"ready","required_resource_ids":["resources::guide"],"reconciliation":{"attestation_breakdown":{"artifact_referenced":0,"externally_verified":0,"self_report_only":1},"disclaimer":"Protocol completion records issue and attestation events. Artifact references are agent-supplied attributions and are not externally verified in Arsenalero v1.","evidence_coverage":{"artifact_referenced":0,"expected_artifact_references":0,"ratio":1.0},"missing_attestations":[],"per_resource_evidence":[{"attained_evidence_level":"attestation","resource_id":"resources::guide","verification_status":"not_supported_in_v1"}],"protocol_completion":{"attested":1,"issued":1,"ratio":1.0,"required":1},"required_but_never_issued":[],"resource_modifications_post_attestation":[],"stale_receipts":[],"status":"complete","unresolved_resources":[],"verification":{"status":"not_supported_in_v1","verified_resources":0}}},"exit_code":0,"stderr":""}
END 2026-07-16T16:52:55Z
EXIT 0
```

This is direct binary/stdio evidence for exactly five tools and a `complete` reconciliation result for `tests/fixtures/complete_case` with `resources::guide` issued and attested.

### Relevant verification checks

| Timestamp | Command | Result |
|---|---|---|
| 2026-07-16T16:52:35Z | `git diff --check` in the target worktree | PASS, exit `0`; target status remained clean |
| 2026-07-16 (after the initial sandbox failure) | `env CARGO_TARGET_DIR=/tmp/arsenalero-task12-cargo-target-v1 cargo test --workspace --locked` | PASS with escalated filesystem access: 57 core tests, 13 MCP unit tests, 13 stdio integration tests; 0 failed |
| 2026-07-16 (no diagnostic output) | `cargo fmt --all --check` | PASS, exit `0` |

## BLOCKED

### Required `--version` smoke command

**Timestamp:** `2026-07-16T16:51:12Z`–`2026-07-16T16:51:13Z`

**Command:**

```bash
/tmp/arsenalero-task12-install-v1/bin/arsenalero-mcp --version
```

**Result:** BLOCKED, exit `1`.

```text
Error: ConnectionClosed("initialize request")
END 2026-07-16T16:51:13Z
EXIT 1
```

The expected output was `arsenalero-mcp 0.1.0`. The installed binary exposes the MCP stdio server but does not handle `--version` as a CLI version path; it starts the protocol server and exits because no initialize request was supplied. This cannot be corrected in Task 12 because crate/source paths are outside the allowed edit scope.

### Fresh Codex session and session-level tool discovery

**Timestamp:** `2026-07-16T16:51:35Z`–`2026-07-16T16:52:13Z`

**Command:**

```bash
CODEX_HOME=/tmp/arsenalero-task12-codex-home-v3 codex exec --ephemeral --json --skip-git-repo-check -C <arsenalero-root> 'Read-only smoke test. Do not spawn sub-agents and do not modify files. Report the MCP tools available to this fresh Codex session, with exact names and count. If the Arsenalero plugin cannot load, report the exact error.'
```

**Result:** BLOCKED, exit `1`; a fresh thread was created (`thread.started`), but the turn never reached a model response or tool listing.

**Exact blocker excerpts:**

```text
{"type":"thread.started","thread_id":"019f6bd7-5497-7cd1-878e-e877c5bea325"}
{"type":"turn.started"}
failed to lookup address information: nodename nor servname provided, or not known
streamable HTTP post_message failed ... https://chatgpt.com/backend-api/ps/mcp ... mcp_method="initialize"
{"type":"turn.failed","error":{"message":"stream disconnected before completion: error sending request for url (https://chatgpt.com/backend-api/codex/responses)"}}
END 2026-07-16T16:52:13Z
EXIT 1
```

No fresh-session five-tool discovery is claimed. The direct installed-binary evidence in the PASS section is the closest verifiable substitute.

## Environment-only observations

- The default login-shell wrapper emitted `(eval):5: parse error near `end'` on several zsh commands. Clean Bash, non-login reruns were used for the timestamped evidence above; the commands' exit statuses and outputs were then unambiguous.
- The first sandboxed Cargo install/test attempts could not resolve `index.crates.io`, and the first sandboxed workspace test also hit `Operation not permitted` while creating test filesystem state. Escalated reruns succeeded; no target worktree files were changed.
- The installed Codex CLI is `codex-cli 0.144.4`. Its local marketplace loader accepts `.agents/plugins/marketplace.json` under the temporary marketplace root; a root-level temporary `marketplace.json` alone was rejected with `marketplace root does not contain a supported manifest` before the harness was corrected. This was temporary tooling setup only.
