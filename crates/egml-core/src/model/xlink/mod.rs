//! XLink attribute types used by GML `AssociationAttributeGroup`.
//!
//! The types here correspond to the W3C XLink 1.1 simple-link attribute model
//! (`xlink:type`, `xlink:href`, `xlink:show`, `xlink:actuate`, …) as reused by
//! the GML 3.2.1 `gml:AssociationAttributeGroup`.

mod actuate_type;
mod href;
mod show_type;

pub use actuate_type::ActuateType;
pub use href::HRef;
pub use show_type::ShowType;
