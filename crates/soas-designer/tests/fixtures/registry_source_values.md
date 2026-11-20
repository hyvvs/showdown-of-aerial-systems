# S3-01d structural and synthetic probe ledger

No new aircraft number, physical constant or numerical tolerance is introduced.
Shared fixture values retain semantic_values.md / closed_kind_values.md as
their synthetic ledgers; no shared fixture or expected number is edited.

## Fix 8

- Counts 6 and 42 are the existing approved supplied/computed catalogue sizes,
  not physical data. Slots 0..5 and 0..41 are independently written structural
  test indices corresponding to those documented variants. Boolean arrays
  check presence/duplicates; no discriminant numbering assumption is used.
- Exhaustive match functions have no wildcard arm. A new variant must be
  explicitly classified even if omitted from every runtime list.
- Scratch WingArea uses the existing computed wing_area_m2 identity, Tail
  scope and m^2 unit solely as a forbidden supplied-enum mutation. No value
  is introduced as a real design input. A computed variant omitted from ALL
  and list-entry removal are also deliberately wrong structural mutations.
- Source: owner's Fix 8, current semantic-model closed catalogues, and
  verifier h2 (unlisted WingArea plus method arms, leaving ALL unchanged).

## Fix 9

- Existing SYNTHETIC TEST value -0.7 is cloned from the tail-law fixture for
  source-validation probes; legitimate kinds/scopes/units are declared
  explicitly. The dimensionless fuselage probe is an arbitrary synthetic
  test value, not real aircraft data or an assembled aerodynamic contribution.
  No shared value or expected-number entry changes.
- ASCII display label: 256 repetitions of x. Kind-label cases: existing
  canonical fuselage_pitching_moment_contribution once and repeated 8 times.
  The enum kind remains FuselagePitchingMomentContribution in both cases;
  there is no new/unknown kind or kind-field serialization. The identifier is
  37 ASCII bytes; the eight-copy label is 296 bytes (derived string lengths).
- Unicode display label: U+7FFC repeated 64 times, 192 UTF-8 bytes. These are
  synthetic string-length settings, not physical numbers or source exceptions.
- Empty list, empty string, ASCII space/tab/CR/LF, Unicode U+2003/U+00A0
  whitespace and a valid-plus-empty reference list are intentional invalid
  metadata probes. Contribution source text is checked separately. Positive
  controls keep valid sources to establish that labels/kinds are accepted.
- Mutation k uses the verifier's exact condition: USER_SUPPLIED plus a label
  slice length greater than 128 bytes exempts numeric source checking. The
  cutoff exists only in the uncommitted fault, never production validation.
- Component/contribution slot 0 refers to the existing tail-law record. Source:
  owner Fix 9; target/verifier/scratch/s3_01c2_mutations.py, mutation k; existing
  validation.rs source contract. No new tolerance, formula or dependency.
