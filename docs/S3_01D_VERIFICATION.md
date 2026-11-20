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
