use stable_bridge::directions;

/// The 3D shape is a parameter, and `11 x 10` reproduces the shipped table bit for bit.
#[test]
fn a_three_d_table_can_be_any_shape() {
    let shipped = directions::build(3).expect("shipped 3D table");
    let rebuilt = directions::build_3d(11, 10).expect("11 x 10");
    assert_eq!(shipped, rebuilt, "11 x 10 must be the shipped table exactly");

    let wider = directions::build_3d(16, 15).expect("16 x 15");
    assert_eq!(wider.len(), 240);
    assert_eq!(wider[0].len(), 3);
    for psi in &wider {
        let norm = psi.iter().map(|value| value * value).sum::<f64>();
        assert!((norm - 1.0).abs() < 1e-12, "not a unit direction: {norm}");
    }

    assert!(directions::build_3d(1, 10).is_none(), "one azimuth is not a table");
    assert!(directions::build_3d(10, 1).is_none(), "one elevation is not a table");
    // The single-count signature still refuses 3D counts other than the shipped product.
    assert!(directions::build_with(3, 240).is_none());
}
