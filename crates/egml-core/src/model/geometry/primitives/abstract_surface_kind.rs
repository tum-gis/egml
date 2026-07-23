use crate::model::common::{
    ApplyTransform, ComputeEnvelope, GeometryType, HasGeometryType, IterGeometries, Triangulate,
    Triangulation,
};
use crate::model::geometry::complexes::CompositeSurface;
use crate::model::geometry::primitives::surface_kind::SurfaceKind;
use crate::model::geometry::primitives::{
    AbstractSurface, AsAbstractSurface, AsAbstractSurfaceMut, Polygon, Shell, Surface,
};
use crate::model::geometry::refs::AbstractGeometryKindRef;
use crate::model::geometry::{DirectPosition, Envelope};
use crate::{Error, impl_abstract_surface_mut_traits, impl_abstract_surface_traits};
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub enum AbstractSurfaceKind {
    CompositeSurface(CompositeSurface),
    Polygon(Polygon),
    Shell(Shell),
    Surface(Surface),
    SurfaceKind(SurfaceKind),
}

impl AsAbstractSurface for AbstractSurfaceKind {
    fn abstract_surface(&self) -> &AbstractSurface {
        match self {
            AbstractSurfaceKind::CompositeSurface(x) => x.abstract_surface(),
            AbstractSurfaceKind::Polygon(x) => x.abstract_surface(),
            AbstractSurfaceKind::Shell(x) => x.abstract_surface(),
            AbstractSurfaceKind::Surface(x) => x.abstract_surface(),
            AbstractSurfaceKind::SurfaceKind(x) => x.abstract_surface(),
        }
    }
}

impl AsAbstractSurfaceMut for AbstractSurfaceKind {
    fn abstract_surface_mut(&mut self) -> &mut AbstractSurface {
        match self {
            AbstractSurfaceKind::CompositeSurface(x) => x.abstract_surface_mut(),
            AbstractSurfaceKind::Polygon(x) => x.abstract_surface_mut(),
            AbstractSurfaceKind::Shell(x) => x.abstract_surface_mut(),
            AbstractSurfaceKind::Surface(x) => x.abstract_surface_mut(),
            AbstractSurfaceKind::SurfaceKind(x) => x.abstract_surface_mut(),
        }
    }
}

impl_abstract_surface_traits!(AbstractSurfaceKind);
impl_abstract_surface_mut_traits!(AbstractSurfaceKind);

impl HasGeometryType for AbstractSurfaceKind {
    fn geometry_type(&self) -> GeometryType {
        match self {
            AbstractSurfaceKind::CompositeSurface(x) => x.geometry_type(),
            AbstractSurfaceKind::Polygon(x) => x.geometry_type(),
            AbstractSurfaceKind::Shell(x) => x.geometry_type(),
            AbstractSurfaceKind::Surface(x) => x.geometry_type(),
            AbstractSurfaceKind::SurfaceKind(x) => x.geometry_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_abstract_surface_kind {
    ($variant:ident, $type:ty) => {
        impl From<$type> for $crate::model::geometry::primitives::AbstractSurfaceKind {
            #[allow(unreachable_code)]
            fn from(x: $type) -> Self {
                $crate::model::geometry::primitives::AbstractSurfaceKind::$variant(x.into())
            }
        }
        $crate::impl_from_for_abstract_geometric_primitive_kind!(AbstractSurfaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_abstract_surface_kind!($variant, $variant);
    };
}

#[macro_export]
macro_rules! impl_try_from_for_abstract_surface_kind {
    ($variant:ident, $type:ty) => {
        impl TryFrom<$crate::model::geometry::primitives::AbstractSurfaceKind> for $type {
            type Error = ();
            #[allow(unreachable_code)]
            fn try_from(
                x: $crate::model::geometry::primitives::AbstractSurfaceKind,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::geometry::primitives::AbstractSurfaceKind::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_abstract_geometric_primitive_kind!(AbstractSurfaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_abstract_surface_kind!($variant, $variant);
    };
}

impl_from_for_abstract_surface_kind!(CompositeSurface);
impl_from_for_abstract_surface_kind!(Polygon);
impl_from_for_abstract_surface_kind!(Shell);
impl_from_for_abstract_surface_kind!(Surface);
impl_from_for_abstract_surface_kind!(SurfaceKind);
impl_try_from_for_abstract_surface_kind!(CompositeSurface);
impl_try_from_for_abstract_surface_kind!(Polygon);
impl_try_from_for_abstract_surface_kind!(Shell);
impl_try_from_for_abstract_surface_kind!(Surface);
impl_try_from_for_abstract_surface_kind!(SurfaceKind);

impl AbstractSurfaceKind {
    pub fn area_3d(&self) -> Result<f64, Error> {
        match self {
            AbstractSurfaceKind::CompositeSurface(x) => x.area_3d(),
            AbstractSurfaceKind::Polygon(x) => x.area_3d(),
            AbstractSurfaceKind::Shell(_x) => todo!("needs to be implemented for shells"),
            AbstractSurfaceKind::Surface(x) => x.area_3d(),
            AbstractSurfaceKind::SurfaceKind(x) => x.area_3d(),
        }
    }

    pub fn points(&self) -> Vec<&DirectPosition> {
        match self {
            AbstractSurfaceKind::CompositeSurface(x) => x.points(),
            AbstractSurfaceKind::Polygon(x) => x.points(),
            AbstractSurfaceKind::Shell(x) => x.points(),
            AbstractSurfaceKind::Surface(x) => x.points(),
            AbstractSurfaceKind::SurfaceKind(x) => x.points(),
        }
    }
}

impl IterGeometries for AbstractSurfaceKind {
    fn iter_geometries(&self) -> Box<dyn Iterator<Item = AbstractGeometryKindRef<'_>> + '_> {
        match self {
            AbstractSurfaceKind::CompositeSurface(x) => x.iter_geometries(),
            AbstractSurfaceKind::Polygon(x) => x.iter_geometries(),
            AbstractSurfaceKind::Shell(x) => x.iter_geometries(),
            AbstractSurfaceKind::Surface(x) => x.iter_geometries(),
            AbstractSurfaceKind::SurfaceKind(x) => x.iter_geometries(),
        }
    }
}

impl ApplyTransform for AbstractSurfaceKind {
    fn apply_transform(&mut self, transform: Transform3<f64>) {
        match self {
            AbstractSurfaceKind::CompositeSurface(x) => x.apply_transform(transform),
            AbstractSurfaceKind::Polygon(x) => x.apply_transform(transform),
            AbstractSurfaceKind::Shell(x) => x.apply_transform(transform),
            AbstractSurfaceKind::Surface(x) => x.apply_transform(transform),
            AbstractSurfaceKind::SurfaceKind(x) => x.apply_transform(transform),
        }
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        match self {
            AbstractSurfaceKind::CompositeSurface(x) => x.apply_isometry(isometry),
            AbstractSurfaceKind::Polygon(x) => x.apply_isometry(isometry),
            AbstractSurfaceKind::Shell(x) => x.apply_isometry(isometry),
            AbstractSurfaceKind::Surface(x) => x.apply_isometry(isometry),
            AbstractSurfaceKind::SurfaceKind(x) => x.apply_isometry(isometry),
        }
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        match self {
            AbstractSurfaceKind::CompositeSurface(x) => x.apply_translation(vector),
            AbstractSurfaceKind::Polygon(x) => x.apply_translation(vector),
            AbstractSurfaceKind::Shell(x) => x.apply_translation(vector),
            AbstractSurfaceKind::Surface(x) => x.apply_translation(vector),
            AbstractSurfaceKind::SurfaceKind(x) => x.apply_translation(vector),
        }
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        match self {
            AbstractSurfaceKind::CompositeSurface(x) => x.apply_rotation(rotation),
            AbstractSurfaceKind::Polygon(x) => x.apply_rotation(rotation),
            AbstractSurfaceKind::Shell(x) => x.apply_rotation(rotation),
            AbstractSurfaceKind::Surface(x) => x.apply_rotation(rotation),
            AbstractSurfaceKind::SurfaceKind(x) => x.apply_rotation(rotation),
        }
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        match self {
            AbstractSurfaceKind::CompositeSurface(x) => x.apply_scale(scale),
            AbstractSurfaceKind::Polygon(x) => x.apply_scale(scale),
            AbstractSurfaceKind::Shell(x) => x.apply_scale(scale),
            AbstractSurfaceKind::Surface(x) => x.apply_scale(scale),
            AbstractSurfaceKind::SurfaceKind(x) => x.apply_scale(scale),
        }
    }
}

impl ComputeEnvelope for AbstractSurfaceKind {
    fn compute_envelope(&self) -> Option<Envelope> {
        match self {
            AbstractSurfaceKind::CompositeSurface(x) => x.compute_envelope(),
            AbstractSurfaceKind::Polygon(x) => x.compute_envelope(),
            AbstractSurfaceKind::Shell(x) => x.compute_envelope(),
            AbstractSurfaceKind::Surface(x) => x.compute_envelope(),
            AbstractSurfaceKind::SurfaceKind(x) => x.compute_envelope(),
        }
    }
}

impl Triangulate for AbstractSurfaceKind {
    fn triangulate(&self) -> Result<Triangulation, Error> {
        match self {
            AbstractSurfaceKind::CompositeSurface(x) => x.triangulate(),
            AbstractSurfaceKind::Polygon(x) => x.triangulate(),
            AbstractSurfaceKind::Shell(x) => x.triangulate(),
            AbstractSurfaceKind::Surface(x) => x.triangulate(),
            AbstractSurfaceKind::SurfaceKind(x) => x.triangulate(),
        }
    }
}
