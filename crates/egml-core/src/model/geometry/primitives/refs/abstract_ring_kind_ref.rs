use crate::model::common::{GeometryType, HasGeometryType};
use crate::model::geometry::primitives::{
    AbstractRing, AbstractRingKind, AsAbstractRing, LinearRing,
};

/// Borrowed view over [`AbstractRingKind`], mirroring its variants without
/// requiring an owned copy of the referenced geometry.
///
/// Unlike the owned enum, the recursive `AbstractRingKind` variant does not
/// need a `Box`: a reference is always pointer-sized regardless of what it
/// points to.
#[derive(Debug, Clone, Copy)]
pub enum AbstractRingKindRef<'a> {
    LinearRing(&'a LinearRing),
    AbstractRingKind(&'a AbstractRingKind),
}

impl<'a> From<&'a AbstractRingKind> for AbstractRingKindRef<'a> {
    fn from(x: &'a AbstractRingKind) -> Self {
        match x {
            AbstractRingKind::LinearRing(inner) => Self::LinearRing(inner),
            AbstractRingKind::AbstractRingKind(inner) => Self::AbstractRingKind(inner),
        }
    }
}

impl<'a> AsAbstractRing for AbstractRingKindRef<'a> {
    fn abstract_ring(&self) -> &AbstractRing {
        match self {
            Self::LinearRing(x) => x.abstract_ring(),
            Self::AbstractRingKind(x) => x.abstract_ring(),
        }
    }
}
crate::impl_abstract_ring_traits!(AbstractRingKindRef<'_>);

impl<'a> HasGeometryType for AbstractRingKindRef<'a> {
    fn geometry_type(&self) -> GeometryType {
        match self {
            Self::LinearRing(x) => x.geometry_type(),
            Self::AbstractRingKind(x) => x.geometry_type(),
        }
    }
}

/// Implements `From<&$type>` for [`AbstractRingKindRef`] and forwards to the
/// parent [`AbstractCurveKindRef`](crate::model::geometry::primitives::refs::AbstractCurveKindRef).
#[macro_export]
macro_rules! impl_from_for_abstract_ring_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> From<&'a $type>
            for $crate::model::geometry::primitives::refs::AbstractRingKindRef<'a>
        {
            fn from(x: &'a $type) -> Self {
                $crate::model::geometry::primitives::refs::AbstractRingKindRef::$variant(x.into())
            }
        }
        $crate::impl_from_for_abstract_curve_kind_ref!(AbstractRingKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_abstract_ring_kind_ref!($variant, $variant);
    };
}
impl_from_for_abstract_ring_kind_ref!(LinearRing);

/// Implements `TryFrom<AbstractRingKindRef>` for `&$type` and forwards the
/// downcast up to the parent level.
#[macro_export]
macro_rules! impl_try_from_for_abstract_ring_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> TryFrom<$crate::model::geometry::primitives::refs::AbstractRingKindRef<'a>>
            for &'a $type
        {
            type Error = ();
            fn try_from(
                x: $crate::model::geometry::primitives::refs::AbstractRingKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::geometry::primitives::refs::AbstractRingKindRef::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_abstract_curve_kind_ref!(AbstractRingKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_abstract_ring_kind_ref!($variant, $variant);
    };
}
impl_try_from_for_abstract_ring_kind_ref!(LinearRing);

impl<'a> AbstractRingKindRef<'a> {
    /// Clones the referenced ring (and, for a nested ring, its entire boxed
    /// subtree) into an owned [`AbstractRingKind`].
    pub fn to_owned(&self) -> AbstractRingKind {
        match *self {
            Self::LinearRing(inner) => AbstractRingKind::LinearRing(inner.clone()),
            Self::AbstractRingKind(inner) => {
                AbstractRingKind::AbstractRingKind(Box::new(inner.clone()))
            }
        }
    }
}
