//! Lexical form of `xs:double` ([XSD 1.1 Part 2 §3.3.5](https://www.w3.org/TR/xmlschema11-2/#double)).
//!
//! Rust's `f64` parser is more lenient than XSD (it accepts `inf`, `infinity`
//! and `nan` in any case), so values are checked against the XSD grammar
//! before they are converted.

/// Parses an `xs:double` lexical value without surrounding whitespace.
///
/// Accepts decimal and exponent forms (`1`, `-1.5E3`, `.5`, `1.`) and the
/// special values `INF`, `+INF`, `-INF` and `NaN`. Returns `None` for
/// anything else.
pub(crate) fn parse_double(lexical: &str) -> Option<f64> {
    match lexical {
        "INF" | "+INF" => return Some(f64::INFINITY),
        "-INF" => return Some(f64::NEG_INFINITY),
        "NaN" => return Some(f64::NAN),
        _ => {}
    }

    is_numeric_double(lexical)
        .then(|| lexical.parse().ok())
        .flatten()
}

/// Writes an `xs:double`: `INF`, `-INF` and `NaN` for the special values,
/// otherwise the shortest decimal form that reads back to the same value.
pub(crate) fn format_double(value: f64) -> String {
    if value.is_nan() {
        "NaN".to_owned()
    } else if value == f64::INFINITY {
        "INF".to_owned()
    } else if value == f64::NEG_INFINITY {
        "-INF".to_owned()
    } else {
        value.to_string()
    }
}

/// Checks `(\+|-)?([0-9]+(\.[0-9]*)?|\.[0-9]+)([Ee](\+|-)?[0-9]+)?`.
fn is_numeric_double(lexical: &str) -> bool {
    let rest = strip_sign(lexical);
    let (mantissa, exponent) = match rest.split_once(['e', 'E']) {
        Some((mantissa, exponent)) => (mantissa, Some(exponent)),
        None => (rest, None),
    };

    let (integer, fraction) = mantissa.split_once('.').unwrap_or((mantissa, ""));
    let valid_mantissa =
        (!integer.is_empty() || !fraction.is_empty()) && is_digits(integer) && is_digits(fraction);
    let valid_exponent = exponent.is_none_or(|x| {
        let digits = strip_sign(x);
        !digits.is_empty() && is_digits(digits)
    });

    valid_mantissa && valid_exponent
}

fn strip_sign(lexical: &str) -> &str {
    lexical.strip_prefix(['+', '-']).unwrap_or(lexical)
}

fn is_digits(digits: &str) -> bool {
    digits.bytes().all(|x| x.is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_double_numeric_forms() {
        for (lexical, expected) in [
            ("1", 1.0),
            ("-1.5E3", -1500.0),
            ("+12.25", 12.25),
            (".5", 0.5),
            ("1.", 1.0),
            ("1e-2", 0.01),
            ("0", 0.0),
            ("-0", -0.0),
        ] {
            assert_eq!(parse_double(lexical), Some(expected), "{lexical:?}");
        }
    }

    #[test]
    fn test_parse_double_special_values() {
        assert_eq!(parse_double("INF"), Some(f64::INFINITY));
        assert_eq!(parse_double("+INF"), Some(f64::INFINITY));
        assert_eq!(parse_double("-INF"), Some(f64::NEG_INFINITY));
        assert!(parse_double("NaN").unwrap().is_nan());
    }

    #[test]
    fn test_parse_double_rejects_non_xsd_forms() {
        for lexical in [
            "", "inf", "-inf", "infinity", "nan", "+NaN", "1,5", "0x10", "1e", "e5", ".", "-",
            "1.5.2", " 1", "1 ", "1_000", "missing",
        ] {
            assert_eq!(parse_double(lexical), None, "{lexical:?}");
        }
    }

    #[test]
    fn test_format_double() {
        assert_eq!(format_double(12.5), "12.5");
        assert_eq!(format_double(-1500.0), "-1500");
        assert_eq!(format_double(f64::INFINITY), "INF");
        assert_eq!(format_double(f64::NEG_INFINITY), "-INF");
        assert_eq!(format_double(f64::NAN), "NaN");
    }

    #[test]
    fn test_format_double_round_trips() {
        for value in [0.1, -2.5e-300, 1.7976931348623157e308, 123456789.0] {
            assert_eq!(parse_double(&format_double(value)), Some(value));
        }
    }
}
