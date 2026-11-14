# Aircraft JSON formats 0 and 1 (T5, T9)

`aircraft::from_json(&str)` loads and validates JSON text;
`aircraft::to_json(&Aircraft)` validates and saves JSON text. The simulation core
does not open files; a caller owns filesystem access. Version 0 requires
`format_version: 0` and forbids an `engines` field. Version 1 requires
`format_version: 1` and an explicit `engines` array, which may be empty.
Version 0 remains supported for existing power-off files; no silent migration
or engine defaults are inferred. Other versions are rejected. Missing required fields, unknown fields, unsupported
versions, invalid shapes and invalid values are explicit errors. Parser errors
include a line/column; validation errors name the field or table node.

Named records must be JSON objects (`{...}`), not positional arrays (`[...]`):
the aircraft root, `aerodynamics`, every coefficient cell, every engine,
each engine's `table`, and every engine sample. `from_json`
checks these shapes before typed deserialization and names the offending path,
including row/column for a cell. Unknown, duplicate and missing fields and
trailing input remain errors with the original parser's line/column information.
Loading parses twice to keep the derived schema and those diagnostics intact;
the simulation hot path does not parse JSON. Use `from_json` for aircraft files;
direct generic Serde deserialization is not the validated file-loading API.

Mass and geometry are flat named scalar fields in v0, not nested record objects;
control-surface derivatives are numeric fields in each cell. Arrays cannot
replace those numbers. Coordinate vectors, inertia matrices, node vectors and
table row/cell containers remain arrays as documented below. No schema fields
were renamed or reorganized by this restriction.

## Fields, units and reference frames

| Field | Meaning |
|---|---|
| `format_version` | Required integer 0 (engine-less) or 1 (explicit engines). |
| `name`, `data_note` | Nonempty model identity and provenance/limitations note. A note is not proof of data provenance. |
| `mass_kg` | Finite positive constant total mass. |
| `cg_body_m` | Three body-axis coordinates of the CG relative to a model-defined file datum. |
| `inertia_body_kg_m2` | Full signed 3x3 tensor, row then column, about the CG in body axes. D11 signs; finite, symmetric, positive definite and numerically invertible. |
| `reference_area_m2`, `reference_span_m`, `reference_chord_m` | Finite positive aerodynamic reference area, span and chord. Aspect ratio is span squared divided by area. |
| `aerodynamic_reference_point_body_m` | Three coordinates relative to the SAME datum as the CG. Coefficient moments are about this point; T6 transfers them to the CG. |
| `coefficient_axes` | Must be `wind_forces_body_moments`: force coefficients CL/CD/CY use D12 wind axes; rolling/pitching/yawing moments use body axes. |
| `moments_about` | Must be `aerodynamic_reference_point`. The inertia reference remains the CG. |
| `aerodynamics` | Coefficient table described below. |
| `engines` | Required in version 1, forbidden in version 0; ordered array of engine objects, described below. |

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

## Engine extension: version 1 (T9)

This is an instantaneous **net-thrust lookup**, not a thermodynamic cycle or
real-engine calibration. All installed engines receive one common normalized
throttle command. Declaration order fixes the order of force/fuel summation.

| Engine field | Meaning |
|---|---|
| `name`, `data_note` | Nonempty identity and provenance/limitations. |
| `position_body_m` | Finite [x,y,z] application point in body axes, relative to the SAME file datum as `cg_body_m`. |
| `direction_body_unit` | Finite unit [x,y,z] vector along force on the aircraft, not exhaust; norm must differ from 1 by no more than 64 machine epsilons. No automatic normalization. |
| `dry_throttle_max` | Explicit throttle node strictly between 0 and 1. Dry range is `[0, dry_throttle_max]`; afterburner range is `(dry_throttle_max, 1]`. |
| `table.mach_nodes` | Nonempty finite nonnegative strictly increasing Mach nodes. |
| `table.geometric_altitude_nodes_m` | Nonempty finite nonnegative strictly increasing geometric heights in metres. |
| `table.throttle_nodes` | Finite strictly increasing dimensionless commands, beginning at 0 and ending at 1; must contain the dry/afterburner boundary. |
| `table.samples[h][m][t]` | Rectangular altitude/Mach/throttle array; each cell is a named object with finite nonnegative `thrust_n` and `fuel_flow_kgps`. |

Throttle 0 means the first table node, which can include idle thrust/fuel; it
does not imply engine-off. The dry/afterburner boundary is a file parameter,
not a claimed real-engine threshold. The fixture's boundary of 0.6 is labeled
synthetic. There is no extra afterburner multiplier: both outputs come from the
table, continuously across the boundary node; slopes may change there.

Exact nodes return their samples. Between nodes, interpolate throttle, then
Mach, then altitude: tensor-product **trilinear** interpolation, retaining
all mixed terms. An explicitly indexed singleton Mach/altitude permits only
that node. Out-of-range/non-finite queries error; no clamping or extrapolation.
Tables can store broader coverage, but coupled flight still enforces geometric
ISA `[0, 20000]` metres and **Mach > 0.8 not modeled**.

For each engine, `F = thrust_n * direction_body_unit`,
`r = position_body_m - cg_body_m`, and `M_CG = r cross F`. Sum these loads with
aerodynamic loads before integration; gravity is still added once by dynamics.
`propulsion::evaluate_flight` is the combined stage-load API;
`aerodynamics::evaluate` and `aerodynamics::step_aircraft` retain their explicit
aerodynamic-only meanings. Total reported thrust is the sum of scalar engine
thrusts, not the magnitude of the resultant force. Fuel flow sums in kg/s and
is **reported only**: mass, CG and inertia stay constant.

`synthetic_jet.json` and `synthetic_jet_values.md` label every number, including
every engine cell and coordinate, as SYNTHETIC TEST DATA. Existing version-0
fixture tests remain. No real engine tables have been supplied; published,
cited net-thrust and fuel-flow tables with conditions/units/axis coverage are
required before using real engines. Sources and hand checks:
`data/reference/propulsion/README.md`.

## Software references

- [Serde container attributes](https://serde.rs/container-attrs.html#deny_unknown_fields):
  reject unknown fields rather than ignoring misspellings or wrong unit keys.
- [serde_json from_str](https://docs.rs/serde_json/1/serde_json/fn.from_str.html)
  and [to_string_pretty](https://docs.rs/serde_json/1/serde_json/fn.to_string_pretty.html):
  pure JSON text conversion. Serde and serde_json are pre-approved by AGENTS.md.
  Non-finite values in programmatically edited models are rejected before saving.

Verified online 2026-11-14. Physical conventions come from D3/D4/D6/D11/D12
and CONVENTIONS.md; no external aircraft parameters were added.
