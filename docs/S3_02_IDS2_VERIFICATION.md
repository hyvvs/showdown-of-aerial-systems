# S3-02-ids2: identity comparisons and declaration limits

**Historical report; later decision update (2026-11-21):** the results below
record Fixes 10/11 as run. The then-open attribution/cap questions are now
answered by DECISIONS D23/D24: option (c) author-declared attribution, CLOSED
policy gates/ACCEPTED LIMITS, and a 64-character cap for all designer IDs
to be implemented in S3-02b1. The historical test output remains unchanged;
no reader or cap code is implemented by that documentation decision.

Fresh task/s3-02-ids2 starts from main 80ab1bb. The reader is not started.
No dependency, production Rust source, type, existing assertion, fixture
physical value or numerical tolerance is changed by this batch.

## Fix 10 — complete, case-sensitive station identity

### What changed and files touched

Six tests were appended to crates/soas-designer/tests/station_ids.rs, with
synthetic labels/digest settings in tests/fixtures/station_id_values.md.
Production already compares full String identities correctly; the new tests
pass immediately, without an implementation change. The implementation
commit records evidence/status/concepts only in this report, TASKS and
LEARNING. Tests commit: a7e4355, Fix 10: tests.

Each pair runs on the existing independent wing, horizontal-tail, dorsal-fin
and ventral-fin cases. The tests require successful validation, unchanged
opaque endpoint labels, the complete independently authored numeric-path/
value/unit inventory and distinct station-specific source records. Expected
paths are not obtained from production traversal.

| Accepted synthetic pair | Weak comparisons contradicted |
|---|---|
| A / a | ASCII case-insensitive comparison and length-only identity |
| ab / ba | Byte sum and length |
| ac / ca | Byte sum and length |
| abc / cba | Byte sum and length |
| aa / az | First character and length |
| pa / pb | First character and length |

Actual initial output on unchanged production code:

```text
test fix10_ab_and_ba_remain_distinct_despite_byte_sum_collision ... ok
test fix10_uppercase_a_and_lowercase_a_are_distinct_station_ids ... ok
test fix10_pa_and_pb_remain_distinct_despite_first_character_collision ... ok
test fix10_aa_and_az_remain_distinct_despite_first_character_collision ... ok
test fix10_abc_and_cba_remain_distinct_despite_byte_sum_collision ... ok
test fix10_ac_and_ca_remain_distinct_despite_byte_sum_collision ... ok
test result: ok. 6 passed; 0 failed; 0 ignored; 8 filtered out
```

### Scratch mutations: actual failures

Four independent copies use distinct target/s3-02-ids2/build-NAME build
directories. Before **each mutation is applied**, cargo clean -p
soas-designer --manifest-path ... succeeds (fresh directories report Removed
0 files). Each then compiles its own mutated source with offline/locked
dependencies. TEMP and TMP are target/s3-02-ids2/temp for all test runs.
No mutation/helper is committed and no shared executable is reused.

| Mutation | Deliberately wrong uniqueness key | Failed tests | Exit |
|---|---|---:|---:|
| b_casefold | ASCII-lowercased ID | 1 | 101 |
| h_byte_sum | Unsigned byte sum only | 3 | 101 |
| h_length | Byte length only | 6 | 101 |
| h_first_byte | First byte only | 2 | 101 |

```text
b_casefold:
test fix10_uppercase_a_and_lowercase_a_are_distinct_station_ids ... FAILED
test result: FAILED. 5 passed; 1 failed; 0 ignored; 8 filtered out

h_byte_sum:
test fix10_ab_and_ba_remain_distinct_despite_byte_sum_collision ... FAILED
test fix10_ac_and_ca_remain_distinct_despite_byte_sum_collision ... FAILED
test fix10_abc_and_cba_remain_distinct_despite_byte_sum_collision ... FAILED
test result: FAILED. 3 passed; 3 failed; 0 ignored; 8 filtered out

h_length:
test result: FAILED. 0 passed; 6 failed; 0 ignored; 8 filtered out

h_first_byte:
test fix10_pa_and_pb_remain_distinct_despite_first_character_collision ... FAILED
test fix10_aa_and_az_remain_distinct_despite_first_character_collision ... FAILED
test result: FAILED. 4 passed; 2 failed; 0 ignored; 8 filtered out
```

The failure is a false duplicate-identity error on a pair that the independent
test requires to coexist. These probes protect against the specified weak
digests; they are not a proof about every possible future hash function.
The actual production key remains the full case-sensitive String.

Raw logs: target/s3-02-ids2/fix10-initial.txt, mutation-NAME.txt and
clean-NAME.txt. All three Fix 10 gates pass: cargo fmt --all -- --check;
cargo clippy --all-targets --all-features -- -D warnings; and the full
unfiltered cargo test --all, **249 passed, 0 failed, 0 ignored**. Logs:
fix10-fmt.txt, fix10-clippy.txt and fix10-all.txt. Rust emits cache hard-link
warnings and automatically copies files; this was disclosed, with no
production profile or cache setting changed.

### Assumptions, sources and concepts

Only synthetic ID labels are new; every physical value/setting reuses the
existing station ledger. There is no new physical formula. Sources: owner
Fix 10 scope, DECISIONS D18 and S3_01_SEMANTIC_MODEL's opaque-ID contract.

- Case sensitivity is part of identity, not display formatting.
- Two different strings may have the same cheap digest; equality must still
  compare their complete identities.
- Positive acceptance tests expose an over-restrictive uniqueness check,
  even when all rejection tests pass.

No owner question is needed for Fix 10. Existing production behavior is
correct; only coverage/evidence changes.

## Fix 11 — path presence and historical attribution limits

### What changed and files touched

Three tests and their ledger notes characterize today's model; they are not
future binding acceptance assertions. Production checks were not changed.
ASSUMPTIONS and S3_02_PLAN now explain current path-only coverage, the exact
future ID/order/snapshot mechanisms for every array category, OPEN historical
binding cases, and candidate mechanisms with editing/trust trade-offs.
The 64-character station/component ID cap is PROPOSED, NOT APPROVED, with
no validation code or boundary test. The future type-plus-fixture rule
requires one combined, explicitly named commit so every commit builds.

ROADMAP requires the source-attribution notice on every report and validation
page, with future output/mutation checks. QUESTIONS records the unresolved
choice and cap proposal. TASKS records completion and the narrow dependency
boundary; LEARNING contains concepts only, with owner columns blank.
Tests commit: 73f834f, Fix 11: tests. Implementation commit contains docs only.

### Tests and scratch failure

On unchanged production checks, each of the four original independent surface
cases accepts exchanging existing station IDs without moving the stations,
and accepts exchanging the two valid, same-unit chord source records. Exact
path/value assertions show which data moved and which did not. A separate
negative control removes a required record and demands its exact missing-path
error. All values/indices/sources reuse the existing synthetic ledger.

```text
test fix11_path_presence_still_rejects_a_missing_station_record ... ok
test fix11_current_model_accepts_same_unit_source_record_swaps ... ok
test fix11_current_model_accepts_swapped_existing_station_ids_without_history ... ok
test result: ok. 3 passed; 0 failed; 0 ignored; 14 filtered out
```

After package clean, an independent scratch copy bypasses only the required-
path guard while retaining other metadata validation. Actual failure:

```text
test fix11_path_presence_still_rejects_a_missing_station_record ... FAILED
left: Ok(())
right: Err(DesignError { field: "components[surface-a].surface.stations[tip].chord_m", reason: "numeric provenance missing" })
test result: FAILED. 2 passed; 1 failed; 0 ignored; 14 filtered out
exit: 101
```

This mutation demonstrates the negative control's purpose; it does not
implement or choose historical verification. Clean succeeded before editing
the copy (Removed 0 files). Distinct build directory: build-fix11_missing_path;
raw clean/mutation logs: clean-fix11_missing_path.txt and
mutation-fix11_missing_path.txt under target/s3-02-ids2/. TEMP/TMP are local.

All Fix 11 gates pass: formatting, Clippy with -D warnings, and full unfiltered
cargo test --all: **252 passed, 0 failed, 0 ignored**. Logs: fix11-fmt.txt,
fix11-clippy.txt and fix11-all.txt. Production Rust source/manifests and all
original tests/fixtures/values/tolerances remain unchanged.

### Assumptions, unknowns and owner choices

The owner explicitly chose to record BIND-01/02/03 as OPEN gates, not invent
an attribution mechanism. Only policy-dependent S3-02b2 binding tests wait;
S3-02b1 can be separately authorized with the limit in its acceptance notes.
Object/kind checks and unrelated work need not wait. No reader starts here.

M1 identity paths preserve whole-record reorder, not every pre-existing-ID
reassignment. M2 ordering rejects data permutations, not source-only swaps.
M3 snapshots reject differing values paired with stale snapshots: in the
future file, unequal station coordinate snapshots would catch an ID-only
swap with stale records. Matching values/null plus a different source still
match; the current model has no such snapshots at all.

If the same final document could be an intended declaration or the result
of a swap, its structure cannot reveal which history occurred. An editable
inline hash can be recomputed; unique source records can still exchange.
The plan compares per-record hashes, trusted whole-document hashes/signatures,
explicit acceptance/disclosure of declared attribution, and unique source
records. Recommendation: explicit declaration/disclosure for the first
format; **no candidate is selected**. Until a mechanism is chosen, outputs
must say: "Source attribution is declared by the author and is not
machine-verified." A later check must state exactly what it verifies.

The cap proposal bounds path/error/report size and accidental resource use;
64 is the owner's requested proposed value, not a standard. Current ASCII
identifiers have equal byte/character counts. No truncation, case folding,
digest identity or length restriction is implemented. Future shared-type
changes must include all constructor/fixture migration in the same buildable
commit; tests-first commits must compile, even when an intended assertion
fails. The earlier station-ID history is retained.

### Sources and concepts

Sources: owner's Fix 11 request and follow-up conditions; DECISIONS D18;
approved S3_02_PLAN inventory; current validation.rs/numbers.rs and the
independent tests. No new physical formula/value is introduced.

- Path coverage and source ownership/history are different guarantees.
- A snapshot/fingerprint checks a declared relationship; without a trusted
  reference it cannot determine whether an author changed that declaration.
- An approved documentation rule is distinct from a proposed length cap or
  an unchosen attribution mechanism.

Questions remaining: which attribution policy/mechanism should close the
scoped BIND gates, and whether to approve the proposed ID cap. Options and
recommendations are in S3_02_PLAN sections 3.3.1 and 3.3.2. Neither question
blocks unrelated work or authorizes the reader in this batch.

## Least confident about

No chosen mechanism currently supports a historical-attribution assertion.
The matrix distinguishes implemented path checks, future snapshot consistency
and the irreducible source-only ambiguity; fingerprints must not be presented
as physical source validation. The recommendation remains an owner choice.

## Assumptions I made

Fix 11's tests describe current model behavior and protect path presence;
they do not weaken or replace future binding requirements. No production
change is needed for exact identity comparison or current path guards.
The prospective atomic-migration rule does not rewrite earlier commits.

## Commands for the owner to verify

From the checkout, set TEMP and TMP to target/s3-02-ids2/temp, then run:

```text
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all
```

Inspect the two tests/implementation commit pairs. To rerun a scratch variant,
use a separate copy/build directory, run cargo clean -p soas-designer before
applying the mutation, and run the corresponding fix10_ or fix11_ tests.
Do not apply a mutation to production files or commit one.
