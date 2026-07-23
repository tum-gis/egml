use crate::model::common::{GeometryType, HasGeometryType};
use crate::model::geometry::primitives::refs::{
    AbstractCurveKindRef, AbstractSolidKindRef, AbstractSurfaceKindRef,
};
use crate::model::geometry::primitives::{
    AbstractCurveKind, AbstractGeometricPrimitive, AbstractGeometricPrimitiveKind,
    AbstractSolidKind, AbstractSurfaceKind, AsAbstractGeometricPrimitive, Point,
};

/// Borrowed view over [`AbstractGeometricPrimitiveKind`].
#[derive(Debug, Clone, Copy)]
pub enum AbstractGeometricPrimitiveKindRef<'a> {
    AbstractCurveKind(AbstractCurveKindRef<'a>),
    AbstractSolidKind(AbstractSolidKindRef<'a>),
    AbstractSurfaceKind(AbstractSurfaceKindRef<'a>),
    Point(&'a Point),
}

impl<'a> From<&'a AbstractGeometricPrimitiveKind> for AbstractGeometricPrimitiveKindRef<'a> {
    fn from(x: &'a AbstractGeometricPrimitiveKind) -> Self {
        match x {
            AbstractGeometricPrimitiveKind::AbstractCurveKind(inner) => {
                Self::AbstractCurveKind(inner.into())
            }
            AbstractGeometricPrimitiveKind::AbstractSolidKind(inner) => {
                Self::AbstractSolidKind(inner.into())
            }
            AbstractGeometricPrimitiveKind::AbstractSurfaceKind(inner) => {
                Self::AbstractSurfaceKind(inner.into())
            }
            AbstractGeometricPrimitiveKind::Point(inner) => Self::Point(inner),
        }
    }
}

impl<'a> AsAbstractGeometricPrimitive for AbstractGeometricPrimitiveKindRef<'a> {
    fn abstract_geometric_primitive(&self) -> &AbstractGeometricPrimitive {
        match self {
            Self::AbstractCurveKind(x) => x.abstract_geometric_primitive(),
            Self::AbstractSolidKind(x) => x.abstract_geometric_primitive(),
            Self::AbstractSurfaceKind(x) => x.abstract_geometric_primitive(),
            Self::Point(x) => x.abstract_geometric_primitive(),
        }
    }
}
crate::impl_abstract_geometric_primitive_traits!(AbstractGeometricPrimitiveKindRef<'_>);

impl<'a> HasGeometryType for AbstractGeometricPrimitiveKindRef<'a> {
    fn geometry_type(&self) -> GeometryType {
        match self {
            Self::AbstractCurveKind(x) => x.geometry_type(),
            Self::AbstractSolidKind(x) => x.geometry_type(),
            Self::AbstractSurfaceKind(x) => x.geometry_type(),
            Self::Point(x) => x.geometry_type(),
        }
    }
}

/// Implements `From<&$type>` for [`AbstractGeometricPrimitiveKindRef`] and
/// forwards to the parent [`AbstractGeometryKindRef`](crate::model::geometry::refs::AbstractGeometryKindRef).
#[macro_export]
macro_rules! impl_from_for_abstract_geometric_primitive_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> From<&'a $type>
            for $crate::model::geometry::primitives::refs::AbstractGeometricPrimitiveKindRef<'a>
        {
            fn from(x: &'a $type) -> Self {
                $crate::model::geometry::primitives::refs::AbstractGeometricPrimitiveKindRef::$variant(
                    x.into(),
                )
            }
        }
        $crate::impl_from_for_abstract_geometry_kind_ref!(AbstractGeometricPrimitiveKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_abstract_geometric_primitive_kind_ref!($variant, $variant);
    };
}
impl_from_for_abstract_geometric_primitive_kind_ref!(Point);
impl_from_for_abstract_geometric_primitive_kind_ref!(AbstractCurveKind);
impl_from_for_abstract_geometric_primitive_kind_ref!(AbstractSurfaceKind);
impl_from_for_abstract_geometric_primitive_kind_ref!(AbstractSolidKind);

/// Implements `TryFrom<AbstractGeometricPrimitiveKindRef>` for `&$type` and
/// forwards the downcast up to the parent level.
#[macro_export]
macro_rules! impl_try_from_for_abstract_geometric_primitive_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a>
            TryFrom<
                $crate::model::geometry::primitives::refs::AbstractGeometricPrimitiveKindRef<'a>,
            > for &'a $type
        {
            type Error = ();
            fn try_from(
                x: $crate::model::geometry::primitives::refs::AbstractGeometricPrimitiveKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::geometry::primitives::refs::AbstractGeometricPrimitiveKindRef::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_abstract_geometry_kind_ref!(AbstractGeometricPrimitiveKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_abstract_geometric_primitive_kind_ref!($variant, $variant);
    };
}
impl_try_from_for_abstract_geometric_primitive_kind_ref!(Point);

/// Implements `TryFrom<AbstractGeometricPrimitiveKindRef>` for an intermediate
/// `$EnumRef` and forwards the downcast up to the parent level.
#[macro_export]
macro_rules! impl_try_from_abstract_geometric_primitive_kind_ref_for_enum {
    ($variant:ident, $EnumRef:ident) => {
        impl<'a>
            TryFrom<
                $crate::model::geometry::primitives::refs::AbstractGeometricPrimitiveKindRef<'a>,
            > for $EnumRef<'a>
        {
            type Error = ();
            fn try_from(
                x: $crate::model::geometry::primitives::refs::AbstractGeometricPrimitiveKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::geometry::primitives::refs::AbstractGeometricPrimitiveKindRef::$variant(k) => {
                        $EnumRef::try_from(k).map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_abstract_geometry_kind_ref_for_enum!(
            AbstractGeometricPrimitiveKind,
            $EnumRef
        );
    };
}
impl_try_from_abstract_geometric_primitive_kind_ref_for_enum!(
    AbstractCurveKind,
    AbstractCurveKindRef
);
impl_try_from_abstract_geometric_primitive_kind_ref_for_enum!(
    AbstractSurfaceKind,
    AbstractSurfaceKindRef
);
impl_try_from_abstract_geometric_primitive_kind_ref_for_enum!(
    AbstractSolidKind,
    AbstractSolidKindRef
);

impl<'a> AbstractGeometricPrimitiveKindRef<'a> {
    /// Clones the referenced primitive (recursively, for a nested curve,
    /// solid, or surface) into an owned [`AbstractGeometricPrimitiveKind`].
    pub fn to_owned(&self) -> AbstractGeometricPrimitiveKind {
        match *self {
            Self::AbstractCurveKind(inner) => {
                AbstractGeometricPrimitiveKind::AbstractCurveKind(inner.to_owned())
            }
            Self::AbstractSolidKind(inner) => {
                AbstractGeometricPrimitiveKind::AbstractSolidKind(inner.to_owned())
            }
            Self::AbstractSurfaceKind(inner) => {
                AbstractGeometricPrimitiveKind::AbstractSurfaceKind(inner.to_owned())
            }
            Self::Point(inner) => AbstractGeometricPrimitiveKind::Point(inner.clone()),
        }
    }
}
