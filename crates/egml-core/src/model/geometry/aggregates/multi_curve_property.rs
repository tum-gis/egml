use crate::model::geometry::aggregates::MultiCurve;

#[derive(Debug, Clone, PartialEq)]
pub struct MultiCurveProperty {
    pub object: Option<MultiCurve>,
    pub href: Option<String>,
}

impl MultiCurveProperty {
    pub fn new(object: MultiCurve) -> Self {
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
