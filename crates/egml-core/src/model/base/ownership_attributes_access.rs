use crate::model::base::OwnershipAttributes;

/// Read-only access to the `gml:OwnershipAttributeGroup` on a property element.
pub trait HasOwnershipAttributes {
    fn ownership(&self) -> &OwnershipAttributes;

    /// `gml:owns` — whether this property owns the referenced object.
    fn owns(&self) -> bool {
        self.ownership().owns
    }
}

/// Mutable access to the `gml:OwnershipAttributeGroup` on a property element.
pub trait HasOwnershipAttributesMut: HasOwnershipAttributes {
    fn ownership_mut(&mut self) -> &mut OwnershipAttributes;

    fn set_owns(&mut self, owns: bool) {
        self.ownership_mut().owns = owns;
    }
}
