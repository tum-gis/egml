use crate::model::geometry::DirectPosition;
use nalgebra::Vector3;

/// A plane in R³ defined by a point and a unit normal vector.
///
/// Used internally to project 3-D polygons to 2-D for triangulation.
///
/// # Examples
///
/// ```rust
/// use egml_core::model::geometry::DirectPosition;
/// use egml_core::util::plane::Plane;
/// use nalgebra::Vector3;
///
/// let origin = DirectPosition::new(0.0, 0.0, 0.0).unwrap();
/// let plane = Plane::new(origin, Vector3::z());
/// assert_eq!(plane.normal(), Vector3::z());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Plane {
    /// A point that lies on the plane.
    pub point: DirectPosition,
    normal: Vector3<f64>,
}

impl Plane {
    /// Creates a new plane from a point and a normal vector.
    ///
    /// The normal vector does not have to be a unit vector, but it is treated
    /// as one by callers that compute distances.
    pub fn new(point: DirectPosition, normal: Vector3<f64>) -> Self {
        Self { point, normal }
    }

    /// Returns the normal vector of this plane.
    pub fn normal(&self) -> Vector3<f64> {
        self.normal
    }
}
