//! Pin the CSV schema independently of the implementation's header literal.
//! No numeric test setting or new fixture data: this checks names/order/LF only.
#[test]
fn csv_schema_one_header_is_exactly_the_documented_contract() {
    let expected = "time_s,north_m,east_m,down_m,u_mps,v_mps,w_mps,qw_unitless,qx_unitless,qy_unitless,qz_unitless,p_radps,q_radps,r_radps,commanded_elevator_rad,commanded_aileron_rad,commanded_rudder_rad,applied_elevator_rad,applied_aileron_rad,applied_rudder_rad,mach_unitless,alpha_rad,beta_rad,cl_unitless,cd_unitless,lift_n,drag_n,side_n,lift_limited_unitless,throttle_unitless,thrust_n,fuel_flow_kgps,afterburner_engines_unitless\n";
    assert_eq!(soas_cli::csv_text(&[]), expected);
}
