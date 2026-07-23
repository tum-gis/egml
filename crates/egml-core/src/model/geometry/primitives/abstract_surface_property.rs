use crate::model::base::{
    AssociationAttributes, HasAssociationAttributes, HasAssociationAttributesMut,
    HasOwnershipAttributes, HasOwnershipAttributesMut, OwnershipAttributes,
};
use crate::model::geometry::primitives::abstract_surface_kind::AbstractSurfaceKind;
use crate::model::xlink::HRef;

/// An owned wrapper around a concrete [`AbstractSurfaceKind`].
///
/// Used as a property element in GML to hold an inline surface definition.
#[derive(Debug, Clone, PartialEq)]
pub struct AbstractSurfaceProperty {
    object: Option<AbstractSurfaceKind>,
    association: AssociationAttributes,
    ownership: OwnershipAttributes,
}

impl AbstractSurfaceProperty {
    pub fn new(
        object: Option<AbstractSurfaceKind>,
        association: AssociationAttributes,
        ownership: OwnershipAttributes,
    ) -> Self {
        Self {
            object,
            association,
            ownership,
        }
    }

    pub fn from_object(object: AbstractSurfaceKind) -> Self {
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

    pub fn object(&self) -> Option<&AbstractSurfaceKind> {
        self.object.as_ref()
    }

    pub fn object_mut(&mut self) -> Option<&mut AbstractSurfaceKind> {
        self.object.as_mut()
    }

    pub fn take_object(&mut self) -> Option<AbstractSurfaceKind> {
        self.object.take()
    }

    pub fn set_object(&mut self, object: AbstractSurfaceKind) {
        self.object = Some(object);
    }

    pub fn set_object_opt(&mut self, object: Option<AbstractSurfaceKind>) {
        self.object = object;
    }

    pub fn clear_object(&mut self) {
        self.object = None;
    }
}

impl HasAssociationAttributes for AbstractSurfaceProperty {
    fn association(&self) -> &AssociationAttributes {
        &self.association
    }
}

impl HasAssociationAttributesMut for AbstractSurfaceProperty {
    fn association_mut(&mut self) -> &mut AssociationAttributes {
        &mut self.association
    }
}

impl HasOwnershipAttributes for AbstractSurfaceProperty {
    fn ownership(&self) -> &OwnershipAttributes {
        &self.ownership
    }
}

impl HasOwnershipAttributesMut for AbstractSurfaceProperty {
    fn ownership_mut(&mut self) -> &mut OwnershipAttributes {
        &mut self.ownership
    }
}
