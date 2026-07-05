use crate::model::geometry::complexes::CompositeSurface;
use crate::model::geometry::primitives::{
    AbstractSurface, AsAbstractSurface, AsAbstractSurfaceMut, Polygon, Surface, TriangulatedSurface,
};
use crate::model::geometry::{DirectPosition, Envelope};
use crate::{Error, impl_abstract_surface_traits};
use nalgebra::Isometry3;

/// Discriminated union of all concrete surface implementations.
#[derive(Debug, Clone, PartialEq)]
pub enum SurfaceKind {
    /// A patched [`Surface`].
    Surface(Surface),
    /// A planar [`Polygon`].
    Polygon(Polygon),
    /// A topology-aware [`CompositeSurface`].
    CompositeSurface(CompositeSurface),
}

impl AsAbstractSurface for SurfaceKind {
    fn abstract_surface(&self) -> &AbstractSurface {
        match self {
            Self::Surface(x) => x.abstract_surface(),
            Self::Polygon(x) => x.abstract_surface(),
            Self::CompositeSurface(x) => x.abstract_surface(),
        }
    }
}

impl AsAbstractSurfaceMut for SurfaceKind {
    fn abstract_surface_mut(&mut self) -> &mut AbstractSurface {
        match self {
            Self::Surface(x) => x.abstract_surface_mut(),
            Self::Polygon(x) => x.abstract_surface_mut(),
            Self::CompositeSurface(x) => x.abstract_surface_mut(),
        }
    }
}

impl_abstract_surface_traits!(SurfaceKind);

impl SurfaceKind {
    pub fn area_3d(&self) -> Result<f64, Error> {
        match self {
            Self::Surface(x) => x.area_3d(),
            Self::Polygon(x) => x.area_3d(),
            Self::CompositeSurface(x) => x.area_3d(),
        }
    }

    pub fn triangulate(&self) -> Result<TriangulatedSurface, Error> {
        match self {
            Self::Surface(x) => x.triangulate(),
            Self::Polygon(x) => x.triangulate(),
            Self::CompositeSurface(x) => x.triangulate(),
        }
    }

    pub fn compute_envelope(&self) -> Option<Envelope> {
        match self {
            Self::Surface(x) => x.compute_envelope(),
            Self::Polygon(x) => x.compute_envelope(),
            Self::CompositeSurface(x) => x.compute_envelope(),
        }
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        match self {
            Self::Surface(x) => x.apply_transform(m),
            Self::Polygon(x) => x.apply_transform(m),
            Self::CompositeSurface(x) => x.apply_transform(m),
        }
    }

    pub fn points(&self) -> Vec<&DirectPosition> {
        match self {
            Self::Surface(x) => x.points(),
            Self::Polygon(x) => x.points(),
            Self::CompositeSurface(x) => x.points(),
        }
    }
}
