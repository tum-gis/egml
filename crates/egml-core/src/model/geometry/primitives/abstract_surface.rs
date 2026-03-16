use crate::Error;
use crate::model::geometry::complexes::CompositeSurface;
use crate::model::geometry::primitives::{
    AbstractGeometricPrimitive, AsAbstractGeometricPrimitive, AsAbstractGeometricPrimitiveMut,
    Polygon, Surface, TriangulatedSurface,
};
use crate::model::geometry::{DirectPosition, Envelope};
use nalgebra::Isometry3;

/// Base data shared by all GML surface geometry types (ISO 19136 §10.5.3).
///
/// A surface is a 2-D geometric primitive.  Concrete surface types include
/// [`Polygon`], [`Surface`], and [`CompositeSurface`].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct AbstractSurface {
    pub(crate) abstract_geometric_primitive: AbstractGeometricPrimitive,
}

impl AbstractSurface {
    pub fn new(abstract_geometric_primitive: AbstractGeometricPrimitive) -> Self {
        Self {
            abstract_geometric_primitive,
        }
    }
}

/// Object-safe read accessor for [`AbstractSurface`] fields.
pub trait AsAbstractSurface: AsAbstractGeometricPrimitive {
    /// Returns a reference to the embedded [`AbstractSurface`] base data.
    fn abstract_surface(&self) -> &AbstractSurface;
}

/// Mutable companion to [`AsAbstractSurface`].
pub trait AsAbstractSurfaceMut: AsAbstractSurface + AsAbstractGeometricPrimitiveMut {
    /// Returns a mutable reference to the embedded [`AbstractSurface`] base data.
    fn abstract_surface_mut(&mut self) -> &mut AbstractSurface;
}

impl AsAbstractSurface for AbstractSurface {
    fn abstract_surface(&self) -> &AbstractSurface {
        self
    }
}

impl AsAbstractSurfaceMut for AbstractSurface {
    fn abstract_surface_mut(&mut self) -> &mut AbstractSurface {
        self
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_abstract_surface_traits {
    ($type:ty) => {
        $crate::impl_abstract_geometric_primitive_traits!($type);

        impl $crate::model::geometry::primitives::AsAbstractGeometricPrimitive for $type {
            fn abstract_geometric_primitive(
                &self,
            ) -> &$crate::model::geometry::primitives::AbstractGeometricPrimitive {
                use $crate::model::geometry::primitives::AsAbstractSurface;
                &self.abstract_surface().abstract_geometric_primitive
            }
        }

        impl $crate::model::geometry::primitives::AsAbstractGeometricPrimitiveMut for $type {
            fn abstract_geometric_primitive_mut(
                &mut self,
            ) -> &mut $crate::model::geometry::primitives::AbstractGeometricPrimitive {
                use $crate::model::geometry::primitives::AsAbstractSurfaceMut;
                &mut self.abstract_surface_mut().abstract_geometric_primitive
            }
        }
    };
}

impl_abstract_surface_traits!(AbstractSurface);

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
    pub fn triangulate(&self) -> Result<TriangulatedSurface, Error> {
        match self {
            Self::Surface(x) => x.triangulate(),
            Self::Polygon(x) => x.triangulate(),
            Self::CompositeSurface(x) => x.triangulate(),
        }
    }

    pub fn compute_envelope(&self) -> Envelope {
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
