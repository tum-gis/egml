use crate::error::Error;
use crate::error::Error::NotEnoughElements;

use crate::{DirectPosition, Envelope};
use itertools::Itertools;

const MINIMUM_NUMBER_OF_POINTS: usize = 3;

#[derive(Debug, Clone, PartialEq)]
pub struct LinearRing {
    points: Vec<DirectPosition>,
}

impl LinearRing {
    pub fn new(mut points: Vec<DirectPosition>) -> Result<Self, Error> {
        points.dedup();
        if points.len() < MINIMUM_NUMBER_OF_POINTS {
            return Err(NotEnoughElements(
                "Linear ring must at least have three points (without duplicates)",
            ));
        }

        Ok(Self { points })
    }

    pub fn points(&self) -> &Vec<DirectPosition> {
        self.points.as_ref()
    }

    pub fn get_lower_corner(&self) -> DirectPosition {
        let x_min = self
            .points
            .iter()
            .min_by(|a, b| a.x().partial_cmp(&b.x()).unwrap())
            .unwrap()
            .x();
        let y_min = self
            .points
            .iter()
            .min_by(|a, b| a.y().partial_cmp(&b.y()).unwrap())
            .unwrap()
            .y();
        let z_min = self
            .points
            .iter()
            .min_by(|a, b| a.z().partial_cmp(&b.z()).unwrap())
            .unwrap()
            .z();

        DirectPosition::new(x_min, y_min, z_min).unwrap()
    }

    pub fn get_upper_corner(&self) -> DirectPosition {
        let x_max = self
            .points
            .iter()
            .max_by(|a, b| a.x().partial_cmp(&b.x()).unwrap())
            .unwrap()
            .x();
        let y_max = self
            .points
            .iter()
            .max_by(|a, b| a.y().partial_cmp(&b.y()).unwrap())
            .unwrap()
            .y();
        let z_max = self
            .points
            .iter()
            .max_by(|a, b| a.z().partial_cmp(&b.z()).unwrap())
            .unwrap()
            .z();

        DirectPosition::new(x_max, y_max, z_max).unwrap()
    }

    pub fn get_envelope(&self) -> Envelope {
        let lower = self.get_lower_corner();
        let upper = self.get_upper_corner();

        Envelope::new(lower, upper).expect("Must be constructable with a valid linear ring")
    }

    pub fn set_points(&mut self, mut val: Vec<DirectPosition>) -> Result<(), Error> {
        val.dedup();
        if val.len() < MINIMUM_NUMBER_OF_POINTS {
            return Err(NotEnoughElements(
                "Linear ring must at least have three points (without duplicates)",
            ));
        }
        self.points = val;
        Ok(())
    }
}
