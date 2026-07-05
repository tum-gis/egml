use crate::model::geometry::primitives::{AbstractCurve, AsAbstractCurve, AsAbstractCurveMut};
use crate::model::geometry::{DirectPosition, Envelope};
use crate::{Error, impl_abstract_curve_traits};
use nalgebra::{Isometry3, Vector3};

/// An ordered sequence of two or more coordinate positions forming a 1-D curve.
///
/// Corresponds to `gml:LineString` in [OGC 07-036 §10.4.4](https://docs.ogc.org/is/07-036/07-036.pdf).
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
    pub fn new(points: impl IntoIterator<Item = DirectPosition>) -> Result<Self, Error> {
        let points: Vec<DirectPosition> = points.into_iter().collect();
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

        Ok(Self {
            abstract_curve: AbstractCurve::default(),
            points,
        })
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
    ) -> Result<(), crate::Error> {
        let points: Vec<DirectPosition> = points.into_iter().collect();
        if let Some((index, window)) = points.windows(2).enumerate().find(|(_, w)| w[0] == w[1]) {
            return Err(crate::Error::AdjacentDuplicatePositions {
                index,
                position: window[0],
            });
        }
        if points.len() < 2 {
            return Err(crate::Error::TooFewElements {
                geometry: "gml:LineString",
                minimum: 2,
                spec: Some("OGC 07-036 §10.4.4"),
                id: None,
                detail: None,
            });
        }
        self.points = points;
        Ok(())
    }
}

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
}
