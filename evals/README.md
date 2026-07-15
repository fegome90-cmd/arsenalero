# Bootstrap evaluation contract

The bootstrap defines evaluation contracts before domain implementation. It does not execute
the cases or claim that their future labels pass.

## Evaluation arms

- **A — Original skill:** baseline skill behavior without resource handling.
- **B — Resource handling, no Arsenalero:** isolates the skill-side resource behavior.
- **C — Resource handling plus Arsenalero MCP:** measures the future MCP contribution.

Regression evaluation is a locked, non-tunable dataset. Capability evaluation is separate and
may grow as implementation tasks add behavior. Component evaluation is not end-to-end
evaluation; protocol compliance is not task success.

## Measures and reporting

Record raw traces separately from aggregates. For each arm and repeated run, report task success,
required-resource issuance recall, required-resource attestation recall, required-set precision,
required-set recall, false-complete rate, unnecessary-resource issuance, added tool calls, bytes
delivered, p50/p95 latency, total time, `pass@3`, and `pass^3`.

Do not tune implementation against the locked regression set. `cases.jsonl` and `labels.jsonl`
are future contracts only; the current zero-tool bootstrap deliberately does not evaluate them.
