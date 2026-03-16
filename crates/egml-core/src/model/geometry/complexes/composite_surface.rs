use crate::Error::TooFewElements;
use crate::model::geometry::aggregates::AggregationType;
use crate::model::geometry::primitives::{
    AbstractSurface, AsAbstractSurface, AsAbstractSurfaceMut, SurfaceProperty, TriangulatedSurface,
};
use crate::model::geometry::{DirectPosition, Envelope};
use crate::{Error, impl_abstract_surface_traits};
use nalgebra::Isometry3;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;

/// A topology-aware surface composed of connected [`SurfaceProperty`] members.
///
/// Corresponds to `gml:CompositeSurface` in ISO 19136 §10.7.6.  Unlike
/// [`MultiSurface`](crate::model::geometry::aggregates::MultiSurface), a
/// `CompositeSurface` requires that its constituent surfaces share boundaries
/// coherently, forming a single connected manifold.
#[derive(Debug, Clone, PartialEq)]
pub struct CompositeSurface {
    pub(crate) abstract_surface: AbstractSurface,
    surface_member: Vec<SurfaceProperty>,
    aggregation_type: AggregationType,
}

impl CompositeSurface {
    /// Creates a new `CompositeSurface` from surface members and an aggregation type.
    ///
    /// # Errors
    ///
    /// Returns [`Error::TooFewElements`] if `surface_members` is empty.
    pub fn new(
        abstract_surface: AbstractSurface,
        surface_members: Vec<SurfaceProperty>,
        aggregation_type: AggregationType,
    ) -> Result<Self, Error> {
        if surface_members.is_empty() {
            return Err(TooFewElements {
                geometry: "gml:CompositeSurface",
                minimum: 1,
                spec: Some("ISO 19136 §10.5.14"),
                id: None,
                message: None,
            });
        }

        Ok(CompositeSurface {
            abstract_surface,
            surface_member: surface_members,
            aggregation_type,
        })
    }

    /// Returns the surface members as a slice.
    pub fn surface_member(&self) -> &[SurfaceProperty] {
        &self.surface_member
    }

    /// Returns the aggregation type that qualifies how members relate.
    pub fn aggregation_type(&self) -> AggregationType {
        self.aggregation_type
    }

    /// Returns the number of surface members.
    pub fn surface_member_count(&self) -> usize {
        self.surface_member.len()
    }

    /// Triangulates all surface members and merges them into a single [`TriangulatedSurface`].
    ///
    /// # Errors
    ///
    /// Returns [`Error::TriangulationFailed`] if any member cannot be triangulated.
    pub fn triangulate(&self) -> Result<TriangulatedSurface, Error> {
        let triangulated_surfaces = self
            .surface_member
            .iter()
            .map(|x| x.content.triangulate())
            .collect::<Result<Vec<TriangulatedSurface>, Error>>()?;

        TriangulatedSurface::from_triangulated_surfaces(triangulated_surfaces)
    }

    /// Returns the union of the bounding boxes of all surface members.
    pub fn compute_envelope(&self) -> Envelope {
        let envelopes: Vec<Envelope> = self
            .surface_member
            .iter()
            .map(|x| x.compute_envelope())
            .collect::<Vec<_>>();

        Envelope::from_envelopes(&envelopes)
            .expect("CompositeSurface must have at least one surface member")
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
