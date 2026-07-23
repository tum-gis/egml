use crate::impl_measure_type;
use crate::model::basic_types::Measure;

/// A length measured in grid coordinates, with its unit of measure.
///
/// Corresponds to `gml:GridLengthType` in ISO 19136 — a vacuous extension of
/// `gml:MeasureType`, used for offsets and spacings expressed in raster or
/// grid coordinate units rather than physical length units.
///
/// # Examples
///
/// ```rust
/// use egml_core::model::measures::GridLength;
///
/// let spacing = GridLength::new(1.0, "gridspacing");
/// assert_eq!(spacing.value(), 1.0);
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct GridLength(Measure);

impl_measure_type!(GridLength);
