# T15 diagnosis — 2026-11-14

## Scope, reproduction and provenance

No production file changed during diagnosis. Before/after SHA-256 of vlm.rs:
`00687F0F2096E589D8709E097B0C3499475D48566D595E0CD677C91FA475B0D5`.
Checkout was main; task/t15 was created at 8067b1d, retaining unfinished work.
No AVL run/download, new repository dependency, or T10/T11 work occurred.

All diagnostic geometry/operating inputs are **SYNTHETIC TEST VALUES**, not
aircraft data: alpha=0.05 rad, rectangular chord=1 m, span=AR*chord,
twist=alpha_L0=z=0, section slope=2*pi/rad, rectangle leading-edge x=0,
reference=(0,0,0). Ny counts the FULL wing; Nx is chordwise count. The original
comparison remains 64x8, AR=40, alpha=0.05, T13=128 odd Fourier terms. The new
sweep uses 128x8 consistently and does not change the original input.

The appendix contains the scratch Rust program and numerical ledger. After the
diagnosis commit, its final baseline cross-check call was adapted only to add
the new default spacing field; no diagnostic geometry/settings changed.
It reuses the existing T14/T13 APIs and recomputes the VLM equations with
uniform/cosine spacing, an elliptic mesh and near-field drag. Rectangular
cosine 64x8 reproduces production CL/CDi to 15 decimal places; this verifies
transcription consistency, not independence of the shared kernel.

Recreate target/t15-diagnosis/src/main.rs from the appendix; its Cargo.toml
uses a standalone `[workspace]`, nalgebra 0.34.1 with std, and soas-estimator
and soas-core path dependencies to ../../crates/. From project-kit:

```powershell
$env:CARGO_INCREMENTAL='0'
cargo run --offline --release --manifest-path target/t15-diagnosis/Cargo.toml
```

The previously disclosed incremental-compilation setting is retained; no new
filesystem/build workaround was applied. Scratch files are not production.

## Formula sources and verification

- [Drela, JVL formulation](https://web.mit.edu/drela/Public/web/jvl/jvl.pdf),
  sections 3–6: lattice, tangency, bound-vortex Kutta-Joukowski force (108),
  self-bound exclusion (111), far-wake energy drag (123–127). Only the planar
  linearized subset is used, not the jet model.
- [AVL 3.40 primer](https://web.mit.edu/drela/Public/web/avl/avl_doc.txt): spacing,
  implied section slope, coordinate signs and near/far drag. Full-span cosine
  corresponds to root-to-tip negative-sine spacing. No wake length is selected.
- [Caughey, Aerodynamic Background](https://courses.cit.cornell.edu/mae5070/AeroBackground.pdf):
  AR=b^2/S; a0=2*pi; elliptic LLT slope=a0/(1+a0/(pi*AR));
  CDi=CL^2/(pi*AR*e). Efficiency here is e=CL^2/(pi*AR*CDi), beta=0.
- [E. O. Tuck, Some accurate solutions of the lifting surface integral equation,
  1993](https://doi.org/10.1017/S0334270000009097), Table 1 and section 5:
  rectangular lifting-surface benchmarks and tip-loading differences from LLT.
  Its large-AR slope contains log(AR)/AR; differing constant terms give a 1/AR
  slope gap. This supports a lift trend, not a rigorous drag error bound.

Sources checked online in this batch. No external code copied. T14's existing
caveat about garbled extracted NASA PDF equation characters still applies.
Least-squares formulas additionally follow [NIST section 4.4.3.1](https://www.itl.nist.gov/div898/handbook/pmd/section4/pmd431.htm); the sensitivity-band construction is explicitly our empirical inference.
No independent AVL result is available; this is software/theory diagnosis,
not real-wing validation.

## (a) Original failure, inputs unchanged

AR=40; alpha=0.05 rad (2.864788975654116 degrees); VLM=64x8; T13=128 terms.

| Quantity | VLM | T13 | Absolute gap | Gap / abs(T13) |
| --- | --- | --- | --- | --- |
| CL | 0.2909122489637976 | 0.2927807278569365 | 1.868478893139e-03 | 0.638183704% |
| CDi | 0.0007977879080538741 | 0.0008624499010484641 | 6.466199299459e-05 | 7.497478163% |

Old allowances remain CL=0.001258745458735, CDi=0.000008910394977062.
Both are exceeded. Unchanged rerun: 0 passed, 1 failed, 7 filtered out;
`CL discrepancy exceeds predeclared budget; STOP rather than widen`.
The owner authorized replacement acceptance, not widening these allowances.

## (b) Spacing series — AR=40, alpha=0.05, Nx=8

### Uniform
| Ny | CL | CDi | e |
| --- | --- | --- | --- |
| 8 | 0.298523875069 | 0.000690834147 | 1.026536700838 |
| 16 | 0.295972525987 | 0.000755230534 | 0.923024850411 |
| 32 | 0.293735301442 | 0.000788390204 | 0.870885789286 |
| 64 | 0.292090032638 | 0.000799792192 | 0.848880290981 |
| 128 | 0.291079378672 | 0.000802542125 | 0.840127453779 |
| 256 | 0.290523174209 | 0.000803019846 | 0.836421943904 |

Three-grid observed order: `p=log2(abs(f_N-f_2N)/abs(f_2N-f_4N))`.

| Ny triple | p(CL) | p(CDi) | p(e) |
| --- | --- | --- | --- |
| 8/16/32 | 0.189550 | 0.957550 | 0.989359 |
| 16/32/64 | 0.443387 | 1.540144 | 1.244501 |
| 32/64/128 | 0.703034 | 2.051817 | 1.330041 |
| 64/128/256 | 0.861602 | 2.525155 | 1.240079 |

### Cosine
| Ny | CL | CDi | e |
| --- | --- | --- | --- |
| 8 | 0.295815221578 | 0.000738758854 | 0.942602250051 |
| 16 | 0.293340729153 | 0.000774334728 | 0.884313252028 |
| 32 | 0.291806010153 | 0.000791161408 | 0.856472663930 |
| 64 | 0.290912248964 | 0.000797787908 | 0.844163756245 |
| 128 | 0.290433313208 | 0.000800541980 | 0.838491923050 |
| 256 | 0.290185901679 | 0.000801765548 | 0.835786521710 |

Three-grid observed order: `p=log2(abs(f_N-f_2N)/abs(f_2N-f_4N))`.

| Ny triple | p(CL) | p(CDi) | p(e) |
| --- | --- | --- | --- |
| 8/16/32 | 0.689158 | 1.080149 | 1.066034 |
| 16/32/64 | 0.780013 | 1.344432 | 1.177487 |
| 32/64/128 | 0.900057 | 1.266681 | 1.117816 |
| 64/128/256 | 0.952919 | 1.170471 | 1.067972 |

Cosine resolves tips better at equal count, especially CL. Uniform Ny=8 e>1
is a discretization warning. Both spacings approach common values. Observed
orders depend on mesh and quantity; coarse-grid cancellation precludes a
universal-order claim. Cosine is a justified default numerical mesh policy,
not a physical tolerance.

## (c) Rectangular AR sweep — cosine 128x8, alpha=0.05

| AR | VLM CL/alpha | T13 CL/alpha | VLM CDi | T13 CDi | VLM e | T13 e |
| --- | --- | --- | --- | --- | --- | --- |
| 6 | 4.2338698333 | 4.5304249854 | 0.002396538369 | 0.002853632849 | 0.9920407292 | 0.9539347672 |
| 10 | 4.8556302498 | 5.0467908496 | 0.001940358073 | 0.002200966187 | 0.9669398765 | 0.9208891992 |
| 20 | 5.4469226135 | 5.5442736588 | 0.001296061095 | 0.001420876449 | 0.9108294752 | 0.8607819293 |
| 40 | 5.8086662642 | 5.8556145571 | 0.000800541980 | 0.000862449901 | 0.8384919231 | 0.7909357966 |

| AR | Relative slope difference | Relative CDi difference | Relative e difference |
| --- | --- | --- | --- |
| 6 | -6.54585724% | -16.01798497% | +3.99460878% |
| 10 | -3.78776544% | -11.84062326% | +5.00067515% |
| 20 | -1.75588456% | -8.78439178% | +5.81419569% |
| 40 | -0.80176543% | -7.17814695% | +6.01264056% |

Absolute relative slope and CDi gaps decrease strictly across every AR.
Efficiency gaps instead increase: an all-quantity decreasing-gap claim is false.

## (d) Elliptic geometry

Analytic c(y)=sqrt(1-(2y/b)^2) m, b=AR*pi/4 m, S=pi*b/4 m^2, leading edge
x=c/4 gives straight quarter chord at x=0. No twist/alpha_L0; alpha=0.05 rad.
Chords at cosine edges are linearly joined, including zero tip chords. Exact
continuum S/AR normalize results. This scratch mesh is an approximation to the
ellipse, not a new T12 API; T12 positive station-tip chords remain unchanged.
Ny=128, Nx=8; elliptic LLT e=1 and slope=a0/(1+a0/(pi*AR)).

| AR | VLM e | VLM CL/alpha | LLT CL/alpha | Relative slope difference |
| --- | --- | --- | --- | --- |
| 6 | 1.0080970929 | 4.4097808294 | 4.7123889804 | -6.42154441% |
| 10 | 1.0088052165 | 5.0697331737 | 5.2359877560 | -3.17522863% |
| 20 | 1.0094090538 | 5.6512689398 | 5.7119866429 | -1.06298748% |
| 40 | 1.0096330831 | 5.9650642991 | 5.9839860068 | -0.31620575% |

Efficiency overshoot decreases with refinement:

| AR | Ny | CL/alpha | e |
| --- | --- | --- | --- |
| 6 | 32 | 4.4539671994 | 1.0390086878 |
| 6 | 64 | 4.4253559934 | 1.0183202768 |
| 6 | 128 | 4.4097808294 | 1.0080970929 |
| 6 | 256 | 4.4015186472 | 1.0030273658 |

| AR | Ny | CL/alpha | e |
| --- | --- | --- | --- |
| 40 | 32 | 5.9682394076 | 1.0392881428 |
| 40 | 64 | 5.9675335077 | 1.0195003838 |
| 40 | 128 | 5.9650642991 | 1.0096330831 |
| 40 | 256 | 5.9632892074 | 1.0047351478 |

The e>1 overshoot is numerical, not physical superiority. Remaining slope
differences shrink with AR, consistent with finite-chord differences. Chord
resolution, ellipse approximation and wake quadrature remain contributors;
finite-AR lifting surfaces need not match elliptic LLT exactly.

## (e) Induced drag two ways

Trailing legs are semi-infinite along **-body x**, the linearized freestream,
not (-cos(alpha),0,-sin(alpha)) at finite alpha. The Trefftz plane is the
analytic downstream-infinity limit; no finite distance/wake length is chosen.

Far field retains strip-midpoint energy quadrature. Near-field diagnostic
evaluates induced z velocity at each bound midpoint, omitting its own singular
bound segment but retaining its trailing legs. From Kutta-Joukowski and
linearized wind projection, `CDi_KJ=2*sum(g*w_induced_over_V*dy)/S`, g=Gamma/V.
The alpha axial term cancels on projection; no factor is fitted to Trefftz.

| AR | Trefftz CDi | KJ CDi | KJ minus Trefftz | Relative / Trefftz |
| --- | --- | --- | --- | --- |
| 6 | 0.002396538369 | 0.002398744647 | +2.206277851000e-06 | +0.09206103% |
| 10 | 0.001940358073 | 0.001942305192 | +1.947119343000e-06 | +0.10034845% |
| 20 | 0.001296061095 | 0.001297542445 | +1.481349700000e-06 | +0.11429629% |
| 40 | 0.000800541980 | 0.000801600706 | +1.058725819000e-06 | +0.13225113% |

Original AR40 64x8: Trefftz=0.000797787908054, KJ=0.000799679399783;
gap=0.000001891491729, about 0.2371%. Uniform rectangular widths give midpoint
reciprocity cancellation to displayed precision; unequal cosine widths leave
quadrature differences. Both share circulation/kernel, so their agreement
argues against a missing factor of two/sign error but cannot exclude all bugs.

## (f) Evidence and untested hypotheses

- Discretization: coarse e>1 decreases toward the planar bound; both spacings
  approach common results. AR40 64-to-128 changes CL by 0.000478936, smaller
  than the LLT gap but material. No certified tail bound is established.
- Method difference: monotonically shrinking lift/CDi gaps, elliptic slope
  trends and Tuck's independent lifting-surface theory support finite-chord
  and wing-tip-loading differences. Tuck Table 1 slopes at AR10/20 are
  4.83848/5.43349; finite-mesh VLM gives 4.85563025/5.44692261, differences
  about 0.3545%/0.2472%, consistent in direction/scale with mesh bias. These
  cited mathematical results are not invented aircraft fixture numbers.
- Acceptance-design defect: ellipse-based slope allowances with LLT e held
  fixed did not bound rectangular loading differences; they were not rigorous
  planform-specific error bounds. No production bug is established.
- Untested: freestream-aligned wakes at finite alpha, continuous wake integrals,
  fully converged elliptic chordwise grids, independent AVL, swept/low-AR data,
  and extrapolation beyond measured AR. A kernel or assembly defect remains
  possible; these results are not proof of correctness or physical validity.

## Acceptance derivation for the authorized replacement

Require positive, strictly decreasing absolute relative CL-slope and CDi gaps
at AR6/10/20/40. Require positive local log-log exponents
`p=ln(delta_i/delta_next)/ln(AR_next/AR_i)`. Positive fitted power-law exponent
means the EMPIRICAL continuation C*AR^(-p) tends to zero, conditional on that
continuation; four finite points cannot prove the physical infinite-AR limit.

To avoid simply imposing zero, also fit a FREE intercept `delta=a+b*x`:
x=1/AR for lift (Tuck's differing asymptotic constant terms), x=1/ln(AR) for
relative drag (HYPOTHESIS: constant tip-drag difference divided by a logarithmic
rectangular tip contribution, not a published bound). Require
`abs(a) <= mesh_propagation + leave_one_out_curvature`. Same construction for
both quantities, no selected safety multiplier. Each row's empirical mesh
uncertainty is (abs(128x8-64x8)+abs(128x8-128x4))/abs(LLT). Add measured T13
128-versus-64 change when implementing. Last changes are uncertainty proxies,
not certified asymptotic error bounds.

Ordinary least-squares normal equations give intercept weights
`w_i=1/n-xbar*(x_i-xbar)/sum((x-xbar)^2)`; mesh propagation is
`sum(abs(w_i)*u_i)`. Curvature is maximum absolute intercept change when any
one row is left out, a finite-range sensitivity check, not a confidence interval.
Using VLM refinement alone (LLT term change is negligible):

- CL: intercept=-0.00250057306672; mesh=0.00451487154335; curvature=0.000556860139945; total=0.0050717316833. Zero is inside this empirical band.
  Local exponents: 1.070925360, 1.109148994, 1.130945890.
- CDi: intercept=-0.0141590084272; mesh=0.00811775054251; curvature=0.00728281173258; total=0.0154005622751. Zero is inside this empirical band.
  Local exponents: 0.591544099, 0.430730717, 0.291330941.

This can fail on nondecreasing gaps, nonpositive exponents or a persistent
free-intercept bias beyond measured mesh/range sensitivity. A separate synthetic
constant-floor fault will test that rejection. No fixed numeric tolerance is
chosen from the observed inter-method gap. The original failure remains an
expected-difference record, not an assertion that these methods must agree
within its former allowance.

## Numerical ledger

All appendix numerical settings are **SYNTHETIC diagnostic values**, or sourced
algebraic constants. AR6/10/20/40; chord/root chord=1 m; alpha=.05 rad; all
twist/zero-lift/z/xref=0; Ny8/16/32/64/128/256, Nx4/8, T13=128. Index/division
factors 1/2/4 and panel fractions .25/.75 follow definitions; pi, the ellipse
square root and 2*pi slope are mathematical/theoretical constants. Output
15-decimal formatting is only a diagnostic precision setting. Inventory:

| Appendix line | Literal | Provenance |
| --- | --- | --- |
| 11 | 0 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 11 | 2 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 11 | 1 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 11 | 0 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 11 | 0 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 11 | 0 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 11 | 0 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 13 | 0.05 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 14 | 4 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 15 | 4 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 16 | 0 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 16 | 0 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 16 | 2 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 16 | 2 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 16 | 2 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 16 | 2 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 17 | 1 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 17 | 2 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 17 | 2 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 17 | 0 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 17 | 1 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 18 | 4 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 18 | 0 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 20 | 0 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 20 | 1 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 21 | 0 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 21 | 0.25 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 21 | 0.75 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 22 | 0 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 22 | 0 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 23 | 2 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 23 | 2 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 23 | 2 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 23 | 0 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 28 | 0 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 28 | 0 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 28 | 0 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 29 | 2 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 30 | 0 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 37 | 0 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 38 | 0 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 38 | 1 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 38 | 2 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 38 | 0 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 38 | 2 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 38 | 1 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 38 | 1 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 38 | 1 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 38 | 1 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 39 | 2 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 39 | 2 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 44 | 8 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 44 | 16 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 44 | 32 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 44 | 64 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 44 | 128 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 44 | 40 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 44 | 8 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 45 | 6 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 45 | 10 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 45 | 20 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 45 | 40 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 45 | 0.05 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 45 | 128 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 46 | 128 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 46 | 8 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 47 | 0.05 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 48 | 64 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 48 | 8 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 48 | 128 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 48 | 4 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 49 | 128 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 49 | 8 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 49 | 2 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 49 | 1 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 49 | 2 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 49 | 0.05 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 52 | 6 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 52 | 40 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 52 | 32 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 52 | 64 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 52 | 256 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 52 | 8 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 53 | 40 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 53 | 256 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 53 | 8 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 54 | 40 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 54 | 64 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 54 | 8 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 54 | 0.05 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |
| 54 | 0 | SYNTHETIC diagnostic setting / sourced algebraic constant (groups above) |

## Appendix — exact diagnostic source (not production)

```rust
// Diagnostic-only scratch program. All settings are synthetic, not aircraft data.
// Formulas: existing VLM baseline; Drela JVL 3-6; Caughey AeroBackground 2.19.
use nalgebra::{DMatrix,DVector,Vector3};
use soas_estimator::{vortex::{Horseshoe,semi_infinite_velocity_per_circulation as ray},
 planform::{WingPlanform,WingStation,thin_airfoil_slope_per_rad},
 lifting_line::{self,StraightWing},vlm::{self,VlmRequest}};
use soas_core::units::Radians;
use std::f64::consts::PI;
fn rad(x:f64)->Radians {Radians::new(x).unwrap()}
fn rectangle(ar:f64)->WingPlanform {WingPlanform {data_note:"SYNTHETIC diagnostic rectangle".into(),
 stations:[0.,ar/2.].into_iter().map(|y|WingStation {y_m:y,chord_m:1.,leading_edge_x_m:0.,z_m:0.,twist_rad:rad(0.),section_lift_curve_slope_per_rad:thin_airfoil_slope_per_rad(),zero_lift_angle_rad:rad(0.),section_source:"Caughey thin-airfoil theory; synthetic".into()}).collect()}}
fn calc(ar:f64,ny:usize,nx:usize,cosine:bool,ellipse:bool)->(f64,f64,f64,f64) {
 let alpha=0.05;
 let span=if ellipse {ar*PI/4.} else {ar};
 let area=if ellipse {PI*span/4.} else {span};
 let edges:Vec<_>=(0..=ny).map(|i|if i==0 {-span/2.} else if i==ny {span/2.} else if cosine {-span/2.*(PI*i as f64/ny as f64).cos()} else {-span/2.+span*i as f64/ny as f64}).collect();
 let chord=|y:f64| if ellipse {(1.-(2.*y/span).powi(2)).max(0.).sqrt()}else{1.};
 let le=|c:f64| if ellipse {c/4.}else{0.};
 let mut horses=Vec::new(); let mut controls=Vec::new();
 for i in 0..ny { let (yl,yr)=(edges[i],edges[i+1]); let (cl,cr)=(chord(yl),chord(yr));
  for k in 0..nx {let (fb,fc)=((k as f64+0.25)/nx as f64,(k as f64+0.75)/nx as f64);
   horses.push(Horseshoe {bound_start_m:Vector3::new(le(cl)-fb*cl,yl,0.),bound_end_m:Vector3::new(le(cr)-fb*cr,yr,0.),wake_direction_unit:-Vector3::x()});
   controls.push(Vector3::new((le(cl)+le(cr))/2.-fc*(cl+cr)/2.,(yl+yr)/2.,0.));
  }
 }
 let n=horses.len(); let mat=DMatrix::from_fn(n,n,|i,j|horses[j].velocity_per_circulation(controls[i]).unwrap().z);
 let gamma=mat.lu().solve(&DVector::from_element(n,alpha)).unwrap();
 let mut strip=vec![0.;ny];let mut lift=0.;let mut drag_near=0.;
 for (i,h) in horses.iter().enumerate() {let mid=(h.bound_start_m+h.bound_end_m)/2.;let dy=h.bound_end_m.y-h.bound_start_m.y;
  let mut w=0.;
  for (j,other) in horses.iter().enumerate() {
   let v=if i==j {-ray(other.bound_start_m,other.wake_direction_unit,mid).unwrap()+ray(other.bound_end_m,other.wake_direction_unit,mid).unwrap()}
    else {other.velocity_per_circulation(mid).unwrap()};w+=gamma[j]*v.z;
  }
  lift+=gamma[i]*dy;drag_near+=gamma[i]*w*dy;strip[i/nx]+=gamma[i];
 }
 let mut drag_far=0.;
 for i in 0..ny {let y=(edges[i]+edges[i+1])/2.;let w=(0..ny).map(|j|strip[j]/(2.*PI)*(1./(y-edges[j])-1./(y-edges[j+1]))).sum::<f64>();drag_far+=strip[i]*w*(edges[i+1]-edges[i]);}
 let cl=2.*lift/area;let cd=drag_far/area;let kj=2.*drag_near/area;let e=cl*cl/(PI*ar*cd);
 (cl,cd,e,kj)
}
fn main(){
 println!("kind,AR,Ny,Nx,spacing,CL,CDi,e,CDi_KJ,CLalpha_LLT,CDi_LLT,e_LLT");
 for cosine in [false,true] {for ny in [8,16,32,64,128] {let (cl,cd,e,kj)=calc(40.,ny,8,cosine,false);println!("spacing,40,{ny},8,{},{cl:.15},{cd:.15},{e:.15},{kj:.15},,,",if cosine{"cosine"}else{"uniform"});}}
 for ar in [6.,10.,20.,40.] {let w=rectangle(ar);let ll=lifting_line::solve(StraightWing::Stations(&w),rad(0.05),128).unwrap();
  let (cl,cd,e,kj)=calc(ar,128,8,true,false);
  println!("AR,{ar},128,8,cosine,{cl:.15},{cd:.15},{e:.15},{kj:.15},{:.15},{:.15},{:.15}",ll.cl/0.05,ll.induced_drag_coefficient,ll.span_efficiency.unwrap());
  for (ny,nx) in [(64,8),(128,4)] {let (cl,cd,e,kj)=calc(ar,ny,nx,true,false);println!("ARmesh,{ar},{ny},{nx},cosine,{cl:.15},{cd:.15},{e:.15},{kj:.15},,,");}
  let (cl,cd,e,kj)=calc(ar,128,8,true,true);let slope=2.*PI/(1.+2./ar);let cl_ll=slope*0.05;let cd_ll=cl_ll*cl_ll/(PI*ar);
  println!("ellipse,{ar},128,8,cosine,{cl:.15},{cd:.15},{e:.15},{kj:.15},{slope:.15},{cd_ll:.15},1");
 }
 for ar in [6.,40.] {for ny in [32,64,256] {let (cl,cd,e,kj)=calc(ar,ny,8,true,true);println!("ellipse_mesh,{ar},{ny},8,cosine,{cl:.15},{cd:.15},{e:.15},{kj:.15},,,");}}
 for cosine in [false,true] {let (cl,cd,e,kj)=calc(40.,256,8,cosine,false);println!("spacing_tail,40,256,8,{},{cl:.15},{cd:.15},{e:.15},{kj:.15},,,",if cosine{"cosine"}else{"uniform"});}
 let w=rectangle(40.);let base=vlm::solve(&w,VlmRequest{spanwise_panels:64,chordwise_panels:8,alpha_rad:rad(0.05),reference_x_body_m:0.,spanwise_spacing:Default::default()}).unwrap();
 println!("baseline,40,64,8,cosine,{:.15},{:.15},,,,,",base.cl,base.induced_drag_coefficient);
}
```
