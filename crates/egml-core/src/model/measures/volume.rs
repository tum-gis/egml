use crate::impl_measure_type;
use crate::model::basic_types::Measure;

/// A volume quantity with its unit of measure.
///
/// Corresponds to `gml:VolumeType` in ISO 19136 — a vacuous extension of
/// `gml:MeasureType`. The unit referenced by `uom` should be suitable for a
/// volume, such as cubic metres.
///
/// # Examples
///
/// ```rust
/// use egml_core::model::measures::Volume;
///
/// let capacity = Volume::new(45.0, "m3");
/// assert_eq!(capacity.value(), 45.0);
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Volume(Measure);

impl_measure_type!(Volume);
