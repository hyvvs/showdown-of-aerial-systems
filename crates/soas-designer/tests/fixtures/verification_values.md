# S3-01b verification ledger — synthetic test values only

No new physical number is introduced for Fix 1: every design is cloned from
the S3-01 fixture, whose complete numeric-path ledger is semantic_values.md.
Changing a quantity identity, label, scope or provenance flag does not change
its prescribed synthetic value, units or other expected numeric output.

- Parameter/component/flag indices 0 and the ordinal iteration order refer to
  the existing tail-law parameter and fixture controls; they are structural
  test settings, not aircraft data.
- Four spellings per independently listed quantity are canonical underscore,
  upper case, hyphen and space forms. These are label settings, not geometry.
- Wing/tail/fuselage/interference scopes and USER_SUPPLIED/TEMPLATE_DEFAULT
  origins form the exhaustive test cross-product. Registry coverage is checked
  against the independently authored QUANTITIES catalogue, not itself.
- Control sign preflight reverses the existing hinge endpoints with no new
  coordinates. Expected physical sign strings are unchanged from D6 and the
  original S3-01 assertions. No fixture correction is made by this test.
- No tolerance, physical constant, numerical solver setting or new formula
  is introduced. The control sign definitions come from CONVENTIONS, Forces
  and moments; quantity coverage comes from the owner's Fix 1 requirement and
  planform::WingGeometry, VLM/LLT outputs, Fourier terms and section loadings.
- Fix 1 adds only explicit `SuppliedNonWing` identity metadata to the original
  two fixture parameters. Their numbers and the entire expected numeric inventory
  remain unchanged. The scratch control compatibility experiment uses the
  original rudder endpoints reversed in memory, not a committed fixture edit.
  Its dot products -5 and +5 are derived observations from those existing
  endpoint/parent coordinates under the owner's clarified ordering formula,
  not additional physical inputs or tolerances.
