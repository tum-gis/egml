use crate::model::base::{
    AssociationAttributes, HasAssociationAttributes, HasAssociationAttributesMut,
    HasOwnershipAttributes, HasOwnershipAttributesMut, OwnershipAttributes,
};
use crate::model::geometry::abstract_geometry_kind::AbstractGeometryKind;
use crate::model::xlink::HRef;

#[derive(Debug, Clone, PartialEq)]
pub struct AbstractGeometryArrayProperty {
    objects: Vec<AbstractGeometryKind>,
    association: AssociationAttributes,
    ownership: OwnershipAttributes,
}

impl AbstractGeometryArrayProperty {
    pub fn new(
        objects: Vec<AbstractGeometryKind>,
        association: AssociationAttributes,
        ownership: OwnershipAttributes,
    ) -> Self {
        Self {
            objects,
            association,
            ownership,
        }
    }

    pub fn from_objects(objects: Vec<AbstractGeometryKind>) -> Self {
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

    pub fn objects(&self) -> &[AbstractGeometryKind] {
        &self.objects
    }

    pub fn objects_mut(&mut self) -> &mut Vec<AbstractGeometryKind> {
        &mut self.objects
    }

    pub fn set_objects(&mut self, objects: Vec<AbstractGeometryKind>) {
        self.objects = objects;
    }

    pub fn push_object(&mut self, object: AbstractGeometryKind) {
        self.objects.push(object);
    }

    pub fn extend_objects(&mut self, objects: impl IntoIterator<Item = AbstractGeometryKind>) {
        self.objects.extend(objects);
    }
}

impl HasAssociationAttributes for AbstractGeometryArrayProperty {
    fn association(&self) -> &AssociationAttributes {
        &self.association
    }
}

impl HasAssociationAttributesMut for AbstractGeometryArrayProperty {
    fn association_mut(&mut self) -> &mut AssociationAttributes {
        &mut self.association
    }
}

impl HasOwnershipAttributes for AbstractGeometryArrayProperty {
    fn ownership(&self) -> &OwnershipAttributes {
        &self.ownership
    }
}

impl HasOwnershipAttributesMut for AbstractGeometryArrayProperty {
    fn ownership_mut(&mut self) -> &mut OwnershipAttributes {
        &mut self.ownership
    }
}
