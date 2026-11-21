# S3-02-ids2: identity comparisons and declaration limits

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
