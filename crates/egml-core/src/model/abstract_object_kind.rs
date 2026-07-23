use crate::model::base::AbstractGmlKind;
use crate::model::geometry::Envelope;
use crate::model::geometry::primitives::AbstractSurfacePatchKind;

#[derive(Debug, Clone, PartialEq)]
pub enum AbstractObjectKind {
    AbstractGmlKind(AbstractGmlKind),
    AbstractSurfacePatchKind(AbstractSurfacePatchKind),
    Envelope(Envelope),
}

#[macro_export]
macro_rules! impl_from_for_abstract_object_kind {
    ($variant:ident, $type:ty) => {
        impl From<$type> for $crate::model::AbstractObjectKind {
            #[allow(unreachable_code)]
            fn from(x: $type) -> Self {
                $crate::model::AbstractObjectKind::$variant(x.into())
            }
        }
    };
    ($variant:ident) => {
        $crate::impl_from_for_abstract_object_kind!($variant, $variant);
    };
}

#[macro_export]
macro_rules! impl_try_from_for_abstract_object_kind {
    ($variant:ident, $type:ty) => {
        impl TryFrom<$crate::model::AbstractObjectKind> for $type {
            type Error = ();
            #[allow(unreachable_code)]
            fn try_from(x: $crate::model::AbstractObjectKind) -> Result<Self, ()> {
                match x {
                    $crate::model::AbstractObjectKind::$variant(k) => k.try_into().map_err(|_| ()),
                    #[allow(unreachable_patterns)]
                    _ => Err(()),
                }
            }
        }
    };
    ($variant:ident) => {
        $crate::impl_try_from_for_abstract_object_kind!($variant, $variant);
    };
}

impl_from_for_abstract_object_kind!(AbstractGmlKind);
impl_from_for_abstract_object_kind!(AbstractSurfacePatchKind);

impl From<Envelope> for AbstractObjectKind {
    fn from(x: Envelope) -> Self {
        AbstractObjectKind::Envelope(x)
    }
}

impl_try_from_for_abstract_object_kind!(AbstractGmlKind);
impl_try_from_for_abstract_object_kind!(AbstractSurfacePatchKind);

impl TryFrom<AbstractObjectKind> for Envelope {
    type Error = ();
    fn try_from(x: AbstractObjectKind) -> Result<Self, ()> {
        match x {
            AbstractObjectKind::Envelope(e) => Ok(e),
            _ => Err(()),
        }
    }
}
