//! Prandtl/Glauert odd Fourier collocation for a straight symmetric wing.
//! Derivation, normalization, sources and numeric/physical limits:
//! data/reference/estimator/lifting_line.md. No flight-core dependency on this crate.
use crate::planform::{WingPlanform, WingStation};
use nalgebra::{DMatrix, DVector};
use soas_core::units::Radians;
use std::{
    error::Error,
    f64::consts::{FRAC_PI_2, PI},
    fmt,
};

const ASSUMPTIONS: &[&str] = &[
    "straight symmetric wing",
    "incompressible flow",
    "attached flow",
    "linear section lift law",
    "small-angle approximation",
    "high-aspect-ratio lifting-line approximation",
    "no sweep correction",
    "no viscous/profile drag",
];

/// Analytic ellipse, separate from the positive-tip-chord station contract.
/// Its quarter-chord lifting line is straight; c(y)=c0*sqrt(1-(2y/b)^2).
#[derive(Debug, Clone)]
pub struct EllipticWing {
    pub span_m: f64,
    pub root_chord_m: f64,
    pub twist_rad: Radians,
    pub section_lift_curve_slope_per_rad: f64,
    pub zero_lift_angle_rad: Radians,
    pub section_source: String,
    pub data_note: String,
}
#[derive(Debug, Clone, Copy)]
pub enum StraightWing<'a> {
    Stations(&'a WingPlanform),
    Elliptic(&'a EllipticWing),
}
#[derive(Debug, Clone, Copy)]
pub struct FourierTerm {
    pub harmonic: usize,
    pub coefficient: f64,
}
#[derive(Debug, Clone, Copy)]
pub struct SectionLoading {
    pub y_m: f64,
    pub chord_m: f64,
    pub section_cl: f64,
    /// Gamma/V, metres; multiply by airspeed to obtain circulation m^2/s.
    pub circulation_over_airspeed_m: f64,
    /// L'/q_dyn = c*cl, metres; multiply by q_dyn to obtain N/m.
    pub lift_per_span_over_dynamic_pressure_m: f64,
    pub induced_angle_rad: Radians,
}
#[derive(Debug, Clone)]
pub struct LiftingLineOutput {
    pub cl: f64,
    pub induced_drag_coefficient: f64,
    /// None when circulation is identically zero; e is then 0/0, undefined.
    pub span_efficiency: Option<f64>,
    /// N odd modes n=1,3,...,2N-1, with Gamma=2*b*V*sum A_n*sin(n*theta).
    pub fourier_terms: Vec<FourierTerm>,
    /// Mirrored interior collocation points, ascending y, center once; no tips.
    pub sections: Vec<SectionLoading>,
    pub relative_collocation_residual: f64,
    pub assumptions: &'static [&'static str],
    pub validity: &'static str,
}
#[derive(Debug, Clone)]
pub struct LiftingLineError(pub String);
impl fmt::Display for LiftingLineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "lifting line: {}", self.0)
    }
}
impl Error for LiftingLineError {}
fn require(ok: bool, message: &str) -> Result<(), LiftingLineError> {
    if ok {
        Ok(())
    } else {
        Err(LiftingLineError(message.into()))
    }
}
impl StraightWing<'_> {
    fn geometry(self) -> Result<(f64, f64, f64), LiftingLineError> {
        match self {
            Self::Stations(wing) => {
                let g = wing
                    .geometry()
                    .map_err(|e| LiftingLineError(e.to_string()))?;
                let root = &wing.stations[0];
                let quarter = root.leading_edge_x_m - root.chord_m / 4.0;
                for s in &wing.stations {
                    // Roundoff allowance in geometry, NOT a sweep/dihedral
                    // validity threshold. No physical correction is introduced.
                    let scale = g
                        .span_m
                        .max(g.mean_aerodynamic_chord_m)
                        .max(quarter.abs())
                        .max(root.z_m.abs())
                        .max(s.leading_edge_x_m.abs())
                        .max(s.z_m.abs());
                    let tolerance = 64.0 * f64::EPSILON * scale;
                    require(
                        ((s.leading_edge_x_m - s.chord_m / 4.0) - quarter).abs() <= tolerance
                            && (s.z_m - root.z_m).abs() <= tolerance,
                        "requires a straight planar quarter-chord line; sweep/dihedral not modeled",
                    )?;
                }
                Ok((g.span_m, g.area_m2, g.aspect_ratio))
            }
            Self::Elliptic(wing) => {
                require(
                    [
                        wing.span_m,
                        wing.root_chord_m,
                        wing.section_lift_curve_slope_per_rad,
                    ]
                    .into_iter()
                    .all(|v| v.is_finite() && v > 0.0),
                    "elliptic span/chord/section slope must be finite and positive",
                )?;
                require(
                    !wing.data_note.trim().is_empty() && !wing.section_source.trim().is_empty(),
                    "elliptic provenance/section source required",
                )?;
                // Integral of c0*sqrt(1-(2y/b)^2): half ellipse area.
                // MIT Lecture 8 elliptic chord + Caughey area definition.
                let area = PI * wing.span_m * wing.root_chord_m / 4.0;
                let ar = wing.span_m * wing.span_m / area;
                require(
                    area.is_finite() && area > 0.0 && ar.is_finite() && ar > 0.0,
                    "elliptic derived geometry overflow/underflow",
                )?;
                Ok((wing.span_m, area, ar))
            }
        }
    }
    fn sample(self, y_m: f64) -> Result<WingStation, LiftingLineError> {
        match self {
            Self::Stations(wing) => wing
                .sample(y_m)
                .map_err(|e| LiftingLineError(e.to_string())),
            Self::Elliptic(wing) => {
                let eta = 2.0 * y_m / wing.span_m;
                let chord = wing.root_chord_m * (1.0 - eta * eta).sqrt();
                require(
                    chord.is_finite() && chord > 0.0,
                    "elliptic collocation chord unresolved; tips are not collocation points",
                )?;
                Ok(WingStation {
                    y_m,
                    chord_m: chord,
                    leading_edge_x_m: chord / 4.0,
                    z_m: 0.0,
                    twist_rad: wing.twist_rad,
                    section_lift_curve_slope_per_rad: wing.section_lift_curve_slope_per_rad,
                    zero_lift_angle_rad: wing.zero_lift_angle_rad,
                    section_source: wing.section_source.clone(),
                })
            }
        }
    }
}

/// N is the count of ODD harmonics, not the largest harmonic. Collocation:
/// theta_i=i*pi/(2N), i=1..N; y=(b/2)*cos(theta). Tip singularities are excluded.
/// MIT Lectures 8/9 give Gamma, section law and induced angle. Substitution gives:
/// sum A_n*sin(n theta)*[4b/(a0*c)+n/sin(theta)] = alpha+twist-alpha_L0.
pub fn solve(
    wing: StraightWing<'_>,
    alpha_rad: Radians,
    terms: usize,
) -> Result<LiftingLineOutput, LiftingLineError> {
    require(terms > 0, "Fourier term count must be positive")?;
    let cells = terms
        .checked_mul(terms)
        .ok_or_else(|| LiftingLineError("Fourier matrix size overflow".into()))?;
    terms
        .checked_mul(2)
        .and_then(|n| n.checked_sub(1))
        .ok_or_else(|| LiftingLineError("harmonic count overflow".into()))?;
    let (span, _area, ar) = wing.geometry()?;
    let mut entries = Vec::new();
    entries
        .try_reserve_exact(cells)
        .map_err(|_| LiftingLineError("unable to allocate Fourier matrix".into()))?;
    entries.resize(cells, 0.0);
    let mut rhs = Vec::with_capacity(terms);
    let mut collocation = Vec::with_capacity(terms);
    for i in 0..terms {
        let theta = (i + 1) as f64 * FRAC_PI_2 / terms as f64;
        let y = if i + 1 == terms {
            0.0
        } else {
            span * 0.5 * theta.cos()
        };
        let section = wing.sample(y)?;
        let section_term =
            4.0 * (span / section.chord_m) / section.section_lift_curve_slope_per_rad;
        let incidence =
            alpha_rad.value() + section.twist_rad.value() - section.zero_lift_angle_rad.value();
        require(
            section_term.is_finite() && incidence.is_finite(),
            "non-finite section equation",
        )?;
        for j in 0..terms {
            let n = (2 * j + 1) as f64;
            let value = (n * theta).sin() * (section_term + n / theta.sin());
            require(value.is_finite(), "non-finite Fourier matrix entry")?;
            entries[i + j * terms] = value; // nalgebra stores columns contiguously.
        }
        rhs.push(incidence);
        collocation.push((theta, section));
    }
    let matrix = DMatrix::from_vec(terms, terms, entries);
    let rhs = DVector::from_vec(rhs);
    let coefficients = matrix
        .clone()
        .lu()
        .solve(&rhs)
        .ok_or_else(|| LiftingLineError("singular Fourier collocation system".into()))?;
    require(
        coefficients.iter().all(|a| a.is_finite()),
        "non-finite Fourier solution",
    )?;
    // Netlib Templates, stopping Criterion 1 (see reference note): normwise
    // backward residual, not a physical validity or lift-accuracy test.
    let residual = &matrix * &coefficients - &rhs;
    let matrix_norm = (0..terms)
        .map(|i| matrix.row(i).iter().map(|v| v.abs()).sum::<f64>())
        .fold(0.0, f64::max);
    let max_abs = |v: &DVector<f64>| v.iter().map(|v| v.abs()).fold(0.0, f64::max);
    let denominator = matrix_norm * max_abs(&coefficients) + max_abs(&rhs);
    require(denominator.is_finite(), "residual scaling overflow")?;
    let relative_residual = if denominator == 0.0 {
        max_abs(&residual)
    } else {
        max_abs(&residual) / denominator
    };
    require(
        relative_residual.is_finite() && relative_residual <= 128.0 * f64::EPSILON * terms as f64,
        "Fourier linear solve failed numerical residual check",
    )?;
    let fourier_terms: Vec<_> = coefficients
        .iter()
        .enumerate()
        .map(|(j, a)| FourierTerm {
            harmonic: 2 * j + 1,
            coefficient: *a,
        })
        .collect();
    // MIT Lecture 9: CL=pi*AR*A1; CDi=pi*AR*sum(n*A_n^2), e=A1^2/sum.
    let energy = fourier_terms
        .iter()
        .map(|t| t.harmonic as f64 * t.coefficient * t.coefficient)
        .sum::<f64>();
    require(
        energy.is_finite() && (energy > 0.0 || coefficients.iter().all(|a| *a == 0.0)),
        "induced drag sum overflow/underflow",
    )?;
    let cl = PI * ar * coefficients[0];
    let cdi = PI * ar * energy;
    let efficiency = if energy == 0.0 {
        None
    } else {
        Some(coefficients[0] * coefficients[0] / energy)
    };
    require(
        cl.is_finite()
            && cdi.is_finite()
            && (energy == 0.0 || cdi > 0.0)
            && efficiency.is_none_or(|e| e.is_finite() && (0.0..=1.0).contains(&e)),
        "non-finite or unrepresentable lift/drag/efficiency",
    )?;
    let mut positive = Vec::with_capacity(terms);
    for (theta, section) in collocation {
        let sum = fourier_terms
            .iter()
            .map(|t| t.coefficient * (t.harmonic as f64 * theta).sin())
            .sum::<f64>();
        let induced = fourier_terms
            .iter()
            .map(|t| {
                t.harmonic as f64 * t.coefficient * (t.harmonic as f64 * theta).sin() / theta.sin()
            })
            .sum::<f64>();
        let gamma_over_v = 2.0 * span * sum;
        let loading = 2.0 * gamma_over_v;
        let section_cl = loading / section.chord_m;
        require(
            [gamma_over_v, loading, section_cl, induced]
                .into_iter()
                .all(f64::is_finite),
            "section loading overflow",
        )?;
        positive.push(SectionLoading {
            y_m: section.y_m,
            chord_m: section.chord_m,
            section_cl,
            circulation_over_airspeed_m: gamma_over_v,
            lift_per_span_over_dynamic_pressure_m: loading,
            induced_angle_rad: Radians::new(induced).expect("checked finite"),
        });
    }
    let mut sections: Vec<_> = positive[..terms - 1]
        .iter()
        .map(|s| SectionLoading { y_m: -s.y_m, ..*s })
        .collect();
    sections.extend(positive.into_iter().rev());
    Ok(LiftingLineOutput {
        cl,
        induced_drag_coefficient: cdi,
        span_efficiency: efficiency,
        fourier_terms,
        sections,
        relative_collocation_residual: relative_residual,
        assumptions: ASSUMPTIONS,
        validity: "validity unassessed",
    })
}
