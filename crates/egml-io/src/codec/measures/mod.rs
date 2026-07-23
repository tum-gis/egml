//! Serde codecs for the named GML measure types.
//!
//! ISO 19136 §17.3.7 (`measures.xsd`) defines each of these as a vacuous
//! extension of `gml:MeasureType`, so they all share its wire format: a
//! `uom` XML attribute plus the numeric value as element text, e.g.
//! `<volume uom="m3">5.0</volume>`. Each type gets its own serde struct
//! (rather than reusing one shared struct) so it deserializes directly into
//! the matching [`egml_core::model::measures`] type.

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

/// Defines a serde wire struct for a named GML measure type, plus
/// conversions to/from its `egml_core::model::measures` counterpart.
macro_rules! impl_measure_codec {
    ($gml_name:ident, $core_name:ident) => {
        #[derive(Debug, ::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
        pub struct $gml_name {
            #[serde(rename = "@uom")]
            pub uom: String,
            #[serde(rename = "$value")]
            pub value: f64,
        }

        impl From<$gml_name> for egml_core::model::measures::$core_name {
            fn from(item: $gml_name) -> Self {
                Self::new(item.value, item.uom)
            }
        }

        impl From<&egml_core::model::measures::$core_name> for $gml_name {
            fn from(item: &egml_core::model::measures::$core_name) -> Self {
                Self {
                    uom: item.uom().to_string(),
                    value: item.value(),
                }
            }
        }
    };
}

pub(crate) use impl_measure_codec;
