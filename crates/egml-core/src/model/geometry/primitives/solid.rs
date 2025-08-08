use crate::error::Error;
use crate::model::base::Gml;
use crate::model::geometry::{DirectPosition, SurfaceProperty, TriangulatedSurface};
use crate::operations::geometry::Geometry;
use crate::operations::triangulate::Triangulate;
use nalgebra::Isometry3;
use rayon::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Solid {
    pub gml: Gml,
    members: Vec<SurfaceProperty>,
}

impl Solid {
    pub fn new(gml: Gml, members: Vec<SurfaceProperty>) -> Result<Self, Error> {
        if members.is_empty() {
            return Err(Error::MustNotBeEmpty("solid"));
        }

        Ok(Self { gml, members })
    }

    pub fn members(&self) -> &Vec<SurfaceProperty> {
        self.members.as_ref()
    }

    pub fn set_members(&mut self, val: Vec<SurfaceProperty>) -> Result<(), Error> {
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

impl Triangulate for Solid {
    fn triangulate(&self) -> Result<TriangulatedSurface, Error> {
        let triangulated_surfaces: Vec<TriangulatedSurface> = self
            .members
            .iter()
            .flat_map(|x| &x.linear_ring)
            .map(|x| x.triangulate())
            .collect::<Result<Vec<TriangulatedSurface>, Error>>()?;

        let combined_triangulated_surface =
            TriangulatedSurface::from_triangulated_surfaces(triangulated_surfaces)?;
        Ok(combined_triangulated_surface)
    }
}
