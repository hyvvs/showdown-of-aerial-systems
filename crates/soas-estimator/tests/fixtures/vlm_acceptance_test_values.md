# VLM numerical test-setting ledger

Every numeric literal below is a labeled SYNTHETIC TEST input/setting, a hand-derived expectation, or a sourced algebraic constant. None represents real aircraft data. Physical formulas: data/reference/estimator/vlm.md and vlm_diagnosis.md.

Original AR40 comparison and original ledger remain in 8067b1d; its full function and measured failure remain in data/reference/estimator/vlm_expected_difference.md. No original allowance was widened.

New trend bounds are computed from mesh changes and leave-one-out intercept sensitivity with no tuned multiplier. NIST normal-equation source: https://www.itl.nist.gov/div898/handbook/pmd/section4/pmd431.htm . The inverse-log drag extrapolation is an explicitly conditional hypothesis, not a verified physical bound.

| Source location | Literal | Provenance |
|---|---|---|
| `tests/vlm_acceptance.rs:24` | `0.0` | SYNTHETIC test/mesh/roundoff/index setting or hand-derived/sourced mathematical factor; fixture-specific settings are in vlm_trend_values.md. |
| `tests/vlm_acceptance.rs:24` | `2.0` | SYNTHETIC test/mesh/roundoff/index setting or hand-derived/sourced mathematical factor; fixture-specific settings are in vlm_trend_values.md. |
| `tests/vlm_acceptance.rs:29` | `0.0` | SYNTHETIC test/mesh/roundoff/index setting or hand-derived/sourced mathematical factor; fixture-specific settings are in vlm_trend_values.md. |
| `tests/vlm_acceptance.rs:30` | `0.0` | SYNTHETIC test/mesh/roundoff/index setting or hand-derived/sourced mathematical factor; fixture-specific settings are in vlm_trend_values.md. |
| `tests/vlm_acceptance.rs:31` | `0.0` | SYNTHETIC test/mesh/roundoff/index setting or hand-derived/sourced mathematical factor; fixture-specific settings are in vlm_trend_values.md. |
| `tests/vlm_acceptance.rs:33` | `0.0` | SYNTHETIC test/mesh/roundoff/index setting or hand-derived/sourced mathematical factor; fixture-specific settings are in vlm_trend_values.md. |
| `tests/vlm_acceptance.rs:45` | `0.0` | SYNTHETIC test/mesh/roundoff/index setting or hand-derived/sourced mathematical factor; fixture-specific settings are in vlm_trend_values.md. |
| `tests/vlm_acceptance.rs:56` | `2` | SYNTHETIC test/mesh/roundoff/index setting or hand-derived/sourced mathematical factor; fixture-specific settings are in vlm_trend_values.md. |
| `tests/vlm_acceptance.rs:63` | `1.0` | SYNTHETIC test/mesh/roundoff/index setting or hand-derived/sourced mathematical factor; fixture-specific settings are in vlm_trend_values.md. |
| `tests/vlm_acceptance.rs:73` | `0` | SYNTHETIC test/mesh/roundoff/index setting or hand-derived/sourced mathematical factor; fixture-specific settings are in vlm_trend_values.md. |
| `tests/vlm_acceptance.rs:87` | `0` | SYNTHETIC test/mesh/roundoff/index setting or hand-derived/sourced mathematical factor; fixture-specific settings are in vlm_trend_values.md. |
| `tests/vlm_acceptance.rs:89` | `0.0` | SYNTHETIC test/mesh/roundoff/index setting or hand-derived/sourced mathematical factor; fixture-specific settings are in vlm_trend_values.md. |
| `tests/vlm_acceptance.rs:143` | `2` | SYNTHETIC test/mesh/roundoff/index setting or hand-derived/sourced mathematical factor; fixture-specific settings are in vlm_trend_values.md. |
| `tests/vlm_acceptance.rs:169` | `0` | SYNTHETIC test/mesh/roundoff/index setting or hand-derived/sourced mathematical factor; fixture-specific settings are in vlm_trend_values.md. |
| `tests/vlm_acceptance.rs:171` | `0.0` | SYNTHETIC test/mesh/roundoff/index setting or hand-derived/sourced mathematical factor; fixture-specific settings are in vlm_trend_values.md. |
| `tests/vlm_acceptance.rs:172` | `2` | SYNTHETIC test/mesh/roundoff/index setting or hand-derived/sourced mathematical factor; fixture-specific settings are in vlm_trend_values.md. |
| `tests/vlm_acceptance.rs:172` | `2` | SYNTHETIC test/mesh/roundoff/index setting or hand-derived/sourced mathematical factor; fixture-specific settings are in vlm_trend_values.md. |
| `tests/vlm_acceptance.rs:173` | `1` | SYNTHETIC test/mesh/roundoff/index setting or hand-derived/sourced mathematical factor; fixture-specific settings are in vlm_trend_values.md. |
| `tests/vlm_acceptance.rs:173` | `0` | SYNTHETIC test/mesh/roundoff/index setting or hand-derived/sourced mathematical factor; fixture-specific settings are in vlm_trend_values.md. |
| `tests/vlm_acceptance.rs:174` | `0` | SYNTHETIC test/mesh/roundoff/index setting or hand-derived/sourced mathematical factor; fixture-specific settings are in vlm_trend_values.md. |
| `tests/vlm_acceptance.rs:174` | `1` | SYNTHETIC test/mesh/roundoff/index setting or hand-derived/sourced mathematical factor; fixture-specific settings are in vlm_trend_values.md. |
| `tests/vlm_acceptance.rs:174` | `1` | SYNTHETIC test/mesh/roundoff/index setting or hand-derived/sourced mathematical factor; fixture-specific settings are in vlm_trend_values.md. |
| `tests/vlm_acceptance.rs:174` | `0` | SYNTHETIC test/mesh/roundoff/index setting or hand-derived/sourced mathematical factor; fixture-specific settings are in vlm_trend_values.md. |
| `tests/vlm_acceptance.rs:175` | `0.0` | SYNTHETIC test/mesh/roundoff/index setting or hand-derived/sourced mathematical factor; fixture-specific settings are in vlm_trend_values.md. |
| `tests/vlm_acceptance.rs:180` | `1` | SYNTHETIC test/mesh/roundoff/index setting or hand-derived/sourced mathematical factor; fixture-specific settings are in vlm_trend_values.md. |
| `tests/vlm_acceptance.rs:182` | `0.0` | SYNTHETIC test/mesh/roundoff/index setting or hand-derived/sourced mathematical factor; fixture-specific settings are in vlm_trend_values.md. |
| `tests/vlm_acceptance.rs:189` | `0` | SYNTHETIC test/mesh/roundoff/index setting or hand-derived/sourced mathematical factor; fixture-specific settings are in vlm_trend_values.md. |
| `tests/vlm_acceptance.rs:189` | `1.0` | SYNTHETIC test/mesh/roundoff/index setting or hand-derived/sourced mathematical factor; fixture-specific settings are in vlm_trend_values.md. |
| `tests/vlm_acceptance.rs:189` | `1.0` | SYNTHETIC test/mesh/roundoff/index setting or hand-derived/sourced mathematical factor; fixture-specific settings are in vlm_trend_values.md. |
| `tests/vlm_acceptance.rs:205` | `1.0` | SYNTHETIC test/mesh/roundoff/index setting or hand-derived/sourced mathematical factor; fixture-specific settings are in vlm_trend_values.md. |
| `tests/vlm_acceptance.rs:213` | `0.0` | SYNTHETIC test/mesh/roundoff/index setting or hand-derived/sourced mathematical factor; fixture-specific settings are in vlm_trend_values.md. |
| `tests/vlm_acceptance.rs:214` | `2` | SYNTHETIC test/mesh/roundoff/index setting or hand-derived/sourced mathematical factor; fixture-specific settings are in vlm_trend_values.md. |
| `tests/vlm_acceptance.rs:214` | `1` | SYNTHETIC test/mesh/roundoff/index setting or hand-derived/sourced mathematical factor; fixture-specific settings are in vlm_trend_values.md. |
| `tests/vlm_acceptance.rs:214` | `0` | SYNTHETIC test/mesh/roundoff/index setting or hand-derived/sourced mathematical factor; fixture-specific settings are in vlm_trend_values.md. |
| `tests/vlm_acceptance.rs:241` | `4.0` | SYNTHETIC test/mesh/roundoff/index setting or hand-derived/sourced mathematical factor; fixture-specific settings are in vlm_trend_values.md. |
| `tests/vlm_acceptance.rs:241` | `2.0` | SYNTHETIC test/mesh/roundoff/index setting or hand-derived/sourced mathematical factor; fixture-specific settings are in vlm_trend_values.md. |
| `tests/vlm_acceptance.rs:241` | `0.0` | SYNTHETIC test/mesh/roundoff/index setting or hand-derived/sourced mathematical factor; fixture-specific settings are in vlm_trend_values.md. |
| `tests/vlm_acceptance.rs:241` | `2.0` | SYNTHETIC test/mesh/roundoff/index setting or hand-derived/sourced mathematical factor; fixture-specific settings are in vlm_trend_values.md. |
| `tests/vlm_acceptance.rs:241` | `4.0` | SYNTHETIC test/mesh/roundoff/index setting or hand-derived/sourced mathematical factor; fixture-specific settings are in vlm_trend_values.md. |
| `tests/vlm_acceptance.rs:242` | `4.0` | SYNTHETIC test/mesh/roundoff/index setting or hand-derived/sourced mathematical factor; fixture-specific settings are in vlm_trend_values.md. |
| `tests/vlm_acceptance.rs:242` | `2.0` | SYNTHETIC test/mesh/roundoff/index setting or hand-derived/sourced mathematical factor; fixture-specific settings are in vlm_trend_values.md. |
| `tests/vlm_acceptance.rs:242` | `2.0_f64` | SYNTHETIC test/mesh/roundoff/index setting or hand-derived/sourced mathematical factor; fixture-specific settings are in vlm_trend_values.md. |
| `tests/vlm_acceptance.rs:242` | `0.0` | SYNTHETIC test/mesh/roundoff/index setting or hand-derived/sourced mathematical factor; fixture-specific settings are in vlm_trend_values.md. |
| `tests/vlm_acceptance.rs:242` | `2.0` | SYNTHETIC test/mesh/roundoff/index setting or hand-derived/sourced mathematical factor; fixture-specific settings are in vlm_trend_values.md. |
| `tests/vlm_acceptance.rs:242` | `2.0_f64` | SYNTHETIC test/mesh/roundoff/index setting or hand-derived/sourced mathematical factor; fixture-specific settings are in vlm_trend_values.md. |
| `tests/vlm_acceptance.rs:242` | `4.0` | SYNTHETIC test/mesh/roundoff/index setting or hand-derived/sourced mathematical factor; fixture-specific settings are in vlm_trend_values.md. |
| `tests/vlm_acceptance.rs:249` | `1` | SYNTHETIC test/mesh/roundoff/index setting or hand-derived/sourced mathematical factor; fixture-specific settings are in vlm_trend_values.md. |
