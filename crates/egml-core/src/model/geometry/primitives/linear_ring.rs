use crate::error::Error;
use crate::error::Error::NotEnoughElements;

use crate::Error::{ContainsDuplicateElements, ContainsEqualStartAndLastElement};
use crate::impl_abstract_ring_traits;
use crate::model::geometry::primitives::{AbstractRing, AsAbstractRing, AsAbstractRingMut};
use crate::model::geometry::{DirectPosition, Envelope};
use nalgebra::Isometry3;
use rayon::prelude::*;

const MINIMUM_NUMBER_OF_POINTS: usize = 3;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct LinearRing {
    pub(crate) abstract_ring: AbstractRing,
    points: Vec<DirectPosition>,
}

impl LinearRing {
    pub fn new(abstract_ring: AbstractRing, points: Vec<DirectPosition>) -> Result<Self, Error> {
        let duplicates_count = points.windows(2).filter(|x| x[0] == x[1]).count();
        if duplicates_count >= 1 {
            return Err(ContainsDuplicateElements);
        }
        if points.len() < MINIMUM_NUMBER_OF_POINTS {
            return Err(NotEnoughElements(
                "Linear ring must at least have three unique points",
            ));
        }
        if points.first().expect("") == points.last().expect("") {
            return Err(ContainsEqualStartAndLastElement);
        }

        Ok(Self {
            abstract_ring,
            points,
        })
    }

    pub(crate) fn new_unchecked(abstract_ring: AbstractRing, points: Vec<DirectPosition>) -> Self {
        Self {
            abstract_ring,
            points,
        }
    }

    pub fn set_points(&mut self, val: Vec<DirectPosition>) -> Result<(), Error> {
        let duplicates_count = val.windows(2).filter(|x| x[0] == x[1]).count();
        if duplicates_count >= 1 {
            return Err(ContainsDuplicateElements);
        }
        if val.len() < MINIMUM_NUMBER_OF_POINTS {
            return Err(NotEnoughElements(
                "Linear ring must at least have three unique points",
            ));
        }
        self.points = val;
        Ok(())
    }

    pub fn points(&self) -> &Vec<DirectPosition> {
        &self.points
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.points.iter_mut().for_each(|p| {
            p.apply_transform(m);
        });
        // self.points.dedup(); would need error handling
    }

    pub fn compute_envelope(&self) -> Envelope {
        Envelope::from_points(&self.points).expect("linear ring must have valid points")
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
    fn linear_ring_construction_test() {
        let abstract_ring = AbstractRing::default();
        let points = vec![
            DirectPosition::new(601.92791444745251, 1130.4631113024607, 9.0130903915382347)
                .unwrap(),
            DirectPosition::new(601.92791832847342, 1130.4631032795705, 9.0130907233102739)
                .unwrap(),
            DirectPosition::new(601.92791832847342, 1130.4631032795705, 9.0130907233102739)
                .unwrap(),
        ];
        let result = LinearRing::new(abstract_ring, points);

        assert!(matches!(result, Err(Error::ContainsDuplicateElements)));
    }

    #[test]
    fn offset_linear_ring_test() {
        let mut linear_ring = LinearRing::new(
            AbstractRing::default(),
            vec![
                DirectPosition::new(1.0, 2.0, 3.0).unwrap(),
                DirectPosition::new(2.0, 4.0, 6.0).unwrap(),
                DirectPosition::new(4.0, 7.0, 9.0).unwrap(),
            ],
        )
        .unwrap();
        //let offset = nalgebra::Vector3::<f64>::new(1.0, -1.0, 3.0);
        let isometry: Isometry3<f64> =
            Isometry3::new(Vector3::new(1.0, -1.0, 3.0), Default::default());
        let expected_linear_ring = LinearRing::new(
            AbstractRing::default(),
            vec![
                DirectPosition::new(2.0, 1.0, 6.0).unwrap(),
                DirectPosition::new(3.0, 3.0, 9.0).unwrap(),
                DirectPosition::new(5.0, 6.0, 12.0).unwrap(),
            ],
        )
        .unwrap();

        linear_ring.apply_transform(&isometry);

        assert_eq!(linear_ring, expected_linear_ring);
    }
}
