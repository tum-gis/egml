use crate::error::Error;
use crate::error::Error::NotEnoughElements;
use crate::DirectPosition;

const MINIMUM_NUMBER_OF_POINTS: usize = 3;

#[derive(Debug, Clone, PartialEq)]
pub struct LinearRing {
    points: Vec<DirectPosition>,
}

impl LinearRing {
    pub fn new(points: Vec<DirectPosition>) -> Result<Self, Error> {
        if points.len() < MINIMUM_NUMBER_OF_POINTS {
            return Err(NotEnoughElements(
                "Linear ring must at least have three points",
            ));
        }
        Ok(Self { points })
    }

    pub fn points(&self) -> &Vec<DirectPosition> {
        self.points.as_ref()
    }

    pub fn set_points(&mut self, val: Vec<DirectPosition>) -> Result<(), Error> {
        if val.len() < MINIMUM_NUMBER_OF_POINTS {
            return Err(NotEnoughElements(
                "Linear ring must at least have three points",
            ));
        }
        self.points = val;
        Ok(())
    }
}
