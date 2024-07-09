use egml_core::{DirectPosition, LinearRing};

use egml_transform::triangulate::triangulate_linear_ring;

#[test]
fn triangulate() {
    let linear_ring = LinearRing::new(vec![
        DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
        DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
        DirectPosition::new(1.0, 1.0, 0.0).unwrap(),
        DirectPosition::new(0.0, 1.0, 0.0).unwrap(),
    ])
    .unwrap();

    let result = triangulate_linear_ring(&linear_ring).unwrap();

    assert_eq!(result.number_of_patches(), 2);
    assert!(result.patches()[0].area() > 0.0);
    assert!(result.patches()[1].area() > 0.0);
}
