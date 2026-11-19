# T15b production comparison: numerical ledger

Configuration entries are labelled SYNTHETIC TEST VALUES/settings. Output entries are measured unmodified Rust-solver observations or arithmetic comparisons, not new acceptance constants. Archived values come from the existing independent efficiency ledger. Resource guards reuse the disclosed profile-study limits; no new tolerance is introduced.

| JSON path | Value | Origin |
|---|---|---|
| `/config/cases/0/0` | `10` | SYNTHETIC configured setting |
| `/config/cases/0/1` | `128` | SYNTHETIC configured setting |
| `/config/cases/1/0` | `20` | SYNTHETIC configured setting |
| `/config/cases/1/1` | `256` | SYNTHETIC configured setting |
| `/config/cases/2/0` | `40` | SYNTHETIC configured setting |
| `/config/cases/2/1` | `512` | SYNTHETIC configured setting |
| `/config/cases/3/0` | `80` | SYNTHETIC configured setting |
| `/config/cases/3/1` | `1024` | SYNTHETIC configured setting |
| `/config/nx` | `8` | SYNTHETIC configured setting |
| `/config/alpha_rad` | `0.05` | SYNTHETIC configured setting |
| `/config/chord_m` | `1` | SYNTHETIC configured setting |
| `/config/zero_twist_rad` | `0` | SYNTHETIC configured setting |
| `/config/zero_lift_angle_rad` | `0` | SYNTHETIC configured setting |
| `/config/leading_edge_x_m` | `0` | SYNTHETIC configured setting |
| `/config/z_m` | `0` | SYNTHETIC configured setting |
| `/config/reference_x_m` | `0` | SYNTHETIC configured setting |
| `/config/wall_guard_s` | `240` | SYNTHETIC configured setting |
| `/config/memory_guard_bytes` | `2000000000` | SYNTHETIC configured setting |
| `/config/poll_ms` | `100` | SYNTHETIC configured setting |
| `/config/diagnostic_decimal_places` | `17` | SYNTHETIC configured setting |
| `/production_comparisons/0/ar` | `10` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/0/ny` | `128` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/0/nx` | `8` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/0/outputs/CL` | `0.24278151248986285` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/0/outputs/CDi` | `0.0019403580727142035` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/0/outputs/e` | `0.9669398764711201` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/0/outputs/Cm` | `-0.05924832355536973` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/0/outputs/solve_residual` | `1.6294641276419656e-17` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/0/archived_e` | `0.966939876471118` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/0/owner_rounded_e` | `0.96694` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/0/signed_difference` | `2.1094237467877974e-15` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/0/relative_difference` | `2.181545924536917e-15` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/0/difference_from_owner_rounded` | `-1.2352887990285666e-07` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/0/measurement/wall_s` | `0.2062457` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/0/measurement/peak_working_set_bytes` | `23212032` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/0/measurement/peak_paged_commit_bytes` | `18083840` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/0/measurement/maximum_observed_private_bytes` | `18055168` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/0/measurement/poll_ms` | `100` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/0/measurement/wall_limit_s` | `240` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/0/measurement/memory_limit_bytes` | `2000000000` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/0/measurement/exit_code` | `0` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/1/ar` | `20` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/1/ny` | `256` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/1/nx` | `8` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/1/outputs/CL` | `0.27200919978633215` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/1/outputs/CDi` | `0.0012972385817023036` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/1/outputs/e` | `0.9077525144132002` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/1/outputs/Cm` | `-0.06724804437919692` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/1/outputs/solve_residual` | `2.9510667587109e-17` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/1/archived_e` | `0.9077525144131966` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/1/owner_rounded_e` | `0.907753` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/1/signed_difference` | `3.552713678800501e-15` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/1/relative_difference` | `3.913746998648746e-15` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/1/difference_from_owner_rounded` | `-4.855867998809771e-07` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/1/measurement/wall_s` | `0.8937766` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/1/measurement/peak_working_set_bytes` | `73560064` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/1/measurement/peak_paged_commit_bytes` | `68562944` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/1/measurement/maximum_observed_private_bytes` | `68562944` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/1/measurement/poll_ms` | `100` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/1/measurement/wall_limit_s` | `240` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/1/measurement/memory_limit_bytes` | `2000000000` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/1/measurement/exit_code` | `0` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/2/ar` | `40` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/2/ny` | `512` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/2/nx` | `8` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/2/outputs/CL` | `0.2900602280623587` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/2/outputs/CDi` | `0.0008023373041232016` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/2/outputs/e` | `0.8344676775566939` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/2/outputs/Cm` | `-0.07213028519085818` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/2/outputs/solve_residual` | `1.3612015724309473e-16` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/2/archived_e` | `0.8344676775566786` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/2/owner_rounded_e` | `0.834468` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/2/signed_difference` | `1.532107773982716e-14` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/2/relative_difference` | `1.836030100613037e-14` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/2/difference_from_owner_rounded` | `-3.2244330605557536e-07` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/2/measurement/wall_s` | `9.4014636` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/2/measurement/peak_working_set_bytes` | `274853888` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/2/measurement/peak_paged_commit_bytes` | `270647296` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/2/measurement/maximum_observed_private_bytes` | `270647296` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/2/measurement/poll_ms` | `100` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/2/measurement/wall_limit_s` | `240` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/2/measurement/memory_limit_bytes` | `2000000000` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/2/measurement/exit_code` | `0` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/3/ar` | `80` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/3/ny` | `1024` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/3/nx` | `8` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/3/outputs/CL` | `0.3006598409603415` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/3/outputs/CDi` | `0.0004744745190621125` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/3/outputs/e` | `0.7580504202570456` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/3/outputs/Cm` | `-0.074970569411375` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/3/outputs/solve_residual` | `2.0341208309211289e-16` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/3/archived_e` | `0.7580504202570049` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/3/owner_rounded_e` | `0.75805` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/3/signed_difference` | `4.0745185003743245e-14` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/3/relative_difference` | `5.3749966908440264e-14` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/3/difference_from_owner_rounded` | `4.202570456079968e-07` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/3/measurement/wall_s` | `76.8736739` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/3/measurement/peak_working_set_bytes` | `1080778752` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/3/measurement/peak_paged_commit_bytes` | `1078120448` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/3/measurement/maximum_observed_private_bytes` | `1078120448` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/3/measurement/poll_ms` | `100` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/3/measurement/wall_limit_s` | `240` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/3/measurement/memory_limit_bytes` | `2000000000` | Measured result / cited archived value / arithmetic derivation |
| `/production_comparisons/3/measurement/exit_code` | `0` | Measured result / cited archived value / arithmetic derivation |
| `/capacity_limit/ar` | `160` | Measured result / cited archived value / arithmetic derivation |
| `/capacity_limit/ny` | `2048` | Measured result / cited archived value / arithmetic derivation |
| `/capacity_limit/nx` | `8` | Measured result / cited archived value / arithmetic derivation |
| `/capacity_limit/panels` | `16384` | Measured result / cited archived value / arithmetic derivation |
| `/capacity_limit/f64_bytes` | `8` | Measured result / cited archived value / arithmetic derivation |
| `/capacity_limit/dense_matrix_count` | `2` | Measured result / cited archived value / arithmetic derivation |
| `/capacity_limit/one_matrix_bytes` | `2147483648` | Measured result / cited archived value / arithmetic derivation |
| `/capacity_limit/two_matrix_bytes` | `4294967296` | Measured result / cited archived value / arithmetic derivation |
| `/summary_derived/ar80_two_matrix_bytes` | `1073741824` | Measured result / cited archived value / arithmetic derivation |
| `/summary_derived/maximum_absolute_relative_e_difference` | `5.3749966908440264e-14` | Measured result / cited archived value / arithmetic derivation |
| `/summary_derived/upward_rounded_maximum_relative_e_difference` | `5.38e-14` | Measured result / cited archived value / arithmetic derivation |
| `/summary_derived/rounded_reference_decimal_places` | `6` | Measured result / cited archived value / arithmetic derivation |
