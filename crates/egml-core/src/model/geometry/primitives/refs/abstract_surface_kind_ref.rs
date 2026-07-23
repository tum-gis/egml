use crate::model::common::{GeometryType, HasGeometryType};
use crate::model::geometry::complexes::CompositeSurface;
use crate::model::geometry::primitives::refs::SurfaceKindRef;
use crate::model::geometry::primitives::surface_kind::SurfaceKind;
use crate::model::geometry::primitives::{
    AbstractSurface, AbstractSurfaceKind, AsAbstractSurface, Polygon, Shell, Surface,
};

/// Borrowed view over [`AbstractSurfaceKind`].
#[derive(Debug, Clone, Copy)]
pub enum AbstractSurfaceKindRef<'a> {
    CompositeSurface(&'a CompositeSurface),
    Polygon(&'a Polygon),
    Shell(&'a Shell),
    Surface(&'a Surface),
    SurfaceKind(SurfaceKindRef<'a>),
}

impl<'a> From<&'a AbstractSurfaceKind> for AbstractSurfaceKindRef<'a> {
    fn from(x: &'a AbstractSurfaceKind) -> Self {
        match x {
            AbstractSurfaceKind::CompositeSurface(inner) => Self::CompositeSurface(inner),
            AbstractSurfaceKind::Polygon(inner) => Self::Polygon(inner),
            AbstractSurfaceKind::Shell(inner) => Self::Shell(inner),
            AbstractSurfaceKind::Surface(inner) => Self::Surface(inner),
            AbstractSurfaceKind::SurfaceKind(inner) => Self::SurfaceKind(inner.into()),
        }
    }
}

impl<'a> AsAbstractSurface for AbstractSurfaceKindRef<'a> {
    fn abstract_surface(&self) -> &AbstractSurface {
        match self {
            Self::CompositeSurface(x) => x.abstract_surface(),
            Self::Polygon(x) => x.abstract_surface(),
            Self::Shell(x) => x.abstract_surface(),
            Self::Surface(x) => x.abstract_surface(),
            Self::SurfaceKind(x) => x.abstract_surface(),
        }
    }
}
crate::impl_abstract_surface_traits!(AbstractSurfaceKindRef<'_>);

impl<'a> HasGeometryType for AbstractSurfaceKindRef<'a> {
    fn geometry_type(&self) -> GeometryType {
        match self {
            Self::CompositeSurface(x) => x.geometry_type(),
            Self::Polygon(x) => x.geometry_type(),
            Self::Shell(x) => x.geometry_type(),
            Self::Surface(x) => x.geometry_type(),
            Self::SurfaceKind(x) => x.geometry_type(),
        }
    }
}

/// Implements `From<&$type>` for [`AbstractSurfaceKindRef`] and forwards to the
/// parent [`AbstractGeometricPrimitiveKindRef`](crate::model::geometry::primitives::refs::AbstractGeometricPrimitiveKindRef).
#[macro_export]
macro_rules! impl_from_for_abstract_surface_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> From<&'a $type>
            for $crate::model::geometry::primitives::refs::AbstractSurfaceKindRef<'a>
        {
            fn from(x: &'a $type) -> Self {
                $crate::model::geometry::primitives::refs::AbstractSurfaceKindRef::$variant(
                    x.into(),
                )
            }
        }
        $crate::impl_from_for_abstract_geometric_primitive_kind_ref!(AbstractSurfaceKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_abstract_surface_kind_ref!($variant, $variant);
    };
}
impl_from_for_abstract_surface_kind_ref!(CompositeSurface);
impl_from_for_abstract_surface_kind_ref!(Polygon);
impl_from_for_abstract_surface_kind_ref!(Shell);
impl_from_for_abstract_surface_kind_ref!(Surface);
impl_from_for_abstract_surface_kind_ref!(SurfaceKind);

/// Implements `TryFrom<AbstractSurfaceKindRef>` for `&$type` and forwards the
/// downcast up to the parent level.
#[macro_export]
macro_rules! impl_try_from_for_abstract_surface_kind_ref {
    ($variant:ident, $type:ty) => {
        impl<'a> TryFrom<$crate::model::geometry::primitives::refs::AbstractSurfaceKindRef<'a>>
            for &'a $type
        {
            type Error = ();
            fn try_from(
                x: $crate::model::geometry::primitives::refs::AbstractSurfaceKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::geometry::primitives::refs::AbstractSurfaceKindRef::$variant(
                        k,
                    ) => k.try_into().map_err(|_| ()),
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_abstract_geometric_primitive_kind_ref!(
            AbstractSurfaceKind,
            $type
        );
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_abstract_surface_kind_ref!($variant, $variant);
    };
}
impl_try_from_for_abstract_surface_kind_ref!(CompositeSurface);
impl_try_from_for_abstract_surface_kind_ref!(Polygon);
impl_try_from_for_abstract_surface_kind_ref!(Shell);
impl_try_from_for_abstract_surface_kind_ref!(Surface);

/// Implements `TryFrom<AbstractSurfaceKindRef>` for an intermediate `$EnumRef`
/// and forwards the downcast up to the parent level.
#[macro_export]
macro_rules! impl_try_from_abstract_surface_kind_ref_for_enum {
    ($variant:ident, $EnumRef:ident) => {
        impl<'a> TryFrom<$crate::model::geometry::primitives::refs::AbstractSurfaceKindRef<'a>>
            for $EnumRef<'a>
        {
            type Error = ();
            fn try_from(
                x: $crate::model::geometry::primitives::refs::AbstractSurfaceKindRef<'a>,
            ) -> Result<Self, ()> {
                match x {
                    $crate::model::geometry::primitives::refs::AbstractSurfaceKindRef::$variant(
                        k,
                    ) => $EnumRef::try_from(k).map_err(|_| ()),
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_abstract_geometric_primitive_kind_ref_for_enum!(
            AbstractSurfaceKind,
            $EnumRef
        );
    };
}
impl_try_from_abstract_surface_kind_ref_for_enum!(SurfaceKind, SurfaceKindRef);

impl<'a> AbstractSurfaceKindRef<'a> {
    /// Clones the referenced surface (recursively, for a nested [`SurfaceKind`])
    /// into an owned [`AbstractSurfaceKind`].
    pub fn to_owned(&self) -> AbstractSurfaceKind {
        match *self {
            Self::CompositeSurface(inner) => AbstractSurfaceKind::CompositeSurface(inner.clone()),
            Self::Polygon(inner) => AbstractSurfaceKind::Polygon(inner.clone()),
            Self::Shell(inner) => AbstractSurfaceKind::Shell(inner.clone()),
            Self::Surface(inner) => AbstractSurfaceKind::Surface(inner.clone()),
            Self::SurfaceKind(inner) => AbstractSurfaceKind::SurfaceKind(inner.to_owned()),
        }
    }
}
