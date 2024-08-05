use crate::error::Error;
use crate::error::Error::NotEnoughElements;
use itertools::Itertools;

use crate::model::base::Gml;
use crate::model::geometry::DirectPosition;
use crate::operations::geometry::Geometry;
use crate::operations::surface::Surface;
use crate::Error::{ContainsDuplicateElements, ContainsEqualStartAndLastElement};
use nalgebra::Isometry3;
use rayon::prelude::*;

const MINIMUM_NUMBER_OF_POINTS: usize = 3;

#[derive(Debug, Clone, PartialEq)]
pub struct LinearRing {
    pub gml: Gml,
    points: Vec<DirectPosition>,
}

impl LinearRing {
    pub fn new(gml: Gml, points: Vec<DirectPosition>) -> Result<Self, Error> {
        let duplicates_count = points.windows(2).filter(|x| x[0] == x[1]).count();
        if duplicates_count >= 1 {
            return Err(ContainsDuplicateElements);
        }
        if points.len() < MINIMUM_NUMBER_OF_POINTS {
            return Err(NotEnoughElements(
                "Linear ring must at least have three points (without duplicates)",
            ));
        }
        if points.first().expect("") == points.last().expect("") {
            return Err(ContainsEqualStartAndLastElement);
        }

        Ok(Self { gml, points })
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

impl Geometry for LinearRing {
    fn points(&self) -> Vec<&DirectPosition> {
        self.points.iter().collect()
    }

    fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.points.iter_mut().for_each(|p| {
            p.apply_transform(m);
        });
        // self.points.dedup(); would need error handling
    }
}

impl Surface for LinearRing {
    fn outer_boundary_points(&self) -> Vec<&DirectPosition> {
        self.points.iter().collect()
    }
}
