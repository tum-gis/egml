//! Geometric transform operations for GML data.
//!
//! `egml-transform` is under active development. Future versions will expose
//! coordinate-system transforms, affine transformations, and other spatial
//! operations on the geometry types from [`egml-core`].
//!
//! # Errors
//!
//! Operations that fail return [`Error`].

mod error;

#[doc(inline)]
pub use error::Error;
