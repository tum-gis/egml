//! Planar distance helpers shared by curve and ring types.

use nalgebra::Vector2;

/// Distance from `p` to the closed segment `a`–`b` in the plane.
///
/// A degenerate segment (`a == b`) is treated as a single point, so the result
/// is never NaN for finite inputs.
pub(crate) fn point_segment_distance_2d(p: Vector2<f64>, a: Vector2<f64>, b: Vector2<f64>) -> f64 {
    let ab = b - a;
    let len_sq = ab.norm_squared();
    if len_sq == 0.0 {
        return (p - a).norm();
    }
    let t = ((p - a).dot(&ab) / len_sq).clamp(0.0, 1.0);
    (p - (a + ab * t)).norm()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interior_projection() {
        let d = point_segment_distance_2d(
            Vector2::new(1.0, 2.0),
            Vector2::new(0.0, 0.0),
            Vector2::new(4.0, 0.0),
        );
        assert!((d - 2.0).abs() < 1e-10);
    }

    #[test]
    fn degenerate_segment() {
        let d = point_segment_distance_2d(
            Vector2::new(3.0, 4.0),
            Vector2::new(0.0, 0.0),
            Vector2::new(0.0, 0.0),
        );
        assert!((d - 5.0).abs() < 1e-10);
    }
}
