//! Codecs for the named GML measure types.
//!
//! ISO 19136 §17.3.7 (`measures.xsd`) defines each of these as a vacuous
//! extension of `gml:MeasureType`, so they all share its wire format: a
//! `uom` XML attribute plus the numeric value as element text, e.g.
//! `<volume uom="m3">5.0</volume>`. Each type gets its own `deserialize_*`/
//! `serialize_*` pair parsing/writing directly via [`quick_xml`], without
//! going through serde.

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
