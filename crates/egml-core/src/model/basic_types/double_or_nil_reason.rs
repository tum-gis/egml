use crate::model::basic_types::NilReason;

/// A double value, or the reason why it is missing.
///
/// Corresponds to `gml:doubleOrNilReason` in
/// [OGC 07-036 §8.2.3.4](https://docs.ogc.org/is/07-036/07-036.pdf), the union
/// of `gml:NilReasonEnumeration`, `xs:double` and `xs:anyURI` (in that order).
/// The first and last member are both stored as a [`NilReason`].
///
/// Like `xs:double`, the value may be NaN or infinite.
///
/// # Examples
///
/// ```rust
/// use egml_core::model::basic_types::{DoubleOrNilReason, NilReason};
///
/// let value = DoubleOrNilReason::from(12.5);
/// assert_eq!(value.as_double(), Some(12.5));
///
/// let missing = DoubleOrNilReason::from(NilReason::new("missing"));
/// assert!(missing.is_nil());
/// assert_eq!(missing.nil_reason().unwrap().value(), "missing");
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum DoubleOrNilReason {
    /// A double value.
    Double(f64),
    /// The reason why the value is missing.
    NilReason(NilReason),
}

impl DoubleOrNilReason {
    /// Returns the double value, or `None` if a nil reason is given instead.
    pub fn as_double(&self) -> Option<f64> {
        match self {
            DoubleOrNilReason::Double(value) => Some(*value),
            DoubleOrNilReason::NilReason(_) => None,
        }
    }

    /// Returns the nil reason, or `None` if a double value is given instead.
    pub fn nil_reason(&self) -> Option<&NilReason> {
        match self {
            DoubleOrNilReason::Double(_) => None,
            DoubleOrNilReason::NilReason(nil_reason) => Some(nil_reason),
        }
    }

    /// Returns `true` if a nil reason is given instead of a value.
    pub fn is_nil(&self) -> bool {
        matches!(self, DoubleOrNilReason::NilReason(_))
    }
}

impl From<f64> for DoubleOrNilReason {
    fn from(value: f64) -> Self {
        DoubleOrNilReason::Double(value)
    }
}

impl From<NilReason> for DoubleOrNilReason {
    fn from(nil_reason: NilReason) -> Self {
        DoubleOrNilReason::NilReason(nil_reason)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn double_accessors() {
        let value = DoubleOrNilReason::from(-1.5);

        assert_eq!(value, DoubleOrNilReason::Double(-1.5));
        assert_eq!(value.as_double(), Some(-1.5));
        assert_eq!(value.nil_reason(), None);
        assert!(!value.is_nil());
    }

    #[test]
    fn nil_reason_accessors() {
        let nil_reason = NilReason::new("other:notMeasured");
        let value = DoubleOrNilReason::from(nil_reason.clone());

        assert_eq!(value, DoubleOrNilReason::NilReason(nil_reason.clone()));
        assert_eq!(value.as_double(), None);
        assert_eq!(value.nil_reason(), Some(&nil_reason));
        assert!(value.is_nil());
    }

    #[test]
    fn accepts_non_finite_doubles() {
        assert_eq!(
            DoubleOrNilReason::from(f64::INFINITY).as_double(),
            Some(f64::INFINITY)
        );
        assert_eq!(
            DoubleOrNilReason::from(f64::NEG_INFINITY).as_double(),
            Some(f64::NEG_INFINITY)
        );
        assert!(
            DoubleOrNilReason::from(f64::NAN)
                .as_double()
                .unwrap()
                .is_nan()
        );
    }
}
