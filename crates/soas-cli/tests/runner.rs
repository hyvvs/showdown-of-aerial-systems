//! All scenarios and manipulated coefficients are SYNTHETIC TEST VALUES;
//! the sole aircraft source is the existing labeled synthetic JSON fixture.
use nalgebra::Vector3;
use soas_cli::{Scenario, parse_scenario, simulate};
use soas_core::{aircraft::from_json, dynamics::FixedStepRk4};
use std::{
    fs,
    path::PathBuf,
    process::Command,
    sync::atomic::{AtomicUsize, Ordering},
};

const SCENARIO: &str = include_str!("fixtures/synthetic_scenario.json");
static NEXT: AtomicUsize = AtomicUsize::new(0);

struct Scratch(PathBuf);
impl Scratch {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "soas-cli-test-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
}
impl Drop for Scratch {
    fn drop(&mut self) {
        // Recurse only into the exact directory created under the intended
        // temporary root, never into a caller-supplied path.
        assert_eq!(self.0.parent(), Some(std::env::temp_dir().as_path()));
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn aircraft() -> soas_core::aircraft::Aircraft {
    from_json(include_str!(
        "../../soas-core/tests/fixtures/synthetic_aircraft.json"
    ))
    .unwrap()
}
fn command() -> Command {
    Command::new(env!("CARGO_BIN_EXE_soas-cli"))
}

#[test]
fn identical_input_files_give_byte_identical_csv_and_resolve_aircraft_relative_to_scenario() {
    let scratch = Scratch::new();
    fs::write(
        scratch.0.join("aircraft.json"),
        include_str!("../../soas-core/tests/fixtures/synthetic_aircraft.json"),
    )
    .unwrap();
    let text = SCENARIO.replace(
        "../../../soas-core/tests/fixtures/synthetic_aircraft.json",
        "aircraft.json",
    );
    let scenario = scratch.0.join("scenario.json");
    fs::write(&scenario, text).unwrap();
    for out in ["a.csv", "b.csv"] {
        let result = command()
            .current_dir(&scratch.0)
            .arg("run")
            .arg(&scenario)
            .arg("--out")
            .arg(scratch.0.join(out))
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
    }
    let a = fs::read(scratch.0.join("a.csv")).unwrap();
    let b = fs::read(scratch.0.join("b.csv")).unwrap();
    assert_eq!(a, b);
    let saved_scenario = fs::read(&scenario).unwrap();
    let protected = command()
        .arg("run")
        .arg(&scenario)
        .arg("--out")
        .arg(&scenario)
        .output()
        .unwrap();
    assert!(!protected.status.success());
    assert!(String::from_utf8_lossy(&protected.stderr).contains("input"));
    assert_eq!(fs::read(&scenario).unwrap(), saved_scenario);
    let csv = String::from_utf8(a).unwrap();
    let lines: Vec<_> = csv.lines().collect();
    assert!(lines[0].starts_with("time_s,north_m,east_m,down_m,u_mps,v_mps,w_mps,qw_unitless"));
    assert!(lines[0].contains("elevator_rad"));
    assert!(lines[0].contains("mach_unitless"));
    assert_eq!(lines.len(), 6); // Header + t=0,.05,.1,.15,.2 (default dt=1/400).
    for line in &lines[1..] {
        for field in line.split(',') {
            let value: f64 = field.parse().unwrap();
            assert!(value.is_finite());
            assert_eq!(
                value.to_string(),
                field,
                "CSV must use shortest round-trip Display"
            );
        }
    }
    let times: Vec<f64> = lines[1..]
        .iter()
        .map(|line| line.split(',').next().unwrap().parse().unwrap())
        .collect();
    assert_eq!(times, vec![0.0, 0.05, 0.1, 0.15, 0.2]);
}

#[test]
fn scenario_angles_convert_at_boundary_and_schedules_are_piecewise_linear() {
    let s = parse_scenario(SCENARIO).unwrap();
    assert_eq!(s.dt_s, 0.0025);
    let initial = s.initial_state.to_state().unwrap();
    let angles = soas_core::attitude::to_euler321(&initial.attitude_body_to_ned).unwrap();
    assert!((angles.pitch_rad.value() - 4.0_f64.to_radians()).abs() < 1e-14);
    for (time, elevator) in [
        (0.0_f64, 0.0_f64),
        (0.05, -0.5),
        (0.1, -1.0),
        (0.15, -0.5),
        (0.2, 0.0),
    ] {
        let c = s.controls.at(time).unwrap();
        assert!((c.elevator_rad.value() - elevator.to_radians()).abs() < 1e-14);
    }
    assert!(s.controls.at(-0.01).is_err());
}

#[test]
fn timed_rk4_callback_samples_both_midpoints_and_the_endpoint() {
    let a = aircraft();
    let scenario = parse_scenario(SCENARIO).unwrap();
    let mut s = scenario.initial_state.to_state().unwrap();
    s.velocity_body_mps = Vector3::zeros();
    s.angular_rates_body_radps = Vector3::zeros();
    s.attitude_body_to_ned = soas_core::attitude::Attitude::identity();
    let original = s;
    let sim = FixedStepRk4::new(a.mass_properties().unwrap(), 0.2).unwrap();
    let times = std::cell::RefCell::new(Vec::new());
    let next = sim
        .try_step_with_time(&s, |offset, _| {
            times.borrow_mut().push(offset);
            // Non-gravity acceleration x=t (synthetic force), giving exact delta
            // inertial x velocity h^2/2 for identity attitude. Check times here.
            Ok::<_, String>(soas_core::dynamics::Loads {
                force_body_n: Vector3::x() * (a.mass_kg * offset),
                moment_body_nm: Vector3::zeros(),
            })
        })
        .unwrap();
    assert_eq!(*times.borrow(), vec![0.0, 0.1, 0.1, 0.2]);
    assert_eq!(s, original);
    assert_ne!(next.position_ned_m, s.position_ned_m);
    assert!((next.velocity_body_mps.x - 0.2_f64.powi(2) / 2.0).abs() < 1e-14);
    assert!((next.position_ned_m.x - 0.2_f64.powi(3) / 6.0).abs() < 1e-14);
}

#[test]
fn malformed_scenarios_reject_missing_units_versions_invalid_times_and_schedules() {
    let invalid = |text: String, expected: &str| {
        let error = parse_scenario(&text).unwrap_err().to_string();
        assert!(error.contains(expected), "{error}");
    };
    invalid("{".into(), "JSON");
    invalid(
        SCENARIO.replace("\"format_version\": 0", "\"format_version\": 9"),
        "format_version",
    );
    invalid(
        SCENARIO.replace("\"position_ned_m\"", "\"position_feet\""),
        "position_feet",
    );
    invalid(
        SCENARIO.replace("\"duration_s\": 0.2", "\"duration_s\": -0.2"),
        "duration_s",
    );
    invalid(
        SCENARIO.replace("\"duration_s\": 0.2", "\"duration_s\": 0.201"),
        "duration_s",
    );
    invalid(
        SCENARIO.replace("\"duration_s\": 0.2", "\"duration_s\": 1e-20"),
        "duration_s",
    );
    invalid(
        SCENARIO.replace(
            "\"output_interval_s\": 0.05",
            "\"output_interval_s\": 0.001",
        ),
        "output_interval_s",
    );
    invalid(
        SCENARIO.replace("\"time_s\": 0.1", "\"time_s\": 0.0"),
        "elevator",
    );
    invalid(
        SCENARIO.replace("\"duration_s\": 0.2", "\"duration_s\": 0.3"),
        "controls",
    );
    invalid(
        SCENARIO.replace("\"duration_s\": 0.2", "\"duration_s\": 0.2, \"dt_s\": 0.0"),
        "dt_s",
    );
    invalid(
        SCENARIO.replace("[1.0, 4.0, 10.0]", "[1e400, 4.0, 10.0]"),
        "JSON",
    );
}

#[test]
fn command_errors_are_clear_and_a_failed_run_does_not_replace_existing_output() {
    let scratch = Scratch::new();
    let out = scratch.0.join("run.csv");
    fs::write(&out, "keep this").unwrap();
    let bad = scratch.0.join("bad.json");
    fs::write(&bad, "{").unwrap();
    let result = command()
        .arg("run")
        .arg(&bad)
        .arg("--out")
        .arg(&out)
        .output()
        .unwrap();
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("JSON"));
    assert_eq!(fs::read_to_string(&out).unwrap(), "keep this");
    for args in [
        vec![],
        vec!["run"],
        vec!["run", "input.json", "--unknown", "a.csv"],
        vec!["run", "input.json", "--out", "a.csv", "extra"],
    ] {
        let result = command().args(args).output().unwrap();
        assert!(!result.status.success());
        assert!(String::from_utf8_lossy(&result.stderr).contains("Usage:"));
    }
    let missing = command()
        .arg("run")
        .arg(scratch.0.join("absent.json"))
        .arg("--out")
        .arg(&out)
        .output()
        .unwrap();
    assert!(String::from_utf8_lossy(&missing.stderr).contains("absent.json"));
}

#[test]
fn zero_duration_and_final_output_off_the_regular_interval_are_explicit() {
    let mut s = parse_scenario(SCENARIO).unwrap();
    s.duration_s = 0.0;
    let samples = simulate(&s, &aircraft()).unwrap();
    assert_eq!(samples.len(), 1);
    s.duration_s = 0.2;
    s.output_interval_s = 0.075;
    let samples = simulate(&s, &aircraft()).unwrap();
    let times: Vec<_> = samples.iter().map(|row| row.time_s).collect();
    assert_eq!(times, vec![0.0, 0.075, 0.15, 0.2]);
}

#[test]
fn transition_crossings_have_measured_convergence_instead_of_assuming_fourth_order() {
    let mut a = aircraft();
    // A CONFIGURATION OF THE SAME SYNTHETIC AIRCRAFT, not real data:
    // add a geometric-altitude node at 999.8 m with a different table slope.
    let mut middle = a.aerodynamics.coefficients[0].clone();
    for c in &mut middle {
        c.cl_alpha_per_rad += 1.0;
    }
    a.aerodynamics.geometric_altitude_nodes_m = Some(vec![0.0, 999.8, 20000.0]);
    a.aerodynamics.coefficients.insert(1, middle);
    let mut s: Scenario = parse_scenario(SCENARIO).unwrap();
    s.initial_state.velocity_body_mps = [20.0, 0.0, 6.0];
    s.initial_state.attitude_euler_deg = [0.0; 3];
    s.initial_state.angular_rates_body_radps = [0.0; 3];
    s.duration_s = 0.2;
    for schedule in [
        &mut s.controls.elevator,
        &mut s.controls.aileron,
        &mut s.controls.rudder,
    ] {
        for knot in schedule {
            knot.deflection_deg = 0.0;
        }
    }
    let run = |dt: f64| {
        let mut scenario = s.clone();
        scenario.dt_s = dt;
        scenario.output_interval_s = dt;
        simulate(&scenario, &a).unwrap()
    };
    let reference = run(0.000078125); // 1/12800 s, synthetic numerical reference.
    assert!(reference.iter().any(|row| row.aero.stalled));
    assert!(reference.iter().any(|row| !row.aero.stalled));
    assert!(
        reference
            .windows(2)
            .any(|rows| (-rows[0].state.position_ned_m.z - 999.8)
                * (-rows[1].state.position_ned_m.z - 999.8)
                < 0.0),
        "must CROSS the table node"
    );
    let end = reference.last().unwrap().state;
    let mut errors = Vec::new();
    let crossing = reference
        .windows(2)
        .find(|rows| rows[0].aero.stalled != rows[1].aero.stalled)
        .unwrap();
    println!(
        "lift-limit crossing bracket: [{},{}] s",
        crossing[0].time_s, crossing[1].time_s
    );
    let crossing = reference
        .windows(2)
        .find(|rows| {
            (-rows[0].state.position_ned_m.z - 999.8) * (-rows[1].state.position_ned_m.z - 999.8)
                < 0.0
        })
        .unwrap();
    println!(
        "table-node crossing bracket: [{},{}] s",
        crossing[0].time_s, crossing[1].time_s
    );
    for dt in [
        0.01, 0.005, 0.0025, 0.00125, 0.000625, 0.0003125, 0.00015625,
    ] {
        let result = run(dt);
        let state = result.last().unwrap().state;
        // A dimensionless combined state norm, using explicit synthetic scales
        // 1 m, 1 m/s, 1 rad and 1 rad/s for the four quantities.
        let components = [
            (state.position_ned_m - end.position_ned_m).norm(),
            (state.velocity_body_mps - end.velocity_body_mps).norm(),
            state
                .attitude_body_to_ned
                .angle_to(&end.attitude_body_to_ned),
            (state.angular_rates_body_radps - end.angular_rates_body_radps).norm(),
        ];
        let error = components.iter().map(|x| x * x).sum::<f64>().sqrt();
        println!(
            "transition dt={dt}: position={:.12e}, velocity={:.12e}, attitude={:.12e}, rates={:.12e}, combined={error:.12e}",
            components[0], components[1], components[2], components[3]
        );
        errors.push(error);
    }
    for pair in errors.windows(2) {
        println!(
            "transition observed order={:.6}",
            (pair[0] / pair[1]).log2()
        );
    }
    assert!(
        errors[4] < errors[0] / 4.0,
        "refinement must materially reduce error"
    );
    assert!(
        errors[2] < 1e-2,
        "numerical test bound for this synthetic transition, not aircraft validation"
    );
    let finer = run(0.0000390625);
    let fine = finer.last().unwrap().state;
    let delta = [
        (end.position_ned_m - fine.position_ned_m).norm(),
        (end.velocity_body_mps - fine.velocity_body_mps).norm(),
        end.attitude_body_to_ned
            .angle_to(&fine.attitude_body_to_ned),
        (end.angular_rates_body_radps - fine.angular_rates_body_radps).norm(),
    ];
    let delta_norm = delta.iter().map(|x| x * x).sum::<f64>().sqrt();
    println!("transition fine-reference change={delta_norm:.12e}");
    assert!(delta_norm < 0.1 * errors[2]);
}
