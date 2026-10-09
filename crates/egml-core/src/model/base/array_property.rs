use crate::model::base::{HasOwnershipAttributes, HasOwnershipAttributesMut, OwnershipAttributes};

/// A GML property holding an ordered sequence of inline objects of type `T`.
///
/// Corresponds to array property types such as `gml:PointArrayPropertyType` and
/// `gml:GeometryArrayPropertyType`, which derive from `gml:ArrayAssociationType`
/// and declare only `gml:OwnershipAttributeGroup`: members are always inline and
/// can't be referenced via `xlink:href`
/// ([OGC 07-036 §7.2.3.4](https://docs.ogc.org/is/07-036/07-036.pdf)).
#[derive(Debug, Clone, PartialEq)]
pub struct ArrayProperty<T> {
    objects: Vec<T>,
    ownership: OwnershipAttributes,
}

impl<T> ArrayProperty<T> {
    pub fn new(objects: Vec<T>, ownership: OwnershipAttributes) -> Self {
        Self { objects, ownership }
    }

    pub fn from_objects(objects: Vec<T>) -> Self {
        Self::new(objects, OwnershipAttributes::default())
    }

    pub fn objects(&self) -> &[T] {
        &self.objects
    }

    pub fn objects_mut(&mut self) -> &mut Vec<T> {
        &mut self.objects
    }

    pub fn set_objects(&mut self, objects: Vec<T>) {
        self.objects = objects;
    }

    pub fn push_object(&mut self, object: T) {
        self.objects.push(object);
    }

    pub fn extend_objects(&mut self, objects: impl IntoIterator<Item = T>) {
        self.objects.extend(objects);
    }
}

impl<T> Default for ArrayProperty<T> {
    fn default() -> Self {
        Self::from_objects(Vec::new())
    }
}

impl<T> HasOwnershipAttributes for ArrayProperty<T> {
    fn ownership(&self) -> &OwnershipAttributes {
        &self.ownership
    }
}

impl<T> HasOwnershipAttributesMut for ArrayProperty<T> {
    fn ownership_mut(&mut self) -> &mut OwnershipAttributes {
        &mut self.ownership
    }
}

#[cfg(test)]
mod tests {
    use super::ArrayProperty;
    use crate::model::base::{HasOwnershipAttributes, HasOwnershipAttributesMut};

    #[test]
    fn from_objects_does_not_own() {
        let property = ArrayProperty::from_objects(vec![1, 2]);
        assert_eq!(property.objects(), [1, 2]);
        assert!(!property.owns());
    }

    #[test]
    fn objects_can_be_modified() {
        let mut property = ArrayProperty::default();
        property.push_object(1);
        property.extend_objects([2, 3]);
        property.objects_mut().retain(|x| *x != 2);
        assert_eq!(property.objects(), [1, 3]);
        property.set_objects(vec![4]);
        assert_eq!(property.objects(), [4]);
    }

    #[test]
    fn set_owns() {
        let mut property: ArrayProperty<i32> = ArrayProperty::default();
        property.set_owns(true);
        assert!(property.owns());
    }
}
