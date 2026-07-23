//! Named measure quantities.
//!
//! ISO 19136 §17.3.7 (`measures.xsd`) defines a set of convenience measure
//! types, each a vacuous extension of [`gml:MeasureType`](crate::model::basic_types::Measure)
//! that exists only to give a quantity a distinct, self-documenting type.
//! This module mirrors that with newtype wrappers around
//! [`Measure`](crate::model::basic_types::Measure).
//!
//! | Type | GML counterpart |
//! |------|-----------------|
//! | [`Length`] | `gml:LengthType` |
//! | [`Scale`] | `gml:ScaleType` |
//! | [`Time`] | `gml:TimeType` |
//! | [`GridLength`] | `gml:GridLengthType` |
//! | [`Area`] | `gml:AreaType` |
//! | [`Volume`] | `gml:VolumeType` |
//! | [`Speed`] | `gml:SpeedType` |
//! | [`Angle`] | `gml:AngleType` |

mod angle;
mod area;
mod grid_length;
mod length;
mod scale;
mod speed;
mod time;
mod volume;

pub use angle::*;
pub use area::*;
pub use grid_length::*;
pub use length::*;
pub use scale::*;
pub use speed::*;
pub use time::*;
pub use volume::*;

/// Implements the common constructor, accessors, and `Measure` conversions
/// shared by all named measure types.
#[macro_export]
macro_rules! impl_measure_type {
    ($name:ident) => {
        impl $name {
            /// Creates a new measurement with the given value and unit of measure.
            pub fn new(value: f64, uom: impl Into<String>) -> Self {
                Self($crate::model::basic_types::Measure {
                    uom: uom.into(),
                    value,
                })
            }

            /// Returns the numeric measurement value.
            pub fn value(&self) -> f64 {
                self.0.value
            }

            /// Returns the unit-of-measure URI or UCUM expression.
            pub fn uom(&self) -> &str {
                &self.0.uom
            }
        }

        impl From<$crate::model::basic_types::Measure> for $name {
            fn from(measure: $crate::model::basic_types::Measure) -> Self {
                Self(measure)
            }
        }

        impl From<$name> for $crate::model::basic_types::Measure {
            fn from(value: $name) -> Self {
                value.0
            }
        }
    };
}
