# T15b: production Rust versus archived independent solver

## Which implementation ran

These are direct calls to **soas_estimator::vlm::solve**, the production Rust
function. The archived Python solver was not used for these new outputs.
The diagnostic integration target lives in an ignored scratch copy whose
production crate files and Cargo.lock match revision 6121ae7 byte-for-byte.
The only new Rust file is an off-line printing driver, reproduced below;
it is neither committed source nor part of build/CI. The owner-approved
estimator test-profile optimization is active, with debug and overflow checks
enabled as verified in t15b_profile.md. No solver logic, dependency, assertion
or tolerance changed.

Synthetic conditions exactly match the rectangular baseline in
vlm_efficiency_gap.md: chord1 m, span=AR m, alpha .05 rad, zero twist, zero
section zero-lift angle, zero leading-edge/body-z offsets, reference x=0,
ideal section slope 2*pi per rad, full-span cosine spacing and Nx8. Mesh
counts are Ny128/256/512/1024 for AR10/20/40/80. The body-axis wake is
semi-infinite downstream along -x and drag uses the Trefftz infinity limit.

The relation e=CL^2/(pi*AR*CDi) is the induced span-efficiency definition from
[MIT Lecture 9](https://web.mit.edu/16.unified/www/SPRING/fluids/Spring2008/LectureNotes/f09.pdf).
Vortex/tangency and far-wake context:
[Drela JVL formulation](https://web.mit.edu/drela/Public/web/jvl/jvl.pdf).
As in the existing linear model, rescaling to a common nonzero CL leaves e
unchanged. Agreement between these implementations is not real-wing validation.

## Measured comparison

Full archived values are from the already-recorded baseline and its numerical
ledger, not inferred from the six-decimal owner summary. Signed difference is
Rust minus archived; relative difference divides by the archived value. The
last column separately shows the difference from the owner's rounded numbers
(.966940, .907753, .834468; AR80 uses its similarly rounded archived value).

| AR | Ny x Nx | Rust CL | Rust CDi | Rust e | Archived e | Signed e difference | Relative e difference | Difference from 6-decimal e |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 10 | 128x8 | 0.242781512489863 | 0.001940358072714203 | 0.9669398764711201 | 0.9669398764711180 | +2.109423747e-15 | +2.181545925e-15 | -1.235288799e-07 |
| 20 | 256x8 | 0.272009199786332 | 0.001297238581702304 | 0.9077525144132002 | 0.9077525144131966 | +3.552713679e-15 | +3.913746999e-15 | -4.855867999e-07 |
| 40 | 512x8 | 0.290060228062359 | 0.0008023373041232016 | 0.8344676775566939 | 0.8344676775566786 | +1.532107774e-14 | +1.836030101e-14 | -3.224433061e-07 |
| 80 | 1024x8 | 0.300659840960341 | 0.0004744745190621125 | 0.7580504202570456 | 0.7580504202570049 | +4.074518500e-14 | +5.374996691e-14 | +4.202570456e-07 |

The two independent implementations agree at floating-point roundoff scale
on these measured grids. This makes an implementation discrepancy in this
specific comparison small; it does not establish that either mathematical
model reproduces a real wing, prove a causal explanation of the LLT gap, or
define a new acceptance tolerance. Rounded reference precision is separated
from implementation differences explicitly.

## Production feasibility at the largest requested meshes

Each diagnostic was monitored separately with the same 240 s / 2.0 GB guards
and 100 ms polling method documented in t15b_profile.md. Times exclude
compilation and include process startup; resident and paged/commit counters
are process high-water marks, not virtual-address reservations.

| AR | Wall s | Peak resident bytes | Peak paged/commit bytes | Exit | Guard stop |
| --- | --- | --- | --- | --- | --- |
| 10 | 0.2062457 | 23212032 | 18083840 | 0 | None |
| 20 | 0.8937766 | 73560064 | 68562944 | 0 | None |
| 40 | 9.4014636 | 274853888 | 270647296 | 0 | None |
| 80 | 76.8736739 | 1080778752 | 1078120448 | 0 | None |

**AR80, Ny1024 x Nx8, was actually run in production Rust** and fits the
disclosed resource budget. Its numbers above are not script substitutions.

**AR160, Ny2048 x Nx8, was not launched in production Rust.** The current
solver allocates a full-wing dense matrix and retains it while factorizing
matrix.clone().lu() (crates/soas-estimator/src/vlm.rs, allocation and clone).
N=2048*8=16384 unknowns. Each f64 matrix is N^2*8=2,147,483,648 bytes;
two matrices alone are 4,294,967,296 bytes (4.294967296 GB), before panels,
vectors, allocator or executable memory. This deterministic lower bound
exceeds the disclosed 2,000,000,000-byte resource guard. The two-matrix
lower bound at AR80 is 1,073,741,824 bytes, consistent with the observed
scale. No timing for AR160 Rust is claimed or silently replaced with a
Python timing/result. Its existing independent-script output remains labelled
as such in the earlier report. A larger budget or a separately authorized
solver-memory improvement would be needed to run it here.

## Verification and limits

All four no-assertion diagnostic targets completed with exit0. They only print
production outputs; they add no acceptance tests. The full unfiltered workspace
quality gates already passed in Part1 against the same unchanged source:
177 passed, 0 failed, 0 ignored; formatting and Clippy passed. No additional
full-suite run was needed for this documentation-only record.

## Complete numerical ledger

Every input, mesh, rounded/full reference, output, delta and resource setting
is enumerated by JSON path in independent/T15B_PRODUCTION_LEDGER.md. Capacity
numbers follow the explicit N^2*f64-bytes arithmetic above; f64 occupies8
bytes. The synthetic zero/one normalization and two-root/tip station geometry
are the existing baseline, not newly invented aircraft data. Printed bit
strings are exact diagnostics, with17 decimal places and16 hex digits as in
Part1. These formatting choices are not tolerances.

## Reproduction driver (scratch only, no build/CI changes)

Copy the unchanged current crates/lockfile/root manifest into an ignored
scratch workspace, put this driver in the estimator's tests directory, and
build only --test production_probe with cargo test --no-run. Run each exact
function separately using the Part1 monitor. Set TEMP/TMP inside the checkout
and CARGO_INCREMENTAL=0 as already disclosed. No release profile is substituted.

```rust
// Informational only; SYNTHETIC values from the owner-requested rectangular study.
// Uses unmodified production soas_estimator::vlm::solve, not the archived solver.
use soas_core::units::Radians;
use soas_estimator::{planform::{WingPlanform,WingStation,thin_airfoil_slope_per_rad},
 vlm::{solve,VlmRequest,SpanwiseSpacing}};
fn run(ar:f64,ny:usize) {
 let w=WingPlanform {data_note:"SYNTHETIC production comparison; not aircraft data".into(),stations:[0.0,ar/2.0].into_iter().map(|y|WingStation {
  y_m:y,chord_m:1.0,leading_edge_x_m:0.0,z_m:0.0,twist_rad:Radians::new(0.0).unwrap(),
  section_lift_curve_slope_per_rad:thin_airfoil_slope_per_rad(),zero_lift_angle_rad:Radians::new(0.0).unwrap(),
  section_source:"Cited ideal thin-airfoil slope; synthetic diagnostic".into()}).collect()};
 let g=w.geometry().unwrap();
 let out=solve(&w,VlmRequest{spanwise_panels:ny,chordwise_panels:8,spanwise_spacing:SpanwiseSpacing::Cosine,
  alpha_rad:Radians::new(0.05).unwrap(),reference_x_body_m:0.0}).unwrap();
 println!("CASE,AR={ar},Ny={ny},Nx=8");
 for (name,value) in [("CL",out.cl),("CDi",out.induced_drag_coefficient),
  ("e",out.cl.powi(2)/(std::f64::consts::PI*g.aspect_ratio*out.induced_drag_coefficient)),
  ("Cm",out.pitching_moment_coefficient),("solve_residual",out.relative_collocation_residual)] {
  println!("OUTPUT,{name},{value:.17e},{:016x}",value.to_bits());
 }
}
#[test] fn production_ar10() {run(10.0,128);}
#[test] fn production_ar20() {run(20.0,256);}
#[test] fn production_ar40() {run(40.0,512);}
#[test] fn production_ar80() {run(80.0,1024);}
```
