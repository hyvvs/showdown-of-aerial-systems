# S3-01d coverage follow-up

Branch `task/s3-01d` starts at `task/s3-01c2` (`906afe5`). Scope is only
Fixes 8/9 and the owner's requested future acceptance conditions. No S3-02,
new dependency, shared fixture edit, source-data number or tolerance change.

## Fix 8: enum/list completeness

Tests first: `3972491 Fix 8: tests`. The current six supplied and 42 computed
variants were valid, so the new unmutated tests pass immediately (2 passed).
Before committing, an isolated copy reproduced verifier h2: add WingArea
to the supplied enum and all metadata methods but leave ALL unchanged.

```text
old supplied_catalogue_is_closed_and_disjoint_from_all_computed_results:
test result: ok. 1 passed; 0 failed; 0 ignored
independent escape probe: the new WingArea is accepted as supplied data
test result: ok. 1 passed; 0 failed; 0 ignored

new enum_completeness tests:
error[E0004]: non-exhaustive patterns:
  SuppliedQuantityKind::WingArea not covered
error: could not compile soas-designer (test enum_completeness)
```

These are expected scratch results, not a claim that unchanged production
already contains WingArea. Logs under target/s3-01d:
fix8-before-old-catalogue.txt, fix8-before-escape.txt,
fix8-red-unlisted-variant.txt. The last run exited 101 because the new
test match is exhaustive even for variants omitted from ALL.

Production now generates supplied enum/list/name/scope/unit matches from
one declaration, as the computed enum/list already did. Both have independent
no-wildcard test matches. Public variants/order/names/units/scopes are
unchanged. Tests `every_supplied_enum_variant_is_listed_exactly_once` and
`every_computed_enum_variant_is_listed_exactly_once` check every approved
slot, reject duplicates and fail to compile for any unclassified variant.
Existing independent closed/disjoint catalogues remain unchanged.

### Post-change scratch mutations (never committed)

| Case | Deliberate source fault | Observed result |
|---|---|---|
| a, supplied | Add WingArea directly to generated enum and metadata matches, without changing ALL or test classification | E0004: SuppliedQuantityKind::WingArea not covered |
| b | Add WingArea/wing_area_m2 through the supplied declaration, so it is automatically listed | Existing closed/disjoint catalogue test fails an assertion |
| b, direct variant name | Add Area (the computed variant's actual name) with wing_area_m2 through the supplied declaration | Same closed/disjoint catalogue assertion fails |
| c, supplied | Remove the first supplied ALL entry only | New supplied completeness test fails: missing supplied variant |
| c, computed | Remove the first computed ALL entry only | New computed completeness test fails: missing computed variant |
| a, computed | Add ExtraComputed and metadata mapping without adding it to ALL or test classification | E0004: ComputedWingQuantity::ExtraComputed not covered |

Each compiler failure is specifically E0004, not a syntax/build-environment
failure. Each runtime failure compiled and exited 101 with:

```text
test result: FAILED. 0 passed; 1 failed; 0 ignored
```

Logs are target/s3-01d/fix8-a-*.txt, fix8-b-registered-wing-area.txt,
fix8-c-*.txt. Copies restore production source after each mutation series.
TEMP/TMP remain inside the checkout; source/build copies are ignored target
artifacts. No mutant or helper is committed. Synthetic/structural settings:
tests/fixtures/registry_source_values.md. Sources and extension procedure:
S3_01_SEMANTIC_MODEL, Enum/list completeness; owner Fix 8; Rust E0004 and
macro repetition documentation linked there. No numerical formula is added.

The direct-name case also has its own log, fix8-b-computed-area.txt. The
generated list cannot silently omit a newly declared variant; deliberately
breaking generation is caught by independent exhaustive matches/presence
tests. Compilation errors and runtime failures are separate recorded checks.

Fix 8 gates pass: fmt, Clippy with warnings denied, and full unfiltered
workspace suite **231 passed, 0 failed, 0 ignored**. Logs: target/s3-01d/
fix8-green.txt, fix8-clippy.txt and fix8-all.txt. The existing compiler-cache
hard-link warning caused Rust's automatic copy fallback; no cache, profile
or dependency setting changed. Existing tests and fixture data are unchanged.

## Fix 9: USER_SUPPLIED sources with adversarial labels

Tests first: `3c15e7e Fix 9: tests`. Production source validation was already
correct and remains byte unchanged. Three new tests pass immediately on it:

```text
long_user_label_requires_nonempty_sources ... ok
long_kind_label_requires_nonempty_user_sources ... ok
unicode_user_label_requires_nonempty_sources ... ok
test result: ok. 3 passed; 0 failed; 0 ignored
```

Before committing, the verifier's exact mutation k was applied only in an
ignored scratch copy. It exempts numeric-source checking when USER_SUPPLIED
is present and the text after `.parameters[` is longer than 128 bytes. All
three new tests compile, then fail their exact-error assertions:

```text
long_user_label_requires_nonempty_sources ... FAILED
long_kind_label_requires_nonempty_user_sources ... FAILED
unicode_user_label_requires_nonempty_sources ... FAILED
  left: Ok(())
  left: Ok(())
  left: Ok(())
test result: FAILED. 0 passed; 3 failed; 0 ignored
```

Each expected result is the parameter-path DesignError with reason
`source or missing-input explanation required`; full paths/labels are in
target/s3-01d/fix9-red-k.txt. Positive controls have valid sources and pass,
so a label/kind syntax rejection cannot masquerade as a source rejection.

| Test | Independent input and checks |
|---|---|
| Long user label | 256 ASCII x characters, explicit TailPitchingMomentSlope/Tail/`1/rad` unit contract; source references and contribution source text reject empty/whitespace values. |
| Long kind label | Genuine FuselagePitchingMomentContribution canonical ID, once and repeated eight times as display text; scope Fuselage/unit 1. The closed enum kind stays legitimate; no new kind is introduced. |
| Unicode user label | 64 U+7FFC characters, 192 UTF-8 bytes; demonstrates that byte length versus character count cannot exempt a source. |

Every case checks empty reference lists, empty strings, ASCII whitespace,
Unicode whitespace, and a mixed valid/empty reference list. Contribution
source text is checked independently, with valid numeric-source metadata.
All expected field/reason checks are explicit; no existing test or tolerance
changes. The only numeric probe value is the previously labeled synthetic
fixture value, not a real aircraft coefficient. Complete string/count/probe
ledger: tests/fixtures/registry_source_values.md, Fix 9.

The implementation commit is documentation/evidence only; it does not alter
an already-correct production guard. Source: owner Fix 9; verifier mutation
k in target/verifier/scratch/s3_01c2_mutations.py; existing validation.rs
source contract. Logs: fix9-unmutated.txt and fix9-red-k.txt. All test and
scratch runs set TEMP/TMP inside this checkout. No mutant is committed.

### Future acceptance conditions recorded, not implemented

- S3-02 / every serialized designer format: automatically discover actual
  schema-defined kind fields, then separately test unknown, missing,
  wrong-case, numeric and duplicate kind fields. Duplicate cases must use
  raw input text so a map cannot discard the duplicate before parsing.
  Invalid/missing writer-side kind declarations must also reject; no label
  inference or default kind. Field/tag layout/version remains S3-02 work.
- S3-05 / every design-writing export: specify the operation's required
  supplied-kind set before implementation. Remove each required kind from
  a complete independently authored design and assert failure before any
  write, with an absent/unchanged target. Bypassing that check (mutation i)
  must fail the test. No absent kind may be replaced by zero or a guess.

These are attached to ROADMAP S3-02/S3-05 and TASKS, with the semantic-model
contract updated. They do not invent which kinds future configurations require.
Current incomplete-draft permission remains for absent values under valid
kinds, without waiving a kind required by a selected format/operation. No
serializer, export/readiness API, wire-format fixture or new dependency is
introduced. S3-02 and S3-05 remain unstarted; no nonexistent export mutation
or serialized-format check is claimed executed in this batch.

## Final batch verification and file map

All three final gates pass: `cargo fmt --all -- --check`,
`cargo clippy --all-targets --all-features -- -D warnings`, and full unfiltered
`cargo test --all`: **234 passed, 0 failed, 0 ignored**. The designer has
57 tests; the unchanged other crates contribute 177. Logs: target/s3-01d/
final-fmt.txt, final-clippy.txt, final-all.txt. TEMP/TMP are checkout-local
for every test/scratch run. Rust's automatic hard-link-to-copy fallback was
disclosed; no build profile/cache setting or dependency changed.

| Fix | Files touched (relative to checkout) |
|---|---|
| 8 | crates/soas-designer/src/quantities.rs; tests/enum_completeness.rs; tests/fixtures/registry_source_values.md; docs/ASSUMPTIONS.md, LEARNING.md, S3_01_SEMANTIC_MODEL.md, S3_01D_VERIFICATION.md, TASKS.md |
| 9 | crates/soas-designer/tests/source_label_guards.rs; tests/fixtures/registry_source_values.md; docs/ASSUMPTIONS.md, LEARNING.md, ROADMAP.md, S3_01_SEMANTIC_MODEL.md, S3_01D_VERIFICATION.md, TASKS.md |

Unprefixed tests/ paths are under crates/soas-designer/. Fix 9 changes no
production source. Existing tests, shared fixture/inventory, core/CLI/
estimator source, manifests/lockfile, physical values, signs and tolerances
are unchanged. No test is weakened or ignored; no mutant/helper is committed.
The old manual supplied list is replaced structurally, with the same variants
and metadata. No S3-02 serialization or S3-05 export implementation starts.

**Least confident about:** exhaustive classification prevents unlisted
variants escaping tests; it does not certify a proposed kind's physical
meaning. Nonempty source validation cannot establish source truth. Future
wire-format and export behavior remains unimplemented and must satisfy the
new explicit acceptance gates when those tasks are authorized.

**Assumptions made:** current approved catalogues stay at six supplied and
42 computed variants; any extension needs semantic review and explicit test
classification. A long kind-derived label remains display text, not a new
kind. Required-kind sets and wire layout/version are future task specification
work; no such decision is made in this batch. No owner question is pending.

Task statuses and concepts-only LEARNING entries are updated; owner learning
columns remain blank. The batch has four local tests/implementation commits,
with no push, merge or remote change.
