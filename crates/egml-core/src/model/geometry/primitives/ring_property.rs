use crate::model::geometry::primitives::ring_kind::RingKind;

#[derive(Debug, Clone, PartialEq)]
pub struct RingProperty {
    pub object: Option<RingKind>,
    pub href: Option<String>,
}

impl RingProperty {
    pub fn new(object: RingKind) -> Self {
        Self {
            object: Some(object),
            href: None,
        }
    }

    pub fn new_href(href: impl Into<String>) -> Self {
        Self {
            object: None,
            href: Some(href.into()),
        }
    }
}
