# Synthetic VLM trend settings ledger

Every numeric value is a labeled SYNTHETIC TEST setting, not aircraft data. The method quantities are computed, not expected output fitted to the implementation.

| JSON path | Value | Purpose/provenance |
|---|---|---|
| `/aspect_ratios/0` | `6.0` | Owner-requested SYNTHETIC rectangular AR sweep |
| `/aspect_ratios/1` | `10.0` | Owner-requested SYNTHETIC rectangular AR sweep |
| `/aspect_ratios/2` | `20.0` | Owner-requested SYNTHETIC rectangular AR sweep |
| `/aspect_ratios/3` | `40.0` | Owner-requested SYNTHETIC rectangular AR sweep |
| `/alpha_rad` | `0.05` | SYNTHETIC operating angle, same .05 rad as original comparison |
| `/chord_m` | `1.0` | SYNTHETIC geometry scale, span=AR*chord |
| `/fine_span_panels` | `128` | SYNTHETIC numerical mesh/term count or hand spacing geometry, specified before spacing implementation; see vlm_diagnosis.md |
| `/coarse_span_panels` | `64` | SYNTHETIC numerical mesh/term count or hand spacing geometry, specified before spacing implementation; see vlm_diagnosis.md |
| `/fine_chord_panels` | `8` | SYNTHETIC numerical mesh/term count or hand spacing geometry, specified before spacing implementation; see vlm_diagnosis.md |
| `/coarse_chord_panels` | `4` | SYNTHETIC numerical mesh/term count or hand spacing geometry, specified before spacing implementation; see vlm_diagnosis.md |
| `/fine_fourier_terms` | `128` | SYNTHETIC numerical mesh/term count or hand spacing geometry, specified before spacing implementation; see vlm_diagnosis.md |
| `/coarse_fourier_terms` | `64` | SYNTHETIC numerical mesh/term count or hand spacing geometry, specified before spacing implementation; see vlm_diagnosis.md |
| `/spacing_case_span_m` | `8.0` | SYNTHETIC numerical mesh/term count or hand spacing geometry, specified before spacing implementation; see vlm_diagnosis.md |
| `/spacing_case_span_panels` | `4` | SYNTHETIC numerical mesh/term count or hand spacing geometry, specified before spacing implementation; see vlm_diagnosis.md |
| `/spacing_case_chord_panels` | `1` | SYNTHETIC numerical mesh/term count or hand spacing geometry, specified before spacing implementation; see vlm_diagnosis.md |
| `/coordinate_tolerance_m` | `1e-12` | SYNTHETIC coordinate-roundoff check, same existing 1e-12 bound; not a method-difference allowance |
| `/nonzero_floor_fault` | `0.02` | SYNTHETIC deliberately wrong asymptotic floor, analytic delta=.02+.6/AR |
| `/inverse_ar_fault_scale` | `0.6` | SYNTHETIC analytic decreasing part of nonzero-floor fault |
