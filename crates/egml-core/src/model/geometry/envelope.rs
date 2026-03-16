use crate::error::Error;
use crate::error::Error::InvalidEnvelopeBounds;
use crate::model::geometry::DirectPosition;
use nalgebra::{Point3, Vector3};
use std::fmt;

/// Axis-aligned bounding box in 3-D space.
///
/// An `Envelope` is defined by a lower corner and an upper corner such that
/// each coordinate component of the lower corner is ≤ the corresponding
/// component of the upper corner.
///
/// Corresponds to `gml:EnvelopeType` in ISO 19136.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Envelope {
    lower_corner: DirectPosition,
    upper_corner: DirectPosition,

    srs_name: Option<String>,
    srs_dimension: Option<u8>,
}

impl Envelope {
    /// Creates an envelope from explicit lower and upper corners.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidEnvelopeBounds`] if any coordinate
    /// component of `lower_corner` is strictly greater than the corresponding
    /// component of `upper_corner`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use egml_core::model::geometry::{DirectPosition, Envelope};
    ///
    /// let lo = DirectPosition::new(0.0, 0.0, 0.0).unwrap();
    /// let hi = DirectPosition::new(1.0, 2.0, 3.0).unwrap();
    /// let env = Envelope::new(lo, hi).unwrap();
    /// assert_eq!(env.size_x(), 1.0);
    /// ```
    pub fn new(lower_corner: DirectPosition, upper_corner: DirectPosition) -> Result<Self, Error> {
        if lower_corner.x() > upper_corner.x() {
            return Err(InvalidEnvelopeBounds("x"));
        }
        if lower_corner.y() > upper_corner.y() {
            return Err(InvalidEnvelopeBounds("y"));
        }
        if lower_corner.z() > upper_corner.z() {
            return Err(InvalidEnvelopeBounds("z"));
        }

        Ok(Self {
            lower_corner,
            upper_corner,
            srs_name: None,
            srs_dimension: None,
        })
    }

    /// Creates an `Envelope` without validating that `lower_corner <= upper_corner`.
    ///
    /// # Safety (logical)
    /// The caller must ensure that each component of `lower_corner` is less than
    /// or equal to the corresponding component of `upper_corner`. Violating this
    /// will not cause undefined behavior, but will break the type's invariants
    /// and produce incorrect results from methods like `contains`, `size`, etc.
    pub(crate) fn new_unchecked(
        lower_corner: DirectPosition,
        upper_corner: DirectPosition,
    ) -> Self {
        debug_assert!(
            {
                let lc: Point3<f64> = lower_corner.into();
                let uc: Point3<f64> = upper_corner.into();
                lc <= uc
            },
            "lower_corner must be <= upper_corner"
        );

        Self {
            lower_corner,
            upper_corner,
            srs_name: None,
            srs_dimension: None,
        }
    }

    /// Returns the lower (minimum) corner.
    pub fn lower_corner(&self) -> &DirectPosition {
        &self.lower_corner
    }

    /// Returns the upper (maximum) corner.
    pub fn upper_corner(&self) -> &DirectPosition {
        &self.upper_corner
    }

    /// Returns the diagonal vector from the lower corner to the upper corner.
    pub fn size(&self) -> Vector3<f64> {
        let lower_corner_point: Point3<f64> = self.lower_corner.into();
        let upper_corner_point: Point3<f64> = self.upper_corner.into();
        upper_corner_point - lower_corner_point
    }

    /// Returns the extent along the X axis (`upper.x - lower.x`).
    pub fn size_x(&self) -> f64 {
        self.upper_corner.x() - self.lower_corner.x()
    }

    /// Returns the extent along the Y axis (`upper.y - lower.y`).
    pub fn size_y(&self) -> f64 {
        self.upper_corner.y() - self.lower_corner.y()
    }

    /// Returns the extent along the Z axis (`upper.z - lower.z`).
    pub fn size_z(&self) -> f64 {
        self.upper_corner.z() - self.lower_corner.z()
    }

    /// Returns the volume of the box (`size_x * size_y * size_z`).
    ///
    /// Returns `0.0` for degenerate envelopes where one or more extents are zero.
    pub fn volume(&self) -> f64 {
        self.size_x() * self.size_y() * self.size_z()
    }

    /// Returns `true` if `point` lies inside or on the boundary of this envelope.
    pub fn contains(&self, point: &DirectPosition) -> bool {
        let lower_corner: Point3<f64> = self.lower_corner.into();
        let upper_corner: Point3<f64> = self.upper_corner.into();
        let point: Point3<f64> = (*point).into();

        lower_corner <= point && point <= upper_corner
    }

    /// Returns `true` if `envelope` is fully contained within (or touches the boundary of) `self`.
    pub fn contains_envelope(&self, envelope: &Envelope) -> bool {
        self.contains(&envelope.lower_corner) && self.contains(&envelope.upper_corner)
    }

    /// Returns `true` if any corner of `envelope` lies inside or on the boundary of `self`.
    pub fn contains_envelope_partially(&self, envelope: &Envelope) -> bool {
        self.contains(&envelope.lower_corner) || self.contains(&envelope.upper_corner)
    }

    /// Returns a new envelope expanded by `distance` in every direction.
    ///
    /// Each lower-corner coordinate is decreased by `distance` and each
    /// upper-corner coordinate is increased by `distance`.
    ///
    /// # Errors
    ///
    /// Returns [`Error::NonFiniteCoordinate`] if `distance` is NaN or infinite,
    /// or if the resulting coordinates would overflow `f64::MAX`.
    pub fn enlarge(&self, distance: f64) -> Result<Envelope, Error> {
        let lower_corner = DirectPosition::new(
            self.lower_corner.x() - distance,
            self.lower_corner.y() - distance,
            self.lower_corner.z() - distance,
        )?;
        let upper_corner = DirectPosition::new(
            self.upper_corner.x() + distance,
            self.upper_corner.y() + distance,
            self.upper_corner.z() + distance,
        )?;

        Envelope::new(lower_corner, upper_corner)
    }
}

impl Envelope {
    /// Computes the union of a slice of envelopes.
    ///
    /// Returns `None` if `envelopes` is empty; otherwise returns the smallest
    /// envelope that contains all envelopes in the slice.
    pub fn from_envelopes(envelopes: &[Self]) -> Option<Self> {
        let first = envelopes.first()?;

        let (lower, upper) = envelopes.iter().skip(1).fold(
            (first.lower_corner, first.upper_corner),
            |(lo, hi), e| {
                let new_lo = DirectPosition::new(
                    lo.x().min(e.lower_corner.x()),
                    lo.y().min(e.lower_corner.y()),
                    lo.z().min(e.lower_corner.z()),
                )
                .unwrap();
                let new_hi = DirectPosition::new(
                    hi.x().max(e.upper_corner.x()),
                    hi.y().max(e.upper_corner.y()),
                    hi.z().max(e.upper_corner.z()),
                )
                .unwrap();
                (new_lo, new_hi)
            },
        );

        Some(Envelope::new_unchecked(lower, upper))
    }

    /// Computes the smallest envelope that contains all `points`.
    ///
    /// # Errors
    ///
    /// Returns [`Error::EmptyCollection`] if `points` is empty.
    pub fn from_points(points: &[DirectPosition]) -> Result<Self, Error> {
        if points.is_empty() {
            return Err(Error::EmptyCollection("points"));
        }

        let first = &points[0];
        let (mut min_x, mut min_y, mut min_z) = (first.x(), first.y(), first.z());
        let (mut max_x, mut max_y, mut max_z) = (first.x(), first.y(), first.z());

        for point in points.iter().skip(1) {
            min_x = min_x.min(point.x());
            min_y = min_y.min(point.y());
            min_z = min_z.min(point.z());
            max_x = max_x.max(point.x());
            max_y = max_y.max(point.y());
            max_z = max_z.max(point.z());
        }

        let lower_corner = DirectPosition::new(min_x, min_y, min_z)?;
        let upper_corner = DirectPosition::new(max_x, max_y, max_z)?;

        Ok(Self::new_unchecked(lower_corner, upper_corner))
    }
}

impl fmt::Display for Envelope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Envelope[{}, {}, {} -> {}, {}, {}]",
            self.lower_corner.x(),
            self.lower_corner.y(),
            self.lower_corner.z(),
            self.upper_corner.x(),
            self.upper_corner.y(),
            self.upper_corner.z()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pos(x: f64, y: f64, z: f64) -> DirectPosition {
        DirectPosition::new(x, y, z).unwrap()
    }

    fn env(lx: f64, ly: f64, lz: f64, ux: f64, uy: f64, uz: f64) -> Envelope {
        Envelope::new(pos(lx, ly, lz), pos(ux, uy, uz)).unwrap()
    }

    #[test]
    fn from_envelopes_empty_returns_none() {
        let result = Envelope::from_envelopes(&[]);
        assert!(result.is_none());
    }

    #[test]
    fn from_envelopes_single_returns_same_envelope() {
        let e = env(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
        let result = Envelope::from_envelopes(&[e.clone()]).unwrap();

        assert_eq!(result, e);
    }

    #[test]
    fn from_envelopes_two_disjoint() {
        let a = env(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);
        let b = env(5.0, 6.0, 7.0, 8.0, 9.0, 10.0);
        let result = Envelope::from_envelopes(&[a, b]).unwrap();

        assert_eq!(result, env(0.0, 0.0, 0.0, 8.0, 9.0, 10.0));
    }

    #[test]
    fn from_envelopes_overlapping() {
        let a = env(0.0, 0.0, 0.0, 5.0, 5.0, 5.0);
        let b = env(3.0, 3.0, 3.0, 7.0, 7.0, 7.0);
        let result = Envelope::from_envelopes(&[a, b]).unwrap();

        assert_eq!(result, env(0.0, 0.0, 0.0, 7.0, 7.0, 7.0));
    }

    #[test]
    fn from_envelopes_one_contains_the_other() {
        let outer = env(0.0, 0.0, 0.0, 10.0, 10.0, 10.0);
        let inner = env(2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        let result = Envelope::from_envelopes(&[outer.clone(), inner]).unwrap();

        assert_eq!(result, outer);
    }

    #[test]
    fn from_envelopes_multiple() {
        let a = env(0.0, 10.0, 20.0, 1.0, 11.0, 21.0);
        let b = env(-5.0, 8.0, 25.0, 2.0, 12.0, 30.0);
        let c = env(1.0, 9.0, 18.0, 3.0, 15.0, 22.0);
        let result = Envelope::from_envelopes(&[a, b, c]).unwrap();

        assert_eq!(result, env(-5.0, 8.0, 18.0, 3.0, 15.0, 30.0));
    }

    #[test]
    fn from_envelopes_with_negative_coords() {
        let a = env(-10.0, -20.0, -30.0, -1.0, -2.0, -3.0);
        let b = env(-5.0, -25.0, -15.0, 0.0, -1.0, 0.0);
        let result = Envelope::from_envelopes(&[a, b]).unwrap();

        assert_eq!(result, env(-10.0, -25.0, -30.0, 0.0, -1.0, 0.0));
    }

    #[test]
    fn from_envelopes_zero_volume_envelopes() {
        let a = env(1.0, 1.0, 1.0, 1.0, 1.0, 1.0); // point
        let b = env(3.0, 3.0, 3.0, 3.0, 3.0, 3.0); // point
        let result = Envelope::from_envelopes(&[a, b]).unwrap();

        assert_eq!(result, env(1.0, 1.0, 1.0, 3.0, 3.0, 3.0));
    }

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
}
