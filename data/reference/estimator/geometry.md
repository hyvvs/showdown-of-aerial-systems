# T12 planform geometry: definitions and synthetic checks

## Sources, verified online 2026-11-14

- [Caughey, Aerodynamic Background, printed pp. 12-15](https://courses.cit.cornell.edu/mae5070/AeroBackground.pdf):
  eqs. 2.1-2.3 define symmetric area, mean chord integral and aspect ratio;
  eqs. 2.7-2.10 give trapezoid geometry; eqs. 2.11-2.13 define chord-fraction
  sweep/axial location. Eq. 2.15 gives the theoretical thin-airfoil slope 2*pi
  per radian. This ideal section slope is explicit opt-in, not inferred airfoil
  data or a measured universal constant. Zero-lift angle remains an input.
- [NASA NDARC Theory, Appendix A, wing geometry, printed p. 149 / PDF p. 155](https://ntrs.nasa.gov/api/citations/20230003601/downloads/NASA%20TP-20220000355%20appxA.pdf#page=155):
  gives exact area and chord-squared integrals for linearly tapered panels.
- [MIT Unified Fluids Lecture 8, p. 1](https://web.mit.edu/16.unified/www/SPRING/fluids/Spring2008/LectureNotes/f08.pdf):
  defines geometric twist/zero-lift-line incidence relative to a common wing
  reference. Positive twist increases local incidence; outboard decrease is
  washout. Section-angle values are inputs, not fitted by the geometry code.

No reference aircraft or section numbers were copied. The user's T12 request
authorizes synthetic fixtures and a cited theoretical section-slope helper.

## Geometry contract and formulas

soas-estimator depends on soas-core for checked Radians. There is no dependency
from core back to estimator. Runtime code is pure, with no files or global state.

Provide at least root and tip stations on the RIGHT half: y starts at zero and
strictly increases. Chord and section lift slope must be finite and positive;
x/z offsets can have either sign. Angles use checked Radians. Data notes and
section provenance are required nonempty text; text does not prove provenance.
Mirror by replacing y with -y and retaining x/z, chord, twist and section data.
All station fields interpolate linearly in absolute y; exact nodes retain the
input values. Sampling beyond either tip errors rather than extrapolating.

Use existing body axes: x forward, y right, z down. Leading-edge coordinates
share a model-defined datum; trailing edge is x_LE-c and quarter chord is
x_LE-c/4. Positive geometric twist means increased section angle of attack.
Reference planform quantities use the untwisted chord distribution and projected
span in y; z offsets/twist do not stretch area or redefine reference span.

For semispan h, the sourced integrals are:

```text
b=2h; S=2*integral_0^h c(y)dy; AR=b^2/S; taper=c_tip/c_root
MAC=(2/S)*integral_0^h c(y)^2 dy
MAC_LE_right=(2/S)*integral_0^h c(y)*(x_LE(y),y,z(y))dy
MAC_quarter_right=MAC_LE_right-(MAC/4,0,0)
sweep_quarter=atan2(x_quarter_root-x_quarter_tip,h)  [positive aft]
```

The x first moment follows Caughey eq. 2.12; the y/z locations use the same
area-centroid definition. For a broken-edge multi-panel wing this is the
equivalent reference MAC and area-weighted position; the actual local chord
at that y need not equal MAC. The left reference location mirrors y. The single
reported sweep is explicitly root-to-tip, not a guessed average of panel sweeps.
It is a geometric descriptor, not a swept-wing aerodynamic correction.

For a panel of width d with endpoint chords c0,c1 and a linearly varying
coordinate f0,f1, exact integration of the linear polynomials gives:

```text
full-wing panel area = d*(c0+c1)
2*integral c^2 dy = (2d/3)*(c0^2+c0*c1+c1^2)
2*integral c*f dy = (d/3)*(2*c0*f0+c0*f1+c1*f0+2*c1*f1)
```

The first two are also the NDARC panel expressions. The product integral is
the direct polynomial evaluation of the above first-moment definition. Sum
panels before dividing by S. Thus inserting a station on an existing linear
panel cannot change derived geometry. Numeric overflow/underflow producing an
invalid derived quantity is an explicit error, not a usable geometry result.

## Independent hand cases (ALL inputs synthetic)

`crates/soas-estimator/tests/fixtures/planforms.json` contains no real aircraft.
Every numeric path has an entry in planforms_values.md. All new Rust test
literals/expected values and numerical settings are listed separately in
planform_test_values.md. The section inputs in these fixtures are deliberately
synthetic, not estimates for a real airfoil.

| Quantity | Rectangular fixture | Tapered fixture |
|---|---:|---:|
| Semispan m | 3.5 | 5 |
| Root/tip chord m | 1.25 / 1.25 | 3 / 1 |
| Span m | 7 | 10 |
| Area m^2 | 8.75 | 20 |
| Aspect ratio | 5.6 | 5 |
| Taper | 1 | 1/3 |
| MAC m | 1.25 | 13/6 |
| Right MAC leading-edge x,y,z m | (.8,1.75,-.4) | (-7/30,25/12,-37/60) |
| Right MAC quarter-x m | .4875 | -31/40 |
| Root-to-tip quarter sweep rad | 0 | atan(.3)=.2914567944778671 |

The standard trapezoid MAC is `(2/3)*cr*(1+lambda+lambda^2)/(1+lambda)`;
with lambda=1/3 it gives 13/6 independently of panel integration. Its spanwise
centroid is `(b/6)*(1+2*lambda)/(1+lambda)=25/12`. Applying the straight edge
functions x_LE=.6-.4*y and z=-.2-.2*y gives the positions above. Root/tip
quarter-x are -.15 and -1.65, so aft sweep has tangent 1.5/5=.3.

Six tests cover rectangular geometry, tapered geometry, every interpolated
station field and mirroring, linear-panel subdivision invariance, invalid
geometry/provenance and overflow, and the independently cited 2*pi helper.
Distinct span/chord values expose exchanged dimensions. Hand geometry compares
within 1e-12 SI/dimensionless/angle units as applicable; this is a numerical
test tolerance, not D9 or real-wing validation. Blank owner learning columns
are retained.

Before implementation, the tests-only crate harness failed with:

```text
error[E0432]: unresolved import `soas_estimator::planform`
could not find `planform` in `soas_estimator`
```

No T10/T11 work, OPEN decision selection, real-aircraft data or non-approved
dependencies are part of T12. The fixture loader is test-only, not a new wing
JSON file-format promise.

Final T12 gates: fmt check and Clippy with warnings denied pass; all 143
workspace tests pass, with 0 failures/ignored tests. Local checks retain the
previously disclosed CARGO_INCREMENTAL=0 setting for this filesystem.

Files: root Cargo.toml/Cargo.lock; new soas-estimator manifest, src/lib.rs,
src/planform.rs, tests/planform.rs, tests/support/mod.rs, three fixture/ledger
files; docs/TASKS.md, docs/ASSUMPTIONS.md, docs/LEARNING.md; this reference note.
