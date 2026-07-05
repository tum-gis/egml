use crate::model::base::{AbstractGml, AsAbstractGml, AsAbstractGmlMut};

/// Base data shared by all GML geometry types ([OGC 07-036 §10.1.3.1](https://docs.ogc.org/is/07-036/07-036.pdf)).
///
/// Embeds [`AbstractGml`] and is in turn embedded by every concrete and
/// abstract geometry.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct AbstractGeometry {
    pub(crate) abstract_gml: AbstractGml,
    srs_dimension: Option<u32>,
}

impl AbstractGeometry {
    /// Creates a new `AbstractGeometry` wrapping the provided GML base data.
    pub fn new(abstract_gml: AbstractGml) -> Self {
        Self {
            abstract_gml,
            srs_dimension: None,
        }
    }
}

/// Object-safe read accessor for [`AbstractGeometry`] fields.
pub trait AsAbstractGeometry: AsAbstractGml {
    /// Returns a reference to the embedded [`AbstractGeometry`] base data.
    fn abstract_geometry(&self) -> &AbstractGeometry;

    fn srs_dimension(&self) -> Option<u32> {
        self.abstract_geometry().srs_dimension
    }
}

/// Mutable companion to [`AsAbstractGeometry`].
pub trait AsAbstractGeometryMut: AsAbstractGeometry + AsAbstractGmlMut {
    /// Returns a mutable reference to the embedded [`AbstractGeometry`] base data.
    fn abstract_geometry_mut(&mut self) -> &mut AbstractGeometry;

    fn set_srs_dimension(&mut self, srs_dimension: Option<u32>) {
        self.abstract_geometry_mut().srs_dimension = srs_dimension;
    }
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
