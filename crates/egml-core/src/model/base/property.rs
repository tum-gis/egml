use crate::model::base::{
    AssociationAttributes, HasAssociationAttributes, HasAssociationAttributesMut,
    HasOwnershipAttributes, HasOwnershipAttributesMut, OwnershipAttributes,
};
use crate::model::xlink::HRef;

/// A GML property holding at most one object of type `T`, either inline or by
/// reference via `xlink:href`.
///
/// Corresponds to property types declaring both `gml:AssociationAttributeGroup`
/// and `gml:OwnershipAttributeGroup`, such as `gml:SurfacePropertyType`
/// ([OGC 07-036 §7.2.3](https://docs.ogc.org/is/07-036/07-036.pdf)).
///
/// The attributes are rarely present, so they are stored out of line and only
/// allocated once one is set: a `Property<T>` costs one pointer on top of
/// `Option<T>`.
#[derive(Debug, Clone)]
pub struct Property<T> {
    object: Option<T>,
    attributes: Option<Box<PropertyAttributes>>,
}

impl<T> Property<T> {
    pub fn new(
        object: Option<T>,
        association: AssociationAttributes,
        ownership: OwnershipAttributes,
    ) -> Self {
        let attributes = PropertyAttributes {
            association,
            ownership,
        };
        Self {
            object,
            attributes: (!attributes.is_empty()).then(|| Box::new(attributes)),
        }
    }

    pub fn from_object(object: T) -> Self {
        Self {
            object: Some(object),
            attributes: None,
        }
    }

    pub fn from_href(href: HRef) -> Self {
        Self::new(
            None,
            AssociationAttributes::new_href(href),
            OwnershipAttributes::default(),
        )
    }

    pub fn object(&self) -> Option<&T> {
        self.object.as_ref()
    }

    pub fn object_mut(&mut self) -> Option<&mut T> {
        self.object.as_mut()
    }

    pub fn take_object(&mut self) -> Option<T> {
        self.object.take()
    }

    pub fn set_object(&mut self, object: T) {
        self.object = Some(object);
    }

    pub fn set_object_opt(&mut self, object: Option<T>) {
        self.object = object;
    }

    pub fn clear_object(&mut self) {
        self.object = None;
    }

    fn attributes(&self) -> &PropertyAttributes {
        self.attributes.as_deref().unwrap_or(&EMPTY_ATTRIBUTES)
    }

    fn attributes_mut(&mut self) -> &mut PropertyAttributes {
        self.attributes.get_or_insert_with(Box::default)
    }
}

impl<T> Default for Property<T> {
    fn default() -> Self {
        Self {
            object: None,
            attributes: None,
        }
    }
}

/// Compares attributes by content, so a set-then-cleared attribute equals an unset one.
impl<T: PartialEq> PartialEq for Property<T> {
    fn eq(&self, other: &Self) -> bool {
        self.object == other.object && self.attributes() == other.attributes()
    }
}

impl<T> HasAssociationAttributes for Property<T> {
    fn association(&self) -> &AssociationAttributes {
        &self.attributes().association
    }
}

impl<T> HasAssociationAttributesMut for Property<T> {
    fn association_mut(&mut self) -> &mut AssociationAttributes {
        &mut self.attributes_mut().association
    }
}

impl<T> HasOwnershipAttributes for Property<T> {
    fn ownership(&self) -> &OwnershipAttributes {
        &self.attributes().ownership
    }
}

impl<T> HasOwnershipAttributesMut for Property<T> {
    fn ownership_mut(&mut self) -> &mut OwnershipAttributes {
        &mut self.attributes_mut().ownership
    }
}

/// Storage for the attribute groups of a [`Property`].
///
/// Both groups live behind a single box so that a property without attributes
/// costs one pointer; accessed only through the attribute traits.
#[derive(Debug, Clone, PartialEq, Default)]
struct PropertyAttributes {
    association: AssociationAttributes,
    ownership: OwnershipAttributes,
}

impl PropertyAttributes {
    fn is_empty(&self) -> bool {
        *self == EMPTY_ATTRIBUTES
    }
}

static EMPTY_ATTRIBUTES: PropertyAttributes = PropertyAttributes {
    association: AssociationAttributes {
        href: None,
        nil_reason: None,
        title: None,
        role: None,
        arcrole: None,
        show: None,
        actuate: None,
    },
    ownership: OwnershipAttributes { owns: false },
};

#[cfg(test)]
mod tests {
    use super::Property;
    use crate::model::base::{
        AssociationAttributes, HasAssociationAttributes, HasAssociationAttributesMut,
        HasOwnershipAttributes, HasOwnershipAttributesMut, OwnershipAttributes,
    };
    use crate::model::xlink::HRef;
    use std::mem::size_of;

    #[test]
    fn attributes_cost_one_pointer() {
        assert_eq!(
            size_of::<Property<[u64; 4]>>(),
            size_of::<Option<[u64; 4]>>() + size_of::<usize>()
        );
    }

    #[test]
    fn from_object_does_not_allocate_attributes() {
        let property = Property::from_object(1);
        assert!(property.attributes.is_none());
        assert_eq!(property.object(), Some(&1));
        assert_eq!(property.href(), None);
        assert!(!property.owns());
    }

    #[test]
    fn new_with_default_attributes_does_not_allocate() {
        let property = Property::new(
            Some(1),
            AssociationAttributes::default(),
            OwnershipAttributes::default(),
        );
        assert!(property.attributes.is_none());
    }

    #[test]
    fn new_keeps_non_default_attributes() {
        let property: Property<i32> = Property::new(
            None,
            AssociationAttributes::default(),
            OwnershipAttributes { owns: true },
        );
        assert!(property.owns());
    }

    #[test]
    fn from_href_sets_href() {
        let property: Property<i32> = Property::from_href(HRef::from_local("id"));
        assert_eq!(property.href(), Some(&HRef::from_local("id")));
        assert_eq!(property.object(), None);
    }

    #[test]
    fn setters_allocate_on_first_write() {
        let mut property = Property::from_object(1);
        property.set_title("title");
        property.set_owns(true);
        assert_eq!(property.title(), Some("title"));
        assert!(property.owns());
    }

    #[test]
    fn cleared_attributes_equal_unset_attributes() {
        let mut property = Property::from_object(1);
        property.set_title("title");
        property.clear_title();
        assert_eq!(property, Property::from_object(1));
    }

    #[test]
    fn object_accessors() {
        let mut property = Property::default();
        assert_eq!(property.object(), None);
        property.set_object(1);
        *property.object_mut().unwrap() += 1;
        assert_eq!(property.take_object(), Some(2));
        property.set_object_opt(Some(3));
        property.clear_object();
        assert_eq!(property.object(), None);
    }
}
