# S3-01c2 synthetic-value and test-setting ledger

Every aircraft value in these tests is SYNTHETIC, not a real aircraft number.
The starting fixture, numeric inventory, engine data and earlier invalid
probes retain semantic_values.md and verification_values.md as their ledgers.

## Fix 5

- Parameter/component index 0 selects the existing tail-law value -0.7;
  the supplied-kind matrix reuses it except for CLmax, which reuses the
  existing synthetic +1.4 lift-limit value from contribution index 1.
  Missing draft variants use None, not zero. No physical default is added.
- The six supplied identities are the owner's five examples plus the
  existing tail pitching-moment slope. Their expected scope/unit pairs are
  independently written in closed_kinds.rs; these are schema settings,
  not computed aerodynamic results or coefficients to assemble now.
- All six provenance flags and six contribution scopes form the exhaustive
  origin/scope matrix. Present values allow USER_SUPPLIED or TEMPLATE_DEFAULT,
  also with OUTSIDE_COVERAGE; absent draft values require NOT_ESTIMATED.
- Twelve computed kinds reproduce the verifier's list: area, span, aspect
  ratio, taper ratio, MAC, quarter-chord sweep, CL, lift slope, induced drag,
  pitching-moment slope, neutral point, efficiency. Labels use canonical,
  uppercase, hyphen, space, _unit, _m, _m2, est_, calc_, user_, embedded
  text and tail_<quantity>_scale_unit forms. Both supplied origins are checked
  in each legitimate non-wing scope; explicit computed kinds remain rejected.
- Additional computed identities cover sampled leading-edge x/z, twist,
  section slope/zero-lift angle, finite-segment/ray/horseshoe velocity per
  circulation vectors, and the thin-airfoil slope helper. No result values
  are transcribed or used as aircraft inputs.
- Unit tests retain the previous unit-string catalogue, including deliberate
  non-SI/unknown/wrong-case probes. Expected units now come from the declared
  kind, never a free label's ending. All pairwise mismatches are retained.
- No new tolerance, physical constant, aerodynamic formula or solver setting.
  Policy source: owner's Fix 5 design change. Output coverage source:
  estimator planform/sample, lifting_line, vlm and vortex public APIs.
