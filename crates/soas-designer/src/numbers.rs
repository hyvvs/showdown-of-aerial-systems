//! Exhaustive traversal of typed numeric INPUTS. No computed wing outputs exist
//! in Design; missing aggregate objects have a single explicit absent record.
use crate::*;

struct Inventory<'a>(Vec<NumericField<'a>>);
impl<'a> Inventory<'a> {
    fn scalar(&mut self, path: String, value: Option<f64>, unit: &'a str) {
        self.0.push(NumericField { path, value, unit });
    }
    fn vector(&mut self, path: &str, value: Option<[f64; 3]>, unit: &'a str) {
        match value {
            Some(v) => {
                for (axis, value) in v.into_iter().enumerate() {
                    self.scalar(format!("{path}[{axis}]"), Some(value), unit);
                }
            }
            None => self.scalar(path.into(), None, unit),
        }
    }
}
impl Design {
    pub fn numbers(&self) -> Vec<NumericField<'_>> {
        let mut n = Inventory(vec![]);
        for c in &self.components {
            let p = format!("components[{}]", c.id);
            match &c.geometry {
                ComponentGeometry::Surface(s) => {
                    let p = format!("{p}.surface");
                    n.scalar(
                        format!("{p}.incidence_rad"),
                        Some(s.incidence_rad.value()),
                        "rad",
                    );
                    for (i, s) in s.stations.iter().enumerate() {
                        let p = format!("{p}.stations[{i}]");
                        n.vector(
                            &format!("{p}.leading_edge_body_m"),
                            Some(s.leading_edge_body_m),
                            "m",
                        );
                        n.scalar(format!("{p}.chord_m"), Some(s.chord_m), "m");
                        n.scalar(format!("{p}.twist_rad"), Some(s.twist_rad.value()), "rad");
                        if let Some(a) = &s.airfoil {
                            n.scalar(
                                format!("{p}.airfoil.lift_curve_slope_per_rad"),
                                a.lift_curve_slope_per_rad,
                                "1/rad",
                            );
                            n.scalar(
                                format!("{p}.airfoil.zero_lift_angle_rad"),
                                a.zero_lift_angle_rad.map(|v| v.value()),
                                "rad",
                            );
                        }
                    }
                }
                ComponentGeometry::Fuselage(f) => {
                    let p = format!("{p}.fuselage");
                    n.vector(&format!("{p}.nose_body_m"), Some(f.nose_body_m), "m");
                    n.scalar(format!("{p}.length_m"), Some(f.length_m), "m");
                    for (i, s) in f.sections.iter().enumerate() {
                        for (name, value) in [
                            ("aft_m", s.aft_m),
                            ("offset_y_m", s.offset_y_m),
                            ("offset_z_m", s.offset_z_m),
                            ("width_m", s.width_m),
                            ("height_m", s.height_m),
                        ] {
                            n.scalar(format!("{p}.sections[{i}].{name}"), Some(value), "m");
                        }
                    }
                }
                ComponentGeometry::Engine(e) => {
                    n.vector(
                        &format!("{p}.engine.position_body_m"),
                        Some(e.position_body_m),
                        "m",
                    );
                    n.vector(
                        &format!("{p}.engine.direction_body_unit"),
                        Some(e.direction_body_unit),
                        "1",
                    );
                }
            }
            let p = format!("{p}.mass");
            n.scalar(format!("{p}.mass_kg"), c.mass.mass_kg, "kg");
            n.vector(&format!("{p}.cg_body_m"), c.mass.cg_body_m, "m");
            if let Some(inertia) = &c.mass.intrinsic_inertia {
                for (row, values) in inertia.tensor_kg_m2.iter().enumerate() {
                    for (col, value) in values.iter().enumerate() {
                        n.scalar(
                            format!("{p}.intrinsic_inertia.tensor_kg_m2[{row}][{col}]"),
                            Some(*value),
                            "kg*m^2",
                        );
                    }
                }
                for (axis, value) in inertia.axes_euler_rad.iter().enumerate() {
                    n.scalar(
                        format!("{p}.intrinsic_inertia.axes_euler_rad[{axis}]"),
                        Some(value.value()),
                        "rad",
                    );
                }
            } else {
                n.scalar(format!("{p}.intrinsic_inertia"), None, "kg*m^2");
            }
        }
        for c in &self.controls {
            let p = format!("controls[{}]", c.id);
            n.vector(
                &format!("{p}.hinge_start_body_m"),
                Some(c.hinge_start_body_m),
                "m",
            );
            n.vector(
                &format!("{p}.hinge_end_body_m"),
                Some(c.hinge_end_body_m),
                "m",
            );
            for (name, value, unit) in [
                ("chord_fraction", c.chord_fraction, "1"),
                ("min_rad", c.min_rad.value(), "rad"),
                ("max_rad", c.max_rad.value(), "rad"),
            ] {
                n.scalar(format!("{p}.{name}"), Some(value), unit);
            }
        }
        for c in &self.contributions {
            for a in &c.parameters {
                n.scalar(
                    format!("contributions[{}].parameters[{}]", c.id, a.name),
                    a.value,
                    &a.unit,
                );
            }
        }
        for e in &self.engine_tables {
            let p = format!("engine_tables[{}]", e.id);
            n.scalar(
                format!("{p}.dry_throttle_max"),
                Some(e.dry_throttle_max),
                "1",
            );
            for (name, values, unit) in [
                ("mach_nodes", &e.table.mach_nodes, "1"),
                (
                    "geometric_altitude_nodes_m",
                    &e.table.geometric_altitude_nodes_m,
                    "m",
                ),
                ("throttle_nodes", &e.table.throttle_nodes, "1"),
            ] {
                for (i, value) in values.iter().enumerate() {
                    n.scalar(format!("{p}.table.{name}[{i}]"), Some(*value), unit);
                }
            }
            for (h, row) in e.table.samples.iter().enumerate() {
                for (m, column) in row.iter().enumerate() {
                    for (t, s) in column.iter().enumerate() {
                        n.scalar(
                            format!("{p}.table.samples[{h}][{m}][{t}].thrust_n"),
                            Some(s.thrust_n),
                            "N",
                        );
                        n.scalar(
                            format!("{p}.table.samples[{h}][{m}][{t}].fuel_flow_kgps"),
                            Some(s.fuel_flow_kgps),
                            "kg/s",
                        );
                    }
                }
            }
        }
        n.0.sort_by(|a, b| a.path.cmp(&b.path));
        n.0
    }
}
