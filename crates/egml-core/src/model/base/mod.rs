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

mod abstract_gml;
mod id;

pub use self::abstract_gml::*;
pub use self::id::*;
