use crate::model::base::{
    AssociationAttributes, HasAssociationAttributes, HasAssociationAttributesMut,
    HasOwnershipAttributes, HasOwnershipAttributesMut, OwnershipAttributes,
};
use crate::model::geometry::aggregates::MultiSurface;
use crate::model::xlink::HRef;

#[derive(Debug, Clone, PartialEq)]
pub struct MultiSurfaceProperty {
    object: Option<MultiSurface>,
    association: AssociationAttributes,
    ownership: OwnershipAttributes,
}

impl MultiSurfaceProperty {
    pub fn new(
        object: Option<MultiSurface>,
        association: AssociationAttributes,
        ownership: OwnershipAttributes,
    ) -> Self {
        Self {
            object,
            association,
            ownership,
        }
    }

    pub fn from_object(object: MultiSurface) -> Self {
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

    pub fn object(&self) -> Option<&MultiSurface> {
        self.object.as_ref()
    }

    pub fn object_mut(&mut self) -> Option<&mut MultiSurface> {
        self.object.as_mut()
    }

    pub fn take_object(&mut self) -> Option<MultiSurface> {
        self.object.take()
    }

    pub fn set_object(&mut self, object: MultiSurface) {
        self.object = Some(object);
    }

    pub fn set_object_opt(&mut self, object: Option<MultiSurface>) {
        self.object = object;
    }

    pub fn clear_object(&mut self) {
        self.object = None;
    }
}

impl HasAssociationAttributes for MultiSurfaceProperty {
    fn association(&self) -> &AssociationAttributes {
        &self.association
    }
}

impl HasAssociationAttributesMut for MultiSurfaceProperty {
    fn association_mut(&mut self) -> &mut AssociationAttributes {
        &mut self.association
    }
}

impl HasOwnershipAttributes for MultiSurfaceProperty {
    fn ownership(&self) -> &OwnershipAttributes {
        &self.ownership
    }
}

impl HasOwnershipAttributesMut for MultiSurfaceProperty {
    fn ownership_mut(&mut self) -> &mut OwnershipAttributes {
        &mut self.ownership
    }
}
