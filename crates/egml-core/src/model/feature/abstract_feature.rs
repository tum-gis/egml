use crate::model::base::{AbstractGml, AsAbstractGml, AsAbstractGmlMut};
use crate::model::common::ApplyTransform;
use crate::model::feature::bounding_shape::BoundingShape;
use crate::model::geometry::Envelope;
use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Vector3};

/// Base class for all GML features ([OGC 07-036 §9.3.1](https://docs.ogc.org/is/07-036/07-036.pdf), `gml:AbstractFeatureType`).
///
/// Extends [`AbstractGml`] with an optional bounding envelope.  All concrete
/// feature types defined in GML application schemas embed `AbstractFeature`.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct AbstractFeature {
    /// Base GML object data (id, name).
    pub abstract_gml: AbstractGml,
    /// Optional precomputed bounding shape.
    bounded_by: Option<BoundingShape>,
}

impl AbstractFeature {
    /// Creates a new `AbstractFeature` with the given GML base data and no bounding envelope.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use egml_core::model::base::AbstractGml;
    /// use egml_core::model::feature::AbstractFeature;
    /// use crate::egml_core::model::feature::AsAbstractFeature;
    ///
    /// let feature = AbstractFeature::new();
    /// assert!(feature.bounded_by().is_none());
    /// ```
    pub fn new() -> Self {
        Self {
            abstract_gml: AbstractGml::default(),
            bounded_by: None,
        }
    }

    pub fn from_abstract_gml(abstract_gml: AbstractGml) -> Self {
        Self {
            abstract_gml,
            bounded_by: None,
        }
    }
}

impl ApplyTransform for AbstractFeature {
    fn apply_transform(&mut self, transform: Transform3<f64>) {
        if let Some(bounding_shape) = self.bounded_by.as_mut() {
            bounding_shape.apply_transform(transform);
        }
    }

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        if let Some(bounding_shape) = self.bounded_by.as_mut() {
            bounding_shape.apply_isometry(isometry);
        }
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        if let Some(bounding_shape) = self.bounded_by.as_mut() {
            bounding_shape.apply_translation(vector);
        }
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        if let Some(bounding_shape) = self.bounded_by.as_mut() {
            bounding_shape.apply_rotation(rotation);
        }
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        if let Some(bounding_shape) = self.bounded_by.as_mut() {
            bounding_shape.apply_scale(scale);
        }
    }
}

/// Object-safe read accessor for [`AbstractFeature`] fields.
pub trait AsAbstractFeature: AsAbstractGml {
    /// Returns a reference to the embedded [`AbstractFeature`] base data.
    fn abstract_feature(&self) -> &AbstractFeature;

    fn bounded_by(&self) -> Option<&BoundingShape> {
        self.abstract_feature().bounded_by.as_ref()
    }
}

/// Mutable companion to [`AsAbstractFeature`].
pub trait AsAbstractFeatureMut: AsAbstractFeature + AsAbstractGmlMut {
    /// Returns a mutable reference to the embedded [`AbstractFeature`] base data.
    fn abstract_feature_mut(&mut self) -> &mut AbstractFeature;

    fn set_bounded_by(&mut self, bounded_by: Option<BoundingShape>) {
        self.abstract_feature_mut().bounded_by = bounded_by;
    }

    fn bounded_by_mut(&mut self) -> &mut Option<BoundingShape> {
        &mut self.abstract_feature_mut().bounded_by
    }

    fn set_bounding_shape_from_envelope(&mut self, envelope: Option<Envelope>) {
        let bounding_shape = envelope.map(BoundingShape::new);
        <Self as AsAbstractFeatureMut>::abstract_feature_mut(self).bounded_by = bounding_shape;
    }
}

impl AsAbstractFeature for AbstractFeature {
    fn abstract_feature(&self) -> &AbstractFeature {
        self
    }
}

impl AsAbstractFeatureMut for AbstractFeature {
    fn abstract_feature_mut(&mut self) -> &mut AbstractFeature {
        self
    }
}

#[macro_export]
macro_rules! impl_abstract_feature_traits {
    ($type:ty) => {
        $crate::impl_abstract_gml_traits!($type);

        impl $crate::model::base::AsAbstractGml for $type {
            fn abstract_gml(&self) -> &$crate::model::base::AbstractGml {
                &<$type as $crate::model::feature::AsAbstractFeature>::abstract_feature(self)
                    .abstract_gml
            }
        }
    };
}

#[macro_export]
macro_rules! impl_abstract_feature_mut_traits {
    ($type:ty) => {
        $crate::impl_abstract_gml_mut_traits!($type);

        impl $crate::model::base::AsAbstractGmlMut for $type {
            fn abstract_gml_mut(&mut self) -> &mut $crate::model::base::AbstractGml {
                &mut <$type as $crate::model::feature::AsAbstractFeatureMut>::abstract_feature_mut(
                    self,
                )
                .abstract_gml
            }
        }
    };
}

impl_abstract_feature_traits!(AbstractFeature);
impl_abstract_feature_mut_traits!(AbstractFeature);
