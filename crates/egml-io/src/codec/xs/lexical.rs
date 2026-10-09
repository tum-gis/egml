//! Helpers shared by the `xs:date` and `xs:dateTime` codecs for their lexical
//! forms ([XSD 1.1 Part 2 §D.2](https://www.w3.org/TR/xmlschema11-2/#dt-dt-7PropMod)).
//!
//! Date and time fields are parsed by chrono, which is more lenient than XSD
//! (e.g. it accepts single-digit months) but rejects years with more than four
//! digits. Output always follows the canonical XSD form.

use crate::Error;
use chrono::{Datelike, FixedOffset, NaiveDate};

/// chrono format of an `xs:date` without its timezone.
pub(super) const DATE_FORMAT: &str = "%Y-%m-%d";
/// Largest timezone allowed by the lexical grammar, in minutes (14:00).
const MAX_TIMEZONE_MINUTES: u32 = 14 * 60;

pub(super) fn invalid_lexical_value(datatype: &'static str, lexical: &str) -> Error {
    Error::InvalidLexicalValue {
        datatype,
        value: lexical.to_owned(),
    }
}

/// Splits a trailing timezone (`Z` or `±hh:mm`) off a lexical value.
pub(super) fn split_timezone(lexical: &str) -> (&str, Option<&str>) {
    if let Some(body) = lexical.strip_suffix('Z') {
        return (body, Some("Z"));
    }

    let bytes = lexical.as_bytes();
    let has_numeric_timezone = bytes.len() >= 6
        && matches!(bytes[bytes.len() - 6], b'+' | b'-')
        && bytes[bytes.len() - 3] == b':';
    if has_numeric_timezone {
        let (body, timezone) = lexical.split_at(lexical.len() - 6);
        return (body, Some(timezone));
    }

    (lexical, None)
}

/// Uses the explicit timezone if present, otherwise `default_offset`.
/// Returns `None` if the explicit timezone is malformed.
pub(super) fn resolve_timezone(
    timezone: Option<&str>,
    default_offset: FixedOffset,
) -> Option<FixedOffset> {
    match timezone {
        Some(timezone) => parse_timezone(timezone),
        None => Some(default_offset),
    }
}

/// Writes `yyyy-mm-dd`, with a leading `-` for negative years.
pub(super) fn format_naive_date(date: NaiveDate) -> String {
    let year = date.year();
    let sign = if year < 0 { "-" } else { "" };
    format!(
        "{sign}{:04}-{:02}-{:02}",
        year.unsigned_abs(),
        date.month(),
        date.day()
    )
}

/// Writes `Z` for UTC and `±hh:mm` for other offsets.
pub(super) fn format_timezone(offset: FixedOffset) -> String {
    let seconds = offset.local_minus_utc();
    if seconds == 0 {
        return "Z".to_owned();
    }

    let sign = if seconds < 0 { '-' } else { '+' };
    let minutes = seconds.unsigned_abs() / 60;
    format!("{sign}{:02}:{:02}", minutes / 60, minutes % 60)
}

/// Parses `Z` or `±hh:mm` within the lexical range `-14:00..=+14:00`.
fn parse_timezone(timezone: &str) -> Option<FixedOffset> {
    if timezone == "Z" {
        return FixedOffset::east_opt(0);
    }

    let (sign, rest) = match timezone.split_at_checked(1)? {
        ("+", rest) => (1, rest),
        ("-", rest) => (-1, rest),
        _ => return None,
    };
    let (hours, minutes) = rest.split_once(':')?;
    let hours = parse_fixed_digits(hours, 2)?;
    let minutes = parse_fixed_digits(minutes, 2)?;
    let total_minutes = hours * 60 + minutes;
    if minutes > 59 || total_minutes > MAX_TIMEZONE_MINUTES {
        return None;
    }

    FixedOffset::east_opt(sign * (total_minutes * 60) as i32)
}

/// Parses exactly `width` ASCII digits.
fn parse_fixed_digits(digits: &str, width: usize) -> Option<u32> {
    (digits.len() == width && digits.bytes().all(|x| x.is_ascii_digit()))
        .then(|| digits.parse().ok())
        .flatten()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_timezone() {
        assert_eq!(split_timezone("2020-01-02"), ("2020-01-02", None));
        assert_eq!(split_timezone("2020-01-02Z"), ("2020-01-02", Some("Z")));
        assert_eq!(
            split_timezone("2020-01-02-05:00"),
            ("2020-01-02", Some("-05:00"))
        );
        assert_eq!(
            split_timezone("2020-01-02T03:04:05+02:00"),
            ("2020-01-02T03:04:05", Some("+02:00"))
        );
        assert_eq!(
            split_timezone("2020-01-02T03:04:05"),
            ("2020-01-02T03:04:05", None)
        );
    }

    #[test]
    fn test_resolve_timezone() {
        let default_offset = FixedOffset::east_opt(3_600).unwrap();

        assert_eq!(resolve_timezone(None, default_offset), Some(default_offset));
        assert_eq!(
            resolve_timezone(Some("Z"), default_offset),
            FixedOffset::east_opt(0)
        );
        assert_eq!(resolve_timezone(Some("+0200"), default_offset), None);
    }

    #[test]
    fn test_parse_timezone() {
        assert_eq!(parse_timezone("Z"), FixedOffset::east_opt(0));
        assert_eq!(parse_timezone("+02:30"), FixedOffset::east_opt(9_000));
        assert_eq!(parse_timezone("-14:00"), FixedOffset::west_opt(50_400));
        assert_eq!(parse_timezone("+14:01"), None);
        assert_eq!(parse_timezone("+02:60"), None);
        assert_eq!(parse_timezone("+0200"), None);
        assert_eq!(parse_timezone("+2:00"), None);
    }

    #[test]
    fn test_format_naive_date() {
        let date = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();

        assert_eq!(format_naive_date(date(2020, 1, 2)), "2020-01-02");
        assert_eq!(format_naive_date(date(20, 1, 2)), "0020-01-02");
        assert_eq!(format_naive_date(date(-44, 3, 15)), "-0044-03-15");
    }

    #[test]
    fn test_format_timezone() {
        let east = |seconds| FixedOffset::east_opt(seconds).unwrap();

        assert_eq!(format_timezone(east(0)), "Z");
        assert_eq!(format_timezone(east(9_000)), "+02:30");
        assert_eq!(format_timezone(east(-19_800)), "-05:30");
    }
}
