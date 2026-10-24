use crate::Error;
use crate::codec::xs::{collapse_whitespace, format_double, parse_double};
use crate::util::{XmlElement, XmlFragmentWriter, XmlNamespace};
use egml_core::model::basic_types::{DoubleOrNilReason, NilReason};
use quick_xml::de;
use std::io::Write;

/// Name of the datatype used in error messages.
const DATATYPE: &str = "gml:doubleOrNilReason";

/// Reads a `gml:doubleOrNilReason` element.
///
/// See [`double_or_nil_reason_from_lexical`] for how the text is interpreted.
///
/// # Errors
///
/// Returns an error if the XML is not well-formed, or
/// [`Error::InvalidLexicalValue`] if the element has no text.
pub fn deserialize_double_or_nil_reason(xml_document: &[u8]) -> Result<DoubleOrNilReason, Error> {
    let text: String = de::from_reader(xml_document)?;

    double_or_nil_reason_from_lexical(&text)
}

/// Converts a `gml:doubleOrNilReason` lexical value.
///
/// Whitespace is collapsed. A valid `xs:double` (including `INF`, `-INF` and
/// `NaN`) becomes a [`DoubleOrNilReason::Double`]; any other text becomes a
/// [`DoubleOrNilReason::NilReason`], so `nan` or `1,5` are read as nil reasons.
///
/// # Errors
///
/// Returns [`Error::InvalidLexicalValue`] if the value is empty or whitespace only.
pub fn double_or_nil_reason_from_lexical(lexical: &str) -> Result<DoubleOrNilReason, Error> {
    let value = collapse_whitespace(lexical);
    if value.is_empty() {
        return Err(Error::InvalidLexicalValue {
            datatype: DATATYPE,
            value: lexical.to_owned(),
        });
    }

    Ok(match parse_double(&value) {
        Some(double) => DoubleOrNilReason::Double(double),
        None => DoubleOrNilReason::NilReason(NilReason::new(value)),
    })
}

/// Writes a `gml:doubleOrNilReason` element. Doubles use the canonical
/// `xs:double` special values (`INF`, `-INF`, `NaN`); nil reasons are
/// written as stored.
pub fn serialize_double_or_nil_reason<N: XmlNamespace, E: XmlElement, W: Write>(
    double_or_nil_reason: &DoubleOrNilReason,
    xml_fragment_writer: &mut XmlFragmentWriter<W>,
    target_xml_namespace: N,
    target_xml_element: E,
) -> Result<(), Error> {
    let text = match double_or_nil_reason {
        DoubleOrNilReason::Double(value) => format_double(*value),
        DoubleOrNilReason::NilReason(nil_reason) => nil_reason.value().to_owned(),
    };

    xml_fragment_writer.write_leaf_element(target_xml_namespace, target_xml_element, &text)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::{Formatting, GmlElement, GmlNamespace};

    /// Reads `text` as the content of a `gml:doubleOrNilReason` element.
    fn read(text: &str) -> Result<DoubleOrNilReason, Error> {
        let xml_document = format!("<app:height>{text}</app:height>");
        deserialize_double_or_nil_reason(xml_document.as_bytes())
    }

    fn nil(value: &str) -> DoubleOrNilReason {
        DoubleOrNilReason::NilReason(NilReason::new(value))
    }

    /// Writes `value` and returns the whole element.
    fn write(value: &DoubleOrNilReason) -> String {
        let mut writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_double_or_nil_reason(
            value,
            &mut writer,
            GmlNamespace::Gml,
            GmlElement::NameProperty,
        )
        .expect("should serialize");
        String::from_utf8(writer.into_bytes()).expect("valid UTF-8")
    }

    #[test]
    fn test_reads_doubles() {
        assert_eq!(read("12.5").unwrap(), DoubleOrNilReason::Double(12.5));
        assert_eq!(read("-1.5E3").unwrap(), DoubleOrNilReason::Double(-1500.0));
        assert_eq!(
            read("INF").unwrap(),
            DoubleOrNilReason::Double(f64::INFINITY)
        );
        assert!(read("NaN").unwrap().as_double().unwrap().is_nan());
    }

    #[test]
    fn test_reads_nil_reasons() {
        for value in [
            "missing",
            "inapplicable",
            "other:notMeasured",
            "http://example.org/nilReasons#sensorFailure",
        ] {
            assert_eq!(read(value).unwrap(), nil(value));
        }
    }

    #[test]
    fn test_reads_non_xsd_doubles_as_nil_reasons() {
        assert_eq!(read("nan").unwrap(), nil("nan"));
        assert_eq!(read("1,5").unwrap(), nil("1,5"));
    }

    #[test]
    fn test_collapses_whitespace() {
        assert_eq!(read("\n  12.5  ").unwrap(), DoubleOrNilReason::Double(12.5));
        assert_eq!(
            read(" other:not \t measured ").unwrap(),
            nil("other:not measured")
        );
    }

    #[test]
    fn test_rejects_empty_values() {
        for text in ["", "  \n\t "] {
            let result = read(text);
            assert!(
                matches!(&result, Err(Error::InvalidLexicalValue { datatype, .. }) if *datatype == DATATYPE),
                "{text:?} should be rejected, got {result:?}"
            );
        }
        assert!(deserialize_double_or_nil_reason(b"<app:height/>").is_err());
    }

    #[test]
    fn test_writes_element() {
        assert_eq!(
            write(&DoubleOrNilReason::Double(12.5)),
            "<gml:name>12.5</gml:name>"
        );
        assert_eq!(
            write(&DoubleOrNilReason::Double(f64::NEG_INFINITY)),
            "<gml:name>-INF</gml:name>"
        );
        assert_eq!(write(&nil("missing")), "<gml:name>missing</gml:name>");
    }

    #[test]
    fn test_round_trip_through_xml() {
        for value in [
            DoubleOrNilReason::Double(0.1),
            DoubleOrNilReason::Double(f64::INFINITY),
            DoubleOrNilReason::Double(f64::NEG_INFINITY),
            nil("withheld"),
            nil("http://example.org/a?b=1&c=2"),
        ] {
            let round_tripped =
                deserialize_double_or_nil_reason(write(&value).as_bytes()).expect("should work");
            assert_eq!(round_tripped, value);
        }

        let nan = DoubleOrNilReason::Double(f64::NAN);
        let round_tripped =
            deserialize_double_or_nil_reason(write(&nan).as_bytes()).expect("should work");
        assert!(round_tripped.as_double().unwrap().is_nan());
    }
}
