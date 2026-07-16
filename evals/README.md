# Task 13 three-arm agent evaluation contract

This directory defines the end-to-end evaluation dataset and its measurement contract. It
does not execute trials and contains no raw traces or aggregate results. `cases.jsonl` has 20
unique cases; `labels.jsonl` contains the human gold labels for the pre-mutation case state.

## Authority and provenance

The copied repository was at `HEAD=0ca2a0b97f9eba5515d253525cb712e7492cdc40` when this
contract was authored. The authoritative sources are preserved and cited by path and line:

- `docs/architecture/ARSENALERO_MCP_SDD_v1.3.md:294-370` — orthogonal classification source
  (`DECLARED`, `DERIVED`, `UNRESOLVED`) and obligation (`REQUIRED`, `RECOMMENDED`, `OPTIONAL`,
  `UNKNOWN`) rules, including the closed English and Spanish markers.
- `docs/architecture/ARSENALERO_MCP_SDD_v1.3.md:453-459` — the REQUIRED set is fixed at
  `arsenal_init` and is not changed by later stage calls.
- `docs/architecture/ARSENALERO_MCP_SDD_v1.3.md:621-735` — attestation evidence levels,
  pre-attestation staleness, post-attestation modification detection, invalidation, and
  reconciliation invariants.
- `docs/architecture/ARSENALERO_MCP_SDD_v1.3.md:1069-1114` — the exact three arms, primary and
  secondary outcomes, capability/regression split, and minimum three trials with `pass@3` and
  `pass^3`.
- `docs/plans/ARSENALERO_MCP_IMPLEMENTATION_PLAN_v1.3.md:720-764` — Task 13's required 20-case
  categories, metrics, raw-trace separation, and locked-regression anti-tuning rule.
- `docs/audit/ARSENALERO_MCP_AUDIT_AI_ENGINEERING_v1.3.md:325-359` — independent audit wording
  for primary, efficiency, and repeated-trial metrics.

Existing fixture references in `cases.jsonl` are repository-relative and are not copied or
modified by this task. For a fixture case, `target_resources` is the complete deterministic
scope; when it is omitted, the scope is every supported reference discovered in that fixture.
Labels must cover exactly that scope. Cases with `fixture: null` carry a small inline skill
contract so the required optional, contradictory, and bilingual variants can be defined without
adding files outside the three owned eval files.

## Exact evaluation arms

Run each case under these arms exactly as specified by the SDD:

```text
A. Skill original
B. Skill con bloque Resource handling, sin MCP
C. Skill con bloque + Arsenalero
```

Arm A is the baseline. Arm B isolates the value of explicit Resource handling instructions
without MCP. Arm C measures the incremental contribution of the Resource handling block plus
Arsenalero MCP. Do not rename, merge, or reorder these arms in reported results.

## Trial protocol

Run every case exactly three times per arm, with immutable `trial_id` values `1`, `2`, and `3`.
The complete matrix is `20 cases × 3 arms × 3 trials = 180 runs`. Use the same case input,
gold labels, and mutation schedule for all arms; only the arm behavior may differ. The REQUIRED
set is evaluated from the initialization snapshot before any later stage call.

For each run, normalize the arm output into:

- `predicted_status`: `complete`, `incomplete`, `needs_review`, or `invalidated`;
- `predicted_required_set`: the arm's claimed required resource paths;
- `issued_set`: resources actually issued to the agent;
- `attested_set`: resources with an accepted attestation, not merely a claim;
- `tool_calls`, `bytes_delivered`, per-tool latency, and total task time;
- `success`: the task-success judgment defined below.

The labels are arm-independent. A `complete` gold status requires correct required-set
handling plus issue and attestation coverage. A non-complete gold status is successful only when
the arm preserves the expected fail-closed status and the corresponding unresolved, missing, or
drift condition. Optional and recommended resources do not enter the REQUIRED set; unresolved
resources must not be silently promoted into it.

## Metric semantics

For run `r`, let `R` be `gold_required_set`, `Q` be `predicted_required_set`, `I` be
`issued_set`, and `A` be `attested_set`. Resource identity is the exact normalized path used by
the case label; no fuzzy or semantic matching is allowed.

- **Task success (per arm):** `successful runs / 60` (`20 cases × 3 trials`). If an all-arm
  aggregate is reported, name it **all-arm task success** and use `successful runs / 180`.
  A run is successful only when its normalized
  status equals `expected_terminal_status`, its required-set decision matches the gold set, and
  its issue/attestation behavior satisfies the expected outcome. This is a run-level binary,
  not a text-quality score.
- **Issue recall:** `Σ|R ∩ I| / Σ|R|` across runs. It measures required resources actually
  issued. Report `N/A` with the denominator when `Σ|R| = 0`; do not award a perfect score for
  issuing nothing.
- **Attestation recall:** `Σ|R ∩ A| / Σ|R|` across runs. Count only accepted attestations;
  an `arsenal_issue` claim is not an attestation.
- **Required-set precision:** `Σ|R ∩ Q| / Σ|Q|` across runs. A zero predicted set has an
  undefined precision, reported as `N/A`, rather than being treated as perfect.
- **Required-set recall:** `Σ|R ∩ Q| / Σ|R|` across runs. Use the same zero-denominator rule
  as issue recall.
- **False-complete rate:** `runs where predicted_status=complete and expected_status != complete /
  runs where expected_status != complete`. The denominator is restricted to non-complete gold
  cases; a false completion is never hidden by positive cases.
- **Unnecessary issuance rate:** `Σ|I \ R| / Σ|I|`. Any issued recommended, optional, unknown,
  or otherwise non-required resource is unnecessary for this metric. Report `N/A` when no
  resources were issued.
- **Tool calls:** count every tool invocation in a run, then report per-arm mean, p50, and p95.
  Also report **added tool calls** for B and C as the same-case/trial count minus A.
- **Bytes:** count UTF-8 bytes in resource payloads delivered to the agent, excluding prose,
  tool envelopes, and raw-trace serialization. Report per-arm mean, p50, and p95, plus added
  bytes versus A.
- **Latency:** use monotonic wall-clock measurements. Report p50 and p95 latency for each tool
  and total task time from task start through reconciliation, as required by the SDD.
- **`pass@3`:** for each case and arm, `1` if at least one of its three trials has `success=1`,
  otherwise `0`; report the mean over the 20 cases.
- **`pass^3`:** for each case and arm, `1` only if all three trials have `success=1`, otherwise
  `0`; report the mean over the 20 cases. This is the operational three-success rate, not a
  model-estimated power.

Report the raw counts and denominators alongside every ratio. Aggregate with a fixed case order
and fixed trial IDs so repeated reports remain auditable.

## Deterministic mutation contract

After accepted attestation and before reconciliation, the runner applies each mutation's
`operation` to its exact `path`. `append_utf8` appends the declared `bytes` verbatim as UTF-8
without newline normalization. The resource-drift, skill-drift, inline-resource, and bilingual
locked cases carry their exact mutation bytes in `cases.jsonl`; labels remain the pre-mutation
gold state. Their terminal outcomes are respectively `needs_review`, `invalidated`,
`needs_review`, and `needs_review`.

## Raw traces and aggregates

Raw traces are a separate runtime artifact, never a repository artifact. A runner may write
them outside the repository under a path such as
`<run-root>/raw/<arm>/<case_id>/trial-<trial_id>.jsonl`; aggregate metrics belong under a
separate `<run-root>/aggregates/` path. Do not add either output to this directory, and do not
claim a trial or metric until its raw trace exists and is independently summarized.

## Locked regression and anti-tuning rule

Cases with `split: "locked_regression"` are the locked regression subset. Their inputs, inline
skill text, fixture identity, mutations, and gold labels must not be changed in response to
observed results. Do not tune prompts, Resource handling instructions, Arsenalero behavior, or
thresholds on this subset. Capability cases (`split: "capability"`) may expose limits and guide
future work, but must not be relabeled to improve a regression score. Any intentional dataset
change creates a new version and invalidates direct comparison with prior results.

This task writes definitions only. It does not run the 180 trials, create raw outputs, produce
aggregate metrics, stage files, commit, push, or invoke native review.
