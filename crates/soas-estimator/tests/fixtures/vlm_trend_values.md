# VLM acceptance synthetic numerical ledger

Every numeric entry below is a SYNTHETIC TEST input or numerical setting, not aircraft data. The owner authorized replacing decreasing LLT-slope error with decreasing successive VLM mesh changes. No existing tolerance was widened.

Rectangles use both 64x8 and 128x8; ellipses use a declared single chordwise panel so full quality gates remain tractable. The separate Nx8 experiment is informational in vlm_subset_fits.md. Five doubled span meshes provide a Richardson estimate held out from a band derived from the first four. The band tests contraction of numerical estimates, not physical agreement with e=1. Machine EPSILON regularizes only the positive station-tip contract; ellipse polygons are area-normalized.

| JSON path | Value | Provenance |
|---|---|---|
| `/aspect_ratios/0` | `6.0` | SYNTHETIC benchmark geometry, operating angle, mesh/term count, or unchanged coordinate-roundoff check. |
| `/aspect_ratios/1` | `10.0` | SYNTHETIC benchmark geometry, operating angle, mesh/term count, or unchanged coordinate-roundoff check. |
| `/aspect_ratios/2` | `20.0` | SYNTHETIC benchmark geometry, operating angle, mesh/term count, or unchanged coordinate-roundoff check. |
| `/aspect_ratios/3` | `40.0` | SYNTHETIC benchmark geometry, operating angle, mesh/term count, or unchanged coordinate-roundoff check. |
| `/alpha_rad` | `0.05` | SYNTHETIC benchmark geometry, operating angle, mesh/term count, or unchanged coordinate-roundoff check. |
| `/chord_m` | `1.0` | SYNTHETIC benchmark geometry, operating angle, mesh/term count, or unchanged coordinate-roundoff check. |
| `/fine_span_panels` | `128` | SYNTHETIC benchmark geometry, operating angle, mesh/term count, or unchanged coordinate-roundoff check. |
| `/coarse_span_panels` | `64` | SYNTHETIC benchmark geometry, operating angle, mesh/term count, or unchanged coordinate-roundoff check. |
| `/fine_chord_panels` | `8` | SYNTHETIC benchmark geometry, operating angle, mesh/term count, or unchanged coordinate-roundoff check. |
| `/fine_fourier_terms` | `128` | SYNTHETIC benchmark geometry, operating angle, mesh/term count, or unchanged coordinate-roundoff check. |
| `/spacing_case_span_m` | `8.0` | SYNTHETIC benchmark geometry, operating angle, mesh/term count, or unchanged coordinate-roundoff check. |
| `/spacing_case_span_panels` | `4` | SYNTHETIC benchmark geometry, operating angle, mesh/term count, or unchanged coordinate-roundoff check. |
| `/spacing_case_chord_panels` | `1` | SYNTHETIC benchmark geometry, operating angle, mesh/term count, or unchanged coordinate-roundoff check. |
| `/coordinate_tolerance_m` | `1e-12` | SYNTHETIC benchmark geometry, operating angle, mesh/term count, or unchanged coordinate-roundoff check. |
| `/ellipse_benchmarks/aspect_ratios/0` | `6.0` | SYNTHETIC benchmark geometry, operating angle, mesh/term count, or unchanged coordinate-roundoff check. |
| `/ellipse_benchmarks/aspect_ratios/1` | `40.0` | SYNTHETIC benchmark geometry, operating angle, mesh/term count, or unchanged coordinate-roundoff check. |
| `/ellipse_benchmarks/full_span_panels/0` | `32` | SYNTHETIC benchmark geometry, operating angle, mesh/term count, or unchanged coordinate-roundoff check. |
| `/ellipse_benchmarks/full_span_panels/1` | `64` | SYNTHETIC benchmark geometry, operating angle, mesh/term count, or unchanged coordinate-roundoff check. |
| `/ellipse_benchmarks/full_span_panels/2` | `128` | SYNTHETIC benchmark geometry, operating angle, mesh/term count, or unchanged coordinate-roundoff check. |
| `/ellipse_benchmarks/full_span_panels/3` | `256` | SYNTHETIC benchmark geometry, operating angle, mesh/term count, or unchanged coordinate-roundoff check. |
| `/ellipse_benchmarks/full_span_panels/4` | `512` | SYNTHETIC benchmark geometry, operating angle, mesh/term count, or unchanged coordinate-roundoff check. |
| `/ellipse_benchmarks/chordwise_panels` | `1` | SYNTHETIC benchmark geometry, operating angle, mesh/term count, or unchanged coordinate-roundoff check. |
| `/ellipse_benchmarks/root_chord_m` | `1.0` | SYNTHETIC benchmark geometry, operating angle, mesh/term count, or unchanged coordinate-roundoff check. |

## T15b added settings

All entries are SYNTHETIC TEST settings or sourced mathematical definitions. No real-wing values.

| JSON path / use | Value | Provenance |
|---|---|---|
| `/ellipse_bounds/aspect_ratios/0` | `6.0` | SYNTHETIC owner-specified T15b geometry/mesh setting. |
| `/ellipse_bounds/aspect_ratios/1` | `40.0` | SYNTHETIC owner-specified T15b geometry/mesh setting. |
| `/ellipse_bounds/chordwise_panels/0` | `1` | SYNTHETIC owner-specified T15b geometry/mesh setting. |
| `/ellipse_bounds/chordwise_panels/1` | `8` | SYNTHETIC owner-specified T15b geometry/mesh setting. |
| `/ellipse_bounds/full_span_panels/0` | `128` | SYNTHETIC owner-specified T15b geometry/mesh setting. |
| `/ellipse_bounds/full_span_panels/1` | `256` | SYNTHETIC owner-specified T15b geometry/mesh setting. |
| `/ellipse_bounds/full_span_panels/2` | `512` | SYNTHETIC owner-specified T15b geometry/mesh setting. |
| `/ellipse_bounds/full_span_panels/3` | `1024` | SYNTHETIC owner-specified T15b geometry/mesh setting. |
| `/ellipse_bounds/root_chord_m` | `1.0` | SYNTHETIC owner-specified T15b geometry/mesh setting. |
| Exact Munk efficiency upper bound | `1.0` | Mathematical planar fixed-lift/span minimum-drag bound, NACA-TR-121 / MIT Lecture 9. NO tolerance. |
| Scratch drag mutation | `0.95` | Owner-prescribed SYNTHETIC FAULT multiplier, never production/fixture calibration. |
| Test-build optimization | `3` | Owner-approved compiler optimization level, not a numerical acceptance setting. |
