use crate::model::geometry::DirectPosition;

pub fn distance(p1: &DirectPosition, p2: &DirectPosition) -> f64 {
    let point_p1: nalgebra::Point3<f64> = (*p1).into();
    let point_p2: nalgebra::Point3<f64> = (*p2).into();

    nalgebra::distance(&point_p1, &point_p2)
}
