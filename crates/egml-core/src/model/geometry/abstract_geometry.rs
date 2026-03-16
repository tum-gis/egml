use crate::model::base::{AbstractGml, AsAbstractGml, AsAbstractGmlMut};

/// Base data shared by all GML geometry types (ISO 19136 §10.1.3.1).
///
/// Embeds [`AbstractGml`] and is in turn embedded by every concrete and
/// abstract geometry.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct AbstractGeometry {
    pub(crate) abstract_gml: AbstractGml,
}

impl AbstractGeometry {
    /// Creates a new `AbstractGeometry` wrapping the provided GML base data.
    pub fn new(abstract_gml: AbstractGml) -> Self {
        Self { abstract_gml }
    }
}

/// Object-safe read accessor for [`AbstractGeometry`] fields.
pub trait AsAbstractGeometry: AsAbstractGml {
    /// Returns a reference to the embedded [`AbstractGeometry`] base data.
    fn abstract_geometry(&self) -> &AbstractGeometry;
}

/// Mutable companion to [`AsAbstractGeometry`].
pub trait AsAbstractGeometryMut: AsAbstractGeometry + AsAbstractGmlMut {
    /// Returns a mutable reference to the embedded [`AbstractGeometry`] base data.
    fn abstract_geometry_mut(&mut self) -> &mut AbstractGeometry;
}

impl AsAbstractGeometry for AbstractGeometry {
    fn abstract_geometry(&self) -> &AbstractGeometry {
        self
    }
}

impl AsAbstractGeometryMut for AbstractGeometry {
    fn abstract_geometry_mut(&mut self) -> &mut AbstractGeometry {
        self
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_abstract_geometry_traits {
    ($type:ty) => {
        impl $crate::model::base::AsAbstractGml for $type {
            fn abstract_gml(&self) -> &$crate::model::base::AbstractGml {
                use $crate::model::geometry::AsAbstractGeometry;
                &self.abstract_geometry().abstract_gml
            }
        }

        impl $crate::model::base::AsAbstractGmlMut for $type {
            fn abstract_gml_mut(&mut self) -> &mut $crate::model::base::AbstractGml {
                use $crate::model::geometry::AsAbstractGeometryMut;
                &mut self.abstract_geometry_mut().abstract_gml
            }
        }
    };
}

impl_abstract_geometry_traits!(AbstractGeometry);
