use crate::Error::NotEnoughElements;
use crate::model::geometry::aggregates::AggregationType;
use crate::model::geometry::primitives::{
    AbstractSurface, AsAbstractSurface, AsAbstractSurfaceMut, SurfaceProperty, TriangulatedSurface,
};
use crate::model::geometry::{DirectPosition, Envelope};
use crate::{Error, impl_abstract_surface_traits};
use nalgebra::Isometry3;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;

#[derive(Debug, Clone, PartialEq)]
pub struct CompositeSurface {
    pub(crate) abstract_surface: AbstractSurface,
    surface_member: Vec<SurfaceProperty>,
    aggregation_type: AggregationType,
}

impl CompositeSurface {
    pub fn new(
        abstract_surface: AbstractSurface,
        surface_members: Vec<SurfaceProperty>,
        aggregation_type: AggregationType,
    ) -> Result<Self, Error> {
        if surface_members.is_empty() {
            return Err(NotEnoughElements(
                "Composite surface must have at least one surface member.",
            ));
        }

        Ok(CompositeSurface {
            abstract_surface,
            surface_member: surface_members,
            aggregation_type,
        })
    }

    pub fn surface_member(&self) -> &[SurfaceProperty] {
        &self.surface_member
    }

    pub fn aggregation_type(&self) -> AggregationType {
        self.aggregation_type
    }

    pub fn surface_member_count(&self) -> usize {
        self.surface_member.len()
    }

    pub fn triangulate(&self) -> Result<TriangulatedSurface, Error> {
        let triangulated_surfaces = self
            .surface_member
            .iter()
            .map(|x| x.content.triangulate())
            .collect::<Result<Vec<TriangulatedSurface>, Error>>()?;

        TriangulatedSurface::from_triangulated_surfaces(triangulated_surfaces)
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

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.surface_member
            .par_iter_mut()
            .for_each(|x| x.apply_transform(m));
    }

    pub fn points(&self) -> Vec<&DirectPosition> {
        todo!("needs to be implemented")
    }
}

impl AsAbstractSurface for CompositeSurface {
    fn abstract_surface(&self) -> &AbstractSurface {
        &self.abstract_surface
    }
}

impl AsAbstractSurfaceMut for CompositeSurface {
    fn abstract_surface_mut(&mut self) -> &mut AbstractSurface {
        &mut self.abstract_surface
    }
}

impl_abstract_surface_traits!(CompositeSurface);
