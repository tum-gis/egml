use crate::model::base::{
    AssociationAttributes, HasAssociationAttributes, HasAssociationAttributesMut,
    HasOwnershipAttributes, HasOwnershipAttributesMut, OwnershipAttributes,
};
use crate::model::geometry::primitives::Point;
use crate::model::xlink::HRef;

#[derive(Debug, Clone, PartialEq)]
pub struct PointArrayProperty {
    objects: Vec<Point>,
    association: AssociationAttributes,
    ownership: OwnershipAttributes,
}

impl PointArrayProperty {
    pub fn new(
        objects: Vec<Point>,
        association: AssociationAttributes,
        ownership: OwnershipAttributes,
    ) -> Self {
        Self {
            objects,
            association,
            ownership,
        }
    }

    pub fn from_objects(objects: Vec<Point>) -> Self {
        Self {
            objects,
            association: AssociationAttributes::default(),
            ownership: OwnershipAttributes::default(),
        }
    }

    pub fn from_href(href: HRef) -> Self {
        Self {
            objects: Vec::new(),
            association: AssociationAttributes::new_href(href),
            ownership: OwnershipAttributes::default(),
        }
    }

    pub fn objects(&self) -> &[Point] {
        &self.objects
    }

    pub fn objects_mut(&mut self) -> &mut Vec<Point> {
        &mut self.objects
    }

    pub fn set_objects(&mut self, objects: Vec<Point>) {
        self.objects = objects;
    }

    pub fn push_object(&mut self, object: Point) {
        self.objects.push(object);
    }

    pub fn extend_objects(&mut self, objects: impl IntoIterator<Item = Point>) {
        self.objects.extend(objects);
    }
}

impl HasAssociationAttributes for PointArrayProperty {
    fn association(&self) -> &AssociationAttributes {
        &self.association
    }
}

impl HasAssociationAttributesMut for PointArrayProperty {
    fn association_mut(&mut self) -> &mut AssociationAttributes {
        &mut self.association
    }
}

impl HasOwnershipAttributes for PointArrayProperty {
    fn ownership(&self) -> &OwnershipAttributes {
        &self.ownership
    }
}

impl HasOwnershipAttributesMut for PointArrayProperty {
    fn ownership_mut(&mut self) -> &mut OwnershipAttributes {
        &mut self.ownership
    }
}
