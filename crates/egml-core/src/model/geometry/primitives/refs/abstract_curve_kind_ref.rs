use crate::model::common::{GeometryType, HasGeometryType};
use crate::model::geometry::primitives::refs::AbstractRingKindRef;
use crate::model::geometry::primitives::{
    AbstractCurve, AbstractCurveKind, AbstractRingKind, AsAbstractCurve, LineString,
};

/// Borrowed view over [`AbstractCurveKind`].
#[derive(Debug, Clone, Copy)]
pub enum AbstractCurveKindRef<'a> {
    LineString(&'a LineString),
    AbstractRingKind(AbstractRingKindRef<'a>),
}

impl<'a> From<&'a AbstractCurveKind> for AbstractCurveKindRef<'a> {
    fn from(x: &'a AbstractCurveKind) -> Self {
        match x {
            AbstractCurveKind::LineString(inner) => Self::LineString(inner),
            AbstractCurveKind::AbstractRingKind(inner) => Self::AbstractRingKind(inner.into()),
        }
    }
}

impl<'a> AsAbstractCurve for AbstractCurveKindRef<'a> {
    fn abstract_curve(&self) -> &AbstractCurve {
        match self {
            Self::LineString(x) => x.abstract_curve(),
            Self::AbstractRingKind(x) => x.abstract_curve(),
        }
    }
}
crate::impl_abstract_curve_traits!(AbstractCurveKindRef<'_>);

impl<'a> HasGeometryType for AbstractCurveKindRef<'a> {
    fn geometry_type(&self) -> GeometryType {
        match self {
            Self::LineString(x) => x.geometry_type(),
            Self::AbstractRingKind(x) => x.geometry_type(),
        }
    }
}

/// Implements `From<&$type>` for [`AbstractCurveKindRef`] and forwards to the
/// parent [`AbstractGeometricPrimitiveKindRef`](crate::model::geometry::primitives::refs::AbstractGeometricPrimitiveKindRef).
#[macro_export]
macro_rules! impl_from_for_abstract_curve_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> From<&'a $type>
            for $crate::model::geometry::primitives::refs::AbstractCurveKindRef<'a>
        {
            fn from(x: &'a $type) -> Self {
                $crate::model::geometry::primitives::refs::AbstractCurveKindRef::$variant(x.into())
            }
        }
        $crate::impl_from_for_abstract_geometric_primitive_kind_ref!(AbstractCurveKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_abstract_curve_kind_ref!($variant, $variant);
    };
}
impl_from_for_abstract_curve_kind_ref!(LineString);
impl_from_for_abstract_curve_kind_ref!(AbstractRingKind);

/// Implements `TryFrom<AbstractCurveKindRef>` for `&$type` and forwards the
/// downcast up to the parent level.
#[macro_export]
macro_rules! impl_try_from_for_abstract_curve_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> TryFrom<$crate::model::geometry::primitives::refs::AbstractCurveKindRef<'a>>
            for &'a $type
        {
            type Error = ();
            fn try_from(
                x: $crate::model::geometry::primitives::refs::AbstractCurveKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::geometry::primitives::refs::AbstractCurveKindRef::$variant(
                        k,
                    ) => k.try_into().map_err(|_| ()),
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_abstract_geometric_primitive_kind_ref!(AbstractCurveKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_abstract_curve_kind_ref!($variant, $variant);
    };
}
impl_try_from_for_abstract_curve_kind_ref!(LineString);

/// Implements `TryFrom<AbstractCurveKindRef>` for an intermediate `$EnumRef` and
/// forwards the downcast up to the parent level.
#[macro_export]
macro_rules! impl_try_from_abstract_curve_kind_ref_for_enum {
    ($variant:ident, $EnumRef:ident) => {
        impl<'a> TryFrom<$crate::model::geometry::primitives::refs::AbstractCurveKindRef<'a>>
            for $EnumRef<'a>
        {
            type Error = ();
            fn try_from(
                x: $crate::model::geometry::primitives::refs::AbstractCurveKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::geometry::primitives::refs::AbstractCurveKindRef::$variant(
                        k,
                    ) => $EnumRef::try_from(k).map_err(|_| ()),
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_abstract_geometric_primitive_kind_ref_for_enum!(
            AbstractCurveKind,
            $EnumRef
        );
    };
}
impl_try_from_abstract_curve_kind_ref_for_enum!(AbstractRingKind, AbstractRingKindRef);

impl<'a> AbstractCurveKindRef<'a> {
    /// Clones the referenced curve (recursively, for a nested ring) into an
    /// owned [`AbstractCurveKind`].
    pub fn to_owned(&self) -> AbstractCurveKind {
        match *self {
            Self::LineString(inner) => AbstractCurveKind::LineString(inner.clone()),
            Self::AbstractRingKind(inner) => AbstractCurveKind::AbstractRingKind(inner.to_owned()),
        }
    }
}
