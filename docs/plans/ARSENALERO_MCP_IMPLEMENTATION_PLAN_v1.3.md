# Arsenalero MCP — Implementation Plan v1.3

> **For agentic workers:** REQUIRED SUB-SKILL: use `superpowers:subagent-driven-development` or `superpowers:executing-plans`. Implement one task at a time. Every task ends with tests, review and a commit.

**Goal:** Build a Rust MCP server distributed as a Codex plugin that inventories, issues, attests and reconciles resources for an already-active skill.

**Architecture:** `arsenalero-core` contains deterministic domain logic. `arsenalero-mcp` is a thin tools-only `stdio` adapter exposing exactly five tools. Skills are read-only and runtime evidence is journaled outside them.

**Tech stack:** Rust stable, official Rust MCP SDK, Tokio, Serde, Schemars, selected CommonMark parser, SHA-256, UUIDv7, cross-platform application directories, proptest, cargo-deny.

---

## Global constraints

- Constitución de Código Agéntico v1.0 governs the work.
- Risk classification: medium.
- Use an isolated Git worktree.
- Do not work on `main`.
- TDD is mandatory.
- Use Context7 before every external API integration.
- No network runtime.
- No shell execution.
- No hooks.
- No DB.
- No internal LLM.
- No skill routing or activation.
- No writes inside skill roots.
- Exactly five tools.
- No semantic claim of verification in v1.
- Every task is one independently reviewable commit.
- Any scope expansion requires SDD update and human approval.

---

## File map

```text
arsenalero/
├── .codex-plugin/
│   └── plugin.json
├── .mcp.json
├── Cargo.toml
├── Cargo.lock
├── rust-toolchain.toml
├── deny.toml
├── README.md
├── crates/
│   ├── arsenalero-core/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── domain.rs
│   │       ├── error.rs
│   │       ├── path_policy.rs
│   │       ├── markdown.rs
│   │       ├── inventory.rs
│   │       ├── classify.rs
│   │       ├── case.rs
│   │       ├── receipt.rs
│   │       ├── journal.rs
│   │       └── reconcile.rs
│   └── arsenalero-mcp/
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs
│           ├── server.rs
│           ├── schema.rs
│           └── tools.rs
├── tests/
│   ├── fixtures/
│   │   ├── valid_bilingual/
│   │   ├── unresolved/
│   │   ├── broken_reference/
│   │   ├── path_escape/
│   │   ├── symlink_escape/
│   │   ├── duplicate_id/
│   │   └── digest_drift/
│   └── integration/
│       └── mcp_stdio.rs
├── evals/
│   ├── cases.jsonl
│   ├── labels.jsonl
│   └── README.md
└── docs/
    ├── architecture/
    │   └── ARSENALERO_MCP_SDD_v1.3.md
    ├── evidence/
    │   └── context7-ledger.md
    ├── security/
    │   └── threat-model.md
    └── adr/
        ├── 0001-global-mcp.md
        ├── 0002-deterministic-classification.md
        └── 0003-observer-not-enforcer.md
```

---

## Task 1: Constitutional preflight and worktree

**Produces:** isolated branch, clean workspace, copied approved SDD, initial evidence ledger.

- [ ] Create the worktree:

```bash
git worktree add ../arsenalero-worktree -b feat/arsenalero-mcp-v1
cd ../arsenalero-worktree
git status --short
```

Expected: no output from `git status --short`.

- [ ] Create:

```text
docs/architecture/ARSENALERO_MCP_SDD_v1.3.md
docs/evidence/context7-ledger.md
docs/security/threat-model.md
docs/adr/
```

- [ ] Record the task declaration in the branch documentation:

```markdown
Intent: implement Arsenalero MCP v1.3.
Scope: deterministic resource inventory, issue, attest and reconcile.
Excluded: hooks, network, execution, DB, routing, verification adapters.
Validation: Rust quality gates, MCP integration, adversarial fixtures, three-arm eval harness.
```

- [ ] Commit:

```bash
git add docs
git commit -m "docs: establish arsenalero v1.3 design baseline"
```

---

## Task 2: Context7 dependency resolution

**Produces:** completed Context7 entries and pinned dependency decision.

- [ ] Execute all mandatory queries from `CONTEXT7_EVIDENCE_PROTOCOL.md`.

- [ ] Record exact IDs, versions and selected APIs in:

```text
docs/evidence/context7-ledger.md
```

- [ ] Reject any dependency that introduces:

```text
network transport
shell execution
runtime plugin loading
database
unstable branch-only API
```

- [ ] Create a dependency decision table:

```markdown
| Capability | Crate | Version | Context7 ID | Reason |
```

- [ ] Commit:

```bash
git add docs/evidence/context7-ledger.md
git commit -m "docs: record Context7 dependency contracts"
```

Expected: no implementation code added in this commit.

---

## Task 3: Rust workspace and plugin scaffold

**Produces:** compiling two-crate workspace and plugin metadata.

- [ ] Create root `Cargo.toml`:

```toml
[workspace]
members = [
  "crates/arsenalero-core",
  "crates/arsenalero-mcp",
]
resolver = "2"

[workspace.package]
version = "0.1.0"
edition = "2024"
license = "MIT"

[workspace.lints.rust]
unsafe_code = "forbid"
```

- [ ] Create both crate manifests using versions pinned in Task 2.

- [ ] Use the Codex plugin creator to scaffold the plugin, then remove every generated capability except the MCP server.

- [ ] Verify plugin metadata contains no hooks, skills, apps or browser extensions.

- [ ] Run:

```bash
cargo fmt --all --check
cargo check --workspace
```

Expected: exit `0`.

- [ ] Commit:

```bash
git add .
git commit -m "chore: scaffold arsenalero Rust MCP plugin"
```

---

## Task 4: Domain model and reason codes

**Files:**

```text
crates/arsenalero-core/src/domain.rs
crates/arsenalero-core/src/error.rs
```

**Produces:**

```rust
CaseId
ResourceId
ReceiptId
ClassificationSource
Obligation
ResourceKind
EvidenceContract
AttainedEvidenceLevel
ResourceState
ReconciliationStatus
ArsenalError
```

- [ ] Write failing tests for legal transitions:

```rust
assert!(ResourceState::Discovered.can_transition_to(ResourceState::Classified));
assert!(!ResourceState::Discovered.can_transition_to(ResourceState::Attested));
assert!(!ResourceState::Attested.can_transition_to(ResourceState::Issued));
```

- [ ] Run:

```bash
cargo test -p arsenalero-core domain
```

Expected: failure because types do not exist.

- [ ] Implement minimal enums and typed IDs.

- [ ] Implement:

```rust
impl ArsenalError {
    pub const fn code(&self) -> &'static str;
}
```

- [ ] Test every reason code for stability.

- [ ] Run:

```bash
cargo test -p arsenalero-core
cargo clippy -p arsenalero-core --all-targets -- -D warnings
```

Expected: PASS.

- [ ] Commit:

```bash
git add crates/arsenalero-core
git commit -m "feat: define arsenalero domain contracts"
```

---

## Task 5: Filesystem path policy

**Files:**

```text
crates/arsenalero-core/src/path_policy.rs
tests/fixtures/path_escape/
tests/fixtures/symlink_escape/
```

**Produces:**

```rust
pub struct PathPolicy;
pub struct CanonicalSkillRoot;
pub struct CanonicalResourcePath;
```

- [ ] Write failing tests:

```text
allowed root accepted
outside root rejected
../ traversal rejected
symlink escape rejected
directory rejected as resource
unsupported extension rejected
oversized file rejected
```

- [ ] Add proptest:

```text
For every generated relative path containing arbitrary "." and ".." segments,
an accepted canonical resource path must start with the canonical skill root.
```

- [ ] Implement minimal canonicalization and checks.

- [ ] Run:

```bash
cargo test -p arsenalero-core path_policy
```

Expected: PASS.

- [ ] Commit:

```bash
git add crates/arsenalero-core tests/fixtures
git commit -m "feat: enforce read-only skill path policy"
```

---

## Task 6: Markdown scanner and metadata parser

**Files:**

```text
crates/arsenalero-core/src/markdown.rs
crates/arsenalero-core/src/inventory.rs
tests/fixtures/valid_bilingual/
tests/fixtures/broken_reference/
```

**Produces:**

```rust
pub fn scan_skill(source: &str) -> Result<SkillDocument, ArsenalError>;
pub fn parse_resource_metadata(source: &str) -> Result<Option<ArsenalMetadata>, ArsenalError>;
```

- [ ] Write fixtures for:

```text
relative Markdown link
inline-code resources path
inline-code references path
free filename candidate
external URL ignored
frontmatter metadata
heading and list context
```

- [ ] Write expected golden JSON for every fixture.

- [ ] Implement event-based parsing using the Context7-selected parser.

- [ ] Do not use regex as the primary Markdown parser.

- [ ] Run:

```bash
cargo test -p arsenalero-core markdown
```

Expected: golden snapshots match.

- [ ] Commit:

```bash
git add crates/arsenalero-core tests/fixtures
git commit -m "feat: scan skill resource references"
```

---

## Task 7: Deterministic classification and REQUIRED calculation

**Files:**

```text
crates/arsenalero-core/src/classify.rs
tests/fixtures/unresolved/
tests/fixtures/valid_bilingual/
```

**Produces:**

```rust
pub fn classify_resource(input: ClassificationInput) -> ClassifiedResource;
pub fn required_set(resources: &[ClassifiedResource]) -> BTreeSet<ResourceId>;
```

- [ ] Write table-driven tests for every English and Spanish stage alias.

- [ ] Write tests proving DERIVED does not imply REQUIRED.

- [ ] Write tests:

```text
metadata requirement=required → REQUIRED
"must consult" → REQUIRED
"debe consultar" → REQUIRED
"use this example" → OPTIONAL
heading+purpose verb without normative marker → RECOMMENDED
no heading or purpose verb → UNKNOWN
contradictory markers → UNRESOLVED
```

- [ ] Implement exact normalized-token matching.

- [ ] Do not use fuzzy matching, embeddings or LLM calls.

- [ ] Run:

```bash
cargo test -p arsenalero-core classify
```

Expected: PASS.

- [ ] Commit:

```bash
git add crates/arsenalero-core tests/fixtures
git commit -m "feat: classify skill resources deterministically"
```

---

## Task 8: Cases, receipts and digest drift

**Files:**

```text
crates/arsenalero-core/src/case.rs
crates/arsenalero-core/src/receipt.rs
tests/fixtures/digest_drift/
```

**Produces:**

```rust
pub struct ArsenalCase;
pub struct ResourceReceipt;
pub fn issue_resources(...);
pub fn attest_resources(...);
```

- [ ] Write tests:

```text
receipt bound to case
receipt bound to skill digest
receipt bound to resource digest
issue returns evidence capability, never attained evidence
attest computes attained_evidence_level
cross-case receipt rejected
batch > 4 rejected
attestation batch > 16 rejected
empty usage rejected
resource modified before attest → RECEIPT_STALE
skill modified → SKILL_DIGEST_CHANGED
```

- [ ] Implement UUIDv7 IDs using the Context7-confirmed API.

- [ ] Recalculate digests before accepting attestations.

- [ ] Run:

```bash
cargo test -p arsenalero-core receipt case
```

Expected: PASS.

- [ ] Commit:

```bash
git add crates/arsenalero-core tests/fixtures
git commit -m "feat: issue case-bound resource receipts"
```

---

## Task 9: Journal and reconciliation

**Files:**

```text
crates/arsenalero-core/src/journal.rs
crates/arsenalero-core/src/reconcile.rs
```

**Produces:**

```rust
pub struct JournalWriter;
pub struct ReconciliationReport;
pub fn reconcile(...);
```

- [ ] Write tests for hash-linked event order.

- [ ] Write reconciliation tests:

```text
all required issued+attested → Complete
missing issue → Incomplete
missing attestation → Incomplete
post-attest resource change → NeedsReview
skill change → Invalidated
unresolved resource → NeedsReview
verification.status always not_supported_in_v1
attestation breakdown sums to protocol_completion.attested
externally_verified <= artifact_referenced
evidence_coverage.artifact_referenced == attestation_breakdown.artifact_referenced
```

- [ ] Implement exact report fields from SDD.

- [ ] Implement `validate_reconciliation_invariants(report)` and return
`RECONCILIATION_INVARIANT_VIOLATION` on internal inconsistency.

- [ ] Ensure ratios handle zero denominators without NaN.

- [ ] Run:

```bash
cargo test -p arsenalero-core reconcile journal
```

Expected: PASS.

- [ ] Commit:

```bash
git add crates/arsenalero-core
git commit -m "feat: reconcile resource case evidence"
```

---

## Task 10: MCP schemas and five tool handlers

**Files:**

```text
crates/arsenalero-mcp/src/schema.rs
crates/arsenalero-mcp/src/server.rs
crates/arsenalero-mcp/src/tools.rs
```

**Produces exactly:**

```text
arsenal_init
arsenal_stage
arsenal_issue
arsenal_attest
arsenal_reconcile
```

- [ ] Write a test that lists tools and asserts exact equality with the five names.

- [ ] Write schema snapshot tests for every input and output.

- [ ] Implement concise tool descriptions with:

```text
purpose
precondition
non-goal
when to call
usual next step
```

- [ ] Map domain errors to tool execution errors with `isError=true`.

- [ ] Return `structuredContent` and serialized JSON TextContent.

- [ ] Run:

```bash
cargo test -p arsenalero-mcp
```

Expected: PASS and exactly five tools.

- [ ] Commit:

```bash
git add crates/arsenalero-mcp
git commit -m "feat: expose arsenalero MCP tools"
```

---

## Task 11: Stdio integration and hostile fixture suite

**Files:**

```text
tests/integration/mcp_stdio.rs
tests/fixtures/*
```

- [ ] Spawn the compiled server through stdio.

- [ ] Test protocol initialize.

- [ ] Test list tools.

- [ ] Execute a complete case.

- [ ] Execute an incomplete case.

- [ ] Test malformed request as protocol error.

- [ ] Test domain failure as tool execution error.

- [ ] Run hostile fixtures:

```text
path escape
symlink escape
broken reference
oversized resource
duplicate ID
cross-case receipt
resource drift
skill drift
```

- [ ] Run:

```bash
cargo test --workspace --all-features
```

Expected: PASS.

- [ ] Commit:

```bash
git add tests
git commit -m "test: verify arsenalero MCP over stdio"
```

---

## Task 12: Plugin installation and smoke test

- [ ] Install locked binary:

```bash
cargo install --path crates/arsenalero-mcp --locked
arsenalero-mcp --version
```

Expected:

```text
arsenalero-mcp 0.1.0
```

- [ ] Validate plugin packaging using the current Codex plugin tooling.

- [ ] Start a fresh Codex session.

- [ ] Confirm the five tools are present.

- [ ] Run a fixture skill manually and capture the reconciliation output.

- [ ] Record evidence in:

```text
docs/evidence/plugin-smoke-test.md
```

- [ ] Commit:

```bash
git add docs .codex-plugin .mcp.json
git commit -m "test: validate Codex plugin installation"
```

---

## Task 13: Three-arm agent eval harness

**Files:**

```text
evals/cases.jsonl
evals/labels.jsonl
evals/README.md
```

- [ ] Create 20 labeled cases:

```text
required
recommended
optional
unresolved
bilingual
post-attest drift
```

- [ ] Define arms A, B and C exactly as specified in the SDD.

- [ ] Run each case three times per arm.

- [ ] Calculate:

```text
task success
issue recall
attestation recall
required-set precision
required-set recall
false-complete
unnecessary issuance
tool calls
bytes
latency
pass@3
pass^3
```

- [ ] Store raw traces separately from aggregate metrics.

- [ ] Do not tune the system on the locked regression subset.

- [ ] Commit eval definitions, not transient raw outputs:

```bash
git add evals
git commit -m "test: define arsenalero system evals"
```

---

## Task 14: Constitutional review and final reconciliation

- [ ] Run:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo deny check
git status --short
```

Expected:

```text
all commands exit 0
git status --short produces no unexplained files
```

- [ ] Compare:

```text
approved SDD
implementation diff
test evidence
Context7 ledger
plugin smoke test
eval results
```

- [ ] Reject closure if:

```text
sixth tool exists
hooks exist
network exists
execution exists
verified semantic claim exists
DERIVED automatically becomes REQUIRED
```

- [ ] Request code review.

- [ ] Fix findings.

- [ ] Re-run all gates.

- [ ] Commit:

```bash
git add .
git commit -m "chore: reconcile arsenalero v1 implementation"
```

- [ ] Prepare PR with:

```text
goal
scope
non-goals
risk
test evidence
Context7 evidence
eval summary
known limitations
rollback
```

---

## Execution order

```text
1 → 2 → 3 → 4 → 5 → 6 → 7 → 8 → 9 → 10 → 11 → 12 → 13 → 14
```

Tasks must not be parallelized when they share contracts. Tasks 5 and 6 may be explored independently only after Task 4 contracts are frozen, but implementation remains sequential to prevent divergent types.

---

## Final no-go rule

Do not merge because the MCP “works” in one demonstration.

Merge requires:

- deterministic regression suite;
- adversarial tests;
- plugin smoke test;
- three-arm evaluation;
- Context7 ledger;
- human review;
- constitutional reconciliation.
