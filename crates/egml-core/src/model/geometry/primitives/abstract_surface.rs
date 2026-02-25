use crate::Error;
use crate::model::geometry::complexes::CompositeSurface;
use crate::model::geometry::primitives::{
    AbstractGeometricPrimitive, AsAbstractGeometricPrimitive, AsAbstractGeometricPrimitiveMut,
    Polygon, Surface, TriangulatedSurface,
};
use crate::model::geometry::{DirectPosition, Envelope};
use nalgebra::Isometry3;

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

pub trait AsAbstractSurface: AsAbstractGeometricPrimitive {
    fn abstract_surface(&self) -> &AbstractSurface;
}

pub trait AsAbstractSurfaceMut: AsAbstractSurface + AsAbstractGeometricPrimitiveMut {
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

#[derive(Debug, Clone, PartialEq)]
pub enum SurfaceKind {
    Surface(Surface),
    Polygon(Polygon),
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
