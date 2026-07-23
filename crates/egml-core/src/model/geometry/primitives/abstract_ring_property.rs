use crate::model::base::{
    AssociationAttributes, HasAssociationAttributes, HasAssociationAttributesMut,
    HasOwnershipAttributes, HasOwnershipAttributesMut, OwnershipAttributes,
};
use crate::model::geometry::primitives::abstract_ring_kind::AbstractRingKind;
use crate::model::xlink::HRef;

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractRingProperty {
    object: Option<AbstractRingKind>,
    association: AssociationAttributes,
    ownership: OwnershipAttributes,
}

impl AbstractRingProperty {
    pub fn new(
        object: Option<AbstractRingKind>,
        association: AssociationAttributes,
        ownership: OwnershipAttributes,
    ) -> Self {
        Self {
            object,
            association,
            ownership,
        }
    }

    pub fn from_object(object: AbstractRingKind) -> Self {
        Self {
            object: Some(object),
            association: AssociationAttributes::default(),
            ownership: OwnershipAttributes::default(),
        }
    }

    pub fn from_href(href: HRef) -> Self {
        Self {
            object: None,
            association: AssociationAttributes::new_href(href),
            ownership: OwnershipAttributes::default(),
        }
    }

    pub fn object(&self) -> Option<&AbstractRingKind> {
        self.object.as_ref()
    }

    pub fn object_mut(&mut self) -> Option<&mut AbstractRingKind> {
        self.object.as_mut()
    }

    pub fn take_object(&mut self) -> Option<AbstractRingKind> {
        self.object.take()
    }

    pub fn set_object(&mut self, object: AbstractRingKind) {
        self.object = Some(object);
    }

    pub fn set_object_opt(&mut self, object: Option<AbstractRingKind>) {
        self.object = object;
    }

    pub fn clear_object(&mut self) {
        self.object = None;
    }
}

impl HasAssociationAttributes for AbstractRingProperty {
    fn association(&self) -> &AssociationAttributes {
        &self.association
    }
}

impl HasAssociationAttributesMut for AbstractRingProperty {
    fn association_mut(&mut self) -> &mut AssociationAttributes {
        &mut self.association
    }
}

impl HasOwnershipAttributes for AbstractRingProperty {
    fn ownership(&self) -> &OwnershipAttributes {
        &self.ownership
    }
}

impl HasOwnershipAttributesMut for AbstractRingProperty {
    fn ownership_mut(&mut self) -> &mut OwnershipAttributes {
        &mut self.ownership
    }
}
