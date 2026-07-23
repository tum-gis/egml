use crate::impl_measure_type;
use crate::model::basic_types::Measure;

/// An angle quantity with its unit of measure.
///
/// Corresponds to `gml:AngleType` in ISO 19136 — a vacuous extension of
/// `gml:MeasureType`. The unit referenced by `uom` should be suitable for an
/// angle, such as degrees or radians.
///
/// # Examples
///
/// ```rust
/// use egml_core::model::measures::Angle;
///
/// let bearing = Angle::new(90.0, "deg");
/// assert_eq!(bearing.value(), 90.0);
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Angle(Measure);

impl_measure_type!(Angle);
