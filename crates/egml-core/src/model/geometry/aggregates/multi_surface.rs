use crate::error::Error;
use crate::model::base::Gml;
use crate::model::geometry::{DirectPosition, Polygon};
use crate::operations::geometry::Geometry;
use nalgebra::Isometry3;
use rayon::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct MultiSurface {
    pub gml: Gml,
    surface_member: Vec<Polygon>,
}

impl MultiSurface {
    pub fn new(gml: Gml, members: Vec<Polygon>) -> Result<Self, Error> {
        if members.is_empty() {
            return Err(Error::MustNotBeEmpty("multi surface"));
        }

        Ok(Self {
            gml,
            surface_member: members,
        })
    }

    pub fn surface_member(&self) -> &Vec<Polygon> {
        self.surface_member.as_ref()
    }

    pub fn set_surface_member(&mut self, val: Vec<Polygon>) -> Result<(), Error> {
        if val.is_empty() {
            return Err(Error::MustNotBeEmpty("multi surface"));
        }
        self.surface_member = val;
        Ok(())
    }
}

impl Geometry for MultiSurface {
    fn points(&self) -> Vec<&DirectPosition> {
        self.surface_member.iter().fold(Vec::new(), |mut acc, x| {
            acc.extend(x.points().iter());
            acc
        })
    }

    fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.surface_member.par_iter_mut().for_each(|p| {
            p.apply_transform(m);
        });
    }
}
