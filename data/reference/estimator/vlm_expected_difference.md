# Original AR40 comparison — expected-difference record

This is a historical measurement, NOT a passing acceptance assertion. The
original tests-only commit 8067b1d retains the active assertion in history.
The owner authorized its replacement by an AR trend test on 2026-11-14.
No old allowance was widened and the original inputs were not changed.

Synthetic original inputs: AR=40, alpha=.05 rad, VLM cosine 64x8; mesh checks
32x8 and 64x4; T13 128 and 64 odd terms; rectangle chord=1 m, span=40 m,
zero twist/alpha_L0, ideal section 2*pi/rad, reference origin. Source and
numeric-setting ledger: vlm_diagnosis.md and tests/fixtures/vlm_test_values.md
in the original commit. All are synthetic test settings, not aircraft data.

| Quantity | VLM | LLT | Absolute gap | Relative gap / LLT | Former allowance |
|---|---|---|---|---|---|
| CL | 0.2909122489637976 | 0.29278072785693654 | 0.001868478893139 | 0.006381837038303854 | 0.001258745458735 |
| CDi | 0.0007977879080538741 | 0.0008624499010484641 | 0.00006466199299459 | 0.07497478162613463 | 0.000008910394977062 |

Both old comparisons fail. These are derived baseline measurements, not
invented fixture inputs or acceptance tolerances. The exact original function
below is retained as a documentation record; its assertions are deliberately
not executed as current acceptance. Diagnosis/source limits and new acceptance
derivation: vlm_diagnosis.md. No expected output is fabricated from AVL.

## Original function, verbatim

```rust
#[test]
fn high_ar_agreement_uses_predeclared_mesh_and_published_method_difference_budget() {
    let w = wing("high_ar");
    let fine = solve(&w, request(64, 8, 0.05)).unwrap();
    let span_coarse = solve(&w, request(32, 8, 0.05)).unwrap();
    let chord_coarse = solve(&w, request(64, 4, 0.05)).unwrap();
    let ll = lifting_line::solve(StraightWing::Stations(&w), rad(0.05), 128).unwrap();
    let ll_coarse = lifting_line::solve(StraightWing::Stations(&w), rad(0.05), 64).unwrap();
    // This rule is committed BEFORE VLM implementation/measurement. Do not
    // substitute the actual inter-method discrepancy into the allowance.
    // Caughey eqs 2.19 (e=1) vs 2.21 estimate omitted finite-chord/AR effects.
    // It is an approximation allowance, not a rigorous physical error bound.
    let ar = 40.0;
    let a0 = 2.0 * PI;
    let a_line = a0 / (1.0 + a0 / (PI * ar));
    let a_surface = PI * ar / (1.0 + (1.0 + (PI * ar / a0).powi(2)).sqrt());
    let model_cl = (a_line - a_surface).abs() * 0.05;
    let mesh_cl = (fine.cl - span_coarse.cl).abs()
        + (fine.cl - chord_coarse.cl).abs()
        + (ll.cl - ll_coarse.cl).abs();
    // Propagate CL allowance through CDi=CL^2/(pi*AR*e), holding LL e fixed;
    // remaining finite-mesh span-loading differences are measured separately.
    let model_cd =
        (2.0 * ll.cl.abs() + model_cl) * model_cl / (PI * ar * ll.span_efficiency.unwrap());
    let mesh_cd = (fine.induced_drag_coefficient - span_coarse.induced_drag_coefficient).abs()
        + (fine.induced_drag_coefficient - chord_coarse.induced_drag_coefficient).abs()
        + (ll.induced_drag_coefficient - ll_coarse.induced_drag_coefficient).abs();
    let delta_cl = (fine.cl - ll.cl).abs();
    let delta_cd = (fine.induced_drag_coefficient - ll.induced_drag_coefficient).abs();
    println!(
        "VLM/LLT AR40: VLM=({},{}), LLT=({},{}), differences=({delta_cl:.12e},{delta_cd:.12e}), mesh=({mesh_cl:.12e},{mesh_cd:.12e}), published-model=({model_cl:.12e},{model_cd:.12e})",
        fine.cl, fine.induced_drag_coefficient, ll.cl, ll.induced_drag_coefficient
    );
    assert!(
        delta_cl <= mesh_cl + model_cl,
        "CL discrepancy exceeds predeclared budget; STOP rather than widen"
    );
    assert!(
        delta_cd <= mesh_cd + model_cd,
        "CDi discrepancy exceeds predeclared budget; STOP rather than widen"
    );
}
```
