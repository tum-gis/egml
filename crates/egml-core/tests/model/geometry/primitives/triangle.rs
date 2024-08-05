use egml_core::model::geometry::{DirectPosition, Triangle};

#[test]
fn triangle_construction_test() {
    let triangle_result = Triangle::new(
        DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
        DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
        DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
    );

    assert!(matches!(
        triangle_result,
        Err(egml_core::Error::ContainsEqualElements)
    ));
}

#[test]
fn triangle_distance_test() {
    let triangle = Triangle::new(
        DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
        DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
        DirectPosition::new(1.0, 1.0, 0.0).unwrap(),
    )
    .unwrap();

    let distance = triangle.distance_to_local_point(&DirectPosition::new(0.5, 0.5, 1.0).unwrap());

    assert_eq!(distance, 1.0);
}
