//! GML data model.
//!
//! The model is organized into sub-modules that mirror the ISO 19136 GML
//! package structure:
//!
//! | Sub-module | Contents |
//! |------------|----------|
//! | [`base`] | Root GML types: [`AbstractGml`](base::AbstractGml), [`Id`](base::Id) |
//! | [`basic_types`] | Scalar value types: [`Code`](basic_types::Code), [`Measure`](basic_types::Measure) |
//! | [`feature`] | Abstract feature base class |
//! | [`geometry`] | Full geometry type hierarchy |
//! | [`measures`] | Named measure quantities: [`Length`](measures::Length), [`Area`](measures::Area), [`Volume`](measures::Volume), ... |

mod abstract_object;
pub mod abstract_object_kind;
pub mod base;
pub mod basic_types;
pub mod common;
pub mod feature;
pub mod geometry;
pub mod measures;
pub mod xlink;

pub use abstract_object::*;
pub use abstract_object_kind::*;
