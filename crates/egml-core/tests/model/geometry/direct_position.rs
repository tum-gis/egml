use approx::relative_eq;
use egml_core::model::geometry::DirectPosition;
use egml_core::operations::geometry::Geometry;
use nalgebra::{Isometry3, Rotation3, Vector3};
use std::f64::consts::FRAC_PI_2;

#[test]
fn position_clone() {
    let p = DirectPosition::new(1.0, 2.0, 3.0).unwrap();
    let p2 = p;
    assert_eq!(p, p2);
}

/*#[test]
fn position_order() {
    let point_a = DirectPosition::new(-1.0, 0.0, -3.0).unwrap();
    let point_b = DirectPosition::new(1.0, 2.0, 3.0).unwrap();
    let point_c = DirectPosition::new(1.0, 2.0, 3.0).unwrap();
    let point_d = DirectPosition::new(1.0, 2.0, -3.0).unwrap();

    assert!(point_a < point_b);
    assert!(point_b <= point_c);
    assert!(point_a <= point_b);
    assert!(point_c > point_d);
}*/

/*#[test]
fn position_order_tw() {
    let point_a = DirectPosition::new(1.0, 2.0, 3.0).unwrap();
    let point_b = DirectPosition::new(2.0, 0.0, 0.0).unwrap();

    assert!(point_a > point_b);
}*/

#[test]
fn apply_basic_transform() {
    let mut position = DirectPosition::new(1.0, 2.0, 3.0).unwrap();
    let isometry: Isometry3<f64> =
        Isometry3::new(Vector3::new(-1.0, -2.0, 3.0), Default::default());

    position.apply_transform(&isometry);

    assert_eq!(position, DirectPosition::new(0.0, 0.0, 6.0).unwrap());
}

#[test]
fn apply_basic_translation_transform() {
    let mut position = DirectPosition::new(1.0, 2.0, 3.0).unwrap();
    let isometry: Isometry3<f64> = Isometry3::new(Vector3::new(1.0, 1.0, 1.0), Default::default());

    position.apply_transform(&isometry);

    assert_eq!(position, DirectPosition::new(2.0, 3.0, 4.0).unwrap());
}

#[test]
fn apply_basic_rotation_transform() {
    let mut position = DirectPosition::new(1.0, 1.0, 0.0).unwrap();
    let isometry: Isometry3<f64> = Isometry3::from_parts(
        Default::default(),
        Rotation3::from_euler_angles(0.0, 0.0, FRAC_PI_2).into(),
    );

    position.apply_transform(&isometry);

    relative_eq!(position.x(), -1.0, epsilon = f64::EPSILON);
    relative_eq!(position.y(), 1.0, epsilon = f64::EPSILON);
    relative_eq!(position.z(), 0.0, epsilon = f64::EPSILON);
}
