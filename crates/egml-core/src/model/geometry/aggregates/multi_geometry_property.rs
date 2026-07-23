use crate::model::base::{
    AssociationAttributes, HasAssociationAttributes, HasAssociationAttributesMut,
    HasOwnershipAttributes, HasOwnershipAttributesMut, OwnershipAttributes,
};
use crate::model::geometry::aggregates::multi_geometry::MultiGeometry;
use crate::model::xlink::HRef;

#[derive(Debug, Clone, PartialEq)]
pub struct MultiGeometryProperty {
    object: Option<MultiGeometry>,
    association: AssociationAttributes,
    ownership: OwnershipAttributes,
}

impl MultiGeometryProperty {
    pub fn new(
        object: Option<MultiGeometry>,
        association: AssociationAttributes,
        ownership: OwnershipAttributes,
    ) -> Self {
        Self {
            object,
            association,
            ownership,
        }
    }

    pub fn from_object(object: MultiGeometry) -> Self {
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

    pub fn object(&self) -> Option<&MultiGeometry> {
        self.object.as_ref()
    }

    pub fn object_mut(&mut self) -> Option<&mut MultiGeometry> {
        self.object.as_mut()
    }

    pub fn take_object(&mut self) -> Option<MultiGeometry> {
        self.object.take()
    }

    pub fn set_object(&mut self, object: MultiGeometry) {
        self.object = Some(object);
    }

    pub fn set_object_opt(&mut self, object: Option<MultiGeometry>) {
        self.object = object;
    }

    pub fn clear_object(&mut self) {
        self.object = None;
    }
}

impl HasAssociationAttributes for MultiGeometryProperty {
    fn association(&self) -> &AssociationAttributes {
        &self.association
    }
}

impl HasAssociationAttributesMut for MultiGeometryProperty {
    fn association_mut(&mut self) -> &mut AssociationAttributes {
        &mut self.association
    }
}

impl HasOwnershipAttributes for MultiGeometryProperty {
    fn ownership(&self) -> &OwnershipAttributes {
        &self.ownership
    }
}

impl HasOwnershipAttributesMut for MultiGeometryProperty {
    fn ownership_mut(&mut self) -> &mut OwnershipAttributes {
        &mut self.ownership
    }
}
