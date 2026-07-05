use crate::model::geometry::aggregates::MultiSurface;

#[derive(Debug, Clone, PartialEq)]
pub struct MultiSurfaceProperty {
    pub object: Option<MultiSurface>,
    pub href: Option<String>,
}

impl MultiSurfaceProperty {
    pub fn new(object: MultiSurface) -> Self {
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
