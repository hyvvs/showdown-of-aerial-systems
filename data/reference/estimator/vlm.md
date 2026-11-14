# T15 planar vortex lattice — incomplete working-tree implementation

T15 is complete under the owner's 2026-11-14 acceptance redesign. The initial
tests commit is 8067b1d, the docs-only diagnosis is d5d0d01, and continuation
tests were committed before spacing implementation as 88dfc0e. The original
failure is preserved in history and in vlm_expected_difference.md; its inputs
and allowances are unchanged. No new dependency, T10/T11 work or physical
validity threshold is introduced.

## Current acceptance after review

The zero-compatible criterion below is HISTORICAL and no longer asserted.
Current two-mesh AR trends, owner-clarified elliptic checks and coarse-derived
Richardson prediction bands are specified in vlm_acceptance_review.md.
Informational subsets: vlm_subset_fits.md; mesh data: vlm_gap_vs_mesh.md.
The original expected-difference record is unchanged. No production correction
or tolerance widening was needed. Final review gates: 175 tests pass.

## Sources and verification limits

- [Drela, JVL 2.20 Formulation](https://web.mit.edu/drela/Public/web/jvl/jvl.pdf),
  sections 3.1, 4.2, 5 and 6: quarter-chord horseshoes, three-quarter-chord
  controls, Biot-Savart velocities, flow tangency, Kutta-Joukowski forces and
  Trefftz-plane wake/energy drag (including equations 123–127). Checked during
  this batch; the implementation is the simpler planar, linearized subset,
  not JVL's jet model or all of its force/geometry features.
- [Mason, Computational Aerodynamics, chapter 6](https://archive.aoe.vt.edu/mason/Mason_f/CAtxtChap6.pdf),
  section 6.7: classical vortex-lattice placement, flat wake and linear
  influence-coefficient system. The cosine span/uniform chord mesh is an
  explicit numerical choice, not a sourced physical parameter.
- [Drela, AVL user primer, CLAF discussion](https://web.mit.edu/drela/Public/web/avl/avl_doc.txt):
  ordinary quarter/three-quarter placement implies ideal thin-airfoil 2*pi
  section slope; arbitrary slope correction moves the control location.
  Because this task fixes three-quarter controls, other slopes are rejected
  explicitly. The T12 slope input is not silently ignored or recalibrated.
- [Caughey, aerodynamic background](https://courses.cit.cornell.edu/mae5070/AeroBackground.pdf),
  equations 2.15, 2.19 and 2.21: thin-airfoil slope and approximate finite-AR
  slope comparisons. Sections on aerodynamic center/neutral point explain
  independence of pitching moment from alpha at that point. These approximate
  slope relations do **not** establish a rectangular-wing inter-method error
  bound, particularly for span efficiency/induced drag.
- [NASA Glenn, aerodynamic center](https://www.grc.nasa.gov/www/k-12/VirtualAero/BottleRocket/airplane/ac.html):
  quarter-chord thin-airfoil aerodynamic center. The high-AR rectangular test's
  prewritten 1%-chord comparison is a synthetic numerical check, not a universal
  physical tolerance or a validity threshold for swept/low-AR wings.
- [Netlib Templates, stopping criteria](https://netlib.org/templates/templates.html):
  normwise backward residual, as already used in T13. The 128*epsilon*N guard
  is a numerical roundoff policy, not a physical-accuracy claim.
- Elementary segment/ray equations and source-extraction limitations remain in
  [vortex.md](vortex.md), including NASA CR-3208 Appendix A. Some extracted PDF
  equation characters are garbled; independent scalar hand cases check T14's
  integral evaluation. The continuation diagnosis cites Tuck's independent lifting-surface results; no external VLM code or real-wing dataset was imported. Further sources and conditional trend derivation are in vlm_diagnosis.md.

## Equations, signs and numerical scope

Body axes are x forward, y right, z down; linearized free airflow is
(-1,0,-alpha) times airspeed. A positive bound vortex points left-to-right;
its wake extends downstream along -x. Geometry is planar. Full-span cosine
edges default to y_j=-(b/2)*cos(pi*j/Ny); the explicit uniform option is
y_j=-b/2+b*j/Ny. SpanwiseSpacing::default() is Cosine, as justified by the
diagnosis. Chord panels are uniform fractions of each
strip. A panel's bound is at (k+1/4)/Nx of local chord and its control at
(k+3/4)/Nx. Controls use the midpoint of the spanwise panel. The existing T12
piecewise-linear station interpolation supplies geometry and local incidence.

Let g=Gamma/V in metres, and W_ij be the T14 induced z velocity per unit
circulation at control i from horseshoe j, in 1/m. Linearized flow tangency is

```text
W*g = alpha + twist - alpha_L0
W*(dg/dalpha) = 1
```

The second solve gives exact derivatives of this linear, fixed-wake model;
there is no finite-difference step. A dense LU solve is checked with
||W*g-rhs||inf / (||W||inf*||g||inf + ||rhs||inf).
Degenerate, non-finite, nonplanar, unsupported-slope or singular cases error.

For bound width dy, Kutta-Joukowski gives Fz/(rho*V^2)=-g*dy. With panel bound
midpoint (x,y), area S, MAC c and reference (xref,0,z_wing):

```text
CL = 2*sum(g*dy)/S
Cm = 2*sum((x-xref)*g*dy)/(S*c)
Cl_roll = -2*sum(y*g*dy)/(S*b)
x_neutral = xref + c*(dCm/dalpha)/(dCL/dalpha)
```

The PLUS sign follows body x forward: shifting the reference forward subtracts
CL times the reference shift divided by MAC from Cm. Section intrinsic Cm is
not supplied by T12 and is assumed zero. Reference y=0, z=wing plane is explicit.

For strip total g_i (sum chordwise horseshoes), Trefftz-plane midpoint drag is

```text
w_far(y_i)/V = sum_j g_j/(2*pi) * [1/(y_i-y_left_j)-1/(y_i-y_right_j)]
CDi = sum_i g_i*(w_far(y_i)/V)*dy_i/S
```

The far-wake downwash is twice the wing-plane wake value; the energy relation
supplies the half factor before dividing by dynamic pressure. This is
strip-midpoint quadrature, not an exact continuous wake integral. Coarse-panel
results need refinement; the one-panel hand case is a discrete-system check,
not physical span-efficiency validation.

Assumptions listed by every output: planar wing, incompressible flow, attached
flow, small angles, ideal thin-airfoil section slope, flat linearized wake, no
thickness/profile drag and zero intrinsic section pitching moment. No stall,
compressible, unsteady, fuselage or tail effect is added. Every result says
**validity unassessed**, including low-aspect-ratio swept hero-wing geometry.

## Independent tests and number provenance

`tests/fixtures/vlm_cases.json` contains only labeled SYNTHETIC TEST VALUES:
one-panel b=4 m, c=2 m; rectangular b=8 m, c=1 m; high-AR b=40 m, c=1 m.
`vlm_cases_values.md` records every numeric fixture path;
`vlm_test_values.md` records every numerical test setting/literal, including
mesh counts, alpha, error bounds, reference location and arithmetic factors.
No value represents an aircraft measurement.

For the one-panel case alpha=0.1, a=(-0.5,-2,0), b=(-0.5,2,0), control=(-1.5,0,0).
The scalar T14 integral gives W=(1+sqrt(5))/(4*pi), hence
g=0.4*pi/(1+sqrt(5)), CL=g, CDi=g^2/(4*pi), Cm=-g/4 and x_neutral=-0.5 m.
This independently checks geometry, influence, solve, force scaling, moment
reference and derivative. Other tests check mirrored circulation/zero roll;
separate span/chord refinement; quarter-MAC neutral point, reference shift and
finite-difference moment derivative; twist/alpha_L0 cancellation; explicit
assumptions on a planar swept input; invalid counts, reference, geometry/slope.
Seven original tests pass. The old eighth comparison is retained as the historical
expected-difference record below, not current acceptance. Three continuation
tests add decreasing gaps/conditional zero limit, rejection of an analytic
nonzero-limit fault and hand-calculated uniform/cosine spacing with cosine default.

## Measured rectangular refinement (synthetic AR=8, alpha=0.05 rad)

Span refinement holds Nx=4. Ny counts the FULL wing.

| Ny | CL | CDi | Cm at xref=0 | Neutral x, m |
|---|---|---|---|---|
| 8 | 0.242033816727 | 0.002122810887 | -0.059185957552 | -0.244535901439 |
| 16 | 0.236214657068 | 0.002146255425 | -0.057543452799 | -0.243606614059 |
| 32 | 0.232857786680 | 0.002150993898 | -0.056604983570 | -0.243088214386 |
| 64 | 0.231071085674 | 0.002151348121 | -0.056110328779 | -0.242827130948 |
| 128 reference | 0.230152242838 | 0.002151009386 | — | -0.242696026806 |

CL errors against Ny=128 decrease by factors 1.960, 2.241, 2.944 (apparent
orders 0.971, 1.164, 1.558). The finite reference biases the last estimate;
this is roughly first-order spanwise behavior, not a demonstrated universal
order. CDi errors are 2.81985e-5, 4.75396e-6, 1.54882e-8, 3.38735e-7:
nonmonotonic cancellation means no clean drag convergence order is claimed.
The existing test requires the last CL/CDi errors to improve by more than a
factor of four over the first; it does not hide that intermediate behavior.

Chord refinement holds Ny=32.

| Nx | CL | CDi | Cm at xref=0 | Neutral x, m |
|---|---|---|---|---|
| 1 | 0.231785554721 | 0.002126633581 | -0.057946388680 | -0.250000000000 |
| 2 | 0.232648568682 | 0.002146103499 | -0.056893579801 | -0.244547302069 |
| 4 | 0.232857786680 | 0.002150993898 | -0.056604983570 | -0.243088214386 |
| 8 | 0.232905080669 | 0.002152127221 | -0.056529233004 | -0.242713610376 |
| 16 reference | 0.232916091034 | 0.002152395567 | — | -0.242620105506 |

CL error ratios are 4.226, 4.588, 5.295; CDi ratios 4.094, 4.489, 5.223.
Apparent orders against this finite reference are 2.079–2.405 and 2.034–2.385:
approximately second-order chordwise behavior in this synthetic case.

## Blocking high-AR comparison — allowance NOT widened

Rule committed before implementation in `8067b1d`:

```text
a_line = a0/(1+a0/(pi*AR))
a_surface = pi*AR/(1+sqrt(1+(pi*AR/a0)^2))
model_CL = abs(a_line-a_surface)*alpha
mesh_CL = abs(VLM_64x8-VLM_32x8) + abs(VLM_64x8-VLM_64x4)
          + abs(LLT_128-LLT_64)                       [CL values]
model_CD = (2*abs(CList)+model_CL)*model_CL/(pi*AR*e_list)
mesh_CD = same refinement sum, using CDi values
require each absolute inter-method gap <= mesh + model
```

Here CList/e_list are T13's CL/span efficiency. The model_CD expression is
the algebraic propagation through CL^2/(pi*AR*e) HOLDING e FIXED. No observed
inter-method gap was used to set the allowance. However, the cited approximate
slope comparison is not a proven bound for this rectangular wing and does not
bound differing span efficiencies. This is a limitation of the acceptance
design, not permission to enlarge it. Numerical/implementation errors also
remain possible; the cause has not been isolated.

At synthetic AR=40, alpha=0.05 rad:

| Quantity | VLM 64x8 | T13 128 terms | Absolute gap | Mesh part | Model part | Total allowance |
|---|---|---|---|---|---|---|
| CL | 0.2909122489637976 | 0.29278072785693654 | 0.001868478893139 | 0.0009032011589706 | 0.0003555442997644 | 0.001258745458735 |
| CDi | 0.0007977879080538741 | 0.0008624499010484641 | 0.00006466199299459 | 0.000006814455567569 | 0.000002095939409493 | 0.000008910394977062 |

CL discrepancy is 0.638184% of T13, 1.4844 times its allowance. CDi discrepancy
is 7.497478%, 7.2569 times its allowance. The CL assertion fails before the
CDi assertion; both measured values are printed before either assertion.

```text
test high_ar_agreement_uses_predeclared_mesh_and_published_method_difference_budget ... FAILED
CL discrepancy exceeds predeclared budget; STOP rather than widen
test result: FAILED. 7 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

The original tests-only snapshot failed to compile with the VLM module absent.
The historical failure above stopped the initial batch. The owner then
explicitly authorized diagnosis and replacement acceptance, without widening
this allowance or altering its inputs. The full original function remains in
vlm_expected_difference.md as a record, not a live passing assertion.

## Historical b66b434 acceptance (superseded)

Exact empirical assertions and bound derivation are in vlm_diagnosis.md.
At AR6/10/20/40 the relative lift gaps decrease from 6.545857% to 0.801765%;
CDi gaps decrease from 16.017985% to 7.178147%. Efficiency gaps instead grow;
no decreasing-efficiency-gap test is asserted. Both local and fitted power
exponents must be positive. A free-intercept fit must also be compatible with
zero within propagated last-mesh/term changes plus leave-one-out range
sensitivity, with no fitted safety multiplier. Current measured intercept/bands:

| Quantity | Free intercept | Propagated mesh/term | Leave-one-out sensitivity | Combined band |
|---|---|---|---|---|
| Relative CL gap | -0.002500573066714 | 0.004514923294135 | 0.0005568601399455 | 0.0050717834340805 |
| Relative CDi gap | -0.01415900842614 | 0.008119139840560 | 0.007282811733700 | 0.015401951574260 |

The extrapolation is conditional: finite data cannot prove the true infinite-AR
limit. Its inverse-log drag hypothesis is unverified outside this range. A
hand-defined delta=.02+.6/AR has a known nonzero .02 limit and decreasing gaps;
the free-intercept check rejects it, with zero mesh uncertainty and only
2.081668171172e-17 fitting sensitivity. This prevents a decreasing trend alone
from satisfying the zero-limit check.

Tests were committed before the new spacing API existed:

```text
error[E0432]: unresolved import `soas_estimator::vlm::SpanwiseSpacing`
error[E0560]: struct `VlmRequest` has no field named `spanwise_spacing`
error: could not compile `soas-estimator` (test "vlm_acceptance") due to 3 previous errors
```

The implementation then added selectable uniform/cosine spacing with cosine
default. No production physical bug was established, so the kernel/load laws
were not altered to fit LLT. No existing numerical tolerance was widened.

The required deliberate fault was made only in a scratch workspace copy under
target/t15-quarter-control-check: replace panel control fraction .75 by .25.
The bound/control coincide in the one-panel hand case, giving:

```text
called Result::unwrap() on an Err value:
VlmError("influence[0][0]: vortex singularity: point lies on a filament segment/ray")
test one_panel_has_independent_hand_circulation_lift_drag_moment_and_neutral_point ... FAILED
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 6 filtered out
```

The fault was never applied to production or committed. The original .25/.75
placement settings are mathematical definitions, not new fixture data. Final
formatting and Clippy with warnings denied pass, and all 169 workspace tests
pass (0 failed, 0 ignored); ten of them are current T15 tests. An unused import
left by moving the old comparison to documentation was removed, changing no
assertion. Existing CARGO_INCREMENTAL=0 is retained for the earlier disclosed
filesystem issue; no new build workaround was applied.

## AVL and remaining confidence limits

No AVL output existed. No AVL source, binary or output was downloaded/run.
Authored synthetic rectangular and elliptic polygon inputs, command scripts,
per-number ledger and exact owner/license instructions live in ../avl/.
The original 64x8 and new 128x8 rectangles are both supplied. Panel/coordinate,
linearized-alpha, elliptic-polygon and output interpretation differences are
stated there. No fabricated external comparison test is added. Incompressible,
attached, planar, small-angle assumptions still apply; swept/low-AR hero-wing
physical validity remains unassessed.

## Files changed for T15

Estimator: src/lib.rs, src/vlm.rs; tests/vlm.rs, tests/vlm_acceptance.rs;
fixtures/vlm_cases.json, vlm_cases_values.md, vlm_test_values.md,
vlm_trend.json, vlm_trend_values.md, vlm_acceptance_test_values.md.
Documentation: docs/TASKS.md, ASSUMPTIONS.md, LEARNING.md, QUESTIONS.md;
estimator/vlm.md, vlm_diagnosis.md, vlm_expected_difference.md; avl/README.md,
INPUT_VALUES.md and three authored geometry/command pairs. No dependencies or
core/CLI production behavior changed in this task.
