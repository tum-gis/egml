use crate::model::common::{GeometryType, HasGeometryType};
use crate::model::geometry::primitives::{
    AbstractSolid, AbstractSolidKind, AsAbstractSolid, Solid,
};

/// Borrowed view over [`AbstractSolidKind`].
#[derive(Debug, Clone, Copy)]
pub enum AbstractSolidKindRef<'a> {
    Solid(&'a Solid),
}

impl<'a> From<&'a AbstractSolidKind> for AbstractSolidKindRef<'a> {
    fn from(x: &'a AbstractSolidKind) -> Self {
        match x {
            AbstractSolidKind::Solid(inner) => Self::Solid(inner),
        }
    }
}

impl<'a> AsAbstractSolid for AbstractSolidKindRef<'a> {
    fn abstract_solid(&self) -> &AbstractSolid {
        match self {
            Self::Solid(x) => x.abstract_solid(),
        }
    }
}
crate::impl_abstract_solid_traits!(AbstractSolidKindRef<'_>);

impl<'a> HasGeometryType for AbstractSolidKindRef<'a> {
    fn geometry_type(&self) -> GeometryType {
        match self {
            Self::Solid(x) => x.geometry_type(),
        }
    }
}

/// Implements `From<&$type>` for [`AbstractSolidKindRef`] and forwards to the
/// parent [`AbstractGeometricPrimitiveKindRef`](crate::model::geometry::primitives::refs::AbstractGeometricPrimitiveKindRef).
#[macro_export]
macro_rules! impl_from_for_abstract_solid_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> From<&'a $type>
            for $crate::model::geometry::primitives::refs::AbstractSolidKindRef<'a>
        {
            fn from(x: &'a $type) -> Self {
                $crate::model::geometry::primitives::refs::AbstractSolidKindRef::$variant(x.into())
            }
        }
        $crate::impl_from_for_abstract_geometric_primitive_kind_ref!(AbstractSolidKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_abstract_solid_kind_ref!($variant, $variant);
    };
}
impl_from_for_abstract_solid_kind_ref!(Solid);

/// Implements `TryFrom<AbstractSolidKindRef>` for `&$type` and forwards the
/// downcast up to the parent level.
#[macro_export]
macro_rules! impl_try_from_for_abstract_solid_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> TryFrom<$crate::model::geometry::primitives::refs::AbstractSolidKindRef<'a>>
            for &'a $type
        {
            type Error = ();
            fn try_from(
                x: $crate::model::geometry::primitives::refs::AbstractSolidKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::geometry::primitives::refs::AbstractSolidKindRef::$variant(
                        k,
                    ) => k.try_into().map_err(|_| ()),
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_abstract_geometric_primitive_kind_ref!(AbstractSolidKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_abstract_solid_kind_ref!($variant, $variant);
    };
}
impl_try_from_for_abstract_solid_kind_ref!(Solid);

impl<'a> AbstractSolidKindRef<'a> {
    /// Clones the referenced solid into an owned [`AbstractSolidKind`].
    pub fn to_owned(&self) -> AbstractSolidKind {
        match *self {
            Self::Solid(inner) => AbstractSolidKind::Solid(inner.clone()),
        }
    }
}
