use crate::*;
use nalgebra::Matrix3;
use soas_core::{aircraft::AircraftError, propulsion::Engine};
use std::collections::BTreeSet;

pub(crate) fn error(field: &str, reason: &'static str) -> DesignError {
    DesignError {
        field: field.into(),
        reason,
    }
}
fn require(ok: bool, field: &str, reason: &'static str) -> Result<(), DesignError> {
    if ok {
        Ok(())
    } else {
        Err(error(field, reason))
    }
}
fn text(value: &str, field: &str) -> Result<(), DesignError> {
    require(!value.trim().is_empty(), field, "must not be empty")
}
fn positive(value: f64, field: &str) -> Result<(), DesignError> {
    require(
        value.is_finite() && value > 0.0,
        field,
        "must be finite and positive",
    )
}
fn finite(values: &[f64], field: &str) -> Result<(), DesignError> {
    require(
        values.iter().all(|v| v.is_finite()),
        field,
        "must be finite",
    )
}
fn identity(id: &str, field: &str, seen: &mut BTreeSet<String>) -> Result<(), DesignError> {
    require(
        !id.is_empty()
            && id
                .bytes()
                .all(|c| c.is_ascii_alphanumeric() || c == b'-' || c == b'_'),
        field,
        "identity must contain only ASCII letters, digits, hyphens or underscores",
    )?;
    require(seen.insert(id.into()), field, "duplicate identity")
}
fn core_error(e: AircraftError) -> DesignError {
    match e {
        AircraftError::Validation { field, reason } => DesignError { field, reason },
        _ => error("engine_tables", "invalid engine data"),
    }
}

impl Design {
    /// Draft validity, not export readiness or physical/experimental validation.
    pub fn validate(&self) -> Result<(), DesignError> {
        text(&self.name, "name")?;
        text(&self.datum, "datum")?;
        if self.identity == DesignIdentity::SyntheticTemplate {
            require(
                matches!(
                    self.name.as_str(),
                    "Synthetic aircraft study" | "Synthetic wing study"
                ),
                "name",
                "synthetic template requires a registered neutral name",
            )?;
        }
        require(
            self.real_aircraft_claim.is_none(),
            "real_aircraft_claim",
            "real aircraft identity requires a future validated-data contract",
        )?;
        let mut seen = BTreeSet::new();
        for c in &self.components {
            identity(&c.id, &format!("components[{}].id", c.id), &mut seen)?;
        }
        for e in &self.engine_tables {
            identity(&e.id, &format!("engine_tables[{}].id", e.id), &mut seen)?;
        }
        for c in &self.controls {
            identity(&c.id, &format!("controls[{}].id", c.id), &mut seen)?;
        }
        for c in &self.contributions {
            identity(&c.id, &format!("contributions[{}].id", c.id), &mut seen)?;
        }
        for e in &self.engine_tables {
            let p = format!("engine_tables[{}]", e.id);
            text(&e.source, &format!("{p}.source"))?;
            e.table
                .validate(&format!("{p}.table"))
                .map_err(core_error)?;
            require(
                e.dry_throttle_max.is_finite()
                    && e.dry_throttle_max > 0.0
                    && e.dry_throttle_max < 1.0
                    && e.table.throttle_nodes.contains(&e.dry_throttle_max),
                &format!("{p}.dry_throttle_max"),
                "must be an explicit throttle node strictly between 0 and 1",
            )?;
        }
        for c in &self.components {
            let p = format!("components[{}]", c.id);
            text(&c.name, &format!("{p}.name"))?;
            match &c.geometry {
                ComponentGeometry::Surface(s) => validate_surface(s, &format!("{p}.surface"))?,
                ComponentGeometry::Fuselage(f) => validate_fuselage(f, &format!("{p}.fuselage"))?,
                ComponentGeometry::Engine(e) => {
                    let p = format!("{p}.engine");
                    let table = self
                        .engine_tables
                        .iter()
                        .find(|t| t.id == e.table_id)
                        .ok_or_else(|| error(&format!("{p}.table_id"), "unknown engine table"))?;
                    // Reuse the existing core Engine rule and its unchanged
                    // unit-direction tolerance, without constructing aircraft loads.
                    Engine {
                        name: c.name.clone(),
                        data_note: table.source.clone(),
                        position_body_m: e.position_body_m,
                        direction_body_unit: e.direction_body_unit,
                        dry_throttle_max: table.dry_throttle_max,
                        table: table.table.clone(),
                    }
                    .validate(&p)
                    .map_err(core_error)?;
                }
            }
            if let Some(m) = c.mass.mass_kg {
                positive(m, &format!("{p}.mass.mass_kg"))?;
            }
            if let Some(cg) = c.mass.cg_body_m {
                finite(&cg, &format!("{p}.mass.cg_body_m"))?;
            }
            if let Some(i) = &c.mass.intrinsic_inertia {
                // Same signed, exact-symmetry/SPD/invertibility checks as core
                // dynamics::MassProperties; no shift or mass buildup is done.
                let tensor = Matrix3::from_fn(|r, c| i.tensor_kg_m2[r][c]);
                require(
                    tensor.iter().all(|v| v.is_finite())
                        && tensor == tensor.transpose()
                        && tensor
                            .cholesky()
                            .is_some_and(|ch| ch.inverse().iter().all(|v| v.is_finite())),
                    &format!("{p}.mass.intrinsic_inertia"),
                    "invalid signed intrinsic inertia tensor",
                )?;
            }
        }
        let surfaces: BTreeSet<_> = self
            .components
            .iter()
            .filter(|c| matches!(c.geometry, ComponentGeometry::Surface(_)))
            .map(|c| c.id.as_str())
            .collect();
        for c in &self.controls {
            let p = format!("controls[{}]", c.id);
            require(
                surfaces.contains(c.parent_surface_id.as_str()),
                &format!("{p}.parent_surface_id"),
                "must reference a lifting surface",
            )?;
            finite(&c.hinge_start_body_m, &format!("{p}.hinge_start_body_m"))?;
            finite(&c.hinge_end_body_m, &format!("{p}.hinge_end_body_m"))?;
            require(
                c.hinge_start_body_m != c.hinge_end_body_m,
                &format!("{p}.hinge"),
                "hinge endpoints must differ",
            )?;
            require(
                c.chord_fraction.is_finite() && c.chord_fraction > 0.0 && c.chord_fraction <= 1.0,
                &format!("{p}.chord_fraction"),
                "must be a finite fraction in (0,1]",
            )?;
            require(
                c.min_rad.value() <= c.max_rad.value(),
                &format!("{p}.limits"),
                "minimum must not exceed maximum",
            )?;
        }
        for c in &self.contributions {
            let p = format!("contributions[{}]", c.id);
            // Quantity identity is checked before label syntax or scope, so
            // neither separators/casing nor another component scope bypass it.
            for a in &c.parameters {
                require(
                    !matches!(a.quantity, QuantityIdentity::ComputedWing(_))
                        && ComputedWingQuantity::from_name(&a.name).is_none(),
                    &format!("{p}.parameters[{}]", a.name),
                    "computed wing quantities cannot be supplied",
                )?;
            }
            require(
                !matches!(
                    c.scope,
                    ContributionScope::Wing | ContributionScope::WholeAircraft
                ),
                &format!("{p}.scope"),
                "static wing or opaque aircraft totals are forbidden",
            )?;
            text(&c.source, &format!("{p}.source"))?;
            text(&c.coverage, &format!("{p}.coverage"))?;
            let mut names = BTreeSet::new();
            for a in &c.parameters {
                let p = format!("{p}.parameters[{}]", a.name);
                identity(&a.name, &p, &mut names)?;
                text(&a.unit, &p)?;
                if let Some(v) = a.value {
                    finite(&[v], &p)?;
                }
                if c.scope == ContributionScope::ClMax {
                    require(a.name == "cl_max", &p, "CLmax scope accepts only cl_max")?;
                    if let Some(v) = a.value {
                        positive(v, &p)?;
                    }
                }
            }
        }
        let numbers = self.numbers();
        let paths: BTreeSet<_> = numbers.iter().map(|n| n.path.as_str()).collect();
        for n in &numbers {
            let meta = self
                .provenance
                .get(&n.path)
                .ok_or_else(|| error(&n.path, "numeric provenance missing"))?;
            validate_metadata(meta, n, &paths)?;
        }
        for path in self.provenance.keys() {
            require(
                paths.contains(path.as_str()),
                path,
                "orphan numeric provenance",
            )?;
        }
        Ok(())
    }
}
fn validate_metadata(
    meta: &NumberProvenance,
    n: &NumericField,
    paths: &BTreeSet<&str>,
) -> Result<(), DesignError> {
    let p = &n.path;
    require(
        !meta.flags.is_empty(),
        p,
        "provenance flags must not be empty",
    )?;
    require(
        meta.flags.windows(2).all(|w| w[0] < w[1]),
        p,
        "flags must be unique and in declared order",
    )?;
    require(meta.unit == n.unit, p, "provenance unit mismatch")?;
    require(
        !meta.source_refs.is_empty() && meta.source_refs.iter().all(|s| !s.trim().is_empty()),
        p,
        "source or missing-input explanation required",
    )?;
    let origins: Vec<_> = meta
        .flags
        .iter()
        .filter(|f| **f != ProvenanceFlag::OutsideCoverage)
        .copied()
        .collect();
    if n.value.is_some() {
        require(
            matches!(
                origins.as_slice(),
                [ProvenanceFlag::UserSupplied] | [ProvenanceFlag::TemplateDefault]
            ),
            p,
            "present input requires supplied or template provenance",
        )?;
    } else {
        require(
            origins == [ProvenanceFlag::NotEstimated],
            p,
            "absent input requires NOT_ESTIMATED",
        )?;
    }
    for input in &meta.input_paths {
        require(
            input != p && paths.contains(input.as_str()),
            p,
            "invalid provenance dependency path",
        )?;
    }
    Ok(())
}
fn validate_surface(s: &Surface, p: &str) -> Result<(), DesignError> {
    require(
        s.stations.len() >= 2,
        p,
        "need at least root and tip stations",
    )?;
    for (i, a) in s.stations.iter().enumerate() {
        let p = format!("{p}.stations[{i}]");
        finite(&a.leading_edge_body_m, &format!("{p}.leading_edge_body_m"))?;
        positive(a.chord_m, &format!("{p}.chord_m"))?;
        if i > 0 {
            let before = &s.stations[i - 1].leading_edge_body_m;
            let ordered = match s.kind {
                SurfaceKind::VerticalTail => a.leading_edge_body_m[2] < before[2],
                _ => a.leading_edge_body_m[1] > before[1],
            };
            require(ordered, &p, "stations must progress outboard")?;
        }
        if s.mirrored {
            require(
                a.leading_edge_body_m[1] >= 0.0,
                &p,
                "declare only the nonnegative-y side of a mirrored surface",
            )?;
        }
        if let Some(airfoil) = &a.airfoil {
            text(&airfoil.name, &format!("{p}.airfoil.name"))?;
            text(&airfoil.source, &format!("{p}.airfoil.source"))?;
            if let Some(slope) = airfoil.lift_curve_slope_per_rad {
                positive(slope, &format!("{p}.airfoil.lift_curve_slope_per_rad"))?;
            }
        }
    }
    Ok(())
}
fn validate_fuselage(f: &Fuselage, p: &str) -> Result<(), DesignError> {
    finite(&f.nose_body_m, &format!("{p}.nose_body_m"))?;
    positive(f.length_m, &format!("{p}.length_m"))?;
    require(
        f.sections.len() >= 2,
        p,
        "need at least two sampled sections",
    )?;
    for (i, s) in f.sections.iter().enumerate() {
        let p = format!("{p}.sections[{i}]");
        finite(&[s.aft_m, s.offset_y_m, s.offset_z_m], &p)?;
        require(
            s.aft_m >= 0.0
                && s.aft_m <= f.length_m
                && (i == 0 || s.aft_m > f.sections[i - 1].aft_m),
            &p,
            "section positions must increase within the declared length",
        )?;
        positive(s.width_m, &format!("{p}.width_m"))?;
        positive(s.height_m, &format!("{p}.height_m"))?;
    }
    Ok(())
}
