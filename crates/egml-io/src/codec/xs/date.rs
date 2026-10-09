use crate::Error;
use crate::codec::xs::lexical;
use crate::util::{DeserializationConfig, XmlElement, XmlFragmentWriter, XmlNamespace};
use chrono::NaiveDate;
use egml_core::model::xs::Date;
use quick_xml::de;
use std::io::Write;

/// Name of the datatype used in error messages.
const DATATYPE: &str = "xs:date";

/// Reads an `xs:date` element, assigning
/// [`DeserializationConfig::default_offset`] if the value has no timezone.
/// Surrounding whitespace is ignored.
///
/// # Errors
///
/// Returns [`Error::InvalidLexicalValue`] if the text is malformed, or a
/// wrapped core error if the default offset is invalid.
pub fn deserialize_date(
    xml_document: &[u8],
    config: &DeserializationConfig,
) -> Result<Date, Error> {
    let text: String = de::from_reader(xml_document)?;
    let invalid = || lexical::invalid_lexical_value(DATATYPE, &text);

    let (body, timezone) = lexical::split_timezone(text.trim());
    let date = NaiveDate::parse_from_str(body, lexical::DATE_FORMAT).map_err(|_| invalid())?;
    let offset =
        lexical::resolve_timezone(timezone, config.default_offset()).ok_or_else(invalid)?;

    Ok(Date::new(date, offset)?)
}

/// Writes an `xs:date` element in canonical form, e.g. `2020-01-02+01:00`.
pub fn serialize_date<N: XmlNamespace, E: XmlElement, W: Write>(
    date: Date,
    xml_fragment_writer: &mut XmlFragmentWriter<W>,
    target_xml_namespace: N,
    target_xml_element: E,
) -> Result<(), Error> {
    let text = format!(
        "{}{}",
        lexical::format_naive_date(date.date()),
        lexical::format_timezone(date.offset())
    );

    xml_fragment_writer.write_leaf_element(target_xml_namespace, target_xml_element, &text)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::{Formatting, GmlElement, GmlNamespace};
    use chrono::FixedOffset;

    fn hours_east(hours: i32) -> FixedOffset {
        FixedOffset::east_opt(hours * 3600).unwrap()
    }

    /// Reads `text` as the content of an `xs:date` element.
    fn read(text: &str, default_offset: FixedOffset) -> Result<Date, Error> {
        let xml_document = format!("<con:dateOfConstruction>{text}</con:dateOfConstruction>");
        let config = DeserializationConfig::default().with_default_offset(default_offset);
        deserialize_date(xml_document.as_bytes(), &config)
    }

    fn read_utc(text: &str) -> Date {
        read(text, hours_east(0)).expect("should deserialize")
    }

    /// Writes `date` and returns the element content.
    fn write(date: Date) -> String {
        let mut writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_date(
            date,
            &mut writer,
            GmlNamespace::Gml,
            GmlElement::NameProperty,
        )
        .expect("should serialize");
        let xml = String::from_utf8(writer.into_bytes()).expect("valid UTF-8");
        xml.strip_prefix("<gml:name>")
            .and_then(|x| x.strip_suffix("</gml:name>"))
            .expect("should be a gml:name element")
            .to_owned()
    }

    #[test]
    fn test_serialize_date_writes_element() {
        let mut writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_date(
            read_utc("2020-01-02+02:00"),
            &mut writer,
            GmlNamespace::Gml,
            GmlElement::NameProperty,
        )
        .expect("should serialize");

        assert_eq!(
            String::from_utf8(writer.into_bytes()).unwrap(),
            "<gml:name>2020-01-02+02:00</gml:name>"
        );
    }

    #[test]
    fn test_round_trips_lexical_form() {
        for text in ["2020-01-02Z", "2020-01-02-05:00", "-0044-03-15+01:00"] {
            assert_eq!(write(read_utc(text)), text);
        }
    }

    #[test]
    fn test_uses_default_offset_only_if_missing() {
        let implicit = read("2020-01-02", hours_east(1)).unwrap();
        let explicit = read("2020-01-02+02:00", hours_east(1)).unwrap();

        assert_eq!(write(implicit), "2020-01-02+01:00");
        assert_eq!(write(explicit), "2020-01-02+02:00");
    }

    #[test]
    fn test_defaults_to_utc() {
        let xml_document = b"<con:dateOfConstruction>2020-01-02</con:dateOfConstruction>";

        let date = deserialize_date(xml_document, &DeserializationConfig::default())
            .expect("should deserialize");

        assert_eq!(write(date), "2020-01-02Z");
    }

    #[test]
    fn test_is_lenient_on_field_widths() {
        assert_eq!(write(read_utc("2020-1-2Z")), "2020-01-02Z");
        assert_eq!(write(read_utc("20-01-02Z")), "0020-01-02Z");
    }

    #[test]
    fn test_rejects_invalid_values() {
        for text in [
            "",
            "not-a-date",
            "2020-01-02T00:00:00Z",
            "2020-13-01Z",
            "2020-01-02+14:30",
        ] {
            let result = read(text, hours_east(0));
            assert!(
                matches!(&result, Err(Error::InvalidLexicalValue { value, .. }) if value == text),
                "{text:?} should be rejected, got {result:?}"
            );
        }
    }

    #[test]
    fn test_round_trip_through_xml() {
        let xml_document = b"<con:dateOfDemolition>2021-06-15+02:00</con:dateOfDemolition>";
        let config = DeserializationConfig::default();

        let date = deserialize_date(xml_document, &config).expect("should deserialize");
        let xml = format!("<x>{}</x>", write(date));
        let round_tripped = deserialize_date(xml.as_bytes(), &config).expect("should deserialize");

        assert_eq!(date, round_tripped);
    }
}
