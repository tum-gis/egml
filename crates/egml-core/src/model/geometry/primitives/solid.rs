use crate::error::Error;
use crate::impl_abstract_solid_traits;
use crate::model::geometry::primitives::{
    AbstractSolid, AsAbstractSolid, AsAbstractSolidMut, SurfaceProperty, TriangulatedSurface,
};
use crate::model::geometry::{DirectPosition, Envelope};
use nalgebra::Isometry3;
use rayon::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Solid {
    pub(crate) abstract_solid: AbstractSolid,
    members: Vec<SurfaceProperty>,
}

impl Solid {
    pub fn new(
        abstract_solid: AbstractSolid,
        members: Vec<SurfaceProperty>,
    ) -> Result<Self, Error> {
        if members.is_empty() {
            return Err(Error::MustNotBeEmpty("solid"));
        }

        Ok(Self {
            abstract_solid,
            members,
        })
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

    pub fn triangulate(&self) -> Result<TriangulatedSurface, Error> {
        let triangulated_surfaces: Vec<TriangulatedSurface> = self
            .members
            .iter()
            .map(|x| x.content.triangulate())
            .collect::<Result<Vec<TriangulatedSurface>, Error>>()?;

        let combined_triangulated_surface =
            TriangulatedSurface::from_triangulated_surfaces(triangulated_surfaces)?;
        Ok(combined_triangulated_surface)
    }

    pub fn points(&self) -> Vec<&DirectPosition> {
        self.members.iter().fold(Vec::new(), |mut acc, x| {
            acc.extend(x.points().iter());
            acc
        })
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.members.par_iter_mut().for_each(|p| {
            p.apply_transform(m);
        });
    }

    pub fn compute_envelope(&self) -> Envelope {
        let envelopes: Vec<Envelope> = self
            .members
            .iter()
            .map(|x| x.compute_envelope())
            .collect::<Vec<_>>();

        Envelope::from_envelopes(&envelopes.iter().collect::<Vec<_>>())
            .expect("MultiSurface must have at least one surface member")
    }
}

impl AsAbstractSolid for Solid {
    fn abstract_solid(&self) -> &AbstractSolid {
        &self.abstract_solid
    }
}

impl AsAbstractSolidMut for Solid {
    fn abstract_solid_mut(&mut self) -> &mut AbstractSolid {
        &mut self.abstract_solid
    }
}

impl_abstract_solid_traits!(Solid);
