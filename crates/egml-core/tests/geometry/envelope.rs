use egml_core::{DirectPosition, Envelope};

#[test]
fn envelope_contains() {
    let lower_corner = DirectPosition::new(1.0, 2.0, 3.0).unwrap();
    let upper_corner = DirectPosition::new(2.0, 3.0, 4.0).unwrap();
    let envelope = Envelope::new(lower_corner, upper_corner).unwrap();
    let point_a = DirectPosition::new(1.5, 2.5, 3.5).unwrap();
    let point_b = DirectPosition::new(2.5, 3.5, 4.5).unwrap();

    assert!(envelope.contains(&point_a));
    assert!(!envelope.contains(&point_b));
}
