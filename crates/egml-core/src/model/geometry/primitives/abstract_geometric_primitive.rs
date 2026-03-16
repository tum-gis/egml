use crate::model::geometry::{AbstractGeometry, AsAbstractGeometry, AsAbstractGeometryMut};

/// Base data shared by all GML geometry primitives (ISO 19136 §10.1.3.1).
///
/// A geometry primitive is a connected, homogeneous geometric object.
/// All concrete primitive types (Point, LineString, Surface, Solid) embed
/// `AbstractGeometricPrimitive`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct AbstractGeometricPrimitive {
    pub(crate) abstract_geometry: AbstractGeometry,
}

impl AbstractGeometricPrimitive {
    /// Creates a new `AbstractGeometricPrimitive` wrapping the provided geometry base data.
    pub fn new(abstract_geometry: AbstractGeometry) -> Self {
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

#[doc(hidden)]
#[macro_export]
macro_rules! impl_abstract_geometric_primitive_traits {
    ($type:ty) => {
        $crate::impl_abstract_geometry_traits!($type);

        impl $crate::model::geometry::AsAbstractGeometry for $type {
            fn abstract_geometry(&self) -> &$crate::model::geometry::AbstractGeometry {
                use $crate::model::geometry::primitives::AsAbstractGeometricPrimitive;
                &self.abstract_geometric_primitive().abstract_geometry
            }
        }

        impl $crate::model::geometry::AsAbstractGeometryMut for $type {
            fn abstract_geometry_mut(&mut self) -> &mut $crate::model::geometry::AbstractGeometry {
                use $crate::model::geometry::primitives::AsAbstractGeometricPrimitiveMut;
                &mut self.abstract_geometric_primitive_mut().abstract_geometry
            }
        }
    };
}

impl_abstract_geometric_primitive_traits!(AbstractGeometricPrimitive);
