# T15b Part 1: elliptic relative slope and exact Munk screen

Branch task/t15b starts from main at 4c85025. All inputs/counts are labeled
SYNTHETIC TEST VALUES, not aircraft data; per-path and per-literal ledgers are
in tests/fixtures/vlm_trend_values.md and vlm_acceptance_test_values.md.
No solver source, model input or existing tolerance changes.

## Exact assertions

At AR6 and AR40, Nx1 and Nx8, use the existing area-normalized elliptic
polygon and cosine span spacing. Full-span Ny128/256/512/1024, root reference
chord1 m, alpha=.05 rad; zero twist, alpha_L0 and offsets except the straight
quarter-chord leading-edge definition. The positive EPSILON endpoint remains
a numerical T12-contract regularization, not physical tip data.

- At finest Ny1024, abs(a_VLM/a_LLT-1) at AR40 is STRICTLY smaller than at
  AR6 for each chord count, with a_LLT=a0/(1+a0/(pi*AR)), a0=2*pi.
- Each Richardson estimate from 128/256/512 and 256/512/1024 is <=1,
  with NO tolerance. The helper checks immediately after each triple and
  would stop on a violation. Raw finite-grid efficiencies may exceed one;
  this assertion concerns the specified extrapolated estimates only.

Reference conditions are reused exactly; there is no selected mesh interval,
relaxed bound or substituted saved result. Fresh production VLM solves supply
all coefficients. Immutable per-case OnceLock caches share those expensive
solves between the two tests without performing I/O or skipping computation.

## Sources and limits

- [Munk, NACA Report 121, The Minimum Induced Drag of Aerofoils](https://ntrs.nasa.gov/citations/19930091456):
  original report identity verified in the NASA catalog. The full scan could
  not be fetched here (timeout/size limit), so its exact pages were not checked.
- [Mineck and Vijgen, NASA TP-3359, Introduction, PDF p.5](https://ntrs.nasa.gov/api/citations/19940012101/downloads/19940012101.pdf#page=5):
  verified primary-source discussion cites Munk's minimum at constant downwash
  and elliptic circulation for a planar, rigid wake. This is the theoretical
  scope used here; nonplanar systems/relaxed wakes are not covered by the test.
- [MIT Lecture 9, pp.2–3](https://web.mit.edu/16.unified/www/SPRING/fluids/Spring2008/LectureNotes/f09.pdf):
  CL=pi*AR*A1, CDi=pi*AR*sum(n*A_n^2), e=A1^2/sum(n*A_n^2). Nonnegative
  higher-mode contributions give CDi>=CL^2/(pi*AR), hence e<=1. This is a
  derivation of the mathematical bound, not a claim that every finite-mesh
  estimate is automatically within it.
- Elliptic LLT slope and geometry are sourced in lifting_line.md/geometry.md
  (Caughey, MIT Lectures 8/9). Richardson formula and its leading-error-law
  assumptions remain in vlm_acceptance_review.md and the cited NASA grid guide.

Applying the exact physical inequality to these particular extrapolations is
an owner-specified numerical regression screen. Passing does not certify the
extrapolation error, real-wing accuracy or a physical infinite-AR limit.

## Production measurements

| AR | Nx | Relative finest slope gap | Percent |
| --- | --- | --- | --- |
| 6 | 1 | 0.071093590009689 | 7.109359001% |
| 40 | 1 | 0.003868228346626 | 0.386822835% |
| 6 | 8 | 0.067353665326292 | 6.735366533% |
| 40 | 8 | 0.003717310223426 | 0.371731022% |

| AR | Nx | Full-span triple | Extrapolated e | Observed p |
| --- | --- | --- | --- | --- |
| 6 | 1 | 128/256/512 | 0.998776508925988 | 1.003751905734 |
| 6 | 1 | 256/512/1024 | 0.998769844233513 | 1.001835495391 |
| 40 | 1 | 128/256/512 | 0.999928939111532 | 1.009131679975 |
| 40 | 1 | 256/512/1024 | 0.999915648754146 | 1.005138206061 |
| 6 | 8 | 128/256/512 | 0.997986378186488 | 1.004097443920 |
| 6 | 8 | 256/512/1024 | 0.997976872496307 | 1.001369216818 |
| 40 | 8 | 128/256/512 | 0.999895578916286 | 1.008672565236 |
| 40 | 8 | 256/512/1024 | 0.999886578071417 | 1.005964601971 |

All eight estimates are below one, without an allowance. Production was
already correct: the tests-first snapshot passes immediately once compiled.
An initial test-code type-inference error was corrected with an explicit
usize annotation before committing the tests; no numerical condition changed.
The first run used `cargo test --release -p soas-estimator --test vlm_acceptance
elliptic_ -- --nocapture`: 4 passed, 0 failed, 2 filtered out. It includes
two existing ellipse tests plus the two additions.

## Scratch mutation evidence (never committed)

Wrong-AR mutation: in the scratch test ONLY, use AR6 in the LLT reference for
the AR40 comparison as well. Production geometry/loads remain unchanged. The
relative-error assertion fails on Nx1:

```text
AR6 gap=0.07109359000968929; AR40 gap=0.264929233845554
AR40 relative slope gap is not smaller
test result: FAILED. 0 passed; 1 failed; 5 filtered out
```

Drag mutation: in a separate restored scratch copy, report 0.95*CDi from the
production VLM output. This owner-prescribed SYNTHETIC FAULT immediately gives:

```text
STOP: Munk estimate exceeds 1: AR=6 Nx=1 last Ny=512: 1.051343693606303
test result: FAILED. 0 passed; 1 failed; 5 filtered out
```

Intentional scratch violations are the requested regression demonstrations;
they are not production results. Both copies are confined to ignored target/.
Their only added fault values are AR6 (an existing test AR) and drag scale
0.95, explicitly ledgered. No tolerance was added or loosened.

## Build profile and numerical ledgers

The owner approved `[profile.test.package.soas-estimator] opt-level=3` after
the largest case was identified as 8192 panels. It optimizes test compilation
only; the normal fmt/Clippy/cargo test --all gate commands remain unchanged.
No solver algorithm, dependency or release-profile setting is altered. Test
memoization became lazy per case after the first tests commit, allowing a
scratch failure to stop without solving unrelated large cases; formulas and
assertions are unchanged. TEMP/TMP is checkout/target/t15b/temp and the
previously disclosed CARGO_INCREMENTAL=0 remains explicit.

Every new fixture path and test literal is inventoried. Meshes and geometry
are synthetic; mathematical factors (pi, powers, Munk bound1) are sourced;
compiler level3 is owner-approved. The copied verifier and later experiment
records are outside build/CI and add no project dependency.

Final normal quality gates pass: formatting, Clippy with warnings denied, and
177 workspace tests (0 failed, 0 ignored). No solver-source or dependency
change was needed.
