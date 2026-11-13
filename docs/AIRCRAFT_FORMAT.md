# Aircraft JSON format v0 (T5)

`aircraft::from_json(&str)` loads and validates JSON text;
`aircraft::to_json(&Aircraft)` validates and saves JSON text. The simulation core
does not open files; a caller owns filesystem access. Version 0 requires
`format_version: 0`. Missing required fields, unknown fields, unsupported
versions, invalid shapes and invalid values are explicit errors. Parser errors
include a line/column; validation errors name the field or table node.

## Fields, units and reference frames

| Field | Meaning |
|---|---|
| `format_version` | Required integer schema version; currently zero only. |
| `name`, `data_note` | Nonempty model identity and provenance/limitations note. A note is not proof of data provenance. |
| `mass_kg` | Finite positive constant total mass. |
| `cg_body_m` | Three body-axis coordinates of the CG relative to a model-defined file datum. |
| `inertia_body_kg_m2` | Full signed 3x3 tensor, row then column, about the CG in body axes. D11 signs; finite, symmetric, positive definite and numerically invertible. |
| `reference_area_m2`, `reference_span_m`, `reference_chord_m` | Finite positive aerodynamic reference area, span and chord. Aspect ratio is span squared divided by area. |
| `aerodynamic_reference_point_body_m` | Three coordinates relative to the SAME datum as the CG. Coefficient moments are about this point; T6 transfers them to the CG. |
| `coefficient_axes` | Must be `wind_forces_body_moments`: force coefficients CL/CD/CY use D12 wind axes; rolling/pitching/yawing moments use body axes. |
| `moments_about` | Must be `aerodynamic_reference_point`. The inertia reference remains the CG. |
| `aerodynamics` | Coefficient table described below. |

Coordinates must be finite. No automatic unit, source-axis or inertia-sign
conversion is inferred. SI is required, including per-radian angle derivatives.
Source data in other conventions must be converted and verified before loading.
This format does not establish that an aircraft's parameters are physically
realizable or representative of any real aircraft.

## Coefficient fields

Every table cell has all eleven finite numeric fields; no implicit coefficient
defaults. CL, CD, CY and moment coefficients are dimensionless; derivatives
with `_per_rad` multiply angles in radians. Model equations are specified by T6.

| Field | Role / validation |
|---|---|
| `cl0` | Lift intercept. |
| `cl_alpha_per_rad` | Lift slope; positive for this initial model. |
| `cl_max` | Positive upper lift-coefficient limit. |
| `cd0` | Nonnegative zero-lift drag. |
| `oswald_efficiency` | Positive efficiency factor in the induced-drag formula. |
| `cm0`, `cm_alpha_per_rad` | Pitching moment intercept and angle-of-attack derivative. |
| `cm_elevator_per_rad` | Nonpositive: positive elevator gives negative pitching moment (D6). |
| `cy_beta_per_rad` | Wind-axis side-force derivative with sideslip. |
| `roll_aileron_per_rad` | Nonnegative: positive aileron gives positive rolling moment (D6). |
| `yaw_rudder_per_rad` | Nonpositive: positive rudder gives negative yawing moment (D6). |

Zero control derivatives permit an absent/ineffective surface; an active
surface must have the chosen sign. The synthetic fixture uses nonzero controls
so each direction can be tested separately.

## Table layout and interpolation contract

- `mach_nodes`: nonempty, finite, nonnegative, strictly increasing Mach numbers.
- `geometric_altitude_nodes_m`: optional (omit or use null for Mach-only data),
  otherwise nonempty, finite, nonnegative, strictly increasing geometric heights.
- `coefficients[altitude_index][mach_index]`: rectangular array of complete cells.
  A Mach-only table has exactly one row. Every row has one cell per Mach node.
- At an exact node, return that cell. Between Mach nodes, interpolate every
  coefficient linearly. With altitude data, also interpolate linearly between
  altitude rows (bilinear interpolation over each rectangular cell).
- Outside either explicitly tabulated axis, return an error; no extrapolation
  or endpoint clamping. A singleton axis therefore permits only its exact node.
  A Mach-only table is independent of altitude, but the simulator still enforces
  the atmosphere's [0, 20000] geometric-metre range.
- Table support is independent of the v0.1 flight envelope. Storing nodes above
  Mach 0.8 does not authorize their simulation: T6 rejects that region with
  **"Mach > 0.8 not modeled"**.

T5 validates and round-trips the table. T6 implements/tests lookup and interpolation
through `aerodynamics::lookup_coefficients`; the coupled model computes its Mach
and geometric altitude from each RK4 stage state.
This policy is an implementation contract for the requested v0 tables, not a
resolution of any other OPEN decision in DECISIONS.md.

## Synthetic fixture and test provenance

`crates/soas-core/tests/fixtures/synthetic_aircraft.json` is entirely synthetic.
Its top-level warning covers EVERY number; the companion
`synthetic_aircraft_values.md` labels each numeric JSON path individually as a
TEST VALUE. Nothing is measured, borrowed from or estimated for a real aircraft.
The chosen simple numbers allow independent mathematical checks. Format version
and node coordinates are test metadata, not aircraft measurements either.

## Software references

- [Serde container attributes](https://serde.rs/container-attrs.html#deny_unknown_fields):
  reject unknown fields rather than ignoring misspellings or wrong unit keys.
- [serde_json from_str](https://docs.rs/serde_json/1/serde_json/fn.from_str.html)
  and [to_string_pretty](https://docs.rs/serde_json/1/serde_json/fn.to_string_pretty.html):
  pure JSON text conversion. Serde and serde_json are pre-approved by AGENTS.md.
  Non-finite values in programmatically edited models are rejected before saving.

Verified online 2026-11-14. Physical conventions come from D3/D4/D6/D11/D12
and CONVENTIONS.md; no external aircraft parameters were added.
