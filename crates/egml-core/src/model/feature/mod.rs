//! Abstract GML feature base class ([OGC 07-036 §9](https://docs.ogc.org/is/07-036/07-036.pdf)).
//!
//! A *feature* is any real-world phenomenon with identity that can be described
//! using GML.  [`AbstractFeature`] extends [`AbstractGml`](super::base::AbstractGml)
//! and serves as the base for all concrete feature types defined in
//! GML application schemas.

mod abstract_feature;
mod bounding_shape;

pub use self::abstract_feature::*;
pub use self::bounding_shape::*;
