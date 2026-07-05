use crate::model::geometry::primitives::Solid;

#[derive(Debug, Clone, PartialEq)]
pub struct SolidProperty {
    pub object: Option<Solid>,
    pub href: Option<String>,
}

impl SolidProperty {
    pub fn new(object: Solid) -> Self {
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
