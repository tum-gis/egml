use crate::model::base::{
    AssociationAttributes, HasAssociationAttributes, HasAssociationAttributesMut,
    HasOwnershipAttributes, HasOwnershipAttributesMut, OwnershipAttributes,
};
use crate::model::geometry::primitives::LinearRing;
use crate::model::xlink::HRef;

#[derive(Debug, Clone, PartialEq)]
pub struct LinearRingProperty {
    object: Option<LinearRing>,
    association: AssociationAttributes,
    ownership: OwnershipAttributes,
}

impl LinearRingProperty {
    pub fn new(
        object: Option<LinearRing>,
        association: AssociationAttributes,
        ownership: OwnershipAttributes,
    ) -> Self {
        Self {
            object,
            association,
            ownership,
        }
    }

    pub fn from_object(object: LinearRing) -> Self {
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

    pub fn object(&self) -> Option<&LinearRing> {
        self.object.as_ref()
    }

    pub fn object_mut(&mut self) -> Option<&mut LinearRing> {
        self.object.as_mut()
    }

    pub fn take_object(&mut self) -> Option<LinearRing> {
        self.object.take()
    }

    pub fn set_object(&mut self, object: LinearRing) {
        self.object = Some(object);
    }

    pub fn set_object_opt(&mut self, object: Option<LinearRing>) {
        self.object = object;
    }

    pub fn clear_object(&mut self) {
        self.object = None;
    }
}

impl HasAssociationAttributes for LinearRingProperty {
    fn association(&self) -> &AssociationAttributes {
        &self.association
    }
}

impl HasAssociationAttributesMut for LinearRingProperty {
    fn association_mut(&mut self) -> &mut AssociationAttributes {
        &mut self.association
    }
}

impl HasOwnershipAttributes for LinearRingProperty {
    fn ownership(&self) -> &OwnershipAttributes {
        &self.ownership
    }
}

impl HasOwnershipAttributesMut for LinearRingProperty {
    fn ownership_mut(&mut self) -> &mut OwnershipAttributes {
        &mut self.ownership
    }
}
