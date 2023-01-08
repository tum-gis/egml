use egml_core::{DirectPosition, LinearRing};
use egml_transform::offset::offset_linear_ring;
use egml_transform::offset::offset_position;
use nalgebra::Vector3;

#[test]
fn offset_point() {
    let position = DirectPosition::new(1.0, 2.0, 3.0).unwrap();
    let offset: Vector3<f64> = nalgebra::Vector3::<f64>::new(1.0, 1.0, 1.0);

    let result = offset_position(position, &offset).unwrap();

    assert_eq!(result, DirectPosition::new(2.0, 3.0, 4.0).unwrap());
}

#[test]
fn offset_linear_ring_test() {
    let linear_ring = LinearRing::new(vec![
        DirectPosition::new(1.0, 2.0, 3.0).unwrap(),
        DirectPosition::new(2.0, 4.0, 6.0).unwrap(),
        DirectPosition::new(4.0, 7.0, 9.0).unwrap(),
    ])
    .unwrap();
    let offset = nalgebra::Vector3::<f64>::new(1.0, -1.0, 3.0);
    let expected_linear_ring = LinearRing::new(vec![
        DirectPosition::new(2.0, 1.0, 6.0).unwrap(),
        DirectPosition::new(3.0, 3.0, 9.0).unwrap(),
        DirectPosition::new(5.0, 6.0, 12.0).unwrap(),
    ])
    .unwrap();

    let result = offset_linear_ring(linear_ring, &offset).unwrap();

    assert_eq!(result, expected_linear_ring);
}
