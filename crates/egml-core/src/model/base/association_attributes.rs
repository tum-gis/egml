use crate::model::basic_types::NilReason;
use crate::model::xlink::{ActuateType, HRef, ShowType};

/// Attributes from `gml:AssociationAttributeGroup`.
///
/// A property either holds an inline geometry object or references one via
/// [`href`](AssociationAttributes::href).
/// [`nil_reason`](AssociationAttributes::nil_reason) explains why a value is absent
/// when neither is provided.
///
/// Corresponds to `gml:AssociationAttributeGroup` in
/// [OGC 07-036 §7.2.3.1](https://docs.ogc.org/is/07-036/07-036.pdf).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct AssociationAttributes {
    /// `xlink:href` — reference to a remote or local GML object.
    pub href: Option<HRef>,
    /// `gml:nilReason` — explanation for a missing or void value.
    pub nil_reason: Option<NilReason>,
    /// `xlink:title` — human-readable label for the remote resource.
    pub title: Option<String>,
    /// `xlink:role` — URI identifying the semantic role of the remote resource.
    pub role: Option<String>,
    /// `xlink:arcrole` — URI identifying the semantic role of the link arc.
    pub arcrole: Option<String>,
    /// `xlink:show` — desired presentation of the remote resource on traversal.
    pub show: Option<ShowType>,
    /// `xlink:actuate` — timing of link traversal.
    pub actuate: Option<ActuateType>,
}

impl AssociationAttributes {
    /// Creates an `AssociationAttributes` carrying only an `xlink:href` reference.
    pub fn new_href(href: HRef) -> Self {
        Self {
            href: Some(href),
            ..Default::default()
        }
    }

    /// Returns the local GML `gml:id` referenced by `href`.
    ///
    /// Returns `None` if `href` is absent or is a remote reference.
    pub fn local_id(&self) -> Option<&str> {
        self.href.as_ref()?.local_id()
    }
}
