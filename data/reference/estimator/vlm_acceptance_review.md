# T15 robust acceptance review

All benchmark inputs, counts and numerical test settings are labeled
**SYNTHETIC TEST VALUES**, with per-path/per-literal ledgers in
tests/fixtures/vlm_trend_values.md and vlm_acceptance_test_values.md.
No production code, dependency or existing tolerance changed in this review.

## Removed inference and retained evidence

The zero-compatible extrapolation pass/fail criterion is removed, together
with its fit helper and the otherwise unused nonzero-floor control test and
fixture settings. History remains in 88dfc0e/b66b434. All requested subsets
and the full fit, including intercepts, band components and differing verdicts,
are retained in vlm_subset_fits.md as information only. In particular, the
drag subset AR6/10/20 is NOT zero-compatible under the old rule, while all four
are. The old basis/sensitivity construction cannot justify a robust assertion.

The original AR40 64x8 failing comparison is unchanged in
vlm_expected_difference.md and 8067b1d; neither its inputs nor its former
allowances were edited. Mesh measurements are in vlm_gap_vs_mesh.md, committed
before changing acceptance. Most of its drag gap persists with refinement.

## Exact current assertions

1. Rectangles at AR6,10,20,40: absolute relative CL and CDi gaps against T13
   strictly decrease at EVERY adjacent AR pair, independently on **64x8 and
   128x8** cosine-span/uniform-chord meshes, alpha=.05 rad. T13 uses 128 odd
   terms. No zero intercept, power-law continuation or efficiency-gap trend is
   asserted. The two sets of measured gaps are printed by the test.
2. Elliptic polygons at AR6 and AR40, Ny=32,64,128,256,512: excess efficiency
   max(e-1,0) must not increase; successive absolute VLM efficiency changes
   and successive absolute lift-slope changes must strictly decrease.
3. The finest-three-mesh Richardson efficiency estimate must lie inside an
   empirical prediction band derived from the FIRST FOUR meshes, without
   using the held-out finest result to select its width. Derivation below.
4. Existing uniform/cosine hand-coordinate/default checks remain unchanged.

The owner's initial request to assert decreasing lift-slope error versus LLT
contradicted the no-assertion Nx8 probe: at AR6, absolute error rose from
0.253622488807 to 0.315173010845; at AR40 from 0.006640783021 to
0.021675893522 over Ny32→512. The owner explicitly approved **decreasing
successive mesh changes instead, with LLT differences informational**. No
different interval or widened tolerance was selected to manufacture the
original assertion. The complete probe and its numerical ledger are in
vlm_subset_fits.md.

## Elliptic geometry and test scope

The acceptance ellipse uses one uniform chordwise panel so the five-level
tests remain practical in ordinary debug quality gates. This is distinct from
the Nx8 diagnostic, not a claim of chordwise convergence. Both are documented.

Analytic reference chord c(y)=c0*sqrt(1-(2y/b)^2), b=AR*pi*c0/4, c0=1 m.
Cosine-edge right-half stations are mirrored by T12. A positive endpoint
c0*binary64 EPSILON satisfies T12's positive-chord contract; it is a numerical
test regularization, not a physical tip model. The piecewise-linear polygon's
chords are scaled uniformly by (b^2/AR)/polygon_area, maintaining identical span,
area and AR on every mesh. Leading edge x=c/4 keeps quarter chord x=0. Twist,
zero-lift angle, z and reference are zero; a0=2*pi/rad. These choices and every
literal are ledgered. Reference formulas are in estimator geometry.md,
lifting_line.md and vortex/VLM notes (MIT Lectures 8/9, Caughey, JVL).
[Rust f64::EPSILON](https://doc.rust-lang.org/std/primitive.f64.html#associatedconstant.EPSILON)
defines the machine arithmetic value used at the endpoint.

## Richardson prediction band — derived, not fitted to pass

[NASA's grid-convergence tutorial](https://www.grc.nasa.gov/www/wind/valid/tutorial/spatconv.html)
describes the leading-error model f(h)=f0+C*h^p and three-grid Richardson
extrapolation. We apply it conditionally to doubled span grids, not as a proof
of a physical error bound. For each consecutive triple:

```text
d0 = e_2N - e_N; d1 = e_4N - e_2N
p = log2(d0/d1)           [same-sign, contracting changes required]
R = e_4N + d1/(2^p-1)
```

Let R0 use Ny32/64/128, R1 use 64/128/256 and R2 use 128/256/512.
Before consulting the held-out Ny512 result, define:

```text
center = R1
half_width = abs(R1-R0)
prediction_band = [center-half_width, center+half_width]
assert abs(R2-center) <= half_width
```

The ONLY assumption behind this empirical band is continuing contraction of
successive extrapolated estimates. There is no tuned multiplier, required
physical target e=1 or post-fine-grid enlargement. This can fail on reversing,
noncontracting changes or on a finest estimate outside the coarse prediction.
It is a self-consistency test, not a guaranteed continuum-error interval or
statistical confidence band. These finite data do not support either such
guarantee; no band asserting physical equality to LLT e=1 is claimed.

Measured Nx1 acceptance results:

| AR | R0 | R1 | R2 (finest three) | Band from first four |
|---|---|---|---|---|
| 6 | 0.9988950560535235 | 0.9988028358612693 | 0.9987765089259878 | [0.9987106156690151, 0.9988950560535235] |
| 40 | 0.9998419560683480 | 0.9999488899171740 | 0.9999289391115320 | [0.9998419560683480, 1.0000558237660000] |

At AR6 the band is below one; it is deliberately NOT shifted to encompass
one. At AR40 it happens to straddle one. Neither establishes real-wing
validity or exact equality between lifting surface and lifting line.

## Tests-first and scratch evidence

Production already met the new numerical properties. All four tests pass
immediately before any implementation change:

```text
test result: ok. 4 passed; 0 failed; 0 ignored
```

In an isolated scratch copy, multiplying reported CDi by AR produces gaps
4.04397, 7.81160, 17.20587, 36.00101 on the coarse mesh; the strict decreasing
trend test fails (0 passed, 1 failed). Restoring production in the scratch copy
and setting only reported lift slope to the labeled SYNTHETIC FAULT value zero
makes both ellipse tests fail on constant, nondecreasing mesh changes:

```text
mesh changes fail to decrease: [0.0, 0.0, 0.0, 0.0]
test result: FAILED. 0 passed; 2 failed
```

These mutations are never applied to production or committed. The AR multiplier
and zero are declared diagnostic faults, not fixture calibration parameters.
Only tests, ledgers and documentation change. TEMP/TMP is set to
checkout/target/batch-temp for all test runs; existing CARGO_INCREMENTAL=0 is
explicitly retained. No AVL run or T10/T11 work is performed.

Final gates: formatting and Clippy with warnings denied pass. Aggregate
workspace result: 175 passed, 0 failed, 0 ignored.
