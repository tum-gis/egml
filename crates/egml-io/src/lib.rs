//! XML deserialization of GML (Geography Markup Language) documents.
//!
//! `egml-io` converts GML XML into the geometry types provided by [`egml-core`].
//! It uses [`quick-xml`](https://docs.rs/quick-xml) for XML parsing and [`serde`]
//! for deserialization.
//!
//! # Supported GML elements
//!
//! | GML element | Rust type |
//! |-------------|-----------|
//! | `gml:Point` | [`egml_core::model::geometry::primitives::Point`] |
//! | `gml:LineString` | [`egml_core::model::geometry::primitives::LineString`] |
//! | `gml:LinearRing` | [`egml_core::model::geometry::primitives::LinearRing`] |
//! | `gml:Polygon` | [`egml_core::model::geometry::primitives::Polygon`] |
//! | `gml:Triangle` | [`egml_core::model::geometry::primitives::Triangle`] |
//! | `gml:Surface` | [`egml_core::model::geometry::primitives::Surface`] |
//! | `gml:TriangulatedSurface` | [`egml_core::model::geometry::primitives::TriangulatedSurface`] |
//! | `gml:Solid` | [`egml_core::model::geometry::primitives::Solid`] |
//! | `gml:MultiCurve` | [`egml_core::model::geometry::aggregates::MultiCurve`] |
//! | `gml:MultiSurface` | [`egml_core::model::geometry::aggregates::MultiSurface`] |
//! | `gml:CompositeSurface` | [`egml_core::model::geometry::complexes::CompositeSurface`] |
//!
//! # Error handling
//!
//! All parse errors are reported via [`Error`].  The most common variants are
//! [`Error::XmlDe`] (malformed XML) and [`Error::MissingElements`]
//! (required child elements absent from the GML fragment).
//!
//! XLink references (`xlink:href`) are not yet resolved; attempting to parse a
//! document that uses them returns [`Error::UnsupportedXLink`].

mod base;
mod basic;
mod error;
mod geometry;
mod util;

pub use crate::base::*;
pub use crate::basic::*;
pub use crate::geometry::*;

#[doc(inline)]
pub use crate::error::Error;
