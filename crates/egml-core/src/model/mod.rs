//! GML data model.
//!
//! The model is organized into four sub-modules that mirror the ISO 19136 GML
//! package structure:
//!
//! | Sub-module | Contents |
//! |------------|----------|
//! | [`base`] | Root GML types: [`AbstractGml`](base::AbstractGml), [`Id`](base::Id) |
//! | [`basic`] | Scalar value types: [`Code`](basic::Code), [`Measure`](basic::Measure) |
//! | [`feature`] | Abstract feature base class |
//! | [`geometry`] | Full geometry type hierarchy |

pub mod base;
pub mod basic;
pub mod feature;
pub mod geometry;
