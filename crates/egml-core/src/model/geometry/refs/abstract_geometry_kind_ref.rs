use crate::model::common::{GeometryType, HasGeometryType};
use crate::model::geometry::aggregates::AbstractGeometricAggregateKind;
use crate::model::geometry::aggregates::refs::AbstractGeometricAggregateKindRef;
use crate::model::geometry::primitives::AbstractGeometricPrimitiveKind;
use crate::model::geometry::primitives::refs::AbstractGeometricPrimitiveKindRef;
use crate::model::geometry::{AbstractGeometry, AbstractGeometryKind, AsAbstractGeometry};

/// Borrowed view over [`AbstractGeometryKind`], mirroring its shape one level
/// at a time down to the concrete leaf geometry types (`Point`, `Polygon`,
/// `MultiSurface`, ...).
///
/// Exists so code that needs to treat geometries nested at different depths
/// of the property graph uniformly (a `gml:id` resolver, a recursive
/// visitor) can do so without cloning — property structs like
/// `MultiSurfaceProperty` or `ShellProperty` store their concrete leaf type
/// directly rather than the `AbstractGeometryKind` union, so a plain
/// `&'a AbstractGeometryKind` can't represent every node in the tree.
///
/// `From<&Leaf>` and `TryFrom` conversions are generated across the whole
/// inheritance chain by the [`impl_from_for_abstract_geometry_kind_ref!`],
/// [`impl_try_from_for_abstract_geometry_kind_ref!`] and
/// [`impl_try_from_abstract_geometry_kind_ref_for_enum!`] macro families,
/// invoked once per concrete type at its deepest level.
#[derive(Debug, Clone, Copy)]
pub enum AbstractGeometryKindRef<'a> {
    AbstractGeometricAggregateKind(AbstractGeometricAggregateKindRef<'a>),
    AbstractGeometricPrimitiveKind(AbstractGeometricPrimitiveKindRef<'a>),
}

impl<'a> From<&'a AbstractGeometryKind> for AbstractGeometryKindRef<'a> {
    fn from(x: &'a AbstractGeometryKind) -> Self {
        match x {
            AbstractGeometryKind::AbstractGeometricAggregateKind(inner) => {
                Self::AbstractGeometricAggregateKind(inner.into())
            }
            AbstractGeometryKind::AbstractGeometricPrimitiveKind(inner) => {
                Self::AbstractGeometricPrimitiveKind(inner.into())
            }
        }
    }
}

impl<'a> AsAbstractGeometry for AbstractGeometryKindRef<'a> {
    fn abstract_geometry(&self) -> &AbstractGeometry {
        match self {
            Self::AbstractGeometricAggregateKind(x) => x.abstract_geometry(),
            Self::AbstractGeometricPrimitiveKind(x) => x.abstract_geometry(),
        }
    }
}
crate::impl_abstract_geometry_traits!(AbstractGeometryKindRef<'_>);

impl<'a> HasGeometryType for AbstractGeometryKindRef<'a> {
    fn geometry_type(&self) -> GeometryType {
        match self {
            Self::AbstractGeometricAggregateKind(x) => x.geometry_type(),
            Self::AbstractGeometricPrimitiveKind(x) => x.geometry_type(),
        }
    }
}

/// Implements `From<&$type>` for [`AbstractGeometryKindRef`]. This is the root
/// of the geometry ref hierarchy, so — unlike the deeper levels — it does not
/// forward to a parent macro.
#[macro_export]
macro_rules! impl_from_for_abstract_geometry_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> From<&'a $type> for $crate::model::geometry::refs::AbstractGeometryKindRef<'a> {
            fn from(x: &'a $type) -> Self {
                $crate::model::geometry::refs::AbstractGeometryKindRef::$variant(x.into())
            }
        }
    };
    ($variant:ident) => {
        $crate::impl_from_for_abstract_geometry_kind_ref!($variant, $variant);
    };
}
impl_from_for_abstract_geometry_kind_ref!(AbstractGeometricAggregateKind);
impl_from_for_abstract_geometry_kind_ref!(AbstractGeometricPrimitiveKind);

/// Implements `TryFrom<AbstractGeometryKindRef>` for `&$type`, downcasting to a
/// concrete leaf geometry.
#[macro_export]
macro_rules! impl_try_from_for_abstract_geometry_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> TryFrom<$crate::model::geometry::refs::AbstractGeometryKindRef<'a>> for &'a $type {
            type Error = ();
            fn try_from(
                x: $crate::model::geometry::refs::AbstractGeometryKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::geometry::refs::AbstractGeometryKindRef::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_abstract_geometry_kind_ref!($variant, $variant);
    };
}

/// Implements `TryFrom<AbstractGeometryKindRef>` for an intermediate `$EnumRef`,
/// downcasting to a nested ref enum (e.g. `AbstractSurfaceKindRef`).
#[macro_export]
macro_rules! impl_try_from_abstract_geometry_kind_ref_for_enum {
    ($variant:ident, $EnumRef:ident) => {
        impl<'a> TryFrom<$crate::model::geometry::refs::AbstractGeometryKindRef<'a>>
            for $EnumRef<'a>
        {
            type Error = ();
            fn try_from(
                x: $crate::model::geometry::refs::AbstractGeometryKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::geometry::refs::AbstractGeometryKindRef::$variant(k) => {
                        $EnumRef::try_from(k).map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
    };
}
impl_try_from_abstract_geometry_kind_ref_for_enum!(
    AbstractGeometricAggregateKind,
    AbstractGeometricAggregateKindRef
);
impl_try_from_abstract_geometry_kind_ref_for_enum!(
    AbstractGeometricPrimitiveKind,
    AbstractGeometricPrimitiveKindRef
);

impl<'a> AbstractGeometryKindRef<'a> {
    /// Clones the referenced geometry — recursively, all the way down to its
    /// leaves — into an owned [`AbstractGeometryKind`].
    ///
    /// This is an O(n) deep clone of the whole subtree, unlike cloning the
    /// `Ref` itself (which is `Copy` and just duplicates a pointer). Reach
    /// for this only when the result genuinely needs to outlive the borrowed
    /// tree `self` points into.
    pub fn to_owned(&self) -> AbstractGeometryKind {
        match *self {
            Self::AbstractGeometricAggregateKind(inner) => {
                AbstractGeometryKind::AbstractGeometricAggregateKind(inner.to_owned())
            }
            Self::AbstractGeometricPrimitiveKind(inner) => {
                AbstractGeometryKind::AbstractGeometricPrimitiveKind(inner.to_owned())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::geometry::DirectPosition;
    use crate::model::geometry::primitives::{AbstractRingKind, LinearRing, Polygon};

    #[test]
    fn owned_round_trip_clones_through_the_full_chain() {
        let ring = LinearRing::new([
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(0.0, 1.0, 0.0).unwrap(),
        ])
        .unwrap();
        // Nest through the recursive `AbstractRingKind::AbstractRingKind(Box<..>)`
        // variant, the trickiest case in the owning conversion.
        let nested_ring =
            AbstractRingKind::AbstractRingKind(Box::new(AbstractRingKind::LinearRing(ring)));
        let polygon: AbstractGeometryKind = Polygon::new(
            Some(
                crate::model::geometry::primitives::AbstractRingProperty::from_object(nested_ring),
            ),
            [],
        )
        .unwrap()
        .into();

        let geometry_ref: AbstractGeometryKindRef<'_> = (&polygon).into();
        let owned: AbstractGeometryKind = geometry_ref.to_owned();

        assert_eq!(owned, polygon);
    }
}
