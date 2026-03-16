use crate::Error::{AdjacentDuplicatePositions, TooFewElements};
use crate::model::geometry::primitives::{AbstractCurve, AsAbstractCurve, AsAbstractCurveMut};
use crate::model::geometry::{DirectPosition, Envelope};
use crate::{Error, impl_abstract_curve_traits};
use nalgebra::Isometry3;

/// An ordered sequence of two or more coordinate positions forming a 1-D curve.
///
/// Corresponds to `gml:LineString` in ISO 19136 §10.4.4.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LineString {
    pub(crate) abstract_curve: AbstractCurve,
    points: Vec<DirectPosition>,
}

impl LineString {
    /// Creates a new `LineString` from an ordered list of positions.
    ///
    /// # Errors
    ///
    /// Returns [`Error::TooFewElements`] if `points` contains fewer than 2 entries.
    /// Returns [`Error::AdjacentDuplicatePositions`] if adjacent positions are equal.
    pub fn new(abstract_curve: AbstractCurve, points: Vec<DirectPosition>) -> Result<Self, Error> {
        let duplicates_count = points.windows(2).filter(|x| x[0] == x[1]).count();
        if duplicates_count > 0 {
            return Err(AdjacentDuplicatePositions);
        }
        if points.len() < 2 {
            return Err(TooFewElements {
                geometry: "gml:LineString",
                minimum: 2,
                spec: Some("ISO 19136 §10.4.4"),
                id: None,
                message: None,
            });
        }

        Ok(Self {
            abstract_curve,
            points,
        })
    }

    /// Returns the ordered positions of this line string.
    pub fn points(&self) -> &[DirectPosition] {
        &self.points
    }

    /// Applies a rigid-body transform to all positions in place.
    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.points.iter_mut().for_each(|p| {
            p.apply_transform(m);
        });
    }

    pub fn compute_envelope(&self) -> Envelope {
        Envelope::from_points(&self.points).expect("line string must have valid points")
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
