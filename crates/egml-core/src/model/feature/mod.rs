//! Abstract GML feature base class (ISO 19136 §9).
//!
//! A *feature* is any real-world phenomenon with identity that can be described
//! using GML.  [`AbstractFeature`] extends [`AbstractGml`](super::base::AbstractGml)
//! and serves as the base for all concrete feature types defined in
//! GML application schemas.

mod abstract_feature;

pub use self::abstract_feature::*;
