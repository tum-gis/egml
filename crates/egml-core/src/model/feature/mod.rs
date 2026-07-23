//! Abstract GML feature base class ([OGC 07-036 §9](https://docs.ogc.org/is/07-036/07-036.pdf)).
//!
//! A *feature* is any real-world phenomenon with identity that can be described
//! using GML.  [`AbstractFeature`] extends [`AbstractGml`](super::base::AbstractGml)
//! and serves as the base for all concrete feature types defined in
//! GML application schemas.

mod abstract_feature;
mod abstract_feature_kind;
mod bounding_shape;

pub use abstract_feature::*;
pub use abstract_feature_kind::*;
pub use bounding_shape::*;
