use serde_json::Value;
use soas_core::units::Radians;
use soas_estimator::planform::{WingPlanform, WingStation};

pub fn planform(name: &str) -> WingPlanform {
    let all: Value = serde_json::from_str(include_str!("../fixtures/planforms.json")).unwrap();
    let v = all["wings"]
        .as_array()
        .unwrap()
        .iter()
        .find(|v| v["name"] == name)
        .unwrap();
    WingPlanform {
        data_note: v["data_note"].as_str().unwrap().into(),
        stations: v["stations"]
            .as_array()
            .unwrap()
            .iter()
            .map(|s| WingStation {
                y_m: s["y_m"].as_f64().unwrap(),
                chord_m: s["chord_m"].as_f64().unwrap(),
                leading_edge_x_m: s["leading_edge_x_m"].as_f64().unwrap(),
                z_m: s["z_m"].as_f64().unwrap(),
                twist_rad: Radians::new(s["twist_rad"].as_f64().unwrap()).unwrap(),
                section_lift_curve_slope_per_rad: s["section_lift_curve_slope_per_rad"]
                    .as_f64()
                    .unwrap(),
                zero_lift_angle_rad: Radians::new(s["zero_lift_angle_rad"].as_f64().unwrap())
                    .unwrap(),
                section_source: s["section_source"].as_str().unwrap().into(),
            })
            .collect(),
    }
}
