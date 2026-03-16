use crate::model::base::AbstractGml;
use crate::model::geometry::Envelope;

/// Base class for all GML features (ISO 19136 §9.3.1, `gml:AbstractFeatureType`).
///
/// Extends [`AbstractGml`] with an optional bounding envelope.  All concrete
/// feature types defined in GML application schemas embed `AbstractFeature`.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct AbstractFeature {
    /// Base GML object data (id, name).
    pub abstract_gml: AbstractGml,
    /// Optional precomputed bounding envelope.
    pub bounded_by: Option<Envelope>,
}

impl AbstractFeature {
    /// Creates a new `AbstractFeature` with the given GML base data and no bounding envelope.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use egml_core::model::base::AbstractGml;
    /// use egml_core::model::feature::AbstractFeature;
    ///
    /// let feature = AbstractFeature::new(AbstractGml::new());
    /// assert!(feature.bounded_by.is_none());
    /// ```
    pub fn new(abstract_gml: AbstractGml) -> Self {
        Self {
            abstract_gml,
            bounded_by: None,
        }
    }
}
