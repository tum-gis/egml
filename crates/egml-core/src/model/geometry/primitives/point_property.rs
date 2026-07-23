use crate::model::base::{
    AssociationAttributes, HasAssociationAttributes, HasAssociationAttributesMut,
    HasOwnershipAttributes, HasOwnershipAttributesMut, OwnershipAttributes,
};
use crate::model::geometry::primitives::Point;
use crate::model::xlink::HRef;

#[derive(Debug, Clone, PartialEq)]
pub struct PointProperty {
    object: Option<Point>,
    association: AssociationAttributes,
    ownership: OwnershipAttributes,
}

impl PointProperty {
    pub fn new(
        object: Option<Point>,
        association: AssociationAttributes,
        ownership: OwnershipAttributes,
    ) -> Self {
        Self {
            object,
            association,
            ownership,
        }
    }

    pub fn from_object(object: Point) -> Self {
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

    pub fn object(&self) -> Option<&Point> {
        self.object.as_ref()
    }

    pub fn object_mut(&mut self) -> Option<&mut Point> {
        self.object.as_mut()
    }

    pub fn take_object(&mut self) -> Option<Point> {
        self.object.take()
    }

    pub fn set_object(&mut self, object: Point) {
        self.object = Some(object);
    }

    pub fn set_object_opt(&mut self, object: Option<Point>) {
        self.object = object;
    }

    pub fn clear_object(&mut self) {
        self.object = None;
    }
}

impl HasAssociationAttributes for PointProperty {
    fn association(&self) -> &AssociationAttributes {
        &self.association
    }
}

impl HasAssociationAttributesMut for PointProperty {
    fn association_mut(&mut self) -> &mut AssociationAttributes {
        &mut self.association
    }
}

impl HasOwnershipAttributes for PointProperty {
    fn ownership(&self) -> &OwnershipAttributes {
        &self.ownership
    }
}

impl HasOwnershipAttributesMut for PointProperty {
    fn ownership_mut(&mut self) -> &mut OwnershipAttributes {
        &mut self.ownership
    }
}
