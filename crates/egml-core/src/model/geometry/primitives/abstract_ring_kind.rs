use crate::model::common::{
    ApplyTransform, ComputeEnvelope, GeometryType, HasGeometryType, IterGeometries,
};
use crate::model::geometry::primitives::{
    AbstractRing, AsAbstractRing, AsAbstractRingMut, LinearRing,
};
use crate::model::geometry::refs::AbstractGeometryKindRef;
use crate::model::geometry::{DirectPosition, Envelope};
use crate::{impl_abstract_ring_mut_traits, impl_abstract_ring_traits};
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub enum AbstractRingKind {
    LinearRing(LinearRing),
    AbstractRingKind(Box<AbstractRingKind>),
}

impl AsAbstractRing for AbstractRingKind {
    fn abstract_ring(&self) -> &AbstractRing {
        match self {
            AbstractRingKind::LinearRing(x) => x.abstract_ring(),
            AbstractRingKind::AbstractRingKind(x) => x.abstract_ring(),
        }
    }
}

impl AsAbstractRingMut for AbstractRingKind {
    fn abstract_ring_mut(&mut self) -> &mut AbstractRing {
        match self {
            AbstractRingKind::LinearRing(x) => x.abstract_ring_mut(),
            AbstractRingKind::AbstractRingKind(x) => x.abstract_ring_mut(),
        }
    }
}

impl_abstract_ring_traits!(AbstractRingKind);
impl_abstract_ring_mut_traits!(AbstractRingKind);

impl HasGeometryType for AbstractRingKind {
    fn geometry_type(&self) -> GeometryType {
        match self {
            AbstractRingKind::LinearRing(x) => x.geometry_type(),
            AbstractRingKind::AbstractRingKind(x) => x.geometry_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_abstract_ring_kind {
    ($variant:ident, $type:ty) => {
        impl From<$type> for $crate::model::geometry::primitives::AbstractRingKind {
            #[allow(unreachable_code)]
            fn from(x: $type) -> Self {
                $crate::model::geometry::primitives::AbstractRingKind::$variant(x.into())
            }
        }
        $crate::impl_from_for_abstract_curve_kind!(AbstractRingKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_abstract_ring_kind!($variant, $variant);
    };
}

#[macro_export]
macro_rules! impl_try_from_for_abstract_ring_kind {
    ($variant:ident, $type:ty) => {
        impl TryFrom<$crate::model::geometry::primitives::AbstractRingKind> for $type {
            type Error = ();
            #[allow(unreachable_code)]
            fn try_from(
                x: $crate::model::geometry::primitives::AbstractRingKind,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::geometry::primitives::AbstractRingKind::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_abstract_curve_kind!(AbstractRingKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_abstract_ring_kind!($variant, $variant);
    };
}

impl_from_for_abstract_ring_kind!(LinearRing);
impl_try_from_for_abstract_ring_kind!(LinearRing);

impl AbstractRingKind {
    pub fn points(&self) -> &[DirectPosition] {
        match self {
            AbstractRingKind::LinearRing(x) => x.points(),
            AbstractRingKind::AbstractRingKind(x) => x.points(),
        }
    }

    pub fn length_3d(&self) -> f64 {
        match self {
            AbstractRingKind::LinearRing(x) => x.length_3d(),
            AbstractRingKind::AbstractRingKind(x) => x.length_3d(),
        }
    }

    pub fn area_3d(&self) -> f64 {
        match self {
            AbstractRingKind::LinearRing(x) => x.area_3d(),
            AbstractRingKind::AbstractRingKind(x) => x.area_3d(),
        }
    }
}

impl IterGeometries for AbstractRingKind {
    fn iter_geometries(&self) -> Box<dyn Iterator<Item = AbstractGeometryKindRef<'_>> + '_> {
        match self {
            AbstractRingKind::LinearRing(x) => x.iter_geometries(),
            AbstractRingKind::AbstractRingKind(x) => x.iter_geometries(),
        }
    }
}

impl ApplyTransform for AbstractRingKind {
    fn apply_transform(&mut self, transform: Transform3<f64>) {
        match self {
            AbstractRingKind::LinearRing(x) => x.apply_transform(transform),
            AbstractRingKind::AbstractRingKind(x) => x.apply_transform(transform),
        }
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        match self {
            AbstractRingKind::LinearRing(x) => x.apply_isometry(isometry),
            AbstractRingKind::AbstractRingKind(x) => x.apply_isometry(isometry),
        }
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        match self {
            AbstractRingKind::LinearRing(x) => x.apply_translation(vector),
            AbstractRingKind::AbstractRingKind(x) => x.apply_translation(vector),
        }
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        match self {
            AbstractRingKind::LinearRing(x) => x.apply_rotation(rotation),
            AbstractRingKind::AbstractRingKind(x) => x.apply_rotation(rotation),
        }
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        match self {
            AbstractRingKind::LinearRing(x) => x.apply_scale(scale),
            AbstractRingKind::AbstractRingKind(x) => x.apply_scale(scale),
        }
    }
}

impl ComputeEnvelope for AbstractRingKind {
    fn compute_envelope(&self) -> Option<Envelope> {
        match self {
            AbstractRingKind::LinearRing(x) => x.compute_envelope(),
            AbstractRingKind::AbstractRingKind(x) => x.compute_envelope(),
        }
    }
}
