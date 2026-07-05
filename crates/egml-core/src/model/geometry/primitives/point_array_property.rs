use crate::model::geometry::primitives::Point;

#[derive(Debug, Clone, PartialEq)]
pub struct PointArrayProperty {
    pub objects: Vec<Point>,
    pub href: Option<String>,
}

impl PointArrayProperty {
    pub fn new(objects: impl IntoIterator<Item = Point>) -> Self {
        Self {
            objects: objects.into_iter().collect(),
            href: None,
        }
    }

    pub fn new_href(href: impl Into<String>) -> Self {
        Self {
            objects: vec![],
            href: Some(href.into()),
        }
    }
}
