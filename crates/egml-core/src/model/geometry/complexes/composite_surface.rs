use crate::model::base::HasAssociationAttributes;
use crate::model::common::{
    ApplyTransform, ComputeEnvelope, IterGeometries, Triangulate, Triangulation,
};
use crate::model::geometry::aggregates::AggregationType;
use crate::model::geometry::primitives::{
    AbstractSurface, AbstractSurfaceProperty, AsAbstractSurface, AsAbstractSurfaceMut,
    TriangulatedSurface,
};
use crate::model::geometry::refs::AbstractGeometryKindRef;
use crate::model::geometry::{DirectPosition, Envelope};
use crate::{
    Error, impl_abstract_surface_mut_traits, impl_abstract_surface_traits, impl_has_geometry_type,
};
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;

/// A topology-aware surface composed of connected [`AbstractSurfaceProperty`] members.
///
/// Corresponds to `gml:CompositeSurface` in [OGC 07-036 §11.2.2.3](https://docs.ogc.org/is/07-036/07-036.pdf).  Unlike
/// [`MultiSurface`](crate::model::geometry::aggregates::MultiSurface), a
/// `CompositeSurface` requires that its constituent surfaces share boundaries
/// coherently, forming a single connected manifold.
#[derive(Debug, Clone, PartialEq)]
pub struct CompositeSurface {
    pub abstract_surface: AbstractSurface,
    surface_member: Vec<AbstractSurfaceProperty>,
    aggregation_type: AggregationType,
}

impl CompositeSurface {
    /// Creates a new `CompositeSurface` from surface members and an aggregation type.
    ///
    /// # Errors
    ///
    /// Returns [`Error::TooFewElements`] if `surface_members` is empty.
    pub fn new(
        surface_member: impl IntoIterator<Item = AbstractSurfaceProperty>,
        aggregation_type: AggregationType,
    ) -> Result<Self, Error> {
        let surface_member: Vec<AbstractSurfaceProperty> = surface_member.into_iter().collect();
        Self::validate(&surface_member)?;

        Ok(CompositeSurface {
            abstract_surface: AbstractSurface::default(),
            surface_member,
            aggregation_type,
        })
    }

    pub fn from_abstract_surface(
        abstract_surface: AbstractSurface,
        surface_member: impl IntoIterator<Item = AbstractSurfaceProperty>,
        aggregation_type: AggregationType,
    ) -> Result<Self, Error> {
        let surface_member: Vec<AbstractSurfaceProperty> = surface_member.into_iter().collect();
        Self::validate(&surface_member)?;

        Ok(Self {
            abstract_surface,
            surface_member,
            aggregation_type,
        })
    }

    fn validate(members: &[AbstractSurfaceProperty]) -> Result<(), Error> {
        if members.is_empty() {
            return Err(Error::TooFewElements {
                geometry: "gml:CompositeSurface",
                minimum: 1,
                spec: Some("OGC 07-036 §10.5.11.4"),
                id: None,
                detail: None,
            });
        }
        Ok(())
    }

    /// Returns the surface members as a slice.
    pub fn surface_member(&self) -> &[AbstractSurfaceProperty] {
        &self.surface_member
    }

    /// Replaces the surface members.
    ///
    /// # Errors
    ///
    /// Returns [`Error::TooFewElements`] if `surface_members` is empty.
    pub fn set_surface_member(
        &mut self,
        surface_members: Vec<AbstractSurfaceProperty>,
    ) -> Result<(), Error> {
        Self::validate(&surface_members)?;
        self.surface_member = surface_members;
        Ok(())
    }

    pub fn push_surface_member(&mut self, member: AbstractSurfaceProperty) {
        self.surface_member.push(member);
    }

    pub fn extend_surface_members(
        &mut self,
        members: impl IntoIterator<Item = AbstractSurfaceProperty>,
    ) {
        self.surface_member.extend(members);
    }

    /// Returns the aggregation type that qualifies how members relate.
    pub fn aggregation_type(&self) -> AggregationType {
        self.aggregation_type
    }

    pub fn set_aggregation_type(&mut self, aggregation_type: AggregationType) {
        self.aggregation_type = aggregation_type;
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
impl_abstract_surface_mut_traits!(CompositeSurface);
impl_has_geometry_type!(CompositeSurface, CompositeSurface);

impl CompositeSurface {
    /// Returns the number of surface members.
    pub fn surface_member_count(&self) -> usize {
        self.surface_member.len()
    }

    pub fn area_3d(&self) -> Result<f64, Error> {
        self.surface_member
            .iter()
            .map(|s| {
                s.object()
                    .ok_or_else(|| Error::UnresolvedSurfaceReference {
                        href: s.href().map(|h| h.to_string()),
                    })
                    .and_then(|kind| kind.area_3d())
            })
            .collect::<Result<Vec<f64>, Error>>()
            .map(|area_3ds| area_3ds.into_iter().sum())
    }

    pub fn points(&self) -> Vec<&DirectPosition> {
        todo!("needs to be implemented")
    }
}

impl ApplyTransform for CompositeSurface {
    fn apply_transform(&mut self, transform: Transform3<f64>) {
        self.surface_member
            .par_iter_mut()
            .flat_map(|x| x.object_mut())
            .for_each(|x| x.apply_transform(transform));
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        self.surface_member
            .par_iter_mut()
            .flat_map(|x| x.object_mut())
            .for_each(|x| x.apply_isometry(isometry));
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        self.surface_member
            .par_iter_mut()
            .flat_map(|x| x.object_mut())
            .for_each(|x| x.apply_translation(vector));
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        self.surface_member
            .par_iter_mut()
            .flat_map(|x| x.object_mut())
            .for_each(|x| x.apply_rotation(rotation));
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        self.surface_member
            .par_iter_mut()
            .flat_map(|x| x.object_mut())
            .for_each(|x| x.apply_scale(scale));
    }
}

impl ComputeEnvelope for CompositeSurface {
    /// Returns the union of the bounding boxes of all surface members.
    fn compute_envelope(&self) -> Option<Envelope> {
        let envelopes: Vec<Envelope> = self
            .surface_member
            .iter()
            .flat_map(|x| x.object())
            .flat_map(|x| x.compute_envelope())
            .collect::<Vec<_>>();

        Envelope::from_envelopes(&envelopes)
    }
}

impl IterGeometries for CompositeSurface {
    fn iter_geometries(&self) -> Box<dyn Iterator<Item = AbstractGeometryKindRef<'_>> + '_> {
        Box::new(
            std::iter::once(self.into()).chain(
                self.surface_member
                    .iter()
                    .filter_map(|x| x.object())
                    .flat_map(|x| x.iter_geometries()),
            ),
        )
    }
}

impl Triangulate for CompositeSurface {
    /// Members that fail to triangulate individually (e.g. a degenerate ring) are
    /// skipped rather than failing the whole aggregate; see their errors via
    /// [`Triangulation::skipped`].
    ///
    /// # Errors
    ///
    /// Returns [`Error::TooFewElements`] if no member could be triangulated.
    fn triangulate(&self) -> Result<Triangulation, Error> {
        let mut surfaces = Vec::new();
        let mut skipped = Vec::new();

        for member in self.surface_member.iter().flat_map(|x| x.object()) {
            match member.triangulate() {
                Ok(triangulation) => {
                    let (surface, nested_skipped) = triangulation.into_parts();
                    surfaces.push(surface);
                    skipped.extend(nested_skipped);
                }
                Err(error) => {
                    skipped.push(error);
                }
            }
        }

        let combined = TriangulatedSurface::from_triangulated_surfaces(surfaces)?;
        Ok(Triangulation::new(combined, skipped))
    }
}
