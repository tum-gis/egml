use crate::model::geometry::{AbstractGeometry, AsAbstractGeometry, AsAbstractGeometryMut};

/// Base data shared by all GML geometry primitives ([OGC 07-036 §10.2.1](https://docs.ogc.org/is/07-036/07-036.pdf)).
///
/// A geometry primitive is a connected, homogeneous geometric object.
/// All concrete primitive types (Point, LineString, Surface, Solid) embed
/// `AbstractGeometricPrimitive`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct AbstractGeometricPrimitive {
    pub abstract_geometry: AbstractGeometry,
}

impl AbstractGeometricPrimitive {
    /// Creates a new `AbstractGeometricPrimitive` wrapping the provided geometry base data.
    pub fn new() -> Self {
        Self {
            abstract_geometry: AbstractGeometry::default(),
        }
    }

    pub fn from_abstract_geometry(abstract_geometry: AbstractGeometry) -> Self {
        Self { abstract_geometry }
    }
}

/// Object-safe read accessor for [`AbstractGeometricPrimitive`] fields.
pub trait AsAbstractGeometricPrimitive: AsAbstractGeometry {
    /// Returns a reference to the embedded [`AbstractGeometricPrimitive`] base data.
    fn abstract_geometric_primitive(&self) -> &AbstractGeometricPrimitive;
}

/// Mutable companion to [`AsAbstractGeometricPrimitive`].
pub trait AsAbstractGeometricPrimitiveMut:
    AsAbstractGeometricPrimitive + AsAbstractGeometryMut
{
    /// Returns a mutable reference to the embedded [`AbstractGeometricPrimitive`] base data.
    fn abstract_geometric_primitive_mut(&mut self) -> &mut AbstractGeometricPrimitive;
}

impl AsAbstractGeometricPrimitive for AbstractGeometricPrimitive {
    fn abstract_geometric_primitive(&self) -> &AbstractGeometricPrimitive {
        self
    }
}

impl AsAbstractGeometricPrimitiveMut for AbstractGeometricPrimitive {
    fn abstract_geometric_primitive_mut(&mut self) -> &mut AbstractGeometricPrimitive {
        self
    }
}

#[macro_export]
macro_rules! impl_abstract_geometric_primitive_traits {
    ($type:ty) => {
        $crate::impl_abstract_geometry_traits!($type);

        impl $crate::model::geometry::AsAbstractGeometry for $type {
            fn abstract_geometry(&self) -> &$crate::model::geometry::AbstractGeometry {
                &<$type as $crate::model::geometry::primitives::AsAbstractGeometricPrimitive>::abstract_geometric_primitive(self)
                    .abstract_geometry
            }
        }
    };
}

#[macro_export]
macro_rules! impl_abstract_geometric_primitive_mut_traits {
    ($type:ty) => {
        $crate::impl_abstract_geometry_mut_traits!($type);

        impl $crate::model::geometry::AsAbstractGeometryMut for $type {
            fn abstract_geometry_mut(&mut self) -> &mut $crate::model::geometry::AbstractGeometry {
                &mut <$type as $crate::model::geometry::primitives::AsAbstractGeometricPrimitiveMut>::abstract_geometric_primitive_mut(self)
                    .abstract_geometry
            }
        }
    };
}

impl_abstract_geometric_primitive_traits!(AbstractGeometricPrimitive);
impl_abstract_geometric_primitive_mut_traits!(AbstractGeometricPrimitive);
