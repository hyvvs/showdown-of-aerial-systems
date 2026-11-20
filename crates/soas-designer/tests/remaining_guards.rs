//! Synthetic tensor/probe settings: fixtures/closed_kind_values.md, Fix 6.
mod support;
use soas_designer::*;

#[test]
fn positive_diagonal_indefinite_inertia_is_rejected() {
    let mut d = support::fixture();
    // Eigenvalues 3, -1, 1; determinant -3, so it is invertible but not SPD.
    // For x=(1,-1,0), x^T I x = -2. Positive diagonals alone miss this.
    d.components[0]
        .mass
        .intrinsic_inertia
        .as_mut()
        .unwrap()
        .tensor_kg_m2 = [[1.0, 2.0, 0.0], [2.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
    assert_eq!(
        d.validate(),
        Err(DesignError {
            field: "components[wing].mass.intrinsic_inertia".into(),
            reason: "invalid signed intrinsic inertia tensor",
        })
    );
    // A symmetric matrix with positive off-diagonals can be valid: ensure
    // rejection cannot be implemented by simply prohibiting such entries.
    d.components[0]
        .mass
        .intrinsic_inertia
        .as_mut()
        .unwrap()
        .tensor_kg_m2 = [[2.0, 1.0, 0.0], [1.0, 2.0, 0.0], [0.0, 0.0, 1.0]];
    d.validate().unwrap();
}

#[test]
fn template_default_requires_nonempty_source_for_every_numeric_input() {
    for (path, value, _) in support::expected_numbers(&support::fixture()) {
        if value.is_none() {
            continue;
        }
        for sources in [vec![], vec![String::new()], vec![" \t\n ".into()]] {
            let mut d = support::fixture();
            let m = d.provenance.get_mut(&path).unwrap();
            m.flags = vec![ProvenanceFlag::TemplateDefault];
            m.source_refs = sources;
            assert_eq!(
                d.validate(),
                Err(DesignError {
                    field: path.clone(),
                    reason: "source or missing-input explanation required"
                })
            );
        }
    }
    for i in 0..support::fixture().contributions.len() {
        for blank in ["", " \t\n "] {
            let mut d = support::fixture();
            let c = &mut d.contributions[i];
            let path = format!(
                "contributions[{}].parameters[{}]",
                c.id, c.parameters[0].name
            );
            d.provenance.get_mut(&path).unwrap().flags = vec![ProvenanceFlag::TemplateDefault];
            c.source = blank.into();
            let field = format!("contributions[{}].source", c.id);
            assert_eq!(
                d.validate(),
                Err(DesignError {
                    field,
                    reason: "must not be empty"
                })
            );
        }
    }
}
