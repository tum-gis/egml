use crate::impl_measure_type;
use crate::model::basic_types::Measure;

/// A dimensionless scale factor with its unit of measure.
///
/// Corresponds to `gml:ScaleType` in ISO 19136 — a vacuous extension of
/// `gml:MeasureType`. The unit referenced by `uom` should be suitable for a
/// ratio, such as a percentage or a unitless factor.
///
/// # Examples
///
/// ```rust
/// use egml_core::model::measures::Scale;
///
/// let factor = Scale::new(0.5, "unity");
/// assert_eq!(factor.value(), 0.5);
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Scale(Measure);

impl_measure_type!(Scale);
