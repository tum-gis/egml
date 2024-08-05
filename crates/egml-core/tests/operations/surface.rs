use egml_core::model::base::{Gml, Id};
use egml_core::model::geometry::{DirectPosition, LinearRing};
use egml_core::operations::surface::Surface;
use nalgebra::Vector3;

#[test]
fn basic_normal_vector() {
    let gml = Gml::new(Id::try_from("test_id").expect("must work"));
    let point_a = DirectPosition::new(0.0, 0.0, 1.0).unwrap();
    let point_b = DirectPosition::new(1.0, 0.0, 1.0).unwrap();
    let point_c = DirectPosition::new(1.0, 1.0, 1.0).unwrap();
    let point_d = DirectPosition::new(0.0, 1.0, 1.0).unwrap();
    let linear_ring = LinearRing::new(gml, vec![point_a, point_b, point_c, point_d]).unwrap();
    let normal = linear_ring.normal();

    assert_eq!(normal, Vector3::new(0.0, 0.0, 1.0));
}

#[test]
fn basic_plane_equation() {
    let gml = Gml::new(Id::try_from("test_id").expect("must work"));
    let point_a = DirectPosition::new(0.0, 0.0, 1.0).unwrap();
    let point_b = DirectPosition::new(1.0, 0.0, 1.0).unwrap();
    let point_c = DirectPosition::new(1.0, 1.0, 1.0).unwrap();
    let point_d = DirectPosition::new(0.0, 1.0, 1.0).unwrap();
    let linear_ring = LinearRing::new(gml, vec![point_a, point_b, point_c, point_d]).unwrap();
    let plane_equation = linear_ring.plane_equation();

    assert_eq!(
        plane_equation.point,
        DirectPosition::new(0.0, 0.0, 1.0).unwrap()
    );
    assert_eq!(plane_equation.normal(), Vector3::new(0.0, 0.0, 1.0));
}
