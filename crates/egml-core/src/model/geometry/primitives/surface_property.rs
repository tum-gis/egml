use crate::model::geometry::primitives::surface_kind::SurfaceKind;

/// An owned wrapper around a concrete [`SurfaceKind`].
///
/// Used as a property element in GML to hold an inline surface definition.
#[derive(Debug, Clone, PartialEq)]
pub struct SurfaceProperty {
    pub object: Option<SurfaceKind>,
    pub href: Option<String>,
}

impl SurfaceProperty {
    pub fn new(object: SurfaceKind) -> Self {
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
