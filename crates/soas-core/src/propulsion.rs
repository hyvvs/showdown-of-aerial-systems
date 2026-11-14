//! Instantaneous tabulated net thrust, NOT a thermodynamic engine model.
//! Table contract and synthetic-only provenance: docs/AIRCRAFT_FORMAT.md and
//! data/reference/propulsion/README.md. File access remains outside the core.
use crate::{
    aerodynamics::{self, AeroError, AeroOutput, Controls},
    aircraft::{Aircraft, AircraftError, finite, require, validate_nodes},
    dynamics::{Loads, State},
};
use nalgebra::Vector3;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Engine {
    pub name: String,
    pub data_note: String,
    /// Same file datum and body-axis orientation as Aircraft::cg_body_m.
    pub position_body_m: [f64; 3],
    /// Unit vector along the force ON THE AIRCRAFT, not the exhaust direction.
    pub direction_body_unit: [f64; 3],
    /// [0,dry_throttle_max] dry, (dry_throttle_max,1] afterburning.
    pub dry_throttle_max: f64,
    pub table: EngineTable,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EngineTable {
    pub mach_nodes: Vec<f64>,
    pub geometric_altitude_nodes_m: Vec<f64>,
    pub throttle_nodes: Vec<f64>,
    /// [altitude][Mach][throttle], with a named object at EVERY sample.
    pub samples: Vec<Vec<Vec<EngineSample>>>,
}
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EngineSample {
    pub thrust_n: f64,
    pub fuel_flow_kgps: f64,
}
#[derive(Debug, Clone, Copy)]
pub struct PropulsionOutput {
    pub loads: Loads,
    /// Sum of scalar engine thrusts, distinct from magnitude of summed force.
    pub thrust_n: f64,
    pub fuel_flow_kgps: f64,
    pub afterburner_engines: usize,
}
#[derive(Debug, Clone, Copy)]
pub struct FlightOutput {
    pub aero: AeroOutput,
    pub propulsion: PropulsionOutput,
    /// Non-gravity force and CG moment; gravity is added once by dynamics.
    pub loads: Loads,
}
impl Engine {
    pub fn validate(&self, path: &str) -> Result<(), AircraftError> {
        require(
            !self.name.trim().is_empty(),
            &format!("{path}.name"),
            "must not be empty",
        )?;
        require(
            !self.data_note.trim().is_empty(),
            &format!("{path}.data_note"),
            "must identify provenance or synthetic test data",
        )?;
        finite(
            self.position_body_m.into_iter(),
            &format!("{path}.position_body_m"),
        )?;
        finite(
            self.direction_body_unit.into_iter(),
            &format!("{path}.direction_body_unit"),
        )?;
        let norm = Vector3::from(self.direction_body_unit).norm();
        require(
            norm.is_finite() && (norm - 1.0).abs() <= 64.0 * f64::EPSILON,
            &format!("{path}.direction_body_unit"),
            "must be a unit vector; no automatic normalization",
        )?;
        self.table.validate(&format!("{path}.table"))?;
        require(
            self.dry_throttle_max.is_finite()
                && self.dry_throttle_max > 0.0
                && self.dry_throttle_max < 1.0
                && self.table.throttle_nodes.contains(&self.dry_throttle_max),
            &format!("{path}.dry_throttle_max"),
            "must be an explicit throttle node strictly between 0 and 1",
        )
    }
}
impl EngineTable {
    pub fn validate(&self, path: &str) -> Result<(), AircraftError> {
        validate_nodes(&self.mach_nodes, &format!("{path}.mach_nodes"))?;
        validate_nodes(
            &self.geometric_altitude_nodes_m,
            &format!("{path}.geometric_altitude_nodes_m"),
        )?;
        validate_nodes(&self.throttle_nodes, &format!("{path}.throttle_nodes"))?;
        require(
            self.throttle_nodes.first() == Some(&0.0) && self.throttle_nodes.last() == Some(&1.0),
            &format!("{path}.throttle_nodes"),
            "must cover normalized throttle from 0 to 1 exactly",
        )?;
        require(
            self.samples.len() == self.geometric_altitude_nodes_m.len(),
            &format!("{path}.samples"),
            "altitude row count must match nodes",
        )?;
        for (h, row) in self.samples.iter().enumerate() {
            require(
                row.len() == self.mach_nodes.len(),
                &format!("{path}.samples[{h}]"),
                "Mach column count must match nodes",
            )?;
            for (m, column) in row.iter().enumerate() {
                require(
                    column.len() == self.throttle_nodes.len(),
                    &format!("{path}.samples[{h}][{m}]"),
                    "throttle sample count must match nodes",
                )?;
                for (t, sample) in column.iter().enumerate() {
                    sample.validate(&format!("{path}.samples[{h}][{m}][{t}]"))?;
                }
            }
        }
        Ok(())
    }
}
impl EngineSample {
    fn validate(&self, path: &str) -> Result<(), AircraftError> {
        for (name, value) in [
            ("thrust_n", self.thrust_n),
            ("fuel_flow_kgps", self.fuel_flow_kgps),
        ] {
            require(
                value.is_finite() && value >= 0.0,
                &format!("{path}.{name}"),
                "must be finite and nonnegative",
            )?;
        }
        Ok(())
    }
}
fn bracket(nodes: &[f64], value: f64, axis: &str) -> Result<(usize, usize, f64), AircraftError> {
    require(
        value.is_finite() && value >= nodes[0] && value <= nodes[nodes.len() - 1],
        axis,
        "query must be finite and within tabulated range; no extrapolation",
    )?;
    for (i, node) in nodes.iter().enumerate() {
        if value == *node {
            return Ok((i, i, 0.0));
        }
        if value < *node {
            return Ok((i - 1, i, (value - nodes[i - 1]) / (node - nodes[i - 1])));
        }
    }
    unreachable!("validated finite in-range query")
}
fn mix(a: EngineSample, b: EngineSample, t: f64) -> EngineSample {
    if t == 0.0 {
        return a;
    }
    if t == 1.0 {
        return b;
    }
    EngineSample {
        thrust_n: (1.0 - t) * a.thrust_n + t * b.thrust_n,
        fuel_flow_kgps: (1.0 - t) * a.fuel_flow_kgps + t * b.fuel_flow_kgps,
    }
}
/// Tensor-product linear interpolation. Mixed differences in ALL dimensions
/// survive: interpolate throttle, then Mach, then geometric altitude.
pub fn lookup(
    table: &EngineTable,
    mach: f64,
    geometric_altitude_m: f64,
    throttle: f64,
) -> Result<EngineSample, AircraftError> {
    table.validate("engine.table")?;
    let (h0, h1, h) = bracket(
        &table.geometric_altitude_nodes_m,
        geometric_altitude_m,
        "engine.geometric_altitude_m",
    )?;
    let (m0, m1, m) = bracket(&table.mach_nodes, mach, "engine.mach")?;
    let (t0, t1, t) = bracket(&table.throttle_nodes, throttle, "engine.throttle")?;
    let row = |hi: usize| {
        let at = |mi: usize| mix(table.samples[hi][mi][t0], table.samples[hi][mi][t1], t);
        mix(at(m0), at(m1), m)
    };
    let sample = mix(row(h0), row(h1), h);
    sample.validate("engine.interpolated")?;
    Ok(sample)
}
pub fn zero_output() -> PropulsionOutput {
    PropulsionOutput {
        loads: Loads {
            force_body_n: Vector3::zeros(),
            moment_body_nm: Vector3::zeros(),
        },
        thrust_n: 0.0,
        fuel_flow_kgps: 0.0,
        afterburner_engines: 0,
    }
}
/// One normalized command for all installed engines; declaration order fixes
/// summation order. Zero throttle means the first table node, NOT engine-off.
pub fn evaluate(
    aircraft: &Aircraft,
    mach: f64,
    geometric_altitude_m: f64,
    throttle: f64,
) -> Result<PropulsionOutput, AircraftError> {
    aircraft.validate()?;
    require(
        mach.is_finite() && mach >= 0.0,
        "engine.mach",
        "must be finite and nonnegative",
    )?;
    require(
        geometric_altitude_m.is_finite() && geometric_altitude_m >= 0.0,
        "engine.geometric_altitude_m",
        "must be finite and nonnegative",
    )?;
    require(
        throttle.is_finite() && (0.0..=1.0).contains(&throttle),
        "engine.throttle",
        "must be finite and in [0, 1]",
    )?;
    let mut out = zero_output();
    for engine in &aircraft.engines {
        let sample = lookup(&engine.table, mach, geometric_altitude_m, throttle)?;
        let force = Vector3::from(engine.direction_body_unit) * sample.thrust_n;
        // Moment arm runs FROM the CG TO the application point. Standard
        // force translation r x F, same convention as aerodynamic transfer.
        let r = Vector3::from(engine.position_body_m) - Vector3::from(aircraft.cg_body_m);
        out.loads.force_body_n += force;
        out.loads.moment_body_nm += r.cross(&force);
        out.thrust_n += sample.thrust_n;
        out.fuel_flow_kgps += sample.fuel_flow_kgps;
        out.afterburner_engines += usize::from(throttle > engine.dry_throttle_max);
    }
    finite(
        out.loads
            .force_body_n
            .iter()
            .chain(out.loads.moment_body_nm.iter())
            .copied()
            .chain([out.thrust_n, out.fuel_flow_kgps]),
        "engine.output",
    )?;
    Ok(out)
}
/// Actual coupled stage loads; no fuel mass is removed from Aircraft or State.
pub fn evaluate_flight(
    aircraft: &Aircraft,
    state: &State,
    controls: Controls,
    throttle: f64,
) -> Result<FlightOutput, AeroError> {
    let aero = aerodynamics::evaluate(aircraft, state, controls)?;
    let propulsion = evaluate(aircraft, aero.mach, -state.position_ned_m.z, throttle)
        .map_err(AeroError::Aircraft)?;
    let loads = Loads {
        force_body_n: aero.loads.force_body_n + propulsion.loads.force_body_n,
        moment_body_nm: aero.loads.moment_body_nm + propulsion.loads.moment_body_nm,
    };
    finite(
        loads
            .force_body_n
            .iter()
            .chain(loads.moment_body_nm.iter())
            .copied(),
        "flight.loads",
    )
    .map_err(AeroError::Aircraft)?;
    Ok(FlightOutput {
        aero,
        propulsion,
        loads,
    })
}
