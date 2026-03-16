//! Primitive GML scalar value types.
//!
//! | Type | GML counterpart | Description |
//! |------|----------------|-------------|
//! | [`Code`] | `gml:CodeType` | A string value optionally scoped to a code list |
//! | [`Measure`] | `gml:MeasureType` | A numeric value with an optional unit-of-measure URI |

mod code;
mod measure;

pub use code::*;
pub use measure::*;
