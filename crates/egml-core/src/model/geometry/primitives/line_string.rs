use crate::Error::{ContainsDuplicateElements, NotEnoughElements};
use crate::model::geometry::primitives::{AbstractCurve, AsAbstractCurve, AsAbstractCurveMut};
use crate::model::geometry::{DirectPosition, Envelope};
use crate::{Error, impl_abstract_curve_traits};
use nalgebra::Isometry3;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct LineString {
    pub(crate) abstract_curve: AbstractCurve,
    points: Vec<DirectPosition>,
}

impl LineString {
    pub fn new(abstract_curve: AbstractCurve, points: Vec<DirectPosition>) -> Result<Self, Error> {
        let duplicates_count = points.windows(2).filter(|x| x[0] == x[1]).count();
        if duplicates_count >= 1 {
            return Err(ContainsDuplicateElements);
        }
        if points.len() < 2 {
            return Err(NotEnoughElements(
                "Line string must at least have two unique points",
            ));
        }

        Ok(Self {
            abstract_curve,
            points,
        })
    }

    pub fn points(&self) -> &Vec<DirectPosition> {
        &self.points
    }

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
