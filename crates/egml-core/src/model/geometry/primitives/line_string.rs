use crate::model::common::{ApplyTransform, ComputeEnvelope, IterGeometries};
use crate::model::geometry::primitives::{AbstractCurve, AsAbstractCurve, AsAbstractCurveMut};
use crate::model::geometry::refs::AbstractGeometryKindRef;
use crate::model::geometry::{DirectPosition, Envelope};
use crate::util::distance::point_segment_distance_2d;
use crate::{
    Error, impl_abstract_curve_mut_traits, impl_abstract_curve_traits, impl_has_geometry_type,
};
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector2, Vector3};

/// An ordered sequence of two or more coordinate positions forming a 1-D curve.
///
/// Corresponds to `gml:LineString` in [OGC 07-036 §10.4.4](https://docs.ogc.org/is/07-036/07-036.pdf).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LineString {
    pub abstract_curve: AbstractCurve,
    /// Boxed slice rather than `Vec`: points are written once and then only
    /// read or transformed in place, so the capacity field would be dead weight.
    points: Box<[DirectPosition]>,
}

impl LineString {
    /// Creates a new `LineString` from an ordered list of positions.
    ///
    /// # Errors
    ///
    /// Returns [`Error::TooFewElements`] if `points` contains fewer than 2 entries.
    /// Returns [`Error::AdjacentDuplicatePositions`] if adjacent positions are equal.
    pub fn new(points: impl IntoIterator<Item = DirectPosition>) -> Result<Self, Error> {
        let points: Vec<DirectPosition> = points.into_iter().collect();
        Self::validate(&points)?;

        Ok(Self {
            abstract_curve: AbstractCurve::default(),
            points: points.into_boxed_slice(),
        })
    }

    pub fn from_abstract_curve(
        abstract_curve: AbstractCurve,
        points: impl IntoIterator<Item = DirectPosition>,
    ) -> Result<Self, Error> {
        let points: Vec<DirectPosition> = points.into_iter().collect();
        Self::validate(&points)?;

        Ok(Self {
            abstract_curve,
            points: points.into_boxed_slice(),
        })
    }

    fn validate(points: &[DirectPosition]) -> Result<(), Error> {
        if let Some((index, window)) = points.windows(2).enumerate().find(|(_, w)| w[0] == w[1]) {
            return Err(Error::AdjacentDuplicatePositions {
                index,
                position: window[0],
            });
        }

        if points.len() < 2 {
            return Err(Error::TooFewElements {
                geometry: "gml:LineString",
                minimum: 2,
                spec: Some("OGC 07-036 §10.4.4"),
                id: None,
                detail: None,
            });
        }

        Ok(())
    }

    /// Returns the ordered positions of this line string.
    pub fn points(&self) -> &[DirectPosition] {
        &self.points
    }

    /// Replaces the positions of this line string.
    ///
    /// # Errors
    ///
    /// Returns the same errors as [`new`](Self::new).
    pub fn set_points(
        &mut self,
        points: impl IntoIterator<Item = DirectPosition>,
    ) -> Result<(), Error> {
        let points: Vec<DirectPosition> = points.into_iter().collect();
        Self::validate(&points)?;
        self.points = points.into_boxed_slice();
        Ok(())
    }
}

impl AsAbstractCurve for LineString {
    fn abstract_curve(&self) -> &AbstractCurve {
        &self.abstract_curve
    }
}

impl AsAbstractCurveMut for LineString {
    fn abstract_curve_mut(&mut self) -> &mut AbstractCurve {
        &mut self.abstract_curve
    }
}

impl_abstract_curve_traits!(LineString);
impl_abstract_curve_mut_traits!(LineString);
impl_has_geometry_type!(LineString, LineString);

impl LineString {
    /// Returns the total 3D length as the sum of Euclidean distances between consecutive points.
    pub fn length_3d(&self) -> f64 {
        self.points
            .windows(2)
            .map(|w| {
                let a: Vector3<f64> = w[0].into();
                let b: Vector3<f64> = w[1].into();
                (b - a).norm()
            })
            .sum()
    }

    /// Returns the minimum Euclidean distance between `point` and this line string,
    /// measured in the XY plane (z coordinates are ignored).
    ///
    /// Each segment is treated as bounded, so points beyond an end of the line
    /// string are measured to the nearest endpoint. Segments whose endpoints
    /// differ only in z collapse to a single point in 2D and are handled as such.
    ///
    /// # Examples
    ///
    /// ```
    /// use egml_core::model::geometry::DirectPosition;
    /// use egml_core::model::geometry::primitives::LineString;
    ///
    /// let ls = LineString::new([
    ///     DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
    ///     DirectPosition::new(10.0, 0.0, 0.0).unwrap(),
    /// ])
    /// .unwrap();
    /// let p = DirectPosition::new(5.0, 3.0, 42.0).unwrap();
    /// assert!((ls.distance_2d(&p) - 3.0).abs() < 1e-10);
    /// ```
    pub fn distance_2d(&self, point: &DirectPosition) -> f64 {
        let p = Vector2::new(point.x(), point.y());
        self.points
            .windows(2)
            .map(|w| {
                let a = Vector2::new(w[0].x(), w[0].y());
                let b = Vector2::new(w[1].x(), w[1].y());
                point_segment_distance_2d(p, a, b)
            })
            .fold(f64::INFINITY, f64::min)
    }
}

impl ApplyTransform for LineString {
    fn apply_transform(&mut self, transform: Transform3<f64>) {
        self.points.iter_mut().for_each(|p| {
            p.apply_transform(transform);
        });
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        self.points.iter_mut().for_each(|p| {
            p.apply_isometry(isometry);
        });
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        self.points.iter_mut().for_each(|p| {
            p.apply_translation(vector);
        });
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        self.points.iter_mut().for_each(|p| {
            p.apply_rotation(rotation);
        });
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        self.points.iter_mut().for_each(|p| {
            p.apply_scale(scale);
        });
    }
}

impl ComputeEnvelope for LineString {
    fn compute_envelope(&self) -> Option<Envelope> {
        Some(Envelope::from_points(&self.points).expect("line string must have valid points"))
    }
}

impl IterGeometries for LineString {
    fn iter_geometries(&self) -> Box<dyn Iterator<Item = AbstractGeometryKindRef<'_>> + '_> {
        Box::new(std::iter::once(self.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn length_3d_axis_aligned() {
        let ls = LineString::new([
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(3.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(3.0, 4.0, 0.0).unwrap(),
        ])
        .unwrap();
        assert!((ls.length_3d() - 7.0).abs() < 1e-10);
    }

    #[test]
    fn length_3d_diagonal() {
        // Single segment along the space diagonal of a unit cube — length sqrt(3).
        let ls = LineString::new([
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 1.0, 1.0).unwrap(),
        ])
        .unwrap();
        let expected = 3.0_f64.sqrt();
        assert!((ls.length_3d() - expected).abs() < 1e-10);
    }

    fn pos(x: f64, y: f64, z: f64) -> DirectPosition {
        DirectPosition::new(x, y, z).unwrap()
    }

    #[test]
    fn distance_2d_projects_onto_segment_interior() {
        let ls = LineString::new([pos(0.0, 0.0, 0.0), pos(10.0, 0.0, 0.0)]).unwrap();
        assert!((ls.distance_2d(&pos(5.0, 3.0, 0.0)) - 3.0).abs() < 1e-10);
    }

    #[test]
    fn distance_2d_clamps_to_endpoint() {
        // Beyond the end of the segment: distance to (10,0), not to the infinite line.
        let ls = LineString::new([pos(0.0, 0.0, 0.0), pos(10.0, 0.0, 0.0)]).unwrap();
        assert!((ls.distance_2d(&pos(13.0, 4.0, 0.0)) - 5.0).abs() < 1e-10);
        assert!((ls.distance_2d(&pos(-3.0, -4.0, 0.0)) - 5.0).abs() < 1e-10);
    }

    #[test]
    fn distance_2d_is_zero_on_line() {
        let ls = LineString::new([pos(0.0, 0.0, 0.0), pos(4.0, 4.0, 0.0)]).unwrap();
        assert!(ls.distance_2d(&pos(2.0, 2.0, 0.0)).abs() < 1e-10);
        assert!(ls.distance_2d(&pos(4.0, 4.0, 0.0)).abs() < 1e-10);
    }

    #[test]
    fn distance_2d_takes_minimum_over_segments() {
        // L-shape; the point is closer to the second segment.
        let ls = LineString::new([
            pos(0.0, 0.0, 0.0),
            pos(10.0, 0.0, 0.0),
            pos(10.0, 10.0, 0.0),
        ])
        .unwrap();
        assert!((ls.distance_2d(&pos(8.0, 5.0, 0.0)) - 2.0).abs() < 1e-10);
    }

    #[test]
    fn distance_2d_ignores_z() {
        let ls = LineString::new([pos(0.0, 0.0, 100.0), pos(10.0, 0.0, -50.0)]).unwrap();
        assert!((ls.distance_2d(&pos(5.0, 3.0, 1000.0)) - 3.0).abs() < 1e-10);
    }

    #[test]
    fn distance_2d_handles_vertical_segment() {
        // Points differ only in z, so the segment collapses to a point in 2D.
        let ls = LineString::new([pos(1.0, 1.0, 0.0), pos(1.0, 1.0, 5.0)]).unwrap();
        let d = ls.distance_2d(&pos(4.0, 5.0, 0.0));
        assert!(!d.is_nan());
        assert!((d - 5.0).abs() < 1e-10);
    }
}
