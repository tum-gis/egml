//! Primitive GML scalar value types.
//!
//! | Type | GML counterpart | Description |
//! |------|----------------|-------------|
//! | [`Code`] | `gml:CodeType` | A string value optionally scoped to a code list |
//! | [`DoubleOrNilReason`] | `gml:doubleOrNilReason` | A double value, or the reason why it is missing |
//! | [`Measure`] | `gml:MeasureType` | A numeric value with an optional unit-of-measure URI |
//! | [`NilReason`] | `gml:NilReasonType` | The reason why a value is missing |

mod code;
mod double_or_nil_reason;
mod measure;
mod nil_reason;
mod nil_reason_enumeration;

pub use code::*;
pub use double_or_nil_reason::*;
pub use measure::*;
pub use nil_reason::*;
pub use nil_reason_enumeration::*;
