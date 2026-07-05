use crate::model::geometry::primitives::Point;

#[derive(Debug, Clone, PartialEq)]
pub struct PointProperty {
    pub object: Option<Point>,
    pub href: Option<String>,
}

impl PointProperty {
    pub fn new(object: Point) -> Self {
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
