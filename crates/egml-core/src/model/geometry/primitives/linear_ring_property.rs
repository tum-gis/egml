use crate::model::geometry::primitives::LinearRing;

#[derive(Debug, Clone, PartialEq)]
pub struct LinearRingProperty {
    pub object: Option<LinearRing>,
    pub href: Option<String>,
}

impl LinearRingProperty {
    pub fn new(object: LinearRing) -> Self {
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
