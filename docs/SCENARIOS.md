# Headless scenarios (T7)

```text
soas-cli run <scenario.json> --out <run.csv>
cargo run -p soas-cli -- run crates/soas-cli/tests/fixtures/synthetic_scenario.json --out run.csv
```

Only std parses arguments; no additional external packages were added. Native
paths are accepted through `args_os`. All file access is in soas-cli. Aircraft
paths resolve relative to the scenario file's directory, or can be absolute.

## Scenario JSON versions 0 and 1

The executable reads a JSON object with these fields. Unknown fields and
versions are errors. `data_note` is optional provenance text; the example
explicitly labels every numeric value as a synthetic TEST VALUE.

The scenario root, `initial_state`, `controls`, and every surface's schedule
knot must be named JSON objects, never positional arrays. `parse_scenario`
checks these shapes and reports the offending path (for example
`controls.rudder[1]`) before the existing typed parser runs on the original
text. Unknown, duplicate and missing fields and trailing input retain parser
line/column diagnostics. This performs an extra parse when loading, without
changing integration or file output. Use `parse_scenario` for scenario files.
State vectors and arrays of schedule knots remain arrays; each knot inside its
schedule must be an object containing `time_s` and `deflection_deg`.

| Field | Contract |
|---|---|
| `format_version` | Required integer 0 (legacy power-off) or 1 (explicit throttle). Other versions error. |
| `aircraft_path` | Nonempty path to an aircraft-format-0, -1 or -2 JSON file. A version-0 scenario requires an engine-less aircraft; installed engines require a version-1 scenario. |
| `initial_state.position_ned_m` | Finite North/East/Down coordinates; geometric altitude is -Down. |
| `initial_state.velocity_body_mps` | Finite forward/right/down velocity components. No wind in v0.1. |
| `initial_state.attitude_euler_deg` | Finite [roll,pitch,yaw] in degrees, 3-2-1 convention; converted with tested Degrees/Radians types at this boundary. |
| `initial_state.angular_rates_body_radps` | Finite [p,q,r] body rates in radians/s. |
| `controls.elevator`, `.aileron`, `.rudder` | Nonempty arrays of `{time_s, deflection_deg}`. Times must be finite, strictly increasing, start at zero and cover the entire duration. Values follow D6. |
| `duration_s` | Finite nonnegative duration, an integer multiple of dt_s. Zero means output initial state only. |
| `dt_s` | Optional finite positive fixed step; default 0.0025 s (D10). |
| `output_interval_s` | Finite positive integer multiple of dt_s, at least one step. |
| `throttle` | Required nonempty array in version 1, forbidden in version 0. Each knot is a named object `{time_s, throttle}` with a finite command in `[0,1]`. Times start at 0, strictly increase and cover duration. |

Controls interpolate linearly between knots; no extrapolation beyond schedule
coverage. Knots need not coincide with step boundaries. Every RK4 stage gets the
correct simulation time (start, midpoint, midpoint, end). The core timed callback
receives an offset within the step, never wall-clock time. Existing state-only
callbacks keep their previous interface and equations.

Version 1 adds a separate dimensionless throttle schedule, sampled linearly
at **every RK4 stage** just like surface schedules. Throttle knots cannot be
positional arrays; the error names `throttle[index]`. Unknown, duplicate and
missing fields retain original parser diagnostics. A version-1 file without
the schedule is rejected, rather than assuming a power setting. Version 0 has
no engine commands and is explicitly engine-less. An in-memory version-0
scenario also cannot carry nonempty throttle knots.

One command drives all engines. The aircraft file defines each dry/afterburner
boundary; the boundary itself is dry, larger commands are afterburning.
Throttle 0 is the tabulated idle condition, not an engine-off switch. Engine
forces and CG moments are reevaluated with the stage's Mach and geometric
altitude and added to the aerodynamic loads. Table/domain errors abort the
run. Fuel flow is reported, with mass/CG/inertia held constant.

Powered synthetic example (every numeric path has a companion TEST VALUE ledger):

```text
cargo run -p soas-cli -- run crates/soas-cli/tests/fixtures/synthetic_powered_scenario.json --out run.csv
```

Step/output indices are integers; timestamps come from index*dt, not accumulated
floating-point addition. Integer-multiple checks allow only numerical roundoff
(64 machine epsilons scaled by the quotient); positive durations cannot round to
zero steps. Step counts must be below binary64's exact-integer limit, 2^53.
No shortened final integration step is silently introduced. Output includes t=0,
regular output times, and the final state once even if off the output cadence.

## CSV and failures

Column order is fixed. Units appear in every numeric column name: positions m,
velocity mps, body rates radps, controls/alpha/beta rad, forces n. Quaternion
components are scalar-first qw,qx,qy,qz and labeled unitless; coefficients and
Mach are also unitless. The upper-lift-limit flag is numeric 0/1. Quaternions
avoid undefined Euler output at singular pitch.

T9 appends four columns for both scenario versions: `throttle_unitless`,
`thrust_n` (sum of scalar engine thrusts), `fuel_flow_kgps` (total reported fuel
flow), and `afterburner_engines_unitless` (integer count). They are zero for
engine-less version-0 runs. Existing columns keep their order and meaning.

All numbers use Rust's default f64 Display without an explicit precision:
round-trip decimal digits, no deliberate rounding or timestamps from the clock.
Lines end with LF. Identical inputs produce identical bytes on one machine and
binary; no cross-platform bit-identity is promised.

The run is validated and simulated before opening output, so scenario/aircraft/
integration errors do not replace a prior CSV. A scenario or aircraft input
cannot also be the output path. Output-write errors are explicit; disk writes
are not claimed atomic. Samples and CSV text are buffered in memory, so this
initial runner is intended for bounded runs, not unbounded streaming histories.

The supplied scenario and aircraft are entirely synthetic, not real flight data.
Tests cover two actual process runs and byte comparison, numeric round-trip
formatting, relative paths, schedule interpolation/degree conversion, analytic
time-dependent loads at RK4 stages, invalid files/times/arguments, zero duration,
final-output cadence and preservation of inputs/existing outputs on run errors.

## Sources

[Rust args_os](https://doc.rust-lang.org/std/env/fn.args_os.html) documents native
argument handling. [Rust floating-point formatting](https://doc.rust-lang.org/std/fmt/index.html#precision)
and [f64 Display](https://doc.rust-lang.org/std/primitive.f64.html#impl-Display-for-f64)
are used without a precision setting. Serde/serde_json and physics references
are the already recorded T5/T6 sources. No new aircraft constants were added.

## Commanded versus applied surface travel (T9b)

Aircraft format 2 supplies asymmetric radian travel limits for each surface.
Scheduled degree commands are still converted to Radians at the boundary and
sampled at each RK4 stage. The aerodynamic model saturates each command before
computing loads. There is no actuator rate limit or lag. Aircraft versions 0/1
retain unrestricted travel; their commanded and applied values are identical.

For all scenario versions, the former `elevator_rad`, `aileron_rad`,
`rudder_rad` CSV column names become `commanded_elevator_rad`,
`commanded_aileron_rad`, `commanded_rudder_rad`. Three new columns immediately
follow: `applied_elevator_rad`, `applied_aileron_rad`, `applied_rudder_rad`.
Consumers should select columns by header, not old numeric offsets. Values
remain shortest-round-trip decimals with units in names. The lift-limit flag
now means either upper or lower cap for version-2 aircraft. Scenario JSON itself
remains at versions 0/1; the added limits live in the versioned aircraft file.
`synthetic_limited_scenario.json` and its ledger label every example number.
