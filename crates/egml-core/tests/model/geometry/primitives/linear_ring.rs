use egml_core::model::base::{AbstractGml, Id};
use egml_core::model::geometry::{DirectPosition, LinearRing};
use egml_core::operations::geometry::Geometry;
use nalgebra::{Isometry3, Vector3};

#[test]
fn offset_linear_ring_test() {
    let abstract_gml = AbstractGml::new(Id::try_from("test_id").expect("must work"));
    let mut linear_ring = LinearRing::new(
        abstract_gml.clone(),
        vec![
            DirectPosition::new(1.0, 2.0, 3.0).unwrap(),
            DirectPosition::new(2.0, 4.0, 6.0).unwrap(),
            DirectPosition::new(4.0, 7.0, 9.0).unwrap(),
        ],
    )
    .unwrap();
    //let offset = nalgebra::Vector3::<f64>::new(1.0, -1.0, 3.0);
    let isometry: Isometry3<f64> = Isometry3::new(Vector3::new(1.0, -1.0, 3.0), Default::default());
    let expected_linear_ring = LinearRing::new(
        abstract_gml.clone(),
        vec![
            DirectPosition::new(2.0, 1.0, 6.0).unwrap(),
            DirectPosition::new(3.0, 3.0, 9.0).unwrap(),
            DirectPosition::new(5.0, 6.0, 12.0).unwrap(),
        ],
    )
    .unwrap();

    linear_ring.apply_transform(&isometry);

    assert_eq!(linear_ring, expected_linear_ring);
}
