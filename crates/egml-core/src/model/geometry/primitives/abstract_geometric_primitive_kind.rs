use crate::model::common::{
    ApplyTransform, ComputeEnvelope, GeometryType, HasGeometryType, IterGeometries, Triangulate,
    Triangulation,
};
use crate::model::geometry::Envelope;
use crate::model::geometry::primitives::{
    AbstractCurveKind, AbstractGeometricPrimitive, AbstractSolidKind, AbstractSurfaceKind,
    AsAbstractGeometricPrimitive, AsAbstractGeometricPrimitiveMut, Point,
};
use crate::model::geometry::refs::AbstractGeometryKindRef;
use crate::{
    Error, impl_abstract_geometric_primitive_mut_traits, impl_abstract_geometric_primitive_traits,
};
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

#[derive(Debug, Clone, PartialEq)]
pub enum AbstractGeometricPrimitiveKind {
    AbstractCurveKind(AbstractCurveKind),
    AbstractSolidKind(AbstractSolidKind),
    AbstractSurfaceKind(AbstractSurfaceKind),
    Point(Point),
}

impl AsAbstractGeometricPrimitive for AbstractGeometricPrimitiveKind {
    fn abstract_geometric_primitive(&self) -> &AbstractGeometricPrimitive {
        match self {
            AbstractGeometricPrimitiveKind::Point(x) => x.abstract_geometric_primitive(),
            AbstractGeometricPrimitiveKind::AbstractCurveKind(x) => {
                x.abstract_geometric_primitive()
            }
            AbstractGeometricPrimitiveKind::AbstractSurfaceKind(x) => {
                x.abstract_geometric_primitive()
            }
            AbstractGeometricPrimitiveKind::AbstractSolidKind(x) => {
                x.abstract_geometric_primitive()
            }
        }
    }
}

impl AsAbstractGeometricPrimitiveMut for AbstractGeometricPrimitiveKind {
    fn abstract_geometric_primitive_mut(&mut self) -> &mut AbstractGeometricPrimitive {
        match self {
            AbstractGeometricPrimitiveKind::Point(x) => x.abstract_geometric_primitive_mut(),
            AbstractGeometricPrimitiveKind::AbstractCurveKind(x) => {
                x.abstract_geometric_primitive_mut()
            }
            AbstractGeometricPrimitiveKind::AbstractSurfaceKind(x) => {
                x.abstract_geometric_primitive_mut()
            }
            AbstractGeometricPrimitiveKind::AbstractSolidKind(x) => {
                x.abstract_geometric_primitive_mut()
            }
        }
    }
}

impl_abstract_geometric_primitive_traits!(AbstractGeometricPrimitiveKind);
impl_abstract_geometric_primitive_mut_traits!(AbstractGeometricPrimitiveKind);

impl HasGeometryType for AbstractGeometricPrimitiveKind {
    fn geometry_type(&self) -> GeometryType {
        match self {
            AbstractGeometricPrimitiveKind::Point(x) => x.geometry_type(),
            AbstractGeometricPrimitiveKind::AbstractCurveKind(x) => x.geometry_type(),
            AbstractGeometricPrimitiveKind::AbstractSurfaceKind(x) => x.geometry_type(),
            AbstractGeometricPrimitiveKind::AbstractSolidKind(x) => x.geometry_type(),
        }
    }
}

#[macro_export]
macro_rules! impl_from_for_abstract_geometric_primitive_kind {
    ($variant:ident, $type:ty) => {
        impl From<$type> for $crate::model::geometry::primitives::AbstractGeometricPrimitiveKind {
            #[allow(unreachable_code)]
            fn from(x: $type) -> Self {
                $crate::model::geometry::primitives::AbstractGeometricPrimitiveKind::$variant(
                    x.into(),
                )
            }
        }
        $crate::impl_from_for_abstract_geometry_kind!(AbstractGeometricPrimitiveKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_abstract_geometric_primitive_kind!($variant, $variant);
    };
}

#[macro_export]
macro_rules! impl_try_from_for_abstract_geometric_primitive_kind {
    ($variant:ident, $type:ty) => {
        impl TryFrom<$crate::model::geometry::primitives::AbstractGeometricPrimitiveKind>
            for $type
        {
            type Error = ();
            #[allow(unreachable_code)]
            fn try_from(
                x: $crate::model::geometry::primitives::AbstractGeometricPrimitiveKind,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::geometry::primitives::AbstractGeometricPrimitiveKind::$variant(
                        k,
                    ) => k.try_into().map_err(|_| ()),
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_abstract_geometry_kind!(AbstractGeometricPrimitiveKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_abstract_geometric_primitive_kind!($variant, $variant);
    };
}

impl_from_for_abstract_geometric_primitive_kind!(Point);
impl_from_for_abstract_geometric_primitive_kind!(AbstractCurveKind);
impl_from_for_abstract_geometric_primitive_kind!(AbstractSurfaceKind);
impl_from_for_abstract_geometric_primitive_kind!(AbstractSolidKind);
impl_try_from_for_abstract_geometric_primitive_kind!(Point);
impl_try_from_for_abstract_geometric_primitive_kind!(AbstractCurveKind);
impl_try_from_for_abstract_geometric_primitive_kind!(AbstractSurfaceKind);
impl_try_from_for_abstract_geometric_primitive_kind!(AbstractSolidKind);

impl Triangulate for AbstractGeometricPrimitiveKind {
    fn triangulate(&self) -> Result<Triangulation, Error> {
        match self {
            AbstractGeometricPrimitiveKind::AbstractSurfaceKind(x) => x.triangulate(),
            AbstractGeometricPrimitiveKind::Point(_) => {
                Err(Error::TriangulationNotSupported { geometry: "Point" })
            }
            AbstractGeometricPrimitiveKind::AbstractCurveKind(_) => {
                Err(Error::TriangulationNotSupported {
                    geometry: "AbstractCurveKind",
                })
            }
            AbstractGeometricPrimitiveKind::AbstractSolidKind(x) => x.triangulate(),
        }
    }
}

impl IterGeometries for AbstractGeometricPrimitiveKind {
    fn iter_geometries(&self) -> Box<dyn Iterator<Item = AbstractGeometryKindRef<'_>> + '_> {
        match self {
            AbstractGeometricPrimitiveKind::Point(x) => x.iter_geometries(),
            AbstractGeometricPrimitiveKind::AbstractCurveKind(x) => x.iter_geometries(),
            AbstractGeometricPrimitiveKind::AbstractSurfaceKind(x) => x.iter_geometries(),
            AbstractGeometricPrimitiveKind::AbstractSolidKind(x) => x.iter_geometries(),
        }
    }
}

impl ApplyTransform for AbstractGeometricPrimitiveKind {
    fn apply_transform(&mut self, transform: Transform3<f64>) {
        match self {
            AbstractGeometricPrimitiveKind::Point(x) => x.apply_transform(transform),
            AbstractGeometricPrimitiveKind::AbstractCurveKind(x) => x.apply_transform(transform),
            AbstractGeometricPrimitiveKind::AbstractSurfaceKind(x) => x.apply_transform(transform),
            AbstractGeometricPrimitiveKind::AbstractSolidKind(x) => x.apply_transform(transform),
        }
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        match self {
            AbstractGeometricPrimitiveKind::Point(x) => x.apply_isometry(isometry),
            AbstractGeometricPrimitiveKind::AbstractCurveKind(x) => x.apply_isometry(isometry),
            AbstractGeometricPrimitiveKind::AbstractSurfaceKind(x) => x.apply_isometry(isometry),
            AbstractGeometricPrimitiveKind::AbstractSolidKind(x) => x.apply_isometry(isometry),
        }
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        match self {
            AbstractGeometricPrimitiveKind::Point(x) => x.apply_translation(vector),
            AbstractGeometricPrimitiveKind::AbstractCurveKind(x) => x.apply_translation(vector),
            AbstractGeometricPrimitiveKind::AbstractSurfaceKind(x) => x.apply_translation(vector),
            AbstractGeometricPrimitiveKind::AbstractSolidKind(x) => x.apply_translation(vector),
        }
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        match self {
            AbstractGeometricPrimitiveKind::Point(x) => x.apply_rotation(rotation),
            AbstractGeometricPrimitiveKind::AbstractCurveKind(x) => x.apply_rotation(rotation),
            AbstractGeometricPrimitiveKind::AbstractSurfaceKind(x) => x.apply_rotation(rotation),
            AbstractGeometricPrimitiveKind::AbstractSolidKind(x) => x.apply_rotation(rotation),
        }
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        match self {
            AbstractGeometricPrimitiveKind::Point(x) => x.apply_scale(scale),
            AbstractGeometricPrimitiveKind::AbstractCurveKind(x) => x.apply_scale(scale),
            AbstractGeometricPrimitiveKind::AbstractSurfaceKind(x) => x.apply_scale(scale),
            AbstractGeometricPrimitiveKind::AbstractSolidKind(x) => x.apply_scale(scale),
        }
    }
}

impl ComputeEnvelope for AbstractGeometricPrimitiveKind {
    fn compute_envelope(&self) -> Option<Envelope> {
        match self {
            AbstractGeometricPrimitiveKind::Point(x) => x.compute_envelope(),
            AbstractGeometricPrimitiveKind::AbstractCurveKind(x) => x.compute_envelope(),
            AbstractGeometricPrimitiveKind::AbstractSurfaceKind(x) => x.compute_envelope(),
            AbstractGeometricPrimitiveKind::AbstractSolidKind(x) => x.compute_envelope(),
        }
    }
}
