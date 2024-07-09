use egml_core::DirectPosition;

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
