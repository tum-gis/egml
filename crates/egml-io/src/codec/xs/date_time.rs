use crate::Error;
use crate::codec::xs::lexical;
use crate::util::{DeserializationConfig, XmlElement, XmlFragmentWriter, XmlNamespace};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime, Timelike};
use egml_core::model::xs::DateTime;
use quick_xml::de;
use std::io::Write;

/// Name of the datatype used in error messages.
const DATATYPE: &str = "xs:dateTime";
/// chrono format of an `xs:dateTime` without its timezone.
const DATE_TIME_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.f";
/// Time of day that XSD accepts for midnight of the following day.
const END_OF_DAY: &str = "24:00:00";
/// Number of fractional-second digits representable as nanoseconds.
const NANOSECOND_DIGITS: usize = 9;
/// chrono marks a leap second by nanosecond values at or above this.
const LEAP_SECOND_NANOSECONDS: u32 = 1_000_000_000;

/// Reads an `xs:dateTime` element, assigning
/// [`DeserializationConfig::default_offset`] if the value has no timezone.
///
/// Surrounding whitespace is ignored, `24:00:00` is read as midnight of the
/// following day, and fractional seconds beyond nanoseconds are truncated.
/// Leap seconds (`23:59:60`) are accepted as in XSD 1.0, which GML schemas use.
///
/// # Errors
///
/// Returns [`Error::InvalidLexicalValue`] if the text is malformed, or a
/// wrapped core error if the default offset is invalid.
pub fn deserialize_date_time(
    xml_document: &[u8],
    config: &DeserializationConfig,
) -> Result<DateTime, Error> {
    let text: String = de::from_reader(xml_document)?;
    let invalid = || lexical::invalid_lexical_value(DATATYPE, &text);

    let (body, timezone) = lexical::split_timezone(text.trim());
    let local = parse_naive_date_time(body).ok_or_else(invalid)?;
    let offset =
        lexical::resolve_timezone(timezone, config.default_offset()).ok_or_else(invalid)?;

    Ok(DateTime::from_local(local, offset)?)
}

/// Writes an `xs:dateTime` element in canonical form, using `Z` for UTC and
/// omitting trailing zeros of fractional seconds.
pub fn serialize_date_time<N: XmlNamespace, E: XmlElement, W: Write>(
    date_time: DateTime,
    xml_fragment_writer: &mut XmlFragmentWriter<W>,
    target_xml_namespace: N,
    target_xml_element: E,
) -> Result<(), Error> {
    let local = date_time.local();
    let text = format!(
        "{}T{}{}",
        lexical::format_naive_date(local.date()),
        format_time(local.time()),
        lexical::format_timezone(date_time.offset())
    );

    xml_fragment_writer.write_leaf_element(target_xml_namespace, target_xml_element, &text)?;

    Ok(())
}

/// Parses a date-time without timezone, adding the XSD `24:00:00` form.
fn parse_naive_date_time(lexical: &str) -> Option<NaiveDateTime> {
    if let Some(end_of_day) = parse_end_of_day(lexical) {
        return Some(end_of_day);
    }

    NaiveDateTime::parse_from_str(lexical, DATE_TIME_FORMAT).ok()
}

/// Parses `yyyy-mm-ddT24:00:00(.0+)?` as midnight of the following day.
fn parse_end_of_day(lexical: &str) -> Option<NaiveDateTime> {
    let (date, time) = lexical.split_once('T')?;
    let fraction = time.strip_prefix(END_OF_DAY)?;
    let has_zero_fraction = fraction.is_empty()
        || fraction
            .strip_prefix('.')
            .is_some_and(|x| !x.is_empty() && x.bytes().all(|b| b == b'0'));
    if !has_zero_fraction {
        return None;
    }

    NaiveDate::parse_from_str(date, lexical::DATE_FORMAT)
        .ok()?
        .succ_opt()
        .map(|x| x.and_time(NaiveTime::MIN))
}

/// Writes `hh:mm:ss` followed by the fractional seconds without trailing
/// zeros. A leap second, which chrono stores as second 59 with nanoseconds
/// of one second or more, is written as second 60.
fn format_time(time: NaiveTime) -> String {
    let is_leap_second = time.nanosecond() >= LEAP_SECOND_NANOSECONDS;
    let (second, nanoseconds) = if is_leap_second {
        (
            time.second() + 1,
            time.nanosecond() - LEAP_SECOND_NANOSECONDS,
        )
    } else {
        (time.second(), time.nanosecond())
    };
    let fraction = if nanoseconds == 0 {
        String::new()
    } else {
        let digits = format!("{nanoseconds:0width$}", width = NANOSECOND_DIGITS);
        format!(".{}", digits.trim_end_matches('0'))
    };

    format!(
        "{:02}:{:02}:{second:02}{fraction}",
        time.hour(),
        time.minute()
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::{Formatting, GmlElement, GmlNamespace};
    use chrono::FixedOffset;

    fn hours_east(hours: i32) -> FixedOffset {
        FixedOffset::east_opt(hours * 3600).unwrap()
    }

    fn naive(y: i32, mo: u32, d: u32, h: u32, mi: u32, s: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(y, mo, d)
            .unwrap()
            .and_hms_opt(h, mi, s)
            .unwrap()
    }

    /// Reads `text` as the content of an `xs:dateTime` element.
    fn read(text: &str, default_offset: FixedOffset) -> Result<DateTime, Error> {
        let xml_document = format!("<creationDate>{text}</creationDate>");
        let config = DeserializationConfig::default().with_default_offset(default_offset);
        deserialize_date_time(xml_document.as_bytes(), &config)
    }

    fn read_utc(text: &str) -> DateTime {
        read(text, hours_east(0)).expect("should deserialize")
    }

    /// Writes `date_time` and returns the element content.
    fn write(date_time: DateTime) -> String {
        let mut writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_date_time(
            date_time,
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
    fn test_serialize_date_time_writes_element() {
        let mut writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_date_time(
            read_utc("2020-01-02T03:04:05+02:00"),
            &mut writer,
            GmlNamespace::Gml,
            GmlElement::NameProperty,
        )
        .expect("should serialize");

        assert_eq!(
            String::from_utf8(writer.into_bytes()).unwrap(),
            "<gml:name>2020-01-02T03:04:05+02:00</gml:name>"
        );
    }

    #[test]
    fn test_round_trips_lexical_form() {
        for text in [
            "2020-01-02T03:04:05+02:00",
            "2020-01-02T03:04:05-05:30",
            "2020-01-02T03:04:05Z",
            "2020-01-02T03:04:05.123Z",
            "-0044-03-15T12:00:00Z",
            "2016-12-31T23:59:60Z",
            "2017-01-01T00:59:60.5+01:00",
        ] {
            assert_eq!(write(read_utc(text)), text);
        }
    }

    #[test]
    fn test_writes_canonical_forms() {
        assert_eq!(
            write(read_utc("2020-01-02T03:04:05+00:00")),
            "2020-01-02T03:04:05Z"
        );
        assert_eq!(
            write(read_utc("2020-01-02T03:04:05.500Z")),
            "2020-01-02T03:04:05.5Z"
        );
        assert_eq!(write(read_utc("2020-1-2T3:4:5Z")), "2020-01-02T03:04:05Z");
    }

    #[test]
    fn test_reads_end_of_day_as_next_midnight() {
        let next_midnight = naive(2021, 1, 1, 0, 0, 0);

        assert_eq!(read_utc("2020-12-31T24:00:00Z").local(), next_midnight);
        assert_eq!(read_utc("2020-12-31T24:00:00.000Z").local(), next_midnight);
    }

    #[test]
    fn test_reads_leap_second() {
        let leap_second = NaiveDate::from_ymd_opt(2016, 12, 31)
            .unwrap()
            .and_hms_milli_opt(23, 59, 59, 1_500)
            .unwrap();

        assert_eq!(read_utc("2016-12-31T23:59:60.5Z").local(), leap_second);
    }

    #[test]
    fn test_trims_whitespace() {
        assert_eq!(
            read_utc("  2020-01-02T03:04:05Z\n").local(),
            naive(2020, 1, 2, 3, 4, 5)
        );
    }

    #[test]
    fn test_uses_default_offset_only_if_missing() {
        let implicit = read("2020-01-02T03:04:05", hours_east(1)).unwrap();
        let explicit = read("2020-01-02T03:04:05-05:00", hours_east(1)).unwrap();

        assert_eq!(write(implicit), "2020-01-02T03:04:05+01:00");
        assert_eq!(write(explicit), "2020-01-02T03:04:05-05:00");
    }

    #[test]
    fn test_rejects_invalid_values() {
        for text in [
            "",
            "not-a-date",
            "2020-01-02Z",
            "2020-01-02 03:04:05Z",
            "2020-01-02T03:04Z",
            "2020-01-02T25:00:00Z",
            "2020-01-02T24:00:00.5Z",
            "2020-01-02T24:00:01Z",
            "2020-02-30T00:00:00Z",
            "2020-01-02T03:04:05+0200",
            "2020-01-02T03:04:05+15:00",
            "2016-12-31T23:59:61Z",
            "12345-01-01T00:00:00Z",
        ] {
            let result = read(text, hours_east(0));
            assert!(
                matches!(&result, Err(Error::InvalidLexicalValue { value, .. }) if value == text),
                "{text:?} should be rejected, got {result:?}"
            );
        }
    }

    #[test]
    fn test_reports_invalid_default_offset() {
        let result = read("2020-01-02T03:04:05", hours_east(15));

        assert!(matches!(
            result,
            Err(Error::EgmlError(
                egml_core::Error::InvalidTimezoneOffset { .. }
            ))
        ));
    }

    #[test]
    fn test_format_time_writes_leap_second() {
        let leap_second = NaiveTime::from_hms_milli_opt(23, 59, 59, 1_250).unwrap();

        assert_eq!(format_time(leap_second), "23:59:60.25");
    }
}
