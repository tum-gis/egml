use std::fmt::Debug;
use std::hash::Hash;

/// Trait for enums that represent XML element tag names.
///
/// Implement this for each domain's element vocabulary (e.g. GML, CityGML).
/// The reader and writer are generic over any type that satisfies these bounds.
pub trait XmlElement: Copy + Eq + Hash + Debug {
    /// Maps an element's local name (without namespace prefix) to a variant.
    /// Returns `None` for unrecognised names.
    fn from_local_name(local_name: &[u8]) -> Option<Self>;

    /// Returns the fully-qualified tag name used in serialized XML (e.g. `"gml:Polygon"`).
    fn as_str(&self) -> &'static str;
}
