//! Dry standard atmosphere over the project's 0..=20 km range.
//!
//! Source: U.S. Standard Atmosphere 1976, Tables 2 and 4 (with corrected R*
//! and radius values from pages 3/4 and the appended errata), equations 18, 23,
//! 33a/33b, 42 and 50. See data/reference/atmosphere/README.md for the
//! transcribed independent table checkpoints and exact source locations.
//! https://ntrs.nasa.gov/citations/19770009539

use std::{error::Error, fmt};

// Physical values below come from that source, not from aircraft estimates.
pub const STANDARD_GRAVITY_MPS2: f64 = 9.80665;
const SEA_LEVEL_TEMPERATURE_K: f64 = 288.15;
const SEA_LEVEL_PRESSURE_PA: f64 = 101325.0;
const GAS_CONSTANT_J_PER_KMOL_K: f64 = 8314.32;
const MOLAR_MASS_KG_PER_KMOL: f64 = 28.9644;
const AIR_GAS_CONSTANT_J_PER_KG_K: f64 = GAS_CONSTANT_J_PER_KMOL_K / MOLAR_MASS_KG_PER_KMOL;
const SPECIFIC_HEAT_RATIO: f64 = 1.4;
const EFFECTIVE_EARTH_RADIUS_M: f64 = 6356766.0;
const TROPOPAUSE_GEOPOTENTIAL_M: f64 = 11000.0;
const TROPOSPHERE_LAPSE_K_PER_M: f64 = -0.0065;
// Project limit from T3, not the full extent of the published atmosphere.
const MAX_ALTITUDE_M: f64 = 20000.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Atmosphere {
    pub temperature_k: f64,
    pub pressure_pa: f64,
    pub density_kg_m3: f64,
    pub speed_of_sound_mps: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AtmosphereError {
    AltitudeOutOfRange,
    InvalidAirData,
    NonFiniteResult,
}

impl fmt::Display for AtmosphereError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::AltitudeOutOfRange => "altitude must be finite and between 0 and 20000 metres",
            Self::InvalidAirData => {
                "air data must be finite, with nonnegative speed/density and positive sound speed"
            }
            Self::NonFiniteResult => "air-data calculation overflowed",
        })
    }
}

impl Error for AtmosphereError {}

/// Evaluate at geopotential height H in the inclusive range [0, 20000] metres.
/// No geometric-height default is implied; callers choose the named entry point.
pub fn isa_at_geopotential_altitude(altitude_m: f64) -> Result<Atmosphere, AtmosphereError> {
    validate_altitude(altitude_m)?;
    let temperature_11_k =
        SEA_LEVEL_TEMPERATURE_K + TROPOSPHERE_LAPSE_K_PER_M * TROPOPAUSE_GEOPOTENTIAL_M;
    let pressure_exponent =
        -STANDARD_GRAVITY_MPS2 / (AIR_GAS_CONSTANT_J_PER_KG_K * TROPOSPHERE_LAPSE_K_PER_M);
    let (temperature_k, pressure_pa) = if altitude_m <= TROPOPAUSE_GEOPOTENTIAL_M {
        let temperature_k = SEA_LEVEL_TEMPERATURE_K + TROPOSPHERE_LAPSE_K_PER_M * altitude_m;
        let pressure_pa = SEA_LEVEL_PRESSURE_PA
            * (temperature_k / SEA_LEVEL_TEMPERATURE_K).powf(pressure_exponent);
        (temperature_k, pressure_pa)
    } else {
        // Derive the layer base from the preceding layer rather than rounding
        // to the printed pressure used by the independent reference test.
        let pressure_11_pa = SEA_LEVEL_PRESSURE_PA
            * (temperature_11_k / SEA_LEVEL_TEMPERATURE_K).powf(pressure_exponent);
        let pressure_pa = pressure_11_pa
            * (-STANDARD_GRAVITY_MPS2 * (altitude_m - TROPOPAUSE_GEOPOTENTIAL_M)
                / (AIR_GAS_CONSTANT_J_PER_KG_K * temperature_11_k))
                .exp();
        (temperature_11_k, pressure_pa)
    };
    Ok(Atmosphere {
        temperature_k,
        pressure_pa,
        density_kg_m3: pressure_pa / (AIR_GAS_CONSTANT_J_PER_KG_K * temperature_k),
        speed_of_sound_mps: (SPECIFIC_HEAT_RATIO * AIR_GAS_CONSTANT_J_PER_KG_K * temperature_k)
            .sqrt(),
    })
}

/// Evaluate at geometric height Z, in metres, after the standard's H(Z)
/// conversion. The inclusive [0, 20000] metre range is checked in geometric
/// metres before conversion, so its upper endpoint differs from the H API's.
pub fn isa_at_geometric_altitude(altitude_m: f64) -> Result<Atmosphere, AtmosphereError> {
    validate_altitude(altitude_m)?;
    let geopotential_m =
        EFFECTIVE_EARTH_RADIUS_M * altitude_m / (EFFECTIVE_EARTH_RADIUS_M + altitude_m);
    isa_at_geopotential_altitude(geopotential_m)
}

pub fn mach_number(airspeed_mps: f64, speed_of_sound_mps: f64) -> Result<f64, AtmosphereError> {
    if !airspeed_mps.is_finite()
        || airspeed_mps < 0.0
        || !speed_of_sound_mps.is_finite()
        || speed_of_sound_mps <= 0.0
    {
        return Err(AtmosphereError::InvalidAirData);
    }
    finite_result(airspeed_mps / speed_of_sound_mps)
}

/// Dynamic pressure q = rho V^2 / 2, as specified in CONVENTIONS.md.
pub fn dynamic_pressure_pa(density_kg_m3: f64, airspeed_mps: f64) -> Result<f64, AtmosphereError> {
    if !density_kg_m3.is_finite()
        || density_kg_m3 < 0.0
        || !airspeed_mps.is_finite()
        || airspeed_mps < 0.0
    {
        return Err(AtmosphereError::InvalidAirData);
    }
    finite_result(0.5 * density_kg_m3 * airspeed_mps * airspeed_mps)
}

fn validate_altitude(altitude_m: f64) -> Result<(), AtmosphereError> {
    if altitude_m.is_finite() && (0.0..=MAX_ALTITUDE_M).contains(&altitude_m) {
        Ok(())
    } else {
        Err(AtmosphereError::AltitudeOutOfRange)
    }
}

fn finite_result(value: f64) -> Result<f64, AtmosphereError> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(AtmosphereError::NonFiniteResult)
    }
}
