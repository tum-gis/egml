use crate::impl_measure_type;
use crate::model::basic_types::Measure;

/// A duration quantity with its unit of measure.
///
/// Corresponds to `gml:TimeType` in ISO 19136 — a vacuous extension of
/// `gml:MeasureType`. The unit referenced by `uom` should be suitable for a
/// duration, such as seconds or hours.
///
/// # Examples
///
/// ```rust
/// use egml_core::model::measures::Time;
///
/// let duration = Time::new(3.0, "s");
/// assert_eq!(duration.value(), 3.0);
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Time(Measure);

impl_measure_type!(Time);
