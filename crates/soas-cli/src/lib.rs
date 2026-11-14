//! Headless scenario boundary and file access. No filesystem or wall-clock
//! operations are added to soas-core. Scenario contract: docs/SCENARIOS.md.
use nalgebra::Vector3;
use serde::Deserialize;
use soas_core::{
    aerodynamics::{AeroOutput, Controls, evaluate},
    aircraft::{Aircraft, from_json},
    attitude::{Euler321, from_euler321},
    dynamics::{DEFAULT_STEP_S, FixedStepRk4, State},
    units::{Degrees, Radians},
};
use std::{
    error::Error,
    fmt::{self, Write},
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct RunnerError(pub String);
impl fmt::Display for RunnerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
impl Error for RunnerError {}
fn err(message: impl Into<String>) -> RunnerError {
    RunnerError(message.into())
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Scenario {
    pub format_version: u32,
    #[serde(default)]
    pub data_note: String,
    pub aircraft_path: PathBuf,
    pub initial_state: InitialState,
    pub controls: ControlSchedules,
    pub duration_s: f64,
    #[serde(default = "default_step")]
    pub dt_s: f64,
    pub output_interval_s: f64,
}
fn default_step() -> f64 {
    DEFAULT_STEP_S
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InitialState {
    pub position_ned_m: [f64; 3],
    pub velocity_body_mps: [f64; 3],
    /// File-boundary degrees, in roll/pitch/yaw order (3-2-1 convention).
    pub attitude_euler_deg: [f64; 3],
    pub angular_rates_body_radps: [f64; 3],
}
impl InitialState {
    pub fn to_state(&self) -> Result<State, RunnerError> {
        for (field, values) in [
            ("position_ned_m", self.position_ned_m),
            ("velocity_body_mps", self.velocity_body_mps),
            ("attitude_euler_deg", self.attitude_euler_deg),
            ("angular_rates_body_radps", self.angular_rates_body_radps),
        ] {
            if !values.into_iter().all(f64::is_finite) {
                return Err(err(format!("initial_state.{field} must be finite")));
            }
        }
        let angles = self.attitude_euler_deg;
        Ok(State {
            position_ned_m: Vector3::from(self.position_ned_m),
            velocity_body_mps: Vector3::from(self.velocity_body_mps),
            attitude_body_to_ned: from_euler321(Euler321 {
                roll_rad: degree_angle(angles[0])?,
                pitch_rad: degree_angle(angles[1])?,
                yaw_rad: degree_angle(angles[2])?,
            }),
            angular_rates_body_radps: Vector3::from(self.angular_rates_body_radps),
        })
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ControlKnot {
    pub time_s: f64,
    pub deflection_deg: f64,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ControlSchedules {
    pub elevator: Vec<ControlKnot>,
    pub aileron: Vec<ControlKnot>,
    pub rudder: Vec<ControlKnot>,
}
impl ControlSchedules {
    pub fn at(&self, time_s: f64) -> Result<Controls, RunnerError> {
        Ok(Controls {
            elevator_rad: degree_angle(schedule_at(&self.elevator, time_s)?)?,
            aileron_rad: degree_angle(schedule_at(&self.aileron, time_s)?)?,
            rudder_rad: degree_angle(schedule_at(&self.rudder, time_s)?)?,
        })
    }
    fn validate(&self, duration_s: f64) -> Result<(), RunnerError> {
        for (name, schedule) in [
            ("elevator", &self.elevator),
            ("aileron", &self.aileron),
            ("rudder", &self.rudder),
        ] {
            if schedule.is_empty()
                || schedule[0].time_s != 0.0
                || schedule.last().is_none_or(|k| k.time_s < duration_s)
                || schedule
                    .iter()
                    .any(|k| !k.time_s.is_finite() || !k.deflection_deg.is_finite())
                || schedule.windows(2).any(|k| k[0].time_s >= k[1].time_s)
            {
                return Err(err(format!(
                    "controls.{name}: finite increasing knots must start at 0 and cover duration_s"
                )));
            }
            for knot in schedule {
                degree_angle(knot.deflection_deg)?;
            }
        }
        Ok(())
    }
}
fn schedule_at(knots: &[ControlKnot], time: f64) -> Result<f64, RunnerError> {
    if !time.is_finite()
        || knots.is_empty()
        || time < knots[0].time_s
        || time > knots[knots.len() - 1].time_s
    {
        return Err(err("controls: time outside schedule"));
    }
    for knot in knots {
        if time == knot.time_s {
            return Ok(knot.deflection_deg);
        }
    }
    for pair in knots.windows(2) {
        if time > pair[0].time_s && time < pair[1].time_s {
            let t = (time - pair[0].time_s) / (pair[1].time_s - pair[0].time_s);
            let value = (1.0 - t) * pair[0].deflection_deg + t * pair[1].deflection_deg;
            if value.is_finite() {
                return Ok(value);
            } else {
                return Err(err("controls: interpolation overflow"));
            }
        }
    }
    Err(err("controls: invalid schedule"))
}
fn degree_angle(degrees: f64) -> Result<Radians, RunnerError> {
    Degrees::new(degrees)
        .and_then(Radians::try_from)
        .map_err(|e| err(format!("angle conversion: {e}")))
}

pub fn parse_scenario(text: &str) -> Result<Scenario, RunnerError> {
    let scenario: Scenario =
        serde_json::from_str(text).map_err(|e| err(format!("scenario JSON: {e}")))?;
    scenario.validate()?;
    Ok(scenario)
}
impl Scenario {
    fn validate(&self) -> Result<(u64, u64), RunnerError> {
        if self.format_version != 0 {
            return Err(err("scenario format_version: only version 0 is supported"));
        }
        if self.aircraft_path.as_os_str().is_empty() {
            return Err(err("aircraft_path must not be empty"));
        }
        if !self.dt_s.is_finite() || self.dt_s <= 0.0 {
            return Err(err("dt_s must be finite and positive"));
        }
        if !self.duration_s.is_finite() || self.duration_s < 0.0 {
            return Err(err("duration_s must be finite and nonnegative"));
        }
        if !self.output_interval_s.is_finite() || self.output_interval_s <= 0.0 {
            return Err(err("output_interval_s must be finite and positive"));
        }
        let steps = aligned_steps(self.duration_s, self.dt_s, "duration_s")?;
        let output_steps = aligned_steps(self.output_interval_s, self.dt_s, "output_interval_s")?;
        if output_steps == 0 {
            return Err(err("output_interval_s must be at least dt_s"));
        }
        self.initial_state.to_state()?;
        self.controls.validate(self.duration_s)?;
        Ok((steps, output_steps))
    }
}
fn aligned_steps(time: f64, dt: f64, field: &str) -> Result<u64, RunnerError> {
    let quotient = time / dt;
    let rounded = quotient.round();
    // Numerical alignment guard only, not an aircraft parameter. Count stays
    // below binary64's exact-integer limit; never silently shorten a final step.
    if !quotient.is_finite()
        || (time > 0.0 && rounded == 0.0)
        || rounded >= 9007199254740992.0
        || (quotient - rounded).abs() > 64.0 * f64::EPSILON * quotient.abs().max(1.0)
    {
        return Err(err(format!(
            "{field} must be an integer multiple of dt_s, with a representable step count"
        )));
    }
    Ok(rounded as u64)
}

#[derive(Debug, Clone, Copy)]
pub struct Sample {
    pub time_s: f64,
    pub state: State,
    pub controls: Controls,
    pub aero: AeroOutput,
}

pub fn simulate(scenario: &Scenario, aircraft: &Aircraft) -> Result<Vec<Sample>, RunnerError> {
    let (steps, output_steps) = scenario.validate()?;
    let mass = aircraft.mass_properties().map_err(|e| err(e.to_string()))?;
    let integrator = FixedStepRk4::new(mass, scenario.dt_s).map_err(|e| err(e.to_string()))?;
    let mut state = scenario.initial_state.to_state()?;
    let mut samples = Vec::new();
    for index in 0..=steps {
        let time = if index == steps {
            scenario.duration_s
        } else {
            index as f64 * scenario.dt_s
        };
        let controls = scenario.controls.at(time)?;
        // Validate every completed state, even when it is not output.
        let aero = evaluate(aircraft, &state, controls)
            .map_err(|e| err(format!("simulation at {time} s: {e}")))?;
        if index % output_steps == 0 || index == steps {
            samples.push(Sample {
                time_s: time,
                state,
                controls,
                aero,
            });
        }
        if index == steps {
            break;
        }
        let step_end = if index + 1 == steps {
            scenario.duration_s
        } else {
            (index + 1) as f64 * scenario.dt_s
        };
        state = integrator
            .try_step_with_time(&state, |offset, stage| {
                // At the last stage use the indexed endpoint, avoiding an extra
                // time+h rounding that could accidentally exceed schedule coverage.
                let stage_time = if offset == scenario.dt_s {
                    step_end
                } else {
                    time + offset
                };
                let controls = scenario
                    .controls
                    .at(stage_time)
                    .map_err(|e| e.to_string())?;
                evaluate(aircraft, stage, controls)
                    .map(|out| out.loads)
                    .map_err(|e| e.to_string())
            })
            .map_err(|e| err(format!("simulation step at {time} s: {e}")))?;
    }
    Ok(samples)
}

pub fn csv_text(samples: &[Sample]) -> String {
    let mut text = String::from(
        "time_s,north_m,east_m,down_m,u_mps,v_mps,w_mps,qw_unitless,qx_unitless,qy_unitless,qz_unitless,p_radps,q_radps,r_radps,elevator_rad,aileron_rad,rudder_rad,mach_unitless,alpha_rad,beta_rad,cl_unitless,cd_unitless,lift_n,drag_n,side_n,lift_limited_unitless\n",
    );
    for sample in samples {
        let s = sample.state;
        let a = sample.aero;
        let q = s.attitude_body_to_ned.quaternion();
        let c = sample.controls;
        let values = [
            sample.time_s,
            s.position_ned_m.x,
            s.position_ned_m.y,
            s.position_ned_m.z,
            s.velocity_body_mps.x,
            s.velocity_body_mps.y,
            s.velocity_body_mps.z,
            q.w,
            q.i,
            q.j,
            q.k,
            s.angular_rates_body_radps.x,
            s.angular_rates_body_radps.y,
            s.angular_rates_body_radps.z,
            c.elevator_rad.value(),
            c.aileron_rad.value(),
            c.rudder_rad.value(),
            a.mach,
            a.angles.alpha_rad.value(),
            a.angles.beta_rad.value(),
            a.cl,
            a.cd,
            a.lift_n,
            a.drag_n,
            a.side_n,
            if a.stalled { 1.0 } else { 0.0 },
        ];
        for (index, value) in values.into_iter().enumerate() {
            if index != 0 {
                text.push(',');
            }
            write!(&mut text, "{value}").expect("writing into a String");
        }
        text.push('\n');
    }
    text
}

pub fn run_files(scenario_path: &Path, output_path: &Path) -> Result<(), RunnerError> {
    let text = fs::read_to_string(scenario_path)
        .map_err(|e| err(format!("read scenario {}: {e}", scenario_path.display())))?;
    let scenario = parse_scenario(&text)?;
    let aircraft_path = scenario_path
        .parent()
        .unwrap_or(Path::new("."))
        .join(&scenario.aircraft_path);
    let aircraft_text = fs::read_to_string(&aircraft_path)
        .map_err(|e| err(format!("read aircraft {}: {e}", aircraft_path.display())))?;
    let aircraft = from_json(&aircraft_text)
        .map_err(|e| err(format!("aircraft {}: {e}", aircraft_path.display())))?;
    let samples = simulate(&scenario, &aircraft)?;
    if let Ok(output) = fs::canonicalize(output_path) {
        for input in [scenario_path, aircraft_path.as_path()] {
            if fs::canonicalize(input).is_ok_and(|path| path == output) {
                return Err(err(
                    "CSV output must not overwrite a scenario or aircraft input",
                ));
            }
        }
    }
    // Finish validation/simulation before opening output. A failed simulation
    // cannot overwrite an existing result with an apparently complete CSV.
    fs::write(output_path, csv_text(&samples))
        .map_err(|e| err(format!("write CSV {}: {e}", output_path.display())))
}
