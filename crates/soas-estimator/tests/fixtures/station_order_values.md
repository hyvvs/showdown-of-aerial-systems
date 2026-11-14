# Station-order synthetic test ledger

All numeric entries are labeled SYNTHETIC TEST VALUES, not aircraft inputs. The owner approved existing structured WingError comparisons and equal-root/tip y=0 as zero span. Coordinates are adversarial validation cases, not aerodynamic data.

| JSON path | Value | Provenance |
|---|---|---|
| `/chord_m` | `2.0` | SYNTHETIC adversarial span station or positive chord, explicitly defined by the named case. |
| `/duplicate_interior/y_m/0` | `0.0` | SYNTHETIC adversarial span station or positive chord, explicitly defined by the named case. |
| `/duplicate_interior/y_m/1` | `1.0` | SYNTHETIC adversarial span station or positive chord, explicitly defined by the named case. |
| `/duplicate_interior/y_m/2` | `1.0` | SYNTHETIC adversarial span station or positive chord, explicitly defined by the named case. |
| `/duplicate_interior/y_m/3` | `4.0` | SYNTHETIC adversarial span station or positive chord, explicitly defined by the named case. |
| `/descending_interior/y_m/0` | `0.0` | SYNTHETIC adversarial span station or positive chord, explicitly defined by the named case. |
| `/descending_interior/y_m/1` | `2.0` | SYNTHETIC adversarial span station or positive chord, explicitly defined by the named case. |
| `/descending_interior/y_m/2` | `1.0` | SYNTHETIC adversarial span station or positive chord, explicitly defined by the named case. |
| `/descending_interior/y_m/3` | `4.0` | SYNTHETIC adversarial span station or positive chord, explicitly defined by the named case. |
| `/missing_root/y_m/0` | `1.0` | SYNTHETIC adversarial span station or positive chord, explicitly defined by the named case. |
| `/missing_root/y_m/1` | `2.0` | SYNTHETIC adversarial span station or positive chord, explicitly defined by the named case. |
| `/missing_root/y_m/2` | `4.0` | SYNTHETIC adversarial span station or positive chord, explicitly defined by the named case. |
| `/equal_root_tip/y_m/0` | `0.0` | SYNTHETIC adversarial span station or positive chord, explicitly defined by the named case. |
| `/equal_root_tip/y_m/1` | `0.0` | SYNTHETIC adversarial span station or positive chord, explicitly defined by the named case. |

| Source line | Literal | Provenance |
|---|---|---|
| `tests/station_order.rs:19` | `0.0` | SYNTHETIC zero datum/twist/zero-lift setting; independent of the invalid ordering. |
| `tests/station_order.rs:20` | `0.0` | SYNTHETIC zero datum/twist/zero-lift setting; independent of the invalid ordering. |
| `tests/station_order.rs:21` | `0.0` | SYNTHETIC zero datum/twist/zero-lift setting; independent of the invalid ordering. |
| `tests/station_order.rs:22` | `0.0` | SYNTHETIC zero datum/twist/zero-lift setting; independent of the invalid ordering. |
