use crate::impl_measure_type;
use crate::model::basic_types::Measure;

/// An area quantity with its unit of measure.
///
/// Corresponds to `gml:AreaType` in ISO 19136 — a vacuous extension of
/// `gml:MeasureType`. The unit referenced by `uom` should be suitable for an
/// area, such as square metres.
///
/// # Examples
///
/// ```rust
/// use egml_core::model::measures::Area;
///
/// let footprint = Area::new(120.0, "m2");
/// assert_eq!(footprint.value(), 120.0);
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Area(Measure);

impl_measure_type!(Area);
