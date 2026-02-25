use crate::model::base::{AbstractGml, AsAbstractGml, AsAbstractGmlMut};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct AbstractGeometry {
    pub(crate) abstract_gml: AbstractGml,
}

impl AbstractGeometry {
    pub fn new(abstract_gml: AbstractGml) -> Self {
        Self { abstract_gml }
    }
}

pub trait AsAbstractGeometry: AsAbstractGml {
    fn abstract_geometry(&self) -> &AbstractGeometry;
}

pub trait AsAbstractGeometryMut: AsAbstractGeometry + AsAbstractGmlMut {
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
