/// Attributes from `gml:OwnershipAttributeGroup`.
///
/// Carried by every GML property element alongside [`AssociationAttributes`](super::AssociationAttributes).
/// `owns` asserts that this property is the primary owner of the referenced object —
/// i.e. the object's lifetime is tied to this property.
///
/// Corresponds to `gml:OwnershipAttributeGroup` in
/// [OGC 07-036 §7.2.3.2](https://docs.ogc.org/is/07-036/07-036.pdf).
/// The schema default for `owns` is `false`.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct OwnershipAttributes {
    /// `gml:owns` — `true` if this property owns the referenced object.
    pub owns: bool,
}
