use crate::error::Error;
use crate::impl_abstract_ring_traits;
use crate::model::base::{AsAbstractGml, Id};
use crate::model::geometry::primitives::{AbstractRing, AsAbstractRing, AsAbstractRingMut};
use crate::model::geometry::{DirectPosition, Envelope};
use nalgebra::{Isometry3, Vector3};

const MINIMUM_NUMBER_OF_POINTS: usize = 3;

/// An implicitly closed ring of at least 3 distinct, non-adjacent positions.
///
/// Corresponds to `gml:LinearRing` in [OGC 07-036 §10.5.8](https://docs.ogc.org/is/07-036/07-036.pdf).  The ring is
/// implicitly closed: the last position is not repeated.
///
/// # Invariants
///
/// - At least 3 positions.
/// - No two adjacent positions are equal.
/// - First and last positions are not equal.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LinearRing {
    pub(crate) abstract_ring: AbstractRing,
    points: Vec<DirectPosition>,
}

impl LinearRing {
    /// Creates a new `LinearRing` from an ordered list of positions.
    ///
    /// # Errors
    ///
    /// Returns [`Error::TooFewElements`] if `points` has fewer than 3 entries.
    /// Returns [`Error::AdjacentDuplicatePositions`] if adjacent positions are equal.
    /// Returns [`Error::RepeatedClosingVertex`] if the first and last positions are equal.
    pub fn new(points: impl IntoIterator<Item = DirectPosition>) -> Result<Self, Error> {
        let points: Vec<DirectPosition> = points.into_iter().collect();
        Self::validate_points(&points, None)?;
        Ok(Self {
            abstract_ring: AbstractRing::default(),
            points,
        })
    }

    /// Returns the positions of this ring.
    pub fn points(&self) -> &[DirectPosition] {
        &self.points
    }

    /// Replaces the positions of this ring.
    ///
    /// # Errors
    ///
    /// Returns the same errors as [`new`](Self::new).
    pub fn set_points(&mut self, val: Vec<DirectPosition>) -> Result<(), Error> {
        Self::validate_points(&val, self.id())?;
        self.points = val;
        Ok(())
    }
}

impl LinearRing {
    /// Applies a rigid-body transform to all positions in place.
    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.points.iter_mut().for_each(|p| {
            p.apply_transform(m);
        });
    }

    /// Returns the 3D area_3d of this ring using the cross-product summation formula.
    ///
    /// Computes `|Σ (vᵢ × vᵢ₊₁)| / 2` over all consecutive vertex pairs (with wrap-around),
    /// which gives the correct planar area_3d regardless of orientation in 3D space.
    pub fn area_3d(&self) -> f64 {
        let n = self.points.len();
        let mut cross_sum = Vector3::zeros();
        for i in 0..n {
            let vi: Vector3<f64> = self.points[i].into();
            let vj: Vector3<f64> = self.points[(i + 1) % n].into();
            cross_sum += vi.cross(&vj);
        }
        cross_sum.norm() * 0.5
    }

    /// Returns the axis-aligned bounding box of all positions in this ring.
    pub fn compute_envelope(&self) -> Envelope {
        Envelope::from_points(&self.points).expect("linear ring must have valid points")
    }

    fn validate_points(points: &[DirectPosition], id: Option<&Id>) -> Result<(), Error> {
        if let Some((index, window)) = points.windows(2).enumerate().find(|(_, w)| w[0] == w[1]) {
            return Err(Error::AdjacentDuplicatePositions {
                index,
                position: window[0],
            });
        }
        if points.len() < MINIMUM_NUMBER_OF_POINTS {
            let detail = if id.is_none() {
                Some(format!(
                    "points: {}",
                    points
                        .iter()
                        .map(|p| p.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                ))
            } else {
                None
            };

            return Err(Error::TooFewElements {
                geometry: "gml:LinearRing",
                minimum: 3,
                spec: Some("OGC 07-036 §10.5.8"),
                id: id.cloned(),
                detail,
            });
        }
        let first = *points.first().expect("non-empty validated above");
        if first == *points.last().expect("non-empty validated above") {
            return Err(Error::RepeatedClosingVertex { position: first });
        }
        Ok(())
    }
}

impl AsAbstractRing for LinearRing {
    fn abstract_ring(&self) -> &AbstractRing {
        &self.abstract_ring
    }
}

impl AsAbstractRingMut for LinearRing {
    fn abstract_ring_mut(&mut self) -> &mut AbstractRing {
        &mut self.abstract_ring
    }
}

impl_abstract_ring_traits!(LinearRing);

#[cfg(test)]
mod test {
    use super::*;
    use nalgebra::Vector3;

    #[test]
    fn area_3d_unit_square_xy() {
        let ring = LinearRing::new([
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 1.0, 0.0).unwrap(),
            DirectPosition::new(0.0, 1.0, 0.0).unwrap(),
        ])
        .unwrap();
        assert!((ring.area_3d() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn area_3d_tilted_rectangle() {
        // Rectangle with sides 1 and sqrt(5) tilted in 3D — area_3d should be sqrt(5).
        let ring = LinearRing::new([
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 1.0, 2.0).unwrap(),
            DirectPosition::new(0.0, 1.0, 2.0).unwrap(),
        ])
        .unwrap();
        let expected = 5.0_f64.sqrt();
        assert!((ring.area_3d() - expected).abs() < 1e-10);
    }

    #[test]
    fn area_3d_triangle() {
        // Right triangle with legs 3 and 4 — area_3d should be 6.
        let ring = LinearRing::new([
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(3.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(0.0, 4.0, 0.0).unwrap(),
        ])
        .unwrap();
        assert!((ring.area_3d() - 6.0).abs() < 1e-10);
    }

    #[test]
    fn linear_ring_construction_test() {
        let points = vec![
            DirectPosition::new(601.92791444745251, 1130.4631113024607, 9.0130903915382347)
                .unwrap(),
            DirectPosition::new(601.92791832847342, 1130.4631032795705, 9.0130907233102739)
                .unwrap(),
            DirectPosition::new(601.92791832847342, 1130.4631032795705, 9.0130907233102739)
                .unwrap(),
        ];
        let result = LinearRing::new(points);

        assert!(matches!(
            result,
            Err(Error::AdjacentDuplicatePositions { .. })
        ));
    }

    #[test]
    fn offset_linear_ring_test() {
        let mut linear_ring = LinearRing::new([
            DirectPosition::new(1.0, 2.0, 3.0).unwrap(),
            DirectPosition::new(2.0, 4.0, 6.0).unwrap(),
            DirectPosition::new(4.0, 7.0, 9.0).unwrap(),
        ])
        .unwrap();
        //let offset = nalgebra::Vector3::<f64>::new(1.0, -1.0, 3.0);
        let isometry: Isometry3<f64> =
            Isometry3::new(Vector3::new(1.0, -1.0, 3.0), Default::default());
        let expected_linear_ring = LinearRing::new([
            DirectPosition::new(2.0, 1.0, 6.0).unwrap(),
            DirectPosition::new(3.0, 3.0, 9.0).unwrap(),
            DirectPosition::new(5.0, 6.0, 12.0).unwrap(),
        ])
        .unwrap();

        linear_ring.apply_transform(&isometry);

        assert_eq!(linear_ring, expected_linear_ring);
    }
}
