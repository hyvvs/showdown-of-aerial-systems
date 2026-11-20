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

## Fix 6

- SYNTHETIC TEST tensor [[1,2,0],[2,1,0],[0,0,1]] kg m² has positive
  diagonal, eigenvalues 3,-1,1 and determinant -3. It is symmetric and
  invertible but indefinite. The hand witness vector (1,-1,0) gives quadratic
  form -2. The 2-by-2 block eigenvectors (1,1) and (1,-1) give eigenvalues
  3 and -1 directly; the last axis has eigenvalue 1.
- SYNTHETIC positive control [[2,1,0],[1,2,0],[0,0,1]] kg m² has eigenvalues
  3,1,1: positive off-diagonals are allowed when the whole tensor is SPD.
  No tensor in the shared fixture or expected-number inventory is changed.
- Source probes are an empty list, an empty string and whitespace/tab/newline,
  applied under TEMPLATE_DEFAULT to every independently inventoried present
  numeric path; each contribution's own source text is also checked. No
  physical number is introduced by those metadata probes.
- Indices 0 and the existing contribution order select fixture records. There
  is no numerical tolerance. Mutation h is the verifier's positive-diagonals
  plus finite-inverse substitute; mutation i exempts TEMPLATE_DEFAULT from
  the source requirement. They are tested only in scratch copies.
- The positive-definite definition is that every nonzero vector has positive
  quadratic form; this hand witness disproves it without numerical eigenvalue
  computation. Source: existing core/semantic SPD contract, owner Fix 6 and
  the verifier's mutation definitions (recorded in S3_01C_VERIFICATION).

## Fix 7 — independent synthetic geometry and numerical settings

The tests in hinge_containment.rs construct their own Design, never the
shared fixture. All geometry below is SYNTHETIC TEST DATA, not aircraft data.

- Root [0,0,0] m; tips [0,8,0], [0,0,-8], [0,0,8], [0,6,8], [6,8,0] m
  exercise horizontal, dorsal, ventral, inverted/canted and swept directions.
  Squared spans are 64 or 100 m², by hand. Both station chords are 2 m;
  incidence/twist 0 rad, no airfoil input, no mass/inertia/engine/contribution.
- Control fraction .2 and limits -.3/.4 rad reuse earlier synthetic settings.
  Source flags distinguish present inputs from missing aggregates. There is
  no real-aircraft or aerodynamic-effectiveness claim.
- Inside fractions .25/.75 imply hand projections 16/48 or 25/75 m². Boundary
  cases use 0/1; partial/full outside cases use -.5,-.25,1.25,1.5, independently
  of any production projection helper. All are deliberate geometric probes.
- Slightly-outside fraction offset sqrt(f64::EPSILON) is approximately
  1.4901161193847656e-8, about 1.1920928955078125e-7 m for the 8 m span.
  It is a probe input, not a tolerance. It exceeds rounding of basic arithmetic
  while remaining small relative to this synthetic geometry. Source: Rust
  f64::EPSILON definition; direct square root and multiplication.
- Scaling cases 1/1024,1,1024 scale that geometry and offset together; a fixed
  metric allowance must not replace geometry-based rounding bounds.
- The chordwise non-check case uses start [16,2,32], end [-16,6,-32] m,
  with a y-directed parent span. These deliberately extreme x/z coordinates
  verify the requested absence of a chordwise containment rule.
- Structural xyz/bound indices 0/1/2 and four endpoint products in interval
  multiplication are algorithm structure, not an empirical tolerance. No fixed
  tolerance number is added: the interval rule is derived and sourced in
  S3_01_SEMANTIC_MODEL, Span containment and rounding allowance.
- Computed rounding observations for the independent 8 m test span at point
  [0,2,0] m (not input tolerances): Rust scratch trace gives root enclosure
  [15.999999999999991,16.000000000000018] m² and tip enclosure
  [-48.00000000000004,-47.999999999999964] m². Relative to rounded projections
  16/-48, derived outward allowances are 1.7763568394002505e-14 and
  4.263256414560601e-14 m²; dividing by span 8 gives
  2.220446049250313e-15 and 5.329070518200751e-15 m. These are results of the
  documented algorithm on synthetic data, never hard-coded acceptance bounds.
  Evidence: target/s3-01c2/span-roundoff-trace.txt.
