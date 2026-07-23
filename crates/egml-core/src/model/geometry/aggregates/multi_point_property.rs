use crate::model::base::{
    AssociationAttributes, HasAssociationAttributes, HasAssociationAttributesMut,
    HasOwnershipAttributes, HasOwnershipAttributesMut, OwnershipAttributes,
};
use crate::model::geometry::aggregates::multi_point::MultiPoint;
use crate::model::xlink::HRef;

#[derive(Debug, Clone, PartialEq)]
pub struct MultiPointProperty {
    object: Option<MultiPoint>,
    association: AssociationAttributes,
    ownership: OwnershipAttributes,
}

impl MultiPointProperty {
    pub fn new(
        object: Option<MultiPoint>,
        association: AssociationAttributes,
        ownership: OwnershipAttributes,
    ) -> Self {
        Self {
            object,
            association,
            ownership,
        }
    }

    pub fn from_object(object: MultiPoint) -> Self {
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

    pub fn object(&self) -> Option<&MultiPoint> {
        self.object.as_ref()
    }

    pub fn object_mut(&mut self) -> Option<&mut MultiPoint> {
        self.object.as_mut()
    }

    pub fn take_object(&mut self) -> Option<MultiPoint> {
        self.object.take()
    }

    pub fn set_object(&mut self, object: MultiPoint) {
        self.object = Some(object);
    }

    pub fn set_object_opt(&mut self, object: Option<MultiPoint>) {
        self.object = object;
    }

    pub fn clear_object(&mut self) {
        self.object = None;
    }
}

impl HasAssociationAttributes for MultiPointProperty {
    fn association(&self) -> &AssociationAttributes {
        &self.association
    }
}

impl HasAssociationAttributesMut for MultiPointProperty {
    fn association_mut(&mut self) -> &mut AssociationAttributes {
        &mut self.association
    }
}

impl HasOwnershipAttributes for MultiPointProperty {
    fn ownership(&self) -> &OwnershipAttributes {
        &self.ownership
    }
}

impl HasOwnershipAttributesMut for MultiPointProperty {
    fn ownership_mut(&mut self) -> &mut OwnershipAttributes {
        &mut self.ownership
    }
}
