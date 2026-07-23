use crate::model::common::{GeometryType, HasGeometryType};
use crate::model::geometry::aggregates::{
    AbstractGeometricAggregate, AbstractGeometricAggregateKind, AsAbstractGeometricAggregate,
    MultiCurve, MultiGeometry, MultiPoint, MultiSurface,
};

/// Borrowed view over [`AbstractGeometricAggregateKind`].
#[derive(Debug, Clone, Copy)]
pub enum AbstractGeometricAggregateKindRef<'a> {
    MultiCurve(&'a MultiCurve),
    MultiGeometry(&'a MultiGeometry),
    MultiPoint(&'a MultiPoint),
    MultiSurface(&'a MultiSurface),
}

impl<'a> From<&'a AbstractGeometricAggregateKind> for AbstractGeometricAggregateKindRef<'a> {
    fn from(x: &'a AbstractGeometricAggregateKind) -> Self {
        match x {
            AbstractGeometricAggregateKind::MultiCurve(inner) => Self::MultiCurve(inner),
            AbstractGeometricAggregateKind::MultiGeometry(inner) => Self::MultiGeometry(inner),
            AbstractGeometricAggregateKind::MultiPoint(inner) => Self::MultiPoint(inner),
            AbstractGeometricAggregateKind::MultiSurface(inner) => Self::MultiSurface(inner),
        }
    }
}

impl<'a> AsAbstractGeometricAggregate for AbstractGeometricAggregateKindRef<'a> {
    fn abstract_geometric_aggregate(&self) -> &AbstractGeometricAggregate {
        match self {
            Self::MultiCurve(x) => x.abstract_geometric_aggregate(),
            Self::MultiGeometry(x) => x.abstract_geometric_aggregate(),
            Self::MultiPoint(x) => x.abstract_geometric_aggregate(),
            Self::MultiSurface(x) => x.abstract_geometric_aggregate(),
        }
    }
}
crate::impl_abstract_geometric_aggregate_traits!(AbstractGeometricAggregateKindRef<'_>);

impl<'a> HasGeometryType for AbstractGeometricAggregateKindRef<'a> {
    fn geometry_type(&self) -> GeometryType {
        match self {
            Self::MultiCurve(x) => x.geometry_type(),
            Self::MultiGeometry(x) => x.geometry_type(),
            Self::MultiPoint(x) => x.geometry_type(),
            Self::MultiSurface(x) => x.geometry_type(),
        }
    }
}

/// Implements `From<&$type>` for [`AbstractGeometricAggregateKindRef`] and
/// forwards to the parent [`AbstractGeometryKindRef`](crate::model::geometry::refs::AbstractGeometryKindRef)
/// level so the conversion is available all the way up the chain.
#[macro_export]
macro_rules! impl_from_for_abstract_geometric_aggregate_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> From<&'a $type>
            for $crate::model::geometry::aggregates::refs::AbstractGeometricAggregateKindRef<'a>
        {
            fn from(x: &'a $type) -> Self {
                $crate::model::geometry::aggregates::refs::AbstractGeometricAggregateKindRef::$variant(
                    x.into(),
                )
            }
        }
        $crate::impl_from_for_abstract_geometry_kind_ref!(AbstractGeometricAggregateKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_abstract_geometric_aggregate_kind_ref!($variant, $variant);
    };
}
impl_from_for_abstract_geometric_aggregate_kind_ref!(MultiCurve);
impl_from_for_abstract_geometric_aggregate_kind_ref!(MultiGeometry);
impl_from_for_abstract_geometric_aggregate_kind_ref!(MultiPoint);
impl_from_for_abstract_geometric_aggregate_kind_ref!(MultiSurface);

/// Implements `TryFrom<AbstractGeometricAggregateKindRef>` for `&$type` and
/// forwards the downcast up to the parent level.
#[macro_export]
macro_rules! impl_try_from_for_abstract_geometric_aggregate_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a>
            TryFrom<
                $crate::model::geometry::aggregates::refs::AbstractGeometricAggregateKindRef<'a>,
            > for &'a $type
        {
            type Error = ();
            fn try_from(
                x: $crate::model::geometry::aggregates::refs::AbstractGeometricAggregateKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::geometry::aggregates::refs::AbstractGeometricAggregateKindRef::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_abstract_geometry_kind_ref!(AbstractGeometricAggregateKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_abstract_geometric_aggregate_kind_ref!($variant, $variant);
    };
}
impl_try_from_for_abstract_geometric_aggregate_kind_ref!(MultiCurve);
impl_try_from_for_abstract_geometric_aggregate_kind_ref!(MultiGeometry);
impl_try_from_for_abstract_geometric_aggregate_kind_ref!(MultiPoint);
impl_try_from_for_abstract_geometric_aggregate_kind_ref!(MultiSurface);

impl<'a> AbstractGeometricAggregateKindRef<'a> {
    /// Clones the referenced aggregate (and all its members) into an owned
    /// [`AbstractGeometricAggregateKind`].
    pub fn to_owned(&self) -> AbstractGeometricAggregateKind {
        match *self {
            Self::MultiCurve(inner) => AbstractGeometricAggregateKind::MultiCurve(inner.clone()),
            Self::MultiGeometry(inner) => {
                AbstractGeometricAggregateKind::MultiGeometry(inner.clone())
            }
            Self::MultiPoint(inner) => AbstractGeometricAggregateKind::MultiPoint(inner.clone()),
            Self::MultiSurface(inner) => {
                AbstractGeometricAggregateKind::MultiSurface(inner.clone())
            }
        }
    }
}
