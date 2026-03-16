use crate::error::Error;
use nalgebra::{Isometry3, Point3};
use std::fmt;

/// A 3-D coordinate triple in a coordinate reference system (CRS).
///
/// Corresponds to `gml:DirectPositionType` in ISO 19136.  All three components
/// must be finite; `NaN` and ±infinity are rejected at construction time.
///
/// # Invariant
///
/// `x`, `y`, and `z` are always finite `f64` values.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct DirectPosition {
    x: f64,
    y: f64,
    z: f64,
}

impl DirectPosition {
    /// Creates a new position from Cartesian coordinates.
    ///
    /// # Errors
    ///
    /// Returns [`Error::NonFiniteCoordinate`] if any coordinate is NaN or infinite.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use egml_core::model::geometry::DirectPosition;
    ///
    /// let pos = DirectPosition::new(1.0, 2.0, 3.0).unwrap();
    /// assert_eq!(pos.x(), 1.0);
    /// assert!(DirectPosition::new(f64::NAN, 0.0, 0.0).is_err());
    /// ```
    pub fn new(x: f64, y: f64, z: f64) -> Result<Self, Error> {
        if !x.is_finite() {
            return Err(Error::NonFiniteCoordinate("x"));
        }
        if !y.is_finite() {
            return Err(Error::NonFiniteCoordinate("y"));
        }
        if !z.is_finite() {
            return Err(Error::NonFiniteCoordinate("z"));
        }

        Ok(Self { x, y, z })
    }

    /// Returns the X coordinate.
    pub fn x(&self) -> f64 {
        self.x
    }

    /// Returns the Y coordinate.
    pub fn y(&self) -> f64 {
        self.y
    }

    /// Returns the Z coordinate.
    pub fn z(&self) -> f64 {
        self.z
    }

    /// Returns the coordinates as a `[x, y, z]` array.
    pub fn coords(&self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }

    /// Sets the X coordinate.
    ///
    /// # Errors
    ///
    /// Returns [`Error::NonFiniteCoordinate`] with the name `"x"` if `val` is NaN or infinite.
    pub fn set_x(&mut self, val: f64) -> Result<(), Error> {
        if !val.is_finite() {
            return Err(Error::NonFiniteCoordinate("x"));
        }
        self.x = val;
        Ok(())
    }

    /// Sets the Y coordinate.
    ///
    /// # Errors
    ///
    /// Returns [`Error::NonFiniteCoordinate`] with the name `"y"` if `val` is NaN or infinite.
    pub fn set_y(&mut self, val: f64) -> Result<(), Error> {
        if !val.is_finite() {
            return Err(Error::NonFiniteCoordinate("y"));
        }
        self.y = val;
        Ok(())
    }

    /// Sets the Z coordinate.
    ///
    /// # Errors
    ///
    /// Returns [`Error::NonFiniteCoordinate`] with the name `"z"` if `val` is NaN or infinite.
    pub fn set_z(&mut self, val: f64) -> Result<(), Error> {
        if !val.is_finite() {
            return Err(Error::NonFiniteCoordinate("z"));
        }
        self.z = val;
        Ok(())
    }

    /// Returns a single-element list containing a reference to `self`.
    ///
    /// This method exists so that `DirectPosition` satisfies the same
    /// point-iteration pattern used by multi-point geometry types.
    pub fn points(&self) -> Vec<&DirectPosition> {
        vec![self]
    }

    /// Applies a rigid-body transform (rotation + translation) to this position in place.
    ///
    /// `m` is a [`nalgebra::Isometry3`] — a combination of a rotation and a translation
    /// that preserves distances and angles.
    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        let p: Point3<f64> = m * Point3::new(self.x, self.y, self.z);
        self.x = p.x;
        self.y = p.y;
        self.z = p.z;
    }

    /// The position with the smallest representable coordinates `(f64::MIN, f64::MIN, f64::MIN)`.
    pub const MIN: DirectPosition = DirectPosition {
        x: f64::MIN,
        y: f64::MIN,
        z: f64::MIN,
    };
    /// The position with the largest representable coordinates `(f64::MAX, f64::MAX, f64::MAX)`.
    pub const MAX: DirectPosition = DirectPosition {
        x: f64::MAX,
        y: f64::MAX,
        z: f64::MAX,
    };
    /// The origin `(0.0, 0.0, 0.0)`.
    pub const ORIGIN: DirectPosition = DirectPosition {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
}

impl fmt::Display for DirectPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl From<DirectPosition> for nalgebra::Vector3<f64> {
    fn from(item: DirectPosition) -> Self {
        Self::new(item.x, item.y, item.z)
    }
}

impl From<nalgebra::Vector3<f64>> for DirectPosition {
    fn from(item: nalgebra::Vector3<f64>) -> Self {
        Self::new(item.x, item.y, item.z).unwrap()
    }
}

impl From<&DirectPosition> for nalgebra::Vector3<f64> {
    fn from(item: &DirectPosition) -> Self {
        Self::new(item.x, item.y, item.z)
    }
}

impl From<&nalgebra::Vector3<f64>> for DirectPosition {
    fn from(item: &nalgebra::Vector3<f64>) -> Self {
        Self::new(item.x, item.y, item.z).unwrap()
    }
}

impl From<DirectPosition> for nalgebra::Point3<f64> {
    fn from(item: DirectPosition) -> Self {
        Self::new(item.x, item.y, item.z)
    }
}

impl From<DirectPosition> for nalgebra::Point3<f32> {
    fn from(item: DirectPosition) -> Self {
        Self::new(item.x as f32, item.y as f32, item.z as f32)
    }
}

impl From<nalgebra::Point3<f64>> for DirectPosition {
    fn from(item: nalgebra::Point3<f64>) -> Self {
        // TODO: how to handle error?
        Self::new(item.x, item.y, item.z).expect("Should work")
    }
}

impl From<DirectPosition> for parry3d_f64::math::Vector {
    fn from(item: DirectPosition) -> Self {
        Self::new(item.x, item.y, item.z)
    }
}

impl From<parry3d_f64::math::Vector> for DirectPosition {
    fn from(item: parry3d_f64::math::Vector) -> Self {
        Self::new(item.x, item.y, item.z).expect("Should work")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::geometry::DirectPosition;
    use approx::relative_eq;
    use nalgebra::{Isometry3, Rotation3, Vector3};
    use std::f64::consts::FRAC_PI_2;

    #[test]
    fn position_clone() {
        let p = DirectPosition::new(1.0, 2.0, 3.0).unwrap();
        let p2 = p;
        assert_eq!(p, p2);
    }

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
        let isometry: Isometry3<f64> =
            Isometry3::new(Vector3::new(1.0, 1.0, 1.0), Default::default());

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
}
