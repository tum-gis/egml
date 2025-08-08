use crate::model::geometry::DirectPosition;
use parry3d_f64::na;

pub fn distance(p1: &DirectPosition, p2: &DirectPosition) -> f64 {
    let point_p1: parry3d_f64::math::Point<f64> = (*p1).into();
    let point_p2: parry3d_f64::math::Point<f64> = (*p2).into();

    na::distance(&point_p1, &point_p2)
}
