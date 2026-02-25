use crate::error::Error;
use crate::error::Error::LowerCornerMustBeEqualOrBelowUpperCorner;
use crate::model::geometry::DirectPosition;
use nalgebra::{Point3, Vector3};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Envelope {
    lower_corner: DirectPosition,
    upper_corner: DirectPosition,

    srs_name: Option<String>,
    srs_dimension: Option<u8>,
}

impl Envelope {
    pub fn new(lower_corner: DirectPosition, upper_corner: DirectPosition) -> Result<Self, Error> {
        let lower_corner_point: Point3<f64> = lower_corner.into();
        let upper_corner_point: Point3<f64> = upper_corner.into();
        if lower_corner_point > upper_corner_point {
            return Err(LowerCornerMustBeEqualOrBelowUpperCorner(""));
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

    pub fn lower_corner(&self) -> &DirectPosition {
        &self.lower_corner
    }

    pub fn upper_corner(&self) -> &DirectPosition {
        &self.upper_corner
    }

    pub fn size(&self) -> Vector3<f64> {
        let lower_corner_point: Point3<f64> = self.lower_corner.into();
        let upper_corner_point: Point3<f64> = self.upper_corner.into();
        upper_corner_point - lower_corner_point
    }

    pub fn size_x(&self) -> f64 {
        self.upper_corner.x() - self.lower_corner.x()
    }

    pub fn size_y(&self) -> f64 {
        self.upper_corner.y() - self.lower_corner.y()
    }

    pub fn size_z(&self) -> f64 {
        self.upper_corner.z() - self.lower_corner.z()
    }

    pub fn volume(&self) -> f64 {
        self.size_x() * self.size_y() * self.size_z()
    }

    pub fn contains(&self, point: &DirectPosition) -> bool {
        let lower_corner: Point3<f64> = self.lower_corner.into();
        let upper_corner: Point3<f64> = self.upper_corner.into();
        let point: Point3<f64> = (*point).into();

        lower_corner <= point && point <= upper_corner
    }

    /// Returns `true` if the envelope is fully contained.
    pub fn contains_envelope(&self, envelope: &Envelope) -> bool {
        self.contains(&envelope.lower_corner) && self.contains(&envelope.upper_corner)
    }

    /// Returns `true` if the envelope is fully contained.
    pub fn contains_envelope_partially(&self, envelope: &Envelope) -> bool {
        self.contains(&envelope.lower_corner) || self.contains(&envelope.upper_corner)
    }

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

        let envelope = Envelope::new(lower_corner, upper_corner)?;
        Ok(envelope)
    }
}

impl Envelope {
    pub fn from_envelopes(envelopes: &[&Self]) -> Option<Self> {
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

    pub fn from_points(points: &[DirectPosition]) -> Result<Self, Error> {
        if points.is_empty() {
            return Err(Error::MustNotBeEmpty(
                "Cannot create envelope from empty points",
            ));
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
        let result = Envelope::from_envelopes(&[&e]).unwrap();

        assert_eq!(result, e);
    }

    #[test]
    fn from_envelopes_two_disjoint() {
        let a = env(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);
        let b = env(5.0, 6.0, 7.0, 8.0, 9.0, 10.0);
        let result = Envelope::from_envelopes(&[&a, &b]).unwrap();

        assert_eq!(result, env(0.0, 0.0, 0.0, 8.0, 9.0, 10.0));
    }

    #[test]
    fn from_envelopes_overlapping() {
        let a = env(0.0, 0.0, 0.0, 5.0, 5.0, 5.0);
        let b = env(3.0, 3.0, 3.0, 7.0, 7.0, 7.0);
        let result = Envelope::from_envelopes(&[&a, &b]).unwrap();

        assert_eq!(result, env(0.0, 0.0, 0.0, 7.0, 7.0, 7.0));
    }

    #[test]
    fn from_envelopes_one_contains_the_other() {
        let outer = env(0.0, 0.0, 0.0, 10.0, 10.0, 10.0);
        let inner = env(2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        let result = Envelope::from_envelopes(&[&outer, &inner]).unwrap();

        assert_eq!(result, outer);
    }

    #[test]
    fn from_envelopes_multiple() {
        let a = env(0.0, 10.0, 20.0, 1.0, 11.0, 21.0);
        let b = env(-5.0, 8.0, 25.0, 2.0, 12.0, 30.0);
        let c = env(1.0, 9.0, 18.0, 3.0, 15.0, 22.0);
        let result = Envelope::from_envelopes(&[&a, &b, &c]).unwrap();

        assert_eq!(result, env(-5.0, 8.0, 18.0, 3.0, 15.0, 30.0));
    }

    #[test]
    fn from_envelopes_with_negative_coords() {
        let a = env(-10.0, -20.0, -30.0, -1.0, -2.0, -3.0);
        let b = env(-5.0, -25.0, -15.0, 0.0, -1.0, 0.0);
        let result = Envelope::from_envelopes(&[&a, &b]).unwrap();

        assert_eq!(result, env(-10.0, -25.0, -30.0, 0.0, -1.0, 0.0));
    }

    #[test]
    fn from_envelopes_zero_volume_envelopes() {
        let a = env(1.0, 1.0, 1.0, 1.0, 1.0, 1.0); // point
        let b = env(3.0, 3.0, 3.0, 3.0, 3.0, 3.0); // point
        let result = Envelope::from_envelopes(&[&a, &b]).unwrap();

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
