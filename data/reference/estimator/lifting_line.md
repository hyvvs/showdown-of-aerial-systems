# T13 Prandtl/Glauert Fourier lifting line

## Sources and formulas (verified online 2026-11-14)

- [MIT Unified Fluids Lecture 8, pp. 1-3, equations 1-4](https://web.mit.edu/16.unified/www/SPRING/fluids/Spring2008/LectureNotes/f08.pdf):
  circulation/section loading, linear section incidence, geometric versus
  aerodynamic twist, and analytic elliptic chord distribution.
- [MIT Unified Fluids Lecture 9, pp. 1-4](https://web.mit.edu/16.unified/www/SPRING/fluids/Spring2008/LectureNotes/f09.pdf):
  Fourier circulation and induced angle, total lift/induced drag and span
  efficiency; finite-wing lift-slope relation. This is the incompressible
  Prandtl lifting-line equation solved with a Glauert Fourier expansion.
- [Caughey, Aerodynamic Background, eqs. 2.15-2.19](https://courses.cit.cornell.edu/mae5070/AeroBackground.pdf#page=5):
  the explicit theoretical section input 2*pi/rad and the elliptic finite-wing
  slope. The discussion identifies a large-aspect-ratio approximation without
  selecting a numeric physical validity cutoff for this implementation.
- [Netlib, Templates for the Solution of Linear Systems, stopping Criterion 1](https://netlib.org/templates/templates.html):
  normwise residual scaling by norm(M)*norm(A)+norm(rhs). This numerical check
  does not establish convergence of lift/drag or physical validity.

The formula-level geometry references and body-axis definitions are in
geometry.md. No real-wing dataset was imported or estimated. All benchmark
conditions are synthetic and ledgered; pi/theoretical 2*pi are cited mathematical
inputs, not invented airfoil measurements.

## Numerical contract and derivation

For full span b and y=(b/2)*cos(theta), use the sourced expansion/section laws:

```text
Gamma(theta)/V = 2*b * sum A_n*sin(n*theta)
alpha_i(theta) = sum n*A_n*sin(n*theta)/sin(theta)
cl = 2*Gamma/(V*c) = a0*(alpha + twist - alpha_L0 - alpha_i)
```

Substituting and collecting A_n gives the system implemented directly:

```text
sum A_n*sin(n*theta) * [4*b/(a0*c) + n/sin(theta)]
    = alpha + twist - alpha_L0
CL = pi*AR*A_1
CDi = pi*AR*sum(n*A_n^2)
e = A_1^2/sum(n*A_n^2)
```

Symmetry eliminates even harmonics because sin(n*(pi-theta)) equals
(-1)^(n+1)*sin(n*theta). The term-count parameter N means N odd harmonics,
n=1,3,...,2N-1. The chosen collocation grid is theta_i=i*pi/(2N), i=1..N,
covering right tip interior through the center. This grid is an implementation
choice, checked by the independent analytic/convergence tests, not a claim that
the lecture specifies this particular discretization. It excludes the singular
tips. Section chord/twist/a0/alpha_L0 are evaluated at each collocation y.

The dense system uses the pre-approved nalgebra LU solve. Invalid counts,
size/allocation failure, singular systems and non-finite arithmetic are errors.
The returned backward residual is
`norm_inf(M*A-rhs)/(norm_inf(M)*norm_inf(A)+norm_inf(rhs))` (zero if the exact
zero system has zero residual). Acceptance uses 128*machine_epsilon*N as a
numerical roundoff guard, not an aerodynamic validity threshold or a guarantee
about an ill-conditioned system's forward error. There is no fitted high-order
coefficient, physical cutoff, sweep correction or compressibility correction.

Two geometry inputs use the SAME matrix solve:

- T12 station wings: require a straight planar quarter-chord line; x_LE may
  vary with chord to keep x_LE-c/4 constant. Twist/section inputs may vary.
- Analytic ellipse: c(y)=c0*sqrt(1-(2y/b)^2), S=pi*b*c0/4 from integrating the
  sourced chord distribution with the area definition. Uniform section data
  and twist are explicit inputs. This avoids approximating a zero-chord elliptic
  tip with T12's strictly positive station chords; no special analytic lift or
  drag answer is inserted into the solver.

Station straightness allows only 64 machine epsilons times the geometric scale
for coordinate roundoff. It is not an invented acceptable sweep/dihedral angle;
finite swept/nonplanar examples are rejected explicitly.

Section output reports local cl, Gamma/V in metres, and L'/q_dyn=2*Gamma/V=c*cl
in metres. Multiply by supplied airspeed or dynamic pressure for dimensional
circulation/loading. No density/airspeed is silently chosen. Returned sections
are mirrored interior collocation points in ascending y, with the center once;
tips are excluded. Induced angle uses checked Radians. With identically zero
circulation, CL=CDi=0 and span efficiency is None (undefined 0/0), not guessed 1.

## Assumptions and validity

Every output includes straight symmetric wing, incompressible flow, attached
flow, linear section lift, small-angle and high-aspect-ratio approximations,
no sweep correction, and no viscous/profile drag. It always states
**validity unassessed**. There are no invented limits in Mach, angle, Reynolds
number or aspect ratio. For example, a finite large-angle test still carries
that warning; numerical solvability cannot prove attached flow. Finite-wing
span efficiency here concerns induced drag, not a fitted viscous Oswald factor.
No stalls, fuselage/tail interference, thickness effects, unsteady effects or
simulation integration are supplied. No T10/T11/reference-aircraft validation
was started. Estimator depends on core; core remains independent of estimator.

## Independent elliptic hand answers

All dimensions/angles below are SYNTHETIC benchmark conditions. The supplied
section law uses the cited theoretical a0=2*pi/rad. For untwisted uniform
elliptic wings, only A1 is nonzero, e=1; solving the section relation gives
`a=a0/(1+a0/(pi*AR))`. At alpha=.05=1/20 rad and alpha_L0=0:

| AR | Span m | Derived root chord m | Slope per rad | CL | CDi | Induced angle rad |
|---:|---:|---:|---:|---:|---:|---:|
| 4 | 8 | 8/pi | 4*pi/3 | pi/15 | pi/900 | 1/60 |
| 10 | 15 | 6/pi | 5*pi/3 | pi/12 | pi/1440 | 1/120 |

Both tests exercise N=1,4,12,24; check every local cl/induced angle and analytic
elliptic circulation shape, coefficient mode numbers and higher-mode zeros.
Their expectations are hand rational multiples of pi, not another Fourier
implementation. CL tolerance is 2e-12 absolute, slope 2e-11/rad, CDi and e
2e-13; loading/induced-angle checks are stated in the test/ledger. These are
software analytic checks, not physical accuracy or D9 tolerances.

Further tests check zero lift at the common section alpha_L0 for untwisted
ellipse/rectangle; constant twist and alpha_L0 signs using analytic CL; mirrored
section loading and sign-reversed lift with unchanged induced drag; returned
assumptions/warning; invalid geometry, provenance, term count and nonstraight
station wings. All eight tests can fail independently of production formulas.

## Rectangular term-count convergence (observed)

Synthetic rectangle: span=12 m, chord=2 m, AR=6, alpha=.08 rad,
alpha_L0=-.02 rad, twist=0, a0=2*pi/rad. Reference is N=128 odd terms, not a
published physical measurement. Differences are absolute against that reference.

| N odd terms | CL | CDi | e | CL difference | CDi difference |
|---:|---:|---:|---:|---:|---:|
| 2 | .447695635587 | .010887905288 | .976607969147 | 5.346862949544e-3 | 5.266261095591e-4 |
| 4 | .452728040191 | .011377841173 | .955682861022 | 3.144583455119e-4 | 3.669022511923e-5 |
| 8 | .453024442949 | .011412458575 | .954031979843 | 1.805558816831e-5 | 2.072822814491e-6 |
| 16 | .453041410777 | .011414408850 | .953940427985 | 1.087759992913e-6 | 1.225477681988e-7 |
| 32 | .453042431665 | .011414523925 | .953935110092 | 6.687159337382e-8 | 7.472928538130e-9 |
| 64 | .453042494619 | .011414530961 | .953934787162 | 3.917535129627e-9 | 4.365448991878e-10 |
| 128 | .453042498537 | .011414531398 | .953934767177 | reference | reference |

CL differences decrease by factors 17.0034,17.4161,16.5989,16.2664,17.0698
as N doubles; CDi factors are 14.3533,17.7006,16.9144,16.3989,17.1184.
This is approximately fourth-order term-count convergence for this case/grid,
not universal spectral convergence or a proof that N=128 is exact. At N=64,
relative CL/CDi differences versus N=128 are below the prewritten 2e-6 test
bounds, and e is below 1. Other planforms/twist/section inputs need their own
term-refinement evidence. This static estimator does not change D10's time step.

## Tests first and deliberate wrong-equation check

The tests-only commit initially failed to compile because
`soas_estimator::lifting_line` did not exist. After implementation, all eight
tests pass. In a SCRATCH COPY ONLY, set the matrix's section term
`4*b/(a0*c)` to zero, leaving the induced-angle term and numerical solve intact.
Both elliptic tests then fail on their first independent CL comparison:

```text
AR case 0, N=1: 0.6283185307179586 != 0.20943951023931953
AR case 1, N=1: 1.570796326794897 != 0.2617993877991494
test elliptic_ar_four_matches_independent_hand_slope_lift_drag_and_loading ... FAILED
test elliptic_ar_ten_matches_independent_hand_slope_lift_drag_and_loading ... FAILED
test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 6 filtered out
```

Thus a numerically solvable but physically wrong system cannot pass these
analytic tests. The wrong variant and scratch build were removed after checking
the exact resolved scratch path; neither was committed. No test was weakened.

Fixture numbers are labeled synthetic in lifting_line_cases.json, with a
per-path lifting_line_cases_values.md ledger. New Rust test inputs, rational
expectations and numerical settings are in lifting_line_test_values.md.
Only pre-approved dependencies are used: soas-core plus nalgebra at runtime,
serde_json for tests. All external packages were already in the workspace.

Final T13 gates: fmt check and Clippy with warnings denied pass; all 151
workspace tests pass, with 0 failures/ignored tests. The six T12 tests remain
unchanged and passing. The known filesystem setting CARGO_INCREMENTAL=0 is
retained; no new build workaround was needed. Dependency metadata confirms
soas-estimator -> soas-core and no reverse edge. Ledger audit covers all 28
T12 and all 12 T13 numeric JSON paths. The scratch fault directory is absent.

Files for T13: estimator Cargo.toml/root Cargo.lock, src/lib.rs and
src/lifting_line.rs; tests/lifting_line.rs and its three fixture/ledger files;
docs/TASKS.md, docs/ASSUMPTIONS.md, docs/LEARNING.md; this reference note.
