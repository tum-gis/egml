use crate::impl_measure_type;
use crate::model::basic_types::Measure;

/// A speed (velocity magnitude) quantity with its unit of measure.
///
/// Corresponds to `gml:SpeedType` in ISO 19136 — a vacuous extension of
/// `gml:MeasureType`. The unit referenced by `uom` should be suitable for a
/// speed, such as metres per second.
///
/// # Examples
///
/// ```rust
/// use egml_core::model::measures::Speed;
///
/// let velocity = Speed::new(2.5, "m/s");
/// assert_eq!(velocity.value(), 2.5);
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Speed(Measure);

impl_measure_type!(Speed);
