/// A numeric value with an associated unit-of-measure.
///
/// Corresponds to `gml:MeasureType` in ISO 19136.  The `uom` field is a
/// URI or UCUM expression identifying the unit (e.g. `"m"`, `"deg"`,
/// `"urn:ogc:def:uom:OGC:1.0:metre"`).
///
/// # Examples
///
/// ```rust
/// use egml_core::model::basic::Measure;
///
/// let height = Measure { uom: "m".to_string(), value: 12.5 };
/// assert_eq!(height.value, 12.5);
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Measure {
    /// URI or UCUM expression identifying the unit of measure.
    pub uom: String,
    /// The numeric measurement value.
    pub value: f64,
}
