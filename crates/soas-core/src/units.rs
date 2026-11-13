//! Angle types required by D4. Degrees belong at input/report boundaries;
//! simulation calculations use radians. These types do not wrap angle ranges.

use std::{error::Error, fmt};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NonFiniteAngle;

impl fmt::Display for NonFiniteAngle {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("an angle or its conversion is not finite")
    }
}

impl Error for NonFiniteAngle {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Radians(f64);

impl Radians {
    pub fn new(value_rad: f64) -> Result<Self, NonFiniteAngle> {
        if value_rad.is_finite() {
            Ok(Self(value_rad))
        } else {
            Err(NonFiniteAngle)
        }
    }

    pub fn value(self) -> f64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Degrees(f64);

impl Degrees {
    pub fn new(value_deg: f64) -> Result<Self, NonFiniteAngle> {
        if value_deg.is_finite() {
            Ok(Self(value_deg))
        } else {
            Err(NonFiniteAngle)
        }
    }

    pub fn value(self) -> f64 {
        self.0
    }
}

// Rust's conversions implement the mathematical identities pi rad = 180 deg:
// https://doc.rust-lang.org/std/primitive.f64.html#method.to_radians
impl TryFrom<Degrees> for Radians {
    type Error = NonFiniteAngle;

    fn try_from(value: Degrees) -> Result<Self, Self::Error> {
        Self::new(value.value().to_radians())
    }
}

impl TryFrom<Radians> for Degrees {
    type Error = NonFiniteAngle;

    fn try_from(value: Radians) -> Result<Self, Self::Error> {
        Self::new(value.value().to_degrees())
    }
}
