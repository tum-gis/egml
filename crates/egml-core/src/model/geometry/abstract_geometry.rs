use crate::model::base::{AbstractGml, AsAbstractGml, AsAbstractGmlMut};

/// Base data shared by all GML geometry types ([OGC 07-036 §10.1.3.1](https://docs.ogc.org/is/07-036/07-036.pdf)).
///
/// Embeds [`AbstractGml`] and is in turn embedded by every concrete and
/// abstract geometry.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct AbstractGeometry {
    pub abstract_gml: AbstractGml,
    srs_name: Option<String>,
    srs_dimension: Option<u32>,
}

impl AbstractGeometry {
    /// Creates a new `AbstractGeometry` wrapping the provided GML base data.
    pub fn new() -> Self {
        Self {
            abstract_gml: AbstractGml::default(),
            srs_name: None,
            srs_dimension: None,
        }
    }

    pub fn from_abstract_gml(abstract_gml: AbstractGml) -> Self {
        Self {
            abstract_gml,
            srs_name: None,
            srs_dimension: None,
        }
    }
}

/// Object-safe read accessor for [`AbstractGeometry`] fields.
pub trait AsAbstractGeometry: AsAbstractGml {
    /// Returns a reference to the embedded [`AbstractGeometry`] base data.
    fn abstract_geometry(&self) -> &AbstractGeometry;

    fn srs_name(&self) -> Option<&String> {
        self.abstract_geometry().srs_name.as_ref()
    }

    fn srs_dimension(&self) -> Option<u32> {
        self.abstract_geometry().srs_dimension
    }
}

/// Mutable companion to [`AsAbstractGeometry`].
pub trait AsAbstractGeometryMut: AsAbstractGeometry + AsAbstractGmlMut {
    /// Returns a mutable reference to the embedded [`AbstractGeometry`] base data.
    fn abstract_geometry_mut(&mut self) -> &mut AbstractGeometry;

    fn set_srs_name(&mut self, srs_name: impl Into<String>) {
        self.abstract_geometry_mut().srs_name = Some(srs_name.into());
    }

    fn set_srs_name_opt(&mut self, srs_name: Option<String>) {
        self.abstract_geometry_mut().srs_name = srs_name;
    }

    fn clear_srs_name(&mut self) {
        self.abstract_geometry_mut().srs_name = None;
    }

    fn set_srs_dimension(&mut self, srs_dimension: u32) {
        self.abstract_geometry_mut().srs_dimension = Some(srs_dimension);
    }

    fn set_srs_dimension_opt(&mut self, srs_dimension: Option<u32>) {
        self.abstract_geometry_mut().srs_dimension = srs_dimension;
    }

    fn clear_srs_dimension(&mut self) {
        self.abstract_geometry_mut().srs_dimension = None;
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

#[macro_export]
macro_rules! impl_abstract_geometry_traits {
    ($type:ty) => {
        $crate::impl_abstract_gml_traits!($type);

        impl $crate::model::base::AsAbstractGml for $type {
            fn abstract_gml(&self) -> &$crate::model::base::AbstractGml {
                &<$type as $crate::model::geometry::AsAbstractGeometry>::abstract_geometry(self)
                    .abstract_gml
            }
        }
    };
}

#[macro_export]
macro_rules! impl_abstract_geometry_mut_traits {
    ($type:ty) => {
        $crate::impl_abstract_gml_mut_traits!($type);

        impl $crate::model::base::AsAbstractGmlMut for $type {
            fn abstract_gml_mut(&mut self) -> &mut $crate::model::base::AbstractGml {
                &mut <$type as $crate::model::geometry::AsAbstractGeometryMut>::abstract_geometry_mut(self)
                    .abstract_gml
            }
        }
    };
}

impl_abstract_geometry_traits!(AbstractGeometry);
impl_abstract_geometry_mut_traits!(AbstractGeometry);
