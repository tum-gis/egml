use crate::model::base::{
    AssociationAttributes, HasAssociationAttributes, HasAssociationAttributesMut,
    HasOwnershipAttributes, HasOwnershipAttributesMut, OwnershipAttributes,
};
use crate::model::geometry::primitives::shell::Shell;
use crate::model::xlink::HRef;

#[derive(Debug, Clone, PartialEq)]
pub struct ShellProperty {
    object: Option<Shell>,
    association: AssociationAttributes,
    ownership: OwnershipAttributes,
}

impl ShellProperty {
    pub fn new(
        object: Option<Shell>,
        association: AssociationAttributes,
        ownership: OwnershipAttributes,
    ) -> Self {
        Self {
            object,
            association,
            ownership,
        }
    }

    pub fn from_object(object: Shell) -> Self {
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

    pub fn object(&self) -> Option<&Shell> {
        self.object.as_ref()
    }

    pub fn object_mut(&mut self) -> Option<&mut Shell> {
        self.object.as_mut()
    }

    pub fn take_object(&mut self) -> Option<Shell> {
        self.object.take()
    }

    pub fn set_object(&mut self, object: Shell) {
        self.object = Some(object);
    }

    pub fn set_object_opt(&mut self, object: Option<Shell>) {
        self.object = object;
    }

    pub fn clear_object(&mut self) {
        self.object = None;
    }
}

impl HasAssociationAttributes for ShellProperty {
    fn association(&self) -> &AssociationAttributes {
        &self.association
    }
}

impl HasAssociationAttributesMut for ShellProperty {
    fn association_mut(&mut self) -> &mut AssociationAttributes {
        &mut self.association
    }
}

impl HasOwnershipAttributes for ShellProperty {
    fn ownership(&self) -> &OwnershipAttributes {
        &self.ownership
    }
}

impl HasOwnershipAttributesMut for ShellProperty {
    fn ownership_mut(&mut self) -> &mut OwnershipAttributes {
        &mut self.ownership
    }
}
