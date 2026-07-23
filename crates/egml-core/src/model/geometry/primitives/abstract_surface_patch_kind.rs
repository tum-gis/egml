use crate::Error;
use crate::model::common::{ApplyTransform, ComputeEnvelope, Triangulate, Triangulation};
use crate::model::geometry::Envelope;
use crate::model::geometry::primitives::{
    AbstractSurfacePatch, AsAbstractSurfacePatch, AsAbstractSurfacePatchMut, PolygonPatch, Triangle,
};
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub enum AbstractSurfacePatchKind {
    PolygonPatch(PolygonPatch),
    Triangle(Triangle),
}

impl AsAbstractSurfacePatch for AbstractSurfacePatchKind {
    fn abstract_surface_patch(&self) -> &AbstractSurfacePatch {
        match self {
            AbstractSurfacePatchKind::PolygonPatch(x) => x.abstract_surface_patch(),
            AbstractSurfacePatchKind::Triangle(x) => x.abstract_surface_patch(),
        }
    }
}

impl AsAbstractSurfacePatchMut for AbstractSurfacePatchKind {
    fn abstract_surface_patch_mut(&mut self) -> &mut AbstractSurfacePatch {
        match self {
            AbstractSurfacePatchKind::PolygonPatch(x) => x.abstract_surface_patch_mut(),
            AbstractSurfacePatchKind::Triangle(x) => x.abstract_surface_patch_mut(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_abstract_surface_patch_kind {
    ($variant:ident, $type:ty) => {
        impl From<$type> for $crate::model::geometry::primitives::AbstractSurfacePatchKind {
            #[allow(unreachable_code)]
            fn from(x: $type) -> Self {
                $crate::model::geometry::primitives::AbstractSurfacePatchKind::$variant(x.into())
            }
        }
        $crate::impl_from_for_abstract_object_kind!(AbstractSurfacePatchKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_abstract_surface_patch_kind!($variant, $variant);
    };
}

#[macro_export]
macro_rules! impl_try_from_for_abstract_surface_patch_kind {
    ($variant:ident, $type:ty) => {
        impl TryFrom<$crate::model::geometry::primitives::AbstractSurfacePatchKind> for $type {
            type Error = ();
            #[allow(unreachable_code)]
            fn try_from(
                x: $crate::model::geometry::primitives::AbstractSurfacePatchKind,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::geometry::primitives::AbstractSurfacePatchKind::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_abstract_object_kind!(AbstractSurfacePatchKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_abstract_surface_patch_kind!($variant, $variant);
    };
}

impl_from_for_abstract_surface_patch_kind!(PolygonPatch);
impl_from_for_abstract_surface_patch_kind!(Triangle);
impl_try_from_for_abstract_surface_patch_kind!(PolygonPatch);
impl_try_from_for_abstract_surface_patch_kind!(Triangle);

impl AbstractSurfacePatchKind {
    pub fn area_3d(&self) -> Result<f64, Error> {
        match self {
            AbstractSurfacePatchKind::PolygonPatch(x) => x.area_3d(),
            AbstractSurfacePatchKind::Triangle(x) => Ok(x.area()),
        }
    }
}

impl ApplyTransform for AbstractSurfacePatchKind {
    fn apply_transform(&mut self, transform: Transform3<f64>) {
        match self {
            AbstractSurfacePatchKind::PolygonPatch(x) => x.apply_transform(transform),
            AbstractSurfacePatchKind::Triangle(x) => x.apply_transform(transform),
        }
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        match self {
            AbstractSurfacePatchKind::PolygonPatch(x) => x.apply_isometry(isometry),
            AbstractSurfacePatchKind::Triangle(x) => x.apply_isometry(isometry),
        }
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        match self {
            AbstractSurfacePatchKind::PolygonPatch(x) => x.apply_translation(vector),
            AbstractSurfacePatchKind::Triangle(x) => x.apply_translation(vector),
        }
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        match self {
            AbstractSurfacePatchKind::PolygonPatch(x) => x.apply_rotation(rotation),
            AbstractSurfacePatchKind::Triangle(x) => x.apply_rotation(rotation),
        }
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        match self {
            AbstractSurfacePatchKind::PolygonPatch(x) => x.apply_scale(scale),
            AbstractSurfacePatchKind::Triangle(x) => x.apply_scale(scale),
        }
    }
}

impl ComputeEnvelope for AbstractSurfacePatchKind {
    fn compute_envelope(&self) -> Option<Envelope> {
        match self {
            AbstractSurfacePatchKind::PolygonPatch(x) => x.compute_envelope(),
            AbstractSurfacePatchKind::Triangle(x) => x.compute_envelope(),
        }
    }
}

impl Triangulate for AbstractSurfacePatchKind {
    fn triangulate(&self) -> Result<Triangulation, Error> {
        match self {
            AbstractSurfacePatchKind::PolygonPatch(x) => x.triangulate(),
            AbstractSurfacePatchKind::Triangle(x) => x.triangulate(),
        }
    }
}
