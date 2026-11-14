//! Mirrored station planforms, with exact integration of linear panels.
//! Equations and coordinate interpretation: data/reference/estimator/geometry.md.
use soas_core::units::Radians;
use std::{error::Error, fmt};

#[derive(Debug, Clone, PartialEq)]
pub struct WingStation {
    pub y_m: f64,
    pub chord_m: f64,
    /// Body x forward; trailing edge is leading_edge_x_m - chord_m.
    pub leading_edge_x_m: f64,
    /// Body z down, relative to the same wing/file datum as leading-edge x.
    pub z_m: f64,
    /// Positive incidence increases local angle of attack.
    pub twist_rad: Radians,
    pub section_lift_curve_slope_per_rad: f64,
    pub zero_lift_angle_rad: Radians,
    /// Citation/applicability or explicit synthetic test provenance.
    pub section_source: String,
}
#[derive(Debug, Clone, PartialEq)]
pub struct WingPlanform {
    pub data_note: String,
    /// Root y=0 through right tip; all fields interpolate linearly in y.
    pub stations: Vec<WingStation>,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WingGeometry {
    pub span_m: f64,
    pub area_m2: f64,
    pub aspect_ratio: f64,
    pub taper_ratio: f64,
    pub mean_aerodynamic_chord_m: f64,
    /// Equivalent MAC at chord-area-weighted right-half location. Left mirrors y.
    pub mac_leading_edge_right_body_m: [f64; 3],
    pub mac_quarter_chord_right_body_m: [f64; 3],
    /// Net root-to-tip PROJECTED quarter-chord sweep, positive aft.
    pub quarter_chord_sweep_rad: Radians,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WingError {
    pub field: String,
    pub reason: &'static str,
}
impl fmt::Display for WingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "wing {}: {}", self.field, self.reason)
    }
}
impl Error for WingError {}
fn require(ok: bool, field: impl Into<String>, reason: &'static str) -> Result<(), WingError> {
    if ok {
        Ok(())
    } else {
        Err(WingError {
            field: field.into(),
            reason,
        })
    }
}
/// Caughey, AeroBackground eq. 2.15 (thin-airfoil, incompressible linear theory):
/// https://courses.cit.cornell.edu/mae5070/AeroBackground.pdf#page=5
/// Explicit opt-in helper only; station data and alpha_L0 are never inferred.
pub fn thin_airfoil_slope_per_rad() -> f64 {
    2.0 * std::f64::consts::PI
}

impl WingPlanform {
    pub fn validate(&self) -> Result<(), WingError> {
        require(
            !self.data_note.trim().is_empty(),
            "data_note",
            "must identify provenance or synthetic test data",
        )?;
        require(
            self.stations.len() >= 2,
            "stations",
            "need at least root and tip",
        )?;
        for (i, s) in self.stations.iter().enumerate() {
            let path = format!("stations[{i}]");
            for (field, value) in [
                ("y_m", s.y_m),
                ("chord_m", s.chord_m),
                ("leading_edge_x_m", s.leading_edge_x_m),
                ("z_m", s.z_m),
                (
                    "section_lift_curve_slope_per_rad",
                    s.section_lift_curve_slope_per_rad,
                ),
            ] {
                require(
                    value.is_finite(),
                    format!("{path}.{field}"),
                    "must be finite",
                )?;
            }
            require(
                s.chord_m > 0.0,
                format!("{path}.chord_m"),
                "must be positive",
            )?;
            require(
                s.section_lift_curve_slope_per_rad > 0.0,
                format!("{path}.section_lift_curve_slope_per_rad"),
                "must be positive",
            )?;
            require(
                s.y_m >= 0.0 && (i == 0 || s.y_m > self.stations[i - 1].y_m),
                format!("{path}.y_m"),
                "right-half stations must be nonnegative and strictly increasing",
            )?;
            require(
                !s.section_source.trim().is_empty(),
                format!("{path}.section_source"),
                "citation or explicit synthetic provenance required",
            )?;
        }
        require(
            self.stations[0].y_m == 0.0,
            "stations[0].y_m",
            "root must be at the symmetry plane y=0",
        )
    }
    pub fn geometry(&self) -> Result<WingGeometry, WingError> {
        self.validate()?;
        let root = &self.stations[0];
        let tip = self.stations.last().expect("validated stations");
        let span = 2.0 * tip.y_m;
        let mut area = 0.0;
        let mut chord_squared = 0.0;
        let mut moments = [0.0; 3];
        for pair in self.stations.windows(2) {
            let (a, b) = (&pair[0], &pair[1]);
            let dy = b.y_m - a.y_m;
            area += dy * (a.chord_m + b.chord_m);
            // 2 * integral c^2 dy for a linear chord; NASA NDARC wing panels.
            chord_squared +=
                2.0 * dy * (a.chord_m * a.chord_m + a.chord_m * b.chord_m + b.chord_m * b.chord_m)
                    / 3.0;
            // 2*integral c*f dy, exact for linear c and f. Cornell eq. 2.12;
            // apply the same area-first-moment definition to x, y and z.
            for (axis, (f0, f1)) in [
                (a.leading_edge_x_m, b.leading_edge_x_m),
                (a.y_m, b.y_m),
                (a.z_m, b.z_m),
            ]
            .into_iter()
            .enumerate()
            {
                moments[axis] += dy
                    * (2.0 * a.chord_m * f0
                        + a.chord_m * f1
                        + b.chord_m * f0
                        + 2.0 * b.chord_m * f1)
                    / 3.0;
            }
        }
        let mac = chord_squared / area;
        let le = moments.map(|m| m / area);
        let quarter = [le[0] - mac / 4.0, le[1], le[2]];
        let root_quarter = root.leading_edge_x_m - root.chord_m / 4.0;
        let tip_quarter = tip.leading_edge_x_m - tip.chord_m / 4.0;
        let aft_quarter_offset = root_quarter - tip_quarter;
        let sweep = aft_quarter_offset.atan2(tip.y_m);
        let ar = span * span / area;
        let taper = tip.chord_m / root.chord_m;
        require(
            [span, area, mac, ar, taper]
                .into_iter()
                .all(|v| v.is_finite() && v > 0.0)
                && le.into_iter().chain(quarter).all(f64::is_finite)
                && root_quarter.is_finite()
                && tip_quarter.is_finite()
                && aft_quarter_offset.is_finite()
                && sweep.is_finite(),
            "derived_geometry",
            "non-finite, overflowed or nonpositive derived quantity",
        )?;
        Ok(WingGeometry {
            span_m: span,
            area_m2: area,
            aspect_ratio: ar,
            taper_ratio: taper,
            mean_aerodynamic_chord_m: mac,
            mac_leading_edge_right_body_m: le,
            mac_quarter_chord_right_body_m: quarter,
            quarter_chord_sweep_rad: Radians::new(sweep).expect("checked finite"),
        })
    }
    pub fn sample(&self, y_m: f64) -> Result<WingStation, WingError> {
        self.validate()?;
        let y = y_m.abs();
        require(
            y.is_finite() && y <= self.stations.last().expect("validated stations").y_m,
            "sample.y_m",
            "must be finite and inside mirrored span; no extrapolation",
        )?;
        for station in &self.stations {
            if station.y_m == y {
                let mut out = station.clone();
                out.y_m = y_m;
                return Ok(out);
            }
        }
        for pair in self.stations.windows(2) {
            let (a, b) = (&pair[0], &pair[1]);
            if y > a.y_m && y < b.y_m {
                let f = (y - a.y_m) / (b.y_m - a.y_m);
                let mix = |x: f64, z: f64| (1.0 - f) * x + f * z;
                let angle = |x: Radians, z: Radians| {
                    Radians::new(mix(x.value(), z.value())).map_err(|_| WingError {
                        field: "sample.angle".into(),
                        reason: "interpolation overflow",
                    })
                };
                let out = WingStation {
                    y_m,
                    chord_m: mix(a.chord_m, b.chord_m),
                    leading_edge_x_m: mix(a.leading_edge_x_m, b.leading_edge_x_m),
                    z_m: mix(a.z_m, b.z_m),
                    twist_rad: angle(a.twist_rad, b.twist_rad)?,
                    section_lift_curve_slope_per_rad: mix(
                        a.section_lift_curve_slope_per_rad,
                        b.section_lift_curve_slope_per_rad,
                    ),
                    zero_lift_angle_rad: angle(a.zero_lift_angle_rad, b.zero_lift_angle_rad)?,
                    section_source: format!(
                        "Linear interpolation: [{}]; [{}]",
                        a.section_source, b.section_source
                    ),
                };
                require(
                    [
                        out.chord_m,
                        out.leading_edge_x_m,
                        out.z_m,
                        out.section_lift_curve_slope_per_rad,
                    ]
                    .into_iter()
                    .all(f64::is_finite)
                        && out.chord_m > 0.0
                        && out.section_lift_curve_slope_per_rad > 0.0,
                    "sample",
                    "interpolation overflow",
                )?;
                return Ok(out);
            }
        }
        unreachable!("validated finite in-range sample")
    }
}
