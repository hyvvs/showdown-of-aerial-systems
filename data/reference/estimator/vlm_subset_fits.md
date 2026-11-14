# T15 subset fits — informational, not acceptance

The previous zero-compatible extrapolation verdict is withdrawn as a pass/fail
criterion by owner instruction. This record preserves the calculations; no
test asserts their verdict. All inputs are SYNTHETIC TEST VALUES: AR6/10/20/40,
alpha=.05 rad, chord1 m, zero datum/twist/alpha_L0, ideal 2*pi section slope,
VLM cosine 128x8; mesh changes against 64x8 and 128x4; T13 128 versus 64 terms.
The old nonzero-floor control has no remaining production/test use after
removing the old extrapolation assertion and helper; it is deleted with
its unused fixture keys. Its history remains in 88dfc0e/b66b434.

Ordinary least squares: [NIST 4.4.3.1](https://www.itl.nist.gov/div898/handbook/pmd/section4/pmd431.htm).
Fit delta=a+b*x with x=1/AR for lift, x=1/ln(AR) for drag. These basis choices
are historical hypotheses, not established extrapolation/error laws. Mesh
uncertainty u=(abs(fine-span_coarse)+abs(fine-chord_coarse)+abs(LLT128-LLT64))/abs(LLT128).
Intercept weights are w_i=1/n-xbar*(x_i-xbar)/sum((x-xbar)^2).
The historical band half-width is sum(abs(w_i)*u_i) plus the maximum absolute
intercept change when one member of THAT subset is omitted. Thus a three-row
subset's range term uses its two-row fits; all four uses three-row fits.
This is a sensitivity band, NOT a statistical confidence interval or physical
error bound. The informational verdict is abs(intercept)<=band.

| AR | CL relative gap | CDi relative gap | u(CL) | u(CDi) |
| --- | --- | --- | --- | --- |
| 6 | 0.06545857244 | 0.160179849656 | 0.00464384949102 | 0.00154178707482 |
| 10 | 0.0378776544342 | 0.118406232588 | 0.00359082936086 | 0.000930955142205 |
| 20 | 0.0175588456255 | 0.0878439178412 | 0.00246548259769 | 0.00220749507379 |
| 40 | 0.00801765425737 | 0.071781469541 | 0.00167569315903 | 0.00345420829307 |

## Lift
| AR subset | Intercept | Mesh part | Subset range part | Band half-width | Old verdict (information only) |
| --- | --- | --- | --- | --- | --- |
| 6,10,20 | -0.00305743320666 | 0.0072601541335 | 0.00043628936779 | 0.00769644350129 | compatible |
| 6,10,40 | -0.00228749539546 | 0.00440307998944 | 0.00120622717899 | 0.00560930716843 | compatible |
| 6,20,40 | -0.0024456273092 | 0.00383595873723 | 0.000922090198407 | 0.00475804893564 | compatible |
| 10,20,40 | -0.00214175014698 | 0.0047038491383 | 0.000618213036187 | 0.00532206217448 | compatible |
| 6,10,20,40 | -0.00250057306671 | 0.00451492329414 | 0.000556860139946 | 0.00507178343408 | compatible |

## Induced drag
| AR subset | Intercept | Mesh part | Subset range part | Band half-width | Old verdict (information only) |
| --- | --- | --- | --- | --- | --- |
| 6,10,20 | -0.0206725125978 | 0.00797646445333 | 0.00744537276054 | 0.0154218372139 | NOT compatible |
| 6,10,40 | -0.0124353289568 | 0.00823623027093 | 0.0156825564015 | 0.0239187866724 | compatible |
| 6,20,40 | -0.0142848759707 | 0.00805005418326 | 0.0166455989307 | 0.024695653114 | compatible |
| 10,20,40 | -0.00687619669244 | 0.0110250497445 | 0.00923691965242 | 0.0202619693969 | compatible |
| 6,10,20,40 | -0.0141590084261 | 0.00811913984056 | 0.0072828117337 | 0.0154019515743 | compatible |

The drag verdict changes with the AR subset, even though all use the same
mesh and underlying solver data. Removing one row changes both extrapolation
leverage and the ad hoc range-sensitivity term. Four finite AR points do not
identify a unique infinite-AR intercept or validate either basis. This
subset dependence makes the verdict unfit as a robust assertion. No band is
widened to preserve a passing outcome.

## Elliptic evidence behind the owner's clarification

Before changing tests, a no-assertion probe used the existing production VLM
API on ellipse polygons at nominal AR6/40, Ny32/64/128/256/512, Nx8. Span is
b=AR*pi/4 m; raw c=sqrt(1-(2y/b)^2), sampled at cosine edges; positive tip
c is floored at binary64 EPSILON solely for T12's positive-chord contract.
Chords are uniformly scaled so the polygon area is exactly b^2/AR at each
mesh. Leading edge x=c/4 keeps quarter chord straight at x=0; no twist or
zero-lift angle. Thus span, area and AR stay fixed while geometry approaches
the same analytic ellipse. These are SYNTHETIC numerical benchmark values,
not a physical tip model. Formula sources: existing estimator geometry/LLT
notes (MIT Lectures 8/9, Caughey), and VLM/JVL notes.

| AR | Ny | VLM slope | LLT slope | Absolute slope error | e |
| --- | --- | --- | --- | --- | --- |
| 6 | 32 | 4.458766491577 | 4.712388980385 | 0.253622488807 | 1.039007512833 |
| 6 | 64 | 4.426537999315 | 4.712388980385 | 0.285850981070 | 1.018319796735 |
| 6 | 128 | 4.410074080428 | 4.712388980385 | 0.302314899956 | 1.008096948615 |
| 6 | 256 | 4.401591665621 | 4.712388980385 | 0.310797314764 | 1.003027326095 |
| 6 | 512 | 4.397215969539 | 4.712388980385 | 0.315173010845 | 1.000499703819 |
| 40 | 32 | 5.977345223817 | 5.983986006838 | 0.006640783021 | 1.039288927038 |
| 40 | 64 | 5.969804469697 | 5.983986006838 | 0.014181537141 | 1.019500367541 |
| 40 | 128 | 5.965631252881 | 5.983986006838 | 0.018354753957 | 1.009633066699 |
| 40 | 256 | 5.963430841602 | 5.983986006838 | 0.020555165235 | 1.004735142809 |
| 40 | 512 | 5.962310113315 | 5.983986006838 | 0.021675893522 | 1.002300858306 |

Efficiency excess over one shrinks, but lift-slope error against LLT grows.
The owner explicitly approved keeping the LLT differences informational and
instead testing decreasing successive VLM mesh changes. No mesh interval was
selected to manufacture decreasing error against LLT.

## Numeric ledger and reproduction

All experiment settings above and in the appendix are labeled SYNTHETIC:
AR6/10/20/40, alpha .05, root chord1, zero offsets/angles, Ny32/64/128/256/512,
Nx8 (mesh comparisons also Nx4), LLT64/128. Binary64 EPSILON is a documented
machine arithmetic value; pi, half-span, quarter chord and squared distances
are mathematical definitions. 17 denotes printed decimal precision only.
The original zero-floor control .02+.6/AR is historical synthetic fault data.

Use the standalone Cargo package described in vlm_gap_vs_mesh.md. Save this
appendix as src/bin/acceptance_probe.rs, set TEMP/TMP to checkout/target/batch-temp
and CARGO_INCREMENTAL=0, then run `cargo run --offline --release --manifest-path
target/t15-review/mesh/Cargo.toml --bin acceptance_probe`. No acceptance
assertions or external programs are involved.

| Appendix line | Literal | Provenance |
| --- | --- | --- |
| 6 | 0 | SYNTHETIC numerical setting or sourced mathematical factor; groups above |
| 6 | 0 | SYNTHETIC numerical setting or sourced mathematical factor; groups above |
| 6 | 0 | SYNTHETIC numerical setting or sourced mathematical factor; groups above |
| 7 | 0.05 | SYNTHETIC numerical setting or sourced mathematical factor; groups above |
| 7 | 0 | SYNTHETIC numerical setting or sourced mathematical factor; groups above |
| 10 | 6 | SYNTHETIC numerical setting or sourced mathematical factor; groups above |
| 10 | 10 | SYNTHETIC numerical setting or sourced mathematical factor; groups above |
| 10 | 20 | SYNTHETIC numerical setting or sourced mathematical factor; groups above |
| 10 | 40 | SYNTHETIC numerical setting or sourced mathematical factor; groups above |
| 11 | 0 | SYNTHETIC numerical setting or sourced mathematical factor; groups above |
| 11 | 1 | SYNTHETIC numerical setting or sourced mathematical factor; groups above |
| 11 | 0 | SYNTHETIC numerical setting or sourced mathematical factor; groups above |
| 11 | 2 | SYNTHETIC numerical setting or sourced mathematical factor; groups above |
| 11 | 1 | SYNTHETIC numerical setting or sourced mathematical factor; groups above |
| 11 | 0 | SYNTHETIC numerical setting or sourced mathematical factor; groups above |
| 12 | 128 | SYNTHETIC numerical setting or sourced mathematical factor; groups above |
| 12 | 8 | SYNTHETIC numerical setting or sourced mathematical factor; groups above |
| 12 | 64 | SYNTHETIC numerical setting or sourced mathematical factor; groups above |
| 12 | 8 | SYNTHETIC numerical setting or sourced mathematical factor; groups above |
| 12 | 128 | SYNTHETIC numerical setting or sourced mathematical factor; groups above |
| 12 | 4 | SYNTHETIC numerical setting or sourced mathematical factor; groups above |
| 13 | 0.05 | SYNTHETIC numerical setting or sourced mathematical factor; groups above |
| 13 | 128 | SYNTHETIC numerical setting or sourced mathematical factor; groups above |
| 13 | 0.05 | SYNTHETIC numerical setting or sourced mathematical factor; groups above |
| 13 | 64 | SYNTHETIC numerical setting or sourced mathematical factor; groups above |
| 19 | 6 | SYNTHETIC numerical setting or sourced mathematical factor; groups above |
| 19 | 40 | SYNTHETIC numerical setting or sourced mathematical factor; groups above |
| 19 | 32 | SYNTHETIC numerical setting or sourced mathematical factor; groups above |
| 19 | 64 | SYNTHETIC numerical setting or sourced mathematical factor; groups above |
| 19 | 128 | SYNTHETIC numerical setting or sourced mathematical factor; groups above |
| 19 | 256 | SYNTHETIC numerical setting or sourced mathematical factor; groups above |
| 19 | 512 | SYNTHETIC numerical setting or sourced mathematical factor; groups above |
| 20 | 4 | SYNTHETIC numerical setting or sourced mathematical factor; groups above |
| 21 | 2 | SYNTHETIC numerical setting or sourced mathematical factor; groups above |
| 21 | 2 | SYNTHETIC numerical setting or sourced mathematical factor; groups above |
| 21 | 0 | SYNTHETIC numerical setting or sourced mathematical factor; groups above |
| 21 | 2 | SYNTHETIC numerical setting or sourced mathematical factor; groups above |
| 21 | 1 | SYNTHETIC numerical setting or sourced mathematical factor; groups above |
| 21 | 2 | SYNTHETIC numerical setting or sourced mathematical factor; groups above |
| 21 | 2 | SYNTHETIC numerical setting or sourced mathematical factor; groups above |
| 21 | 0 | SYNTHETIC numerical setting or sourced mathematical factor; groups above |
| 21 | 4 | SYNTHETIC numerical setting or sourced mathematical factor; groups above |
| 25 | 4 | SYNTHETIC numerical setting or sourced mathematical factor; groups above |
| 26 | 8 | SYNTHETIC numerical setting or sourced mathematical factor; groups above |
| 26 | 2 | SYNTHETIC numerical setting or sourced mathematical factor; groups above |
| 26 | 2 | SYNTHETIC numerical setting or sourced mathematical factor; groups above |
| 26 | 1 | SYNTHETIC numerical setting or sourced mathematical factor; groups above |
| 26 | 2 | SYNTHETIC numerical setting or sourced mathematical factor; groups above |

## Appendix: exact informational probe

```rust
// Informational experiment: no assertions or changed acceptance.
// All numeric inputs are SYNTHETIC TEST VALUES. Sources: MIT, JVL, NIST, NASA.
use soas_core::units::Radians;
use soas_estimator::{planform::{WingPlanform,WingStation,thin_airfoil_slope_per_rad},lifting_line::{self,StraightWing},vlm::{solve,VlmRequest,SpanwiseSpacing}};
use std::f64::consts::PI;
fn station(y:f64,c:f64,le:f64)->WingStation {WingStation{y_m:y,chord_m:c,leading_edge_x_m:le,z_m:0.,twist_rad:Radians::new(0.).unwrap(),section_lift_curve_slope_per_rad:thin_airfoil_slope_per_rad(),zero_lift_angle_rad:Radians::new(0.).unwrap(),section_source:"SYNTHETIC case; cited ideal thin-airfoil slope".into()}}
fn req(ny:usize,nx:usize)->VlmRequest{VlmRequest{spanwise_panels:ny,chordwise_panels:nx,spanwise_spacing:SpanwiseSpacing::Cosine,alpha_rad:Radians::new(0.05).unwrap(),reference_x_body_m:0.}}
fn main(){
 println!("kind,AR,Ny,CL,CDi,e,slope,target_slope,gap_CL,gap_CD,u_CL,u_CD");
 for ar in [6.,10.,20.,40.] {
  let w=WingPlanform{data_note:"SYNTHETIC subset-fit experiment".into(),stations:vec![station(0.,1.,0.),station(ar/2.,1.,0.)]};
  let fine=solve(&w,req(128,8)).unwrap();let span=solve(&w,req(64,8)).unwrap();let chord=solve(&w,req(128,4)).unwrap();
  let ll=lifting_line::solve(StraightWing::Stations(&w),Radians::new(0.05).unwrap(),128).unwrap();let old=lifting_line::solve(StraightWing::Stations(&w),Radians::new(0.05).unwrap(),64).unwrap();
  let gc=(fine.cl-ll.cl).abs()/ll.cl;let gd=(fine.induced_drag_coefficient-ll.induced_drag_coefficient).abs()/ll.induced_drag_coefficient;
  let uc=((fine.cl-span.cl).abs()+(fine.cl-chord.cl).abs()+(ll.cl-old.cl).abs())/ll.cl;
  let ud=((fine.induced_drag_coefficient-span.induced_drag_coefficient).abs()+(fine.induced_drag_coefficient-chord.induced_drag_coefficient).abs()+(ll.induced_drag_coefficient-old.induced_drag_coefficient).abs())/ll.induced_drag_coefficient;
  println!("fit,{ar},128,{:.17},{:.17},,{:.17},,{gc:.17},{gd:.17},{uc:.17},{ud:.17}",fine.cl,fine.induced_drag_coefficient,fine.d_cl_d_alpha_per_rad);
 }
 for ar in [6.,40.] {for ny in [32,64,128,256,512] {
  let b=ar*PI/4.;
  let mut w=WingPlanform{data_note:"SYNTHETIC area-normalized elliptic polygon; EPSILON tip, not real wing".into(),stations:(ny/2..=ny).map(|j|{let y=if j==ny/2 {0.} else {-b/2.*(PI*j as f64/ny as f64).cos()};let c=(1.-(2.*y/b).powi(2)).max(0.).sqrt().max(f64::EPSILON);station(y,c,c/4.)}).collect()};
  // Fix b and target area b^2/AR across refinement; the polygon approaches the
  // same analytic ellipse. Tiny positive endpoint satisfies the T12 contract.
  let scale=(b*b/ar)/w.geometry().unwrap().area_m2;
  for s in &mut w.stations{s.chord_m*=scale;s.leading_edge_x_m=s.chord_m/4.;}
  let out=solve(&w,req(ny,8)).unwrap();let g=w.geometry().unwrap();let e=out.cl.powi(2)/(PI*g.aspect_ratio*out.induced_drag_coefficient);let target=2.*PI/(1.+2./ar);
  println!("ellipse,{ar},{ny},{:.17},{:.17},{e:.17},{:.17},{target:.17},,,,",out.cl,out.induced_drag_coefficient,out.d_cl_d_alpha_per_rad);
 }}
}
```
