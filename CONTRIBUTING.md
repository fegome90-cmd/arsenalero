# Contributing

Do not work on `main`. Use an isolated worktree and a focused conventional commit for each approved task.

Before using an external library API, complete the Context7 evidence gate and record the selected version and contract in `docs/evidence/context7-ledger.md`. Keep changes within the approved task scope, use TDD for executable behavior, and obtain a fresh review after the change.

## Bootstrap validation

Run these commands from the repository root. Implementers do not stage or commit; the parent orchestrator stages reviewed paths before receipt validation.

```sh
cargo fmt --all --check
cargo check --workspace --locked
python3 -m json.tool .codex-plugin/plugin.json >/dev/null
python3 -m json.tool .mcp.json >/dev/null
PLUGIN_CREATOR_SKILL_ROOT="${PLUGIN_CREATOR_SKILL_ROOT:-$HOME/.codex/skills/.system/plugin-creator}"
python3 "$PLUGIN_CREATOR_SKILL_ROOT/scripts/validate_plugin.py" .
git diff --cached --check
git status --short
```

For executable behavior, follow RED → GREEN → REFACTOR, record Context7 evidence before implementation, run the focused test, then obtain a fresh scope-aware review. Runtime MCP integration tests, fixtures, evaluation contracts, CI, and `deny.toml` are deferred Commit 4 work and must not be added to this scaffold slice.

Do not add domain behavior, future dependencies, or unrelated infrastructure to the bootstrap.
