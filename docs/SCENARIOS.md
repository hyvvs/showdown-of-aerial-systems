# Headless scenarios (T7)

```text
soas-cli run <scenario.json> --out <run.csv>
cargo run -p soas-cli -- run crates/soas-cli/tests/fixtures/synthetic_scenario.json --out run.csv
```

Only std parses arguments; no additional external packages were added. Native
paths are accepted through `args_os`. All file access is in soas-cli. Aircraft
paths resolve relative to the scenario file's directory, or can be absolute.

## Scenario JSON version 0

The executable reads a JSON object with these fields. Unknown fields and
versions are errors. `data_note` is optional provenance text; the example
explicitly labels every numeric value as a synthetic TEST VALUE.

| Field | Contract |
|---|---|
| `format_version` | Required integer 0. |
| `aircraft_path` | Nonempty path to an aircraft-format-v0 JSON file. |
| `initial_state.position_ned_m` | Finite North/East/Down coordinates; geometric altitude is -Down. |
| `initial_state.velocity_body_mps` | Finite forward/right/down velocity components. No wind in v0.1. |
| `initial_state.attitude_euler_deg` | Finite [roll,pitch,yaw] in degrees, 3-2-1 convention; converted with tested Degrees/Radians types at this boundary. |
| `initial_state.angular_rates_body_radps` | Finite [p,q,r] body rates in radians/s. |
| `controls.elevator`, `.aileron`, `.rudder` | Nonempty arrays of `{time_s, deflection_deg}`. Times must be finite, strictly increasing, start at zero and cover the entire duration. Values follow D6. |
| `duration_s` | Finite nonnegative duration, an integer multiple of dt_s. Zero means output initial state only. |
| `dt_s` | Optional finite positive fixed step; default 0.0025 s (D10). |
| `output_interval_s` | Finite positive integer multiple of dt_s, at least one step. |

Controls interpolate linearly between knots; no extrapolation beyond schedule
coverage. Knots need not coincide with step boundaries. Every RK4 stage gets the
correct simulation time (start, midpoint, midpoint, end). The core timed callback
receives an offset within the step, never wall-clock time. Existing state-only
callbacks keep their previous interface and equations.

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
