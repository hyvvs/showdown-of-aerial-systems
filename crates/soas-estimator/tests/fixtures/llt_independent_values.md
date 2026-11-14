# Independent LLT test-value ledger

All entries are labeled SYNTHETIC TEST VALUES/settings, not a real wing. Expected constants were provided by the owner and reproduced by a separate Python Gaussian-elimination solver from MIT Lecture 9 equations. The 2e-12 absolute tolerance exceeds printed rounding plus 64*128*binary64 epsilon (about 1.82e-12); it is a numerical guard, not a rigorous condition-number bound. The smallest CDi gives relative allowance below 1.8e-10.

| JSON path | Value | Provenance |
|---|---|---|
| `/span_m` | `12.0` | SYNTHETIC owner-specified input, independently solved reference or declared numerical guard. |
| `/chord_m` | `2.0` | SYNTHETIC owner-specified input, independently solved reference or declared numerical guard. |
| `/alpha_rad` | `0.08` | SYNTHETIC owner-specified input, independently solved reference or declared numerical guard. |
| `/zero_lift_angle_rad` | `-0.02` | SYNTHETIC owner-specified input, independently solved reference or declared numerical guard. |
| `/absolute_tolerance` | `2e-12` | SYNTHETIC owner-specified input, independently solved reference or declared numerical guard. |
| `/checks/0/odd_terms` | `4` | SYNTHETIC owner-specified input, independently solved reference or declared numerical guard. |
| `/checks/0/cl` | `0.452728040191428` | SYNTHETIC owner-specified input, independently solved reference or declared numerical guard. |
| `/checks/0/cdi` | `0.011377841172798` | SYNTHETIC owner-specified input, independently solved reference or declared numerical guard. |
| `/checks/0/efficiency` | `0.955682861022209` | SYNTHETIC owner-specified input, independently solved reference or declared numerical guard. |
| `/checks/1/odd_terms` | `128` | SYNTHETIC owner-specified input, independently solved reference or declared numerical guard. |
| `/checks/1/cl` | `0.453042498536939` | SYNTHETIC owner-specified input, independently solved reference or declared numerical guard. |
| `/checks/1/cdi` | `0.011414531397917` | SYNTHETIC owner-specified input, independently solved reference or declared numerical guard. |
| `/checks/1/efficiency` | `0.953934767176948` | SYNTHETIC owner-specified input, independently solved reference or declared numerical guard. |

| Source line | Literal | Provenance |
|---|---|---|
| `tests/llt_independent.rs:17` | `0.0` | SYNTHETIC root/offset/twist setting or half-span arithmetic factor. |
| `tests/llt_independent.rs:17` | `2.0` | SYNTHETIC root/offset/twist setting or half-span arithmetic factor. |
| `tests/llt_independent.rs:22` | `0.0` | SYNTHETIC root/offset/twist setting or half-span arithmetic factor. |
| `tests/llt_independent.rs:23` | `0.0` | SYNTHETIC root/offset/twist setting or half-span arithmetic factor. |
| `tests/llt_independent.rs:24` | `0.0` | SYNTHETIC root/offset/twist setting or half-span arithmetic factor. |
