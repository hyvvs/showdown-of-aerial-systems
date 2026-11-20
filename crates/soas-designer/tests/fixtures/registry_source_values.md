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
