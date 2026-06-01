//! Primitive GML scalar value types.
//!
//! | Type | GML counterpart | Description |
//! |------|----------------|-------------|
//! | [`Code`] | `gml:CodeType` | A string value optionally scoped to a code list |
//! | [`Measure`] | `gml:MeasureType` | A numeric value with an optional unit-of-measure URI |

mod code;
mod measure;
mod nil_reason;
mod nil_reason_enumeration;

pub use code::*;
pub use measure::*;
pub use nil_reason::*;
pub use nil_reason_enumeration::*;
