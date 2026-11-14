# T15: rectangular gap versus mesh — experiment, not acceptance

All inputs/counts are labeled **SYNTHETIC TEST VALUES**, never aircraft data.
No production code or acceptance assertion was changed for this experiment.
Cosine FULL-span panels, uniform chord panels, alpha=.05 rad, chord=1 m,
span=AR m, leading-edge x=z=twist=alpha_L0=0, reference=(0,0,0), section
slope=2*pi/rad. T13 always uses 128 odd terms. No speed/density is invented.
The original AR40 64x8 input and expected-difference record are unchanged.

Equations/sources: [MIT Lecture 9](https://web.mit.edu/16.unified/www/SPRING/fluids/Spring2008/LectureNotes/f09.pdf)
for LLT; [Drela JVL](https://web.mit.edu/drela/Public/web/jvl/jvl.pdf), sections
3–6 for lattice/tangency/Trefftz drag. Existing production APIs are called,
not a second implementation. Efficiency=CL^2/(pi*AR*CDi), beta=0. Relative
gaps are abs(VLM-LLT)/abs(LLT); the VLM values here are below LLT.

Wake legs are semi-infinite along -body x (linearized freestream); Trefftz
is the analytic downstream-infinity limit, not a chosen finite distance.
TEMP and TMP were set to checkout/target/batch-temp, with the existing
CARGO_INCREMENTAL=0 setting. No dependency or build workaround was added.
The scratch program used release optimization for large dense matrices.

## AR 40: fixed lifting-line reference

CL=0.29278072785693654, CDi=0.00086244990104846, e=0.79093579657726265.

### Spanwise refinement, Nx=8
| Ny | Nx | CL | CDi | e | Relative CL gap | Relative CDi gap |
| --- | --- | --- | --- | --- | --- | --- |
| 32 | 8 | 0.291806010153 | 0.000791161408 | 0.8564726639 | 0.33291730% | 8.26581260% |
| 64 | 8 | 0.290912248964 | 0.000797787908 | 0.8441637562 | 0.63818370% | 7.49747816% |
| 128 | 8 | 0.290433313208 | 0.000800541980 | 0.8384919231 | 0.80176543% | 7.17814695% |
| 256 | 8 | 0.290185901679 | 0.000801765548 | 0.8357865217 | 0.88626946% | 7.03627571% |
| 512 | 8 | 0.290060228062 | 0.000802337304 | 0.8344676776 | 0.92919360% | 6.96998131% |

### Chordwise refinement, Ny=128
| Ny | Nx | CL | CDi | e | Relative CL gap | Relative CDi gap |
| --- | --- | --- | --- | --- | --- | --- |
| 128 | 4 | 0.290421652680 | 0.000800317704 | 0.8386595501 | 0.80574811% | 7.20415144% |
| 128 | 8 | 0.290433313208 | 0.000800541980 | 0.8384919231 | 0.80176543% | 7.17814695% |
| 128 | 16 | 0.290436040979 | 0.000800595307 | 0.8384518205 | 0.80083375% | 7.17196368% |
| 128 | 32 | 0.290436700195 | 0.000800608349 | 0.8384419688 | 0.80060859% | 7.17045156% |

At the original-style 64x8 mesh the CDi gap is 7.4974781626%. Span refinement 64→512 at Nx=8 removes 0.527496851 percentage points, or 7.035657% of that discrepancy; 92.964343% remains at 512x8. The absolute CDi increase is 4.549396069330e-06.

At Ny=128, chord refinement 8→32 removes 0.007695398 percentage points, equivalent to 0.102640% of the original 64x8 discrepancy. The absolute CDi increase is 6.636895442003e-08. Its final gap is 7.1704515559%. These independent sweeps cannot be added to claim a measured jointly refined 512x32 result.

Informational three-grid Richardson estimate using Ny=128/256/512, Nx=8: observed p=1.097623167, CDi_infinity≈0.000802838836, gap≈6.91182926%. This assumes one leading power law and is NOT a proved limit or error bound; Nx is still fixed.

## AR 10: fixed lifting-line reference

CL=0.25233954248000912, CDi=0.00220096618696582, e=0.92088919915719603.

### Spanwise refinement, Nx=8
| Ny | Nx | CL | CDi | e | Relative CL gap | Relative CDi gap |
| --- | --- | --- | --- | --- | --- | --- |
| 32 | 8 | 0.245297324216 | 0.001936443979 | 0.9890786137 | 2.79077080% | 12.01845851% |
| 64 | 8 | 0.243638151361 | 0.001939402489 | 0.9742552837 | 3.44828679% | 11.88403981% |
| 128 | 8 | 0.242781512490 | 0.001940358073 | 0.9669398765 | 3.78776544% | 11.84062326% |
| 256 | 8 | 0.242346633228 | 0.001940702188 | 0.9633081025 | 3.96010437% | 11.82498851% |
| 512 | 8 | 0.242127612990 | 0.001940841083 | 0.9614989001 | 4.04690022% | 11.81867787% |

### Chordwise refinement, Ny=128
| Ny | Nx | CL | CDi | e | Relative CL gap | Relative CDi gap |
| --- | --- | --- | --- | --- | --- | --- |
| 128 | 4 | 0.242732046538 | 0.001939264838 | 0.9670907724 | 3.80736838% | 11.89029395% |
| 128 | 8 | 0.242781512490 | 0.001940358073 | 0.9669398765 | 3.78776544% | 11.84062326% |
| 128 | 16 | 0.242793323330 | 0.001940622174 | 0.9669023537 | 3.78308491% | 11.82862394% |
| 128 | 32 | 0.242796149163 | 0.001940685896 | 0.9668931122 | 3.78196505% | 11.82572875% |

At the original-style 64x8 mesh the CDi gap is 11.8840398128%. Span refinement 64→512 at Nx=8 removes 0.065361946 percentage points, or 0.549998% of that discrepancy; 99.450002% remains at 512x8. The absolute CDi increase is 1.438594338150e-06.

At Ny=128, chord refinement 8→32 removes 0.014894509 percentage points, equivalent to 0.125332% of the original 64x8 discrepancy. The absolute CDi increase is 3.278231101300e-07. Its final gap is 11.8257287496%. These independent sweeps cannot be added to claim a measured jointly refined 512x32 result.

Informational three-grid Richardson estimate using Ny=128/256/512, Nx=8: observed p=1.308895557, CDi_infinity≈0.001940935089, gap≈11.81440675%. This assumes one leading power law and is NOT a proved limit or error bound; Nx is still fixed.

## What the data support

Most of the original roughly 7.5% AR40 drag gap persists over both refinement
sweeps. The measured change is a minority of it, not evidence that mesh error
accounts for most of the discrepancy. CL moves farther from LLT as span mesh
is refined, while CDi moves closer. The latest measured nonzero gaps and
decreasing increments are consistent with a persistent lifting-surface versus
lifting-line method difference, with residual mesh error. They do not prove
an exact continuum limit, nor rule out every implementation error. These
gaps alone are neither passing nor failing acceptance criteria.

Richardson relation/source: [NASA, Examining Spatial (Grid) Convergence](https://www.grc.nasa.gov/www/wind/valid/tutorial/spatconv.html).
For a doubled mesh and f(h)=f0+C*h^p, p=log2(abs((f_N-f_2N)/(f_2N-f_4N)))
and f0≈f_4N+(f_4N-f_2N)/(2^p-1). Applying that model here is an explicitly
conditional inference; no safety factor, certified uncertainty or pass band
is selected. Coupled span/chord limits and independent AVL remain untested.

## Reproduction and numerical ledger

Save the appendix as target/t15-review/mesh/src/main.rs. Standalone Cargo.toml:
package t15-mesh-experiment, edition 2024, `[workspace]`, path dependencies
soas-estimator and soas-core at ../../../crates/. All numerical inputs are
SYNTHETIC: AR40/10, chord1 m, alpha.05 rad, zero datum/twist/alpha_L0,
Ny32/64/128/256/512, Nx4/8/16/32, LLT128. Factors 2/pi and squares are
sourced mathematical definitions; 17 is output precision only. No additional
numerical acceptance threshold is present.

```powershell
$env:TEMP=(Resolve-Path target/batch-temp).Path
$env:TMP=$env:TEMP
$env:CARGO_INCREMENTAL='0'
cargo run --offline --release --manifest-path target/t15-review/mesh/Cargo.toml
```

| Appendix line | Literal | Provenance |
| --- | --- | --- |
| 10 | 40 | SYNTHETIC experiment input/count or sourced algebraic factor (groups above) |
| 10 | 10 | SYNTHETIC experiment input/count or sourced algebraic factor (groups above) |
| 11 | 0 | SYNTHETIC experiment input/count or sourced algebraic factor (groups above) |
| 11 | 2 | SYNTHETIC experiment input/count or sourced algebraic factor (groups above) |
| 12 | 1 | SYNTHETIC experiment input/count or sourced algebraic factor (groups above) |
| 12 | 0 | SYNTHETIC experiment input/count or sourced algebraic factor (groups above) |
| 12 | 0 | SYNTHETIC experiment input/count or sourced algebraic factor (groups above) |
| 12 | 0 | SYNTHETIC experiment input/count or sourced algebraic factor (groups above) |
| 13 | 0 | SYNTHETIC experiment input/count or sourced algebraic factor (groups above) |
| 14 | 0.05 | SYNTHETIC experiment input/count or sourced algebraic factor (groups above) |
| 14 | 128 | SYNTHETIC experiment input/count or sourced algebraic factor (groups above) |
| 16 | 32 | SYNTHETIC experiment input/count or sourced algebraic factor (groups above) |
| 16 | 64 | SYNTHETIC experiment input/count or sourced algebraic factor (groups above) |
| 16 | 128 | SYNTHETIC experiment input/count or sourced algebraic factor (groups above) |
| 16 | 256 | SYNTHETIC experiment input/count or sourced algebraic factor (groups above) |
| 16 | 512 | SYNTHETIC experiment input/count or sourced algebraic factor (groups above) |
| 16 | 8 | SYNTHETIC experiment input/count or sourced algebraic factor (groups above) |
| 16 | 4 | SYNTHETIC experiment input/count or sourced algebraic factor (groups above) |
| 16 | 8 | SYNTHETIC experiment input/count or sourced algebraic factor (groups above) |
| 16 | 16 | SYNTHETIC experiment input/count or sourced algebraic factor (groups above) |
| 16 | 32 | SYNTHETIC experiment input/count or sourced algebraic factor (groups above) |
| 16 | 128 | SYNTHETIC experiment input/count or sourced algebraic factor (groups above) |
| 17 | 0 | SYNTHETIC experiment input/count or sourced algebraic factor (groups above) |
| 17 | 2 | SYNTHETIC experiment input/count or sourced algebraic factor (groups above) |

## Appendix: exact scratch program, no acceptance assertions

```rust
// Experiment only, NO acceptance assertions. All inputs/counts SYNTHETIC.
// Source: existing VLM/JVL and T13/MIT Lecture 9; ledger in vlm_gap_vs_mesh.md.
use soas_core::units::Radians;
use soas_estimator::{planform::{WingPlanform,WingStation,thin_airfoil_slope_per_rad},
 lifting_line::{self,StraightWing},vlm::{solve,VlmRequest,SpanwiseSpacing}};
use std::collections::BTreeMap;
use std::f64::consts::PI;
fn main() {
 println!("AR,series,Ny,Nx,CL,CDi,e,LLT_CL,LLT_CDi,LLT_e,relative_CL_gap,relative_CDi_gap");
 for ar in [40.,10.] {
  let w=WingPlanform{data_note:"SYNTHETIC mesh experiment; no real aircraft".into(),stations:[0.,ar/2.].into_iter().map(|y|WingStation{
   y_m:y,chord_m:1.,leading_edge_x_m:0.,z_m:0.,twist_rad:Radians::new(0.).unwrap(),
   section_lift_curve_slope_per_rad:thin_airfoil_slope_per_rad(),zero_lift_angle_rad:Radians::new(0.).unwrap(),section_source:"MIT/Caughey ideal thin-airfoil input; synthetic test".into()}).collect()};
  let alpha=Radians::new(0.05).unwrap();let ll=lifting_line::solve(StraightWing::Stations(&w),alpha,128).unwrap();
  let mut cache=BTreeMap::new();
  for (series,ny,nx) in [32,64,128,256,512].into_iter().map(|ny|("span",ny,8)).chain([4,8,16,32].into_iter().map(|nx|("chord",128,nx))) {
   let (cl,cd,e)=*cache.entry((ny,nx)).or_insert_with(||{let out=solve(&w,VlmRequest{spanwise_panels:ny,chordwise_panels:nx,spanwise_spacing:SpanwiseSpacing::Cosine,alpha_rad:alpha,reference_x_body_m:0.}).unwrap();(out.cl,out.induced_drag_coefficient,out.cl.powi(2)/(PI*ar*out.induced_drag_coefficient))});
   println!("{ar},{series},{ny},{nx},{cl:.17},{cd:.17},{e:.17},{:.17},{:.17},{:.17},{:.17},{:.17}",ll.cl,ll.induced_drag_coefficient,ll.span_efficiency.unwrap(),(cl-ll.cl).abs()/ll.cl.abs(),(cd-ll.induced_drag_coefficient).abs()/ll.induced_drag_coefficient.abs());
  }
 }
}
```
