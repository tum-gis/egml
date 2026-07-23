use crate::model::base::{
    AssociationAttributes, HasAssociationAttributes, HasAssociationAttributesMut,
    HasOwnershipAttributes, HasOwnershipAttributesMut, OwnershipAttributes,
};
use crate::model::xlink::HRef;

/// A by-reference-only property, corresponding to `gml:ReferenceType`.
///
/// Unlike property wrappers such as `AbstractSurfaceProperty`, a `Reference` never carries
/// inline content — the type's content model is empty (`<sequence/>`); it only ever points at
/// another object via [`xlink:href`](AssociationAttributes::href).
///
/// Corresponds to `gml:ReferenceType` in [OGC 07-036 §7.2.3.7](https://docs.ogc.org/is/07-036/07-036.pdf).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Reference {
    pub association: AssociationAttributes,
    pub ownership: OwnershipAttributes,
}

impl Reference {
    /// Creates a `Reference` pointing at `href`.
    pub fn new(href: HRef) -> Self {
        Self {
            association: AssociationAttributes::new_href(href),
            ownership: OwnershipAttributes::default(),
        }
    }
}

impl HasAssociationAttributes for Reference {
    fn association(&self) -> &AssociationAttributes {
        &self.association
    }
}

impl HasAssociationAttributesMut for Reference {
    fn association_mut(&mut self) -> &mut AssociationAttributes {
        &mut self.association
    }
}

impl HasOwnershipAttributes for Reference {
    fn ownership(&self) -> &OwnershipAttributes {
        &self.ownership
    }
}

impl HasOwnershipAttributesMut for Reference {
    fn ownership_mut(&mut self) -> &mut OwnershipAttributes {
        &mut self.ownership
    }
}

#[cfg(test)]
mod tests {
    use super::Reference;
    use crate::model::base::{
        HasAssociationAttributes, HasAssociationAttributesMut, HasOwnershipAttributes,
    };
    use crate::model::xlink::HRef;

    #[test]
    fn new_sets_href() {
        let reference = Reference::new(HRef::from_local("some-id"));
        assert_eq!(reference.href(), Some(&HRef::from_local("some-id")));
        assert!(!reference.owns());
    }

    #[test]
    fn set_role_updates_association() {
        let mut reference = Reference::new(HRef::from_local("some-id"));
        reference.set_role("http://example.com/role");
        assert_eq!(reference.role(), Some("http://example.com/role"));
    }
}
