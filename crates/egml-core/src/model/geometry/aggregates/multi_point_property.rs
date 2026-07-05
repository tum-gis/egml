use crate::model::geometry::aggregates::multi_point::MultiPoint;

#[derive(Debug, Clone, PartialEq)]
pub struct MultiPointProperty {
    pub object: Option<MultiPoint>,
    pub href: Option<String>,
}

impl MultiPointProperty {
    pub fn new(object: MultiPoint) -> Self {
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
