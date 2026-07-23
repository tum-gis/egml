use crate::model::common::{
    ApplyTransform, ComputeEnvelope, GeometryType, HasGeometryType, IterGeometries, Triangulate,
    Triangulation,
};
use crate::model::geometry::primitives::{AsSurface, AsSurfaceMut, Surface, TriangulatedSurface};
use crate::model::geometry::refs::AbstractGeometryKindRef;
use crate::model::geometry::{DirectPosition, Envelope};
use crate::{Error, impl_surface_mut_traits, impl_surface_traits};
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub enum SurfaceKind {
    TriangulatedSurface(TriangulatedSurface),
}

impl AsSurface for SurfaceKind {
    fn surface(&self) -> &Surface {
        match self {
            SurfaceKind::TriangulatedSurface(x) => x.surface(),
        }
    }
}

impl AsSurfaceMut for SurfaceKind {
    fn surface_mut(&mut self) -> &mut Surface {
        match self {
            SurfaceKind::TriangulatedSurface(x) => x.surface_mut(),
        }
    }
}

impl_surface_traits!(SurfaceKind);
impl_surface_mut_traits!(SurfaceKind);

impl HasGeometryType for SurfaceKind {
    fn geometry_type(&self) -> GeometryType {
        match self {
            SurfaceKind::TriangulatedSurface(x) => x.geometry_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_surface_kind {
    ($variant:ident, $type:ty) => {
        impl From<$type> for $crate::model::geometry::primitives::surface_kind::SurfaceKind {
            #[allow(unreachable_code)]
            fn from(x: $type) -> Self {
                $crate::model::geometry::primitives::surface_kind::SurfaceKind::$variant(x.into())
            }
        }
        $crate::impl_from_for_abstract_surface_kind!(SurfaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_surface_kind!($variant, $variant);
    };
}

#[macro_export]
macro_rules! impl_try_from_for_surface_kind {
    ($variant:ident, $type:ty) => {
        impl TryFrom<$crate::model::geometry::primitives::surface_kind::SurfaceKind> for $type {
            type Error = ();
            #[allow(unreachable_code)]
            fn try_from(
                x: $crate::model::geometry::primitives::surface_kind::SurfaceKind,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::geometry::primitives::surface_kind::SurfaceKind::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_abstract_surface_kind!(SurfaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_surface_kind!($variant, $variant);
    };
}

impl_from_for_surface_kind!(TriangulatedSurface);
impl_try_from_for_surface_kind!(TriangulatedSurface);

impl SurfaceKind {
    pub fn area_3d(&self) -> Result<f64, Error> {
        match self {
            SurfaceKind::TriangulatedSurface(x) => x.area_3d(),
        }
    }

    pub fn points(&self) -> Vec<&DirectPosition> {
        match self {
            SurfaceKind::TriangulatedSurface(x) => x.points(),
        }
    }
}

impl IterGeometries for SurfaceKind {
    fn iter_geometries(&self) -> Box<dyn Iterator<Item = AbstractGeometryKindRef<'_>> + '_> {
        match self {
            SurfaceKind::TriangulatedSurface(x) => x.iter_geometries(),
        }
    }
}

impl ApplyTransform for SurfaceKind {
    fn apply_transform(&mut self, transform: Transform3<f64>) {
        match self {
            SurfaceKind::TriangulatedSurface(x) => x.apply_transform(transform),
        }
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        match self {
            SurfaceKind::TriangulatedSurface(x) => x.apply_isometry(isometry),
        }
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        match self {
            SurfaceKind::TriangulatedSurface(x) => x.apply_translation(vector),
        }
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        match self {
            SurfaceKind::TriangulatedSurface(x) => x.apply_rotation(rotation),
        }
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        match self {
            SurfaceKind::TriangulatedSurface(x) => x.apply_scale(scale),
        }
    }
}

impl ComputeEnvelope for SurfaceKind {
    fn compute_envelope(&self) -> Option<Envelope> {
        match self {
            SurfaceKind::TriangulatedSurface(x) => x.compute_envelope(),
        }
    }
}

impl Triangulate for SurfaceKind {
    fn triangulate(&self) -> Result<Triangulation, Error> {
        match self {
            SurfaceKind::TriangulatedSurface(x) => Ok(Triangulation::new(x.clone(), Vec::new())),
        }
    }
}
