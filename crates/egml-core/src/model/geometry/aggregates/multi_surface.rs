use crate::error::Error;
use crate::impl_abstract_geometric_aggregate_traits;
use crate::model::geometry::aggregates::{
    AbstractGeometricAggregate, AsAbstractGeometricAggregate, AsAbstractGeometricAggregateMut,
};
use crate::model::geometry::primitives::{SurfaceKind, TriangulatedSurface};
use crate::model::geometry::{DirectPosition, Envelope};
use nalgebra::Isometry3;
use rayon::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct MultiSurface {
    pub(crate) abstract_geometric_aggregate: AbstractGeometricAggregate,
    surface_member: Vec<SurfaceKind>,
}

impl MultiSurface {
    pub fn new(
        abstract_geometric_aggregate: AbstractGeometricAggregate,
        members: Vec<SurfaceKind>,
    ) -> Result<Self, Error> {
        if members.is_empty() {
            return Err(Error::MustNotBeEmpty("multi surface"));
        }

        Ok(Self {
            abstract_geometric_aggregate,
            surface_member: members,
        })
    }

    pub fn surface_member(&self) -> &Vec<SurfaceKind> {
        self.surface_member.as_ref()
    }

    pub fn set_surface_member(&mut self, val: Vec<SurfaceKind>) -> Result<(), Error> {
        if val.is_empty() {
            return Err(Error::MustNotBeEmpty("multi surface"));
        }
        self.surface_member = val;
        Ok(())
    }

    pub fn triangulate(&self) -> Result<TriangulatedSurface, Error> {
        let triangulated_surfaces: Vec<TriangulatedSurface> = self
            .surface_member
            .iter()
            .map(|x| x.triangulate())
            .collect::<Result<Vec<TriangulatedSurface>, Error>>()?;

        let combined_triangulated_surface =
            TriangulatedSurface::from_triangulated_surfaces(triangulated_surfaces)?;
        Ok(combined_triangulated_surface)
    }

    pub fn points(&self) -> Vec<&DirectPosition> {
        self.surface_member.iter().fold(Vec::new(), |mut acc, x| {
            acc.extend(x.points().iter());
            acc
        })
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.surface_member.par_iter_mut().for_each(|p| {
            p.apply_transform(m);
        });
    }

    pub fn compute_envelope(&self) -> Envelope {
        let envelopes: Vec<Envelope> = self
            .surface_member
            .iter()
            .map(|x| x.compute_envelope())
            .collect::<Vec<_>>();

        Envelope::from_envelopes(&envelopes.iter().collect::<Vec<_>>())
            .expect("MultiSurface must have at least one surface member")
    }
}

impl AsAbstractGeometricAggregate for MultiSurface {
    fn abstract_geometric_aggregate(&self) -> &AbstractGeometricAggregate {
        &self.abstract_geometric_aggregate
    }
}

impl AsAbstractGeometricAggregateMut for MultiSurface {
    fn abstract_geometric_aggregate_mut(&mut self) -> &mut AbstractGeometricAggregate {
        &mut self.abstract_geometric_aggregate
    }
}

impl_abstract_geometric_aggregate_traits!(MultiSurface);
