//! Published checkpoints: data/reference/atmosphere/README.md, USSA 1976
//! Tables I and III. Expected values precede the atmosphere implementation.

use soas_core::atmosphere::{
    AtmosphereError, dynamic_pressure_pa, isa_at_geometric_altitude, isa_at_geopotential_altitude,
    mach_number,
};

// Owner-authorized table comparison tolerance (QUESTIONS.md, 2026-11-13).
const TABLE_RELATIVE_TOLERANCE: f64 = 5e-5;

fn relative_error(actual: f64, printed: f64) -> f64 {
    (actual - printed).abs() / printed.abs()
}

fn matches_table(actual: f64, printed: f64) -> bool {
    relative_error(actual, printed) <= TABLE_RELATIVE_TOLERANCE
}

#[test]
fn published_geopotential_table_checkpoints_match_printed_precision() {
    // H_m, T_K, P_Pa, rho_kg/m3, a_m/s; rechecked against the scanned pages.
    for (h, temperature, pressure, density, sound) in [
        (0.0, 288.150, 101325.0, 1.2250, 340.29),
        (5000.0, 255.650, 54019.0, 0.73612, 320.53),
        (11000.0, 216.650, 22632.0, 0.36392, 295.07),
        (20000.0, 216.650, 5474.9, 0.088035, 295.07),
    ] {
        let actual = isa_at_geopotential_altitude(h).unwrap();
        assert!(
            (actual.temperature_k - temperature).abs() <= 0.0005,
            "H={h}"
        );
        println!(
            "H={h}: P={:.12}, rho={:.12}, relative errors P={:.9e}, rho={:.9e}",
            actual.pressure_pa,
            actual.density_kg_m3,
            relative_error(actual.pressure_pa, pressure),
            relative_error(actual.density_kg_m3, density)
        );
        assert!(matches_table(actual.pressure_pa, pressure), "H={h}");
        assert!(matches_table(actual.density_kg_m3, density), "H={h}");
        assert!((actual.speed_of_sound_mps - sound).abs() <= 0.005, "H={h}");
    }
}

#[test]
fn geometric_height_has_its_own_published_checkpoint() {
    let actual = isa_at_geometric_altitude(20000.0).unwrap();
    assert!((actual.temperature_k - 216.650).abs() <= 0.0005);
    println!(
        "Z=20000: P={:.12}, rho={:.12}, relative errors P={:.9e}, rho={:.9e}",
        actual.pressure_pa,
        actual.density_kg_m3,
        relative_error(actual.pressure_pa, 5529.3),
        relative_error(actual.density_kg_m3, 0.088910)
    );
    assert!(matches_table(actual.pressure_pa, 5529.3));
    assert!(matches_table(actual.density_kg_m3, 0.088910));
    assert!((actual.speed_of_sound_mps - 295.07).abs() <= 0.005);
    assert!(actual.pressure_pa > isa_at_geopotential_altitude(20000.0).unwrap().pressure_pa);
}

// Independent sensitivity calculation, NOT a second reference table. The
// deliberately wrong constants below were requested by the owner. Applying
// the SAME comparison predicate shows that the authorized tolerance rejects
// each mistake, rather than merely asserting that the wrong output differs.
fn perturbed_pressure_and_density(lapse: f64, gravity: f64, gas_constant: f64) -> (f64, f64) {
    let t11 = 288.15 + lapse * 11000.0;
    let p11 = 101325.0 * (t11 / 288.15).powf(-gravity / (gas_constant * lapse));
    let pressure = p11 * (-gravity * 9000.0 / (gas_constant * t11)).exp();
    (pressure, pressure / (gas_constant * t11))
}

#[test]
fn table_tolerance_rejects_each_requested_physical_mistake() {
    let r = 8314.32 / 28.9644;
    for (name, lapse, gravity, gas_constant) in [
        ("lapse 6.0 K/km", -0.006, 9.80665, r),
        ("gravity 9.81", -0.0065, 9.81, r),
        ("gas constant +0.1%", -0.0065, 9.80665, r * 1.001),
        ("gas constant -0.1%", -0.0065, 9.80665, r * 0.999),
    ] {
        let (pressure, density) = perturbed_pressure_and_density(lapse, gravity, gas_constant);
        let p_error = relative_error(pressure, 5474.9);
        let rho_error = relative_error(density, 0.088035);
        println!(
            "{name} at H=20000: P error={p_error:.9e} ({:.3}x tolerance), rho error={rho_error:.9e} ({:.3}x tolerance)",
            p_error / TABLE_RELATIVE_TOLERANCE,
            rho_error / TABLE_RELATIVE_TOLERANCE
        );
        assert!(!matches_table(pressure, 5474.9), "{name}");
        assert!(!matches_table(density, 0.088035), "{name}");
    }
    let wrong_height = isa_at_geometric_altitude(20000.0).unwrap();
    let error = relative_error(wrong_height.pressure_pa, 5474.9);
    println!(
        "geometric in place of geopotential: P error={error:.9e} ({:.3}x tolerance)",
        error / TABLE_RELATIVE_TOLERANCE
    );
    assert!(!matches_table(wrong_height.pressure_pa, 5474.9));
    assert!(!matches_table(wrong_height.density_kg_m3, 0.088035));
    // The owner's printed-value pair also fails without involving our model.
    assert!(!matches_table(5529.3, 5474.9));
}

#[test]
fn geopotential_range_and_non_finite_inputs_are_rejected() {
    for h in [-1.0, 20001.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert_eq!(
            isa_at_geopotential_altitude(h),
            Err(AtmosphereError::AltitudeOutOfRange)
        );
    }
    assert!(isa_at_geopotential_altitude(0.0).is_ok());
    assert!(isa_at_geopotential_altitude(20000.0).is_ok());
}

#[test]
fn geometric_range_is_checked_before_height_conversion() {
    for z in [-1.0, 20001.0, f64::NAN, f64::INFINITY] {
        assert_eq!(
            isa_at_geometric_altitude(z),
            Err(AtmosphereError::AltitudeOutOfRange)
        );
    }
    assert!(isa_at_geometric_altitude(0.0).is_ok());
    assert!(isa_at_geometric_altitude(20000.0).is_ok());
}

#[test]
fn tropopause_temperature_and_pressure_are_continuous() {
    // A synthetic small numerical interval around the published layer boundary.
    let below = isa_at_geopotential_altitude(11000.0 - 0.001).unwrap();
    let above = isa_at_geopotential_altitude(11000.0 + 0.001).unwrap();
    assert!((above.temperature_k - below.temperature_k).abs() < 0.0001);
    assert!((above.pressure_pa - below.pressure_pa).abs() < 0.01);
    assert!(above.pressure_pa < below.pressure_pa);
}

#[test]
fn mach_and_dynamic_pressure_match_hand_calculated_synthetic_cases() {
    assert_eq!(mach_number(150.0, 300.0).unwrap(), 0.5);
    assert_eq!(mach_number(0.0, 300.0).unwrap(), 0.0);
    assert_eq!(dynamic_pressure_pa(2.0, 3.0).unwrap(), 9.0);
    assert_eq!(dynamic_pressure_pa(2.0, 6.0).unwrap(), 36.0);
    assert_eq!(dynamic_pressure_pa(0.0, 3.0).unwrap(), 0.0);
    assert_eq!(dynamic_pressure_pa(2.0, 0.0).unwrap(), 0.0);
}

#[test]
fn invalid_air_data_is_an_error() {
    for speed in [-1.0, f64::NAN, f64::INFINITY] {
        assert!(mach_number(speed, 300.0).is_err());
        assert!(dynamic_pressure_pa(1.0, speed).is_err());
    }
    for sound in [0.0, -1.0, f64::NAN, f64::INFINITY] {
        assert!(mach_number(1.0, sound).is_err());
    }
    for density in [-1.0, f64::NAN, f64::INFINITY] {
        assert!(dynamic_pressure_pa(density, 1.0).is_err());
    }
}

#[test]
fn finite_input_overflow_does_not_escape_as_infinity() {
    assert!(mach_number(f64::MAX, f64::MIN_POSITIVE).is_err());
    assert!(dynamic_pressure_pa(2.0, f64::MAX).is_err());
}
