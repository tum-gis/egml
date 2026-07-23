use crate::model::common::GeometryType;

pub trait HasGeometryType {
    fn geometry_type(&self) -> GeometryType;
}

#[macro_export]
macro_rules! impl_has_geometry_type {
    ($type:ty, $variant:ident) => {
        impl $crate::model::common::HasGeometryType for $type {
            fn geometry_type(&self) -> $crate::model::common::GeometryType {
                $crate::model::common::GeometryType::$variant
            }
        }
    };
}
