//! Timezone offset rules shared by [`Date`](super::Date) and
//! [`DateTime`](super::DateTime).

use crate::Error;
use chrono::{FixedOffset, NaiveDateTime};

/// Largest timezone offset magnitude permitted by XSD (14:00).
const MAX_OFFSET_SECONDS: i32 = 14 * 3600;

/// Returns `offset` unchanged if XSD can represent it.
///
/// # Errors
///
/// Returns [`Error::InvalidTimezoneOffset`] if it is not a whole number of
/// minutes within ±14:00.
pub(super) fn validate_offset(offset: FixedOffset) -> Result<FixedOffset, Error> {
    let seconds = offset.local_minus_utc();
    if seconds.abs() <= MAX_OFFSET_SECONDS && seconds % 60 == 0 {
        Ok(offset)
    } else {
        Err(Error::InvalidTimezoneOffset { seconds })
    }
}

/// Converts seconds east of UTC into a validated offset.
///
/// # Errors
///
/// Returns [`Error::InvalidTimezoneOffset`] if `seconds` is not a whole number
/// of minutes within ±14:00.
pub(super) fn offset_from_seconds(seconds: i32) -> Result<FixedOffset, Error> {
    let offset = FixedOffset::east_opt(seconds).ok_or(Error::InvalidTimezoneOffset { seconds })?;
    validate_offset(offset)
}

/// Attaches `offset` to a local date-time.
///
/// # Errors
///
/// Returns [`Error::DateTimeOutOfRange`] if the corresponding UTC value is
/// not representable.
pub(super) fn localize(
    local: NaiveDateTime,
    offset: FixedOffset,
) -> Result<chrono::DateTime<FixedOffset>, Error> {
    local
        .and_local_timezone(offset)
        .single()
        .ok_or_else(|| Error::DateTimeOutOfRange {
            value: format!("{local} {offset}"),
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, NaiveTime};

    fn seconds_east(seconds: i32) -> FixedOffset {
        FixedOffset::east_opt(seconds).unwrap()
    }

    #[test]
    fn test_validate_offset_accepts_xsd_range() {
        assert!(validate_offset(seconds_east(14 * 3600)).is_ok());
        assert!(validate_offset(seconds_east(-14 * 3600)).is_ok());
    }

    #[test]
    fn test_validate_offset_rejects_beyond_fourteen_hours() {
        assert_eq!(
            validate_offset(seconds_east(15 * 3600)),
            Err(Error::InvalidTimezoneOffset { seconds: 15 * 3600 })
        );
    }

    #[test]
    fn test_validate_offset_rejects_seconds() {
        assert_eq!(
            validate_offset(seconds_east(61)),
            Err(Error::InvalidTimezoneOffset { seconds: 61 })
        );
    }

    #[test]
    fn test_offset_from_seconds() {
        assert_eq!(offset_from_seconds(-3_600), Ok(seconds_east(-3_600)));
        assert_eq!(
            offset_from_seconds(100_000),
            Err(Error::InvalidTimezoneOffset { seconds: 100_000 })
        );
    }

    #[test]
    fn test_localize_fails_at_range_edge() {
        let local = NaiveDate::MIN.and_time(NaiveTime::MIN);

        assert!(localize(local, seconds_east(3_600)).is_err());
    }
}
