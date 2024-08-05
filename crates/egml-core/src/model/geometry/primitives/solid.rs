use crate::error::Error;
use crate::model::base::Gml;
use crate::model::geometry::{DirectPosition, LinearRing};
use crate::operations::geometry::Geometry;
use nalgebra::Isometry3;
use rayon::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Solid {
    pub gml: Gml,
    members: Vec<LinearRing>,
}

impl Solid {
    pub fn new(gml: Gml, members: Vec<LinearRing>) -> Result<Self, Error> {
        if members.is_empty() {
            return Err(Error::MustNotBeEmpty("solid"));
        }

        Ok(Self { gml, members })
    }

    pub fn members(&self) -> &Vec<LinearRing> {
        self.members.as_ref()
    }

    pub fn set_members(&mut self, val: Vec<LinearRing>) -> Result<(), Error> {
        if val.is_empty() {
            return Err(Error::MustNotBeEmpty("solid"));
        }
        self.members = val;
        Ok(())
    }
}

impl Geometry for Solid {
    fn points(&self) -> Vec<&DirectPosition> {
        self.members.iter().fold(Vec::new(), |mut acc, x| {
            acc.extend(x.points().iter());
            acc
        })
    }

    fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.members.par_iter_mut().for_each(|p| {
            p.apply_transform(m);
        });
    }
}
