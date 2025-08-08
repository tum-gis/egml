use crate::error::Error;
use crate::model::geometry::{DirectPosition, Triangle};
use crate::operations::geometry::Geometry;
use nalgebra::Isometry3;
use rayon::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct TriangulatedSurface {
    patches: Vec<Triangle>,
}

impl TriangulatedSurface {
    pub fn new(patches: Vec<Triangle>) -> Result<Self, Error> {
        if patches.is_empty() {
            return Err(Error::MustNotBeEmpty("triangulated surface"));
        }

        Ok(TriangulatedSurface { patches })
    }

    pub fn from_triangulated_surfaces(surfaces: Vec<TriangulatedSurface>) -> Result<Self, Error> {
        if surfaces.is_empty() {
            return Err(Error::MustNotBeEmpty("surfaces to combine"));
        }

        let total_patches: Vec<Triangle> = surfaces
            .into_iter()
            .flat_map(|surface| surface.patches)
            .collect();

        Ok(TriangulatedSurface {
            patches: total_patches,
        })
    }

    pub fn patches(&self) -> &Vec<Triangle> {
        self.patches.as_ref()
    }

    pub fn append_patches(&mut self, mut patches: Vec<Triangle>) {
        self.patches.append(&mut patches)
    }

    pub fn number_of_patches(&self) -> usize {
        self.patches.len()
    }
}

impl Geometry for TriangulatedSurface {
    fn points(&self) -> Vec<&DirectPosition> {
        self.patches.iter().fold(Vec::new(), |mut acc, x| {
            acc.extend(x.points().iter());
            acc
        })
    }

    fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.patches.par_iter_mut().for_each(|p| {
            p.apply_transform(m);
        });
    }
}
