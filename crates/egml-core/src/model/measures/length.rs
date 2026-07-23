use crate::impl_measure_type;
use crate::model::basic_types::Measure;

/// A length (distance) quantity with its unit of measure.
///
/// Corresponds to `gml:LengthType` in ISO 19136 — a vacuous extension of
/// `gml:MeasureType`. The unit referenced by `uom` should be suitable for a
/// length, such as metres or feet.
///
/// # Examples
///
/// ```rust
/// use egml_core::model::measures::Length;
///
/// let height = Length::new(12.5, "m");
/// assert_eq!(height.value(), 12.5);
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Length(Measure);

impl_measure_type!(Length);
