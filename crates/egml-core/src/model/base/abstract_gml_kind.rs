use crate::model::feature::AbstractFeatureKind;
use crate::model::geometry::AbstractGeometryKind;

#[derive(Debug, Clone, PartialEq)]
pub enum AbstractGmlKind {
    AbstractGeometryKind(AbstractGeometryKind),
    AbstractFeatureKind(AbstractFeatureKind),
}

#[macro_export]
macro_rules! impl_from_for_abstract_gml_kind {
    ($variant:ident, $type:ty) => {
        impl From<$type> for $crate::model::base::AbstractGmlKind {
            #[allow(unreachable_code)]
            fn from(x: $type) -> Self {
                $crate::model::base::AbstractGmlKind::$variant(x.into())
            }
        }
        $crate::impl_from_for_abstract_object_kind!(AbstractGmlKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_from_for_abstract_gml_kind!($variant, $variant);
    };
}

#[macro_export]
macro_rules! impl_try_from_for_abstract_gml_kind {
    ($variant:ident, $type:ty) => {
        impl TryFrom<$crate::model::base::AbstractGmlKind> for $type {
            type Error = ();
            #[allow(unreachable_code)]
            fn try_from(x: $crate::model::base::AbstractGmlKind) -> Result<Self, ()> {
                match x {
                    $crate::model::base::AbstractGmlKind::$variant(k) => {
                        k.try_into().map_err(|_| ())
                    }
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
        $crate::impl_try_from_for_abstract_object_kind!(AbstractGmlKind, $type);
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_abstract_gml_kind!($variant, $variant);
    };
}

impl_from_for_abstract_gml_kind!(AbstractGeometryKind);
impl_from_for_abstract_gml_kind!(AbstractFeatureKind);
impl_try_from_for_abstract_gml_kind!(AbstractGeometryKind);
impl_try_from_for_abstract_gml_kind!(AbstractFeatureKind);
