use crate::model::common::{
    ApplyTransform, ComputeEnvelope, GeometryType, HasGeometryType, IterGeometries, Triangulate,
    Triangulation,
};
use crate::model::geometry::primitives::{
    AbstractSolid, AsAbstractSolid, AsAbstractSolidMut, Solid,
};
use crate::model::geometry::refs::AbstractGeometryKindRef;
use crate::model::geometry::{DirectPosition, Envelope};
use crate::{Error, impl_abstract_solid_mut_traits, impl_abstract_solid_traits};
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub enum AbstractSolidKind {
    Solid(Solid),
    // CompositeSolid(CompositeSolid),
}

impl AsAbstractSolid for AbstractSolidKind {
    fn abstract_solid(&self) -> &AbstractSolid {
        match self {
            AbstractSolidKind::Solid(x) => x.abstract_solid(),
        }
    }
}

impl AsAbstractSolidMut for AbstractSolidKind {
    fn abstract_solid_mut(&mut self) -> &mut AbstractSolid {
        match self {
            AbstractSolidKind::Solid(x) => x.abstract_solid_mut(),
        }
    }
}

impl_abstract_solid_traits!(AbstractSolidKind);
impl_abstract_solid_mut_traits!(AbstractSolidKind);

impl HasGeometryType for AbstractSolidKind {
    fn geometry_type(&self) -> GeometryType {
        match self {
            AbstractSolidKind::Solid(x) => x.geometry_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_abstract_solid_kind {
    ($variant:ident, $type:ty) => {
        impl From<$type> for $crate::model::geometry::primitives::AbstractSolidKind {
            #[allow(unreachable_code)]
            fn from(x: $type) -> Self {
                $crate::model::geometry::primitives::AbstractSolidKind::$variant(x.into())
            }
        }
        $crate::impl_from_for_abstract_geometric_primitive_kind!(AbstractSolidKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_abstract_solid_kind!($variant, $variant);
    };
}

#[macro_export]
macro_rules! impl_try_from_for_abstract_solid_kind {
    ($variant:ident, $type:ty) => {
        impl TryFrom<$crate::model::geometry::primitives::AbstractSolidKind> for $type {
            type Error = ();
            #[allow(unreachable_code)]
            fn try_from(
                x: $crate::model::geometry::primitives::AbstractSolidKind,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::geometry::primitives::AbstractSolidKind::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_abstract_geometric_primitive_kind!(AbstractSolidKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_abstract_solid_kind!($variant, $variant);
    };
}

impl_from_for_abstract_solid_kind!(Solid);
impl_try_from_for_abstract_solid_kind!(Solid);

impl AbstractSolidKind {
    pub fn volume_3d(&self) -> Result<f64, Error> {
        match self {
            Self::Solid(x) => x.volume_3d(),
        }
    }

    pub fn points(&self) -> Vec<&DirectPosition> {
        match self {
            Self::Solid(x) => x.points(),
        }
    }
}

impl IterGeometries for AbstractSolidKind {
    fn iter_geometries(&self) -> Box<dyn Iterator<Item = AbstractGeometryKindRef<'_>> + '_> {
        match self {
            Self::Solid(x) => x.iter_geometries(),
        }
    }
}

impl ApplyTransform for AbstractSolidKind {
    fn apply_transform(&mut self, transform: Transform3<f64>) {
        match self {
            Self::Solid(x) => x.apply_transform(transform),
        }
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        match self {
            Self::Solid(x) => x.apply_isometry(isometry),
        }
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        match self {
            Self::Solid(x) => x.apply_translation(vector),
        }
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        match self {
            Self::Solid(x) => x.apply_rotation(rotation),
        }
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        match self {
            Self::Solid(x) => x.apply_scale(scale),
        }
    }
}

impl ComputeEnvelope for AbstractSolidKind {
    fn compute_envelope(&self) -> Option<Envelope> {
        match self {
            Self::Solid(x) => x.compute_envelope(),
        }
    }
}

impl Triangulate for AbstractSolidKind {
    fn triangulate(&self) -> Result<Triangulation, Error> {
        match self {
            Self::Solid(x) => x.triangulate(),
        }
    }
}
