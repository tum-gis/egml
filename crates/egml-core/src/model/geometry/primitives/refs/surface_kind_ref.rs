use crate::model::common::{GeometryType, HasGeometryType};
use crate::model::geometry::primitives::surface_kind::SurfaceKind;
use crate::model::geometry::primitives::{AbstractSurface, AsAbstractSurface, TriangulatedSurface};

/// Borrowed view over [`SurfaceKind`].
#[derive(Debug, Clone, Copy)]
pub enum SurfaceKindRef<'a> {
    TriangulatedSurface(&'a TriangulatedSurface),
}

impl<'a> From<&'a SurfaceKind> for SurfaceKindRef<'a> {
    fn from(x: &'a SurfaceKind) -> Self {
        match x {
            SurfaceKind::TriangulatedSurface(inner) => Self::TriangulatedSurface(inner),
        }
    }
}

impl<'a> AsAbstractSurface for SurfaceKindRef<'a> {
    fn abstract_surface(&self) -> &AbstractSurface {
        match self {
            Self::TriangulatedSurface(x) => x.abstract_surface(),
        }
    }
}
crate::impl_abstract_surface_traits!(SurfaceKindRef<'_>);

impl<'a> HasGeometryType for SurfaceKindRef<'a> {
    fn geometry_type(&self) -> GeometryType {
        match self {
            Self::TriangulatedSurface(x) => x.geometry_type(),
        }
    }
}

/// Implements `From<&$type>` for [`SurfaceKindRef`] and forwards to the parent
/// [`AbstractSurfaceKindRef`](crate::model::geometry::primitives::refs::AbstractSurfaceKindRef).
#[macro_export]
macro_rules! impl_from_for_surface_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> From<&'a $type> for $crate::model::geometry::primitives::refs::SurfaceKindRef<'a> {
            fn from(x: &'a $type) -> Self {
                $crate::model::geometry::primitives::refs::SurfaceKindRef::$variant(x.into())
            }
        }
        $crate::impl_from_for_abstract_surface_kind_ref!(SurfaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_surface_kind_ref!($variant, $variant);
    };
}
impl_from_for_surface_kind_ref!(TriangulatedSurface);

/// Implements `TryFrom<SurfaceKindRef>` for `&$type` and forwards the downcast
/// up to the parent level.
#[macro_export]
macro_rules! impl_try_from_for_surface_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> TryFrom<$crate::model::geometry::primitives::refs::SurfaceKindRef<'a>>
            for &'a $type
        {
            type Error = ();
            fn try_from(
                x: $crate::model::geometry::primitives::refs::SurfaceKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::geometry::primitives::refs::SurfaceKindRef::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_abstract_surface_kind_ref!(SurfaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_surface_kind_ref!($variant, $variant);
    };
}
impl_try_from_for_surface_kind_ref!(TriangulatedSurface);

impl<'a> SurfaceKindRef<'a> {
    /// Clones the referenced surface into an owned [`SurfaceKind`].
    pub fn to_owned(&self) -> SurfaceKind {
        match *self {
            Self::TriangulatedSurface(inner) => SurfaceKind::TriangulatedSurface(inner.clone()),
        }
    }
}
