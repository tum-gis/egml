//! Root GML types common to all GML objects.
//!
//! Every GML object descends from `AbstractGML` ([OGC 07-036 §7.2.2.2](https://docs.ogc.org/is/07-036/07-036.pdf)), which
//! carries an optional [`Id`] and zero-or-more name strings.  This module
//! exposes those two building blocks.
//!
//! | Type | Description |
//! |------|-------------|
//! | [`AbstractGml`] | Base data shared by every GML object |
//! | [`Id`] | Stable, globally unique GML object identifier |
//! | [`Reference`] | A by-reference-only property (`gml:ReferenceType`) |

mod abstract_gml;
mod abstract_gml_kind;
mod association_attributes;
mod association_attributes_access;
mod id;
mod ownership_attributes;
mod ownership_attributes_access;
mod reference;

pub use abstract_gml::*;
pub use abstract_gml_kind::*;
pub use association_attributes::*;
pub use association_attributes_access::*;
pub use id::*;
pub use ownership_attributes::*;
pub use ownership_attributes_access::*;
pub use reference::*;
