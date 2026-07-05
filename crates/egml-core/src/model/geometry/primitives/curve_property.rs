use crate::model::geometry::primitives::CurveKind;

#[derive(Debug, Clone, PartialEq)]
pub struct CurveProperty {
    pub object: Option<CurveKind>,
    pub href: Option<String>,
}

impl CurveProperty {
    pub fn new(object: CurveKind) -> Self {
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
