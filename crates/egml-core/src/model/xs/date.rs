use crate::Error;
use crate::model::xs::{DateTime, offset};
use chrono::{Datelike, FixedOffset, NaiveDate, NaiveTime};

/// A calendar date with a timezone.
///
/// Corresponds to `xs:date` in [XSD 1.1 Part 2 §3.3.9](https://www.w3.org/TR/xmlschema11-2/#date).
/// XSD makes the timezone optional, but this type always carries one.
///
/// Equality, ordering and hashing compare the first instant of each date, so
/// `2020-01-02+12:00 == 2020-01-01-12:00`.
///
/// # Examples
///
/// ```rust
/// use chrono::{FixedOffset, NaiveDate};
/// use egml_core::model::xs::Date;
///
/// let day = NaiveDate::from_ymd_opt(2020, 1, 2).unwrap();
/// let utc = FixedOffset::east_opt(0).unwrap();
///
/// let date = Date::new(day, utc).unwrap();
/// assert_eq!(date.date(), day);
/// assert_eq!(date.start_of_day().local(), day.and_hms_opt(0, 0, 0).unwrap());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Date {
    /// Midnight at the start of the date, in the date's timezone.
    start: DateTime,
}

impl Date {
    /// Creates a date from its calendar value and offset east of UTC.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidTimezoneOffset`] if `offset` is not a whole
    /// number of minutes within `-14:00..=+14:00`, or
    /// [`Error::DateTimeOutOfRange`] if the date is not representable.
    pub fn new(date: NaiveDate, offset: FixedOffset) -> Result<Self, Error> {
        let start = DateTime::from_local(date.and_time(NaiveTime::MIN), offset)?;
        Ok(Self { start })
    }

    /// Creates a date from its calendar fields and offset, without chrono types.
    ///
    /// `offset_seconds` is the offset east of UTC (local minus UTC).
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidDateTimeFields`] if the fields do not form a
    /// valid date (e.g. February 30), [`Error::InvalidTimezoneOffset`] if
    /// `offset_seconds` is not a whole number of minutes within
    /// `-14:00..=+14:00`, or [`Error::DateTimeOutOfRange`] if the date is not
    /// representable.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use egml_core::model::xs::Date;
    ///
    /// let date = Date::from_parts(2020, 1, 2, 3_600).unwrap();
    /// assert_eq!(date.day(), 2);
    /// assert_eq!(date.offset_seconds(), 3_600);
    ///
    /// assert!(Date::from_parts(2021, 2, 29, 0).is_err());
    /// ```
    pub fn from_parts(year: i32, month: u32, day: u32, offset_seconds: i32) -> Result<Self, Error> {
        let date = NaiveDate::from_ymd_opt(year, month, day).ok_or_else(|| {
            Error::InvalidDateTimeFields {
                value: format!("year={year}, month={month}, day={day}"),
            }
        })?;
        let offset = offset::offset_from_seconds(offset_seconds)?;
        Self::new(date, offset)
    }

    /// Returns the calendar date.
    pub fn date(&self) -> NaiveDate {
        self.start.local().date()
    }

    /// Returns the offset east of UTC.
    pub fn offset(&self) -> FixedOffset {
        self.start.offset()
    }

    /// Returns midnight at the start of this date, in its timezone.
    pub fn start_of_day(&self) -> DateTime {
        self.start
    }

    /// Returns the year; negative for years before 1 CE, with 0 for 1 BCE
    /// (proleptic Gregorian, as in XSD 1.1).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use egml_core::model::xs::Date;
    ///
    /// let date = Date::from_parts(1969, 12, 31, 0).unwrap();
    /// assert_eq!(date.year(), 1969);
    /// ```
    pub fn year(&self) -> i32 {
        self.date().year()
    }

    /// Returns the month, starting from 1.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use egml_core::model::xs::Date;
    ///
    /// let date = Date::from_parts(1969, 12, 31, 0).unwrap();
    /// assert_eq!(date.month(), 12);
    /// ```
    pub fn month(&self) -> u32 {
        self.date().month()
    }

    /// Returns the day of the month, starting from 1.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use egml_core::model::xs::Date;
    ///
    /// let date = Date::from_parts(1969, 12, 31, 0).unwrap();
    /// assert_eq!(date.day(), 31);
    /// ```
    pub fn day(&self) -> u32 {
        self.date().day()
    }

    /// Returns the offset east of UTC in seconds (local minus UTC).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use egml_core::model::xs::Date;
    ///
    /// let date = Date::from_parts(2020, 1, 2, -5 * 3_600).unwrap();
    /// assert_eq!(date.offset_seconds(), -18_000);
    /// ```
    pub fn offset_seconds(&self) -> i32 {
        self.offset().local_minus_utc()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    fn hash_of(value: &Date) -> u64 {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish()
    }

    fn hours_east(hours: i32) -> FixedOffset {
        FixedOffset::east_opt(hours * 3600).unwrap()
    }

    fn date(y: i32, m: u32, d: u32, hours: i32) -> Date {
        Date::new(NaiveDate::from_ymd_opt(y, m, d).unwrap(), hours_east(hours))
            .expect("should be valid")
    }

    #[test]
    fn test_new() {
        let value = date(2020, 1, 2, 2);

        assert_eq!(value.date(), NaiveDate::from_ymd_opt(2020, 1, 2).unwrap());
        assert_eq!(value.offset(), hours_east(2));
    }

    #[test]
    fn test_new_rejects_invalid_offset() {
        let day = NaiveDate::from_ymd_opt(2020, 1, 2).unwrap();

        assert_eq!(
            Date::new(day, hours_east(-15)),
            Err(Error::InvalidTimezoneOffset {
                seconds: -15 * 3600
            })
        );
    }

    #[test]
    fn test_start_of_day() {
        let start = date(2020, 1, 2, 2).start_of_day();

        assert_eq!(
            start.local(),
            NaiveDate::from_ymd_opt(2020, 1, 2)
                .unwrap()
                .and_time(NaiveTime::MIN)
        );
        assert_eq!(start.offset(), hours_east(2));
    }

    #[test]
    fn test_equality_compares_starting_instants() {
        let a = date(2020, 1, 2, 12);
        let b = date(2020, 1, 1, -12);

        assert_eq!(a, b);
        assert_eq!(hash_of(&a), hash_of(&b));
    }

    #[test]
    fn test_ordering() {
        assert!(date(2020, 1, 2, 0) < date(2020, 1, 3, 0));
        assert!(date(2020, 1, 2, 2) < date(2020, 1, 2, 0));
    }

    #[test]
    fn test_getters() {
        let value = Date::from_parts(2020, 1, 2, -5 * 3600).expect("should be valid");

        assert_eq!((value.year(), value.month(), value.day()), (2020, 1, 2));
        assert_eq!(value.offset_seconds(), -5 * 3600);
    }

    #[test]
    fn test_getters_before_1970() {
        let late_1969 = Date::from_parts(1969, 12, 31, 3600).expect("should be valid");
        let ancient = Date::from_parts(-44, 3, 15, 0).expect("should be valid");

        assert_eq!(
            (late_1969.year(), late_1969.month(), late_1969.day()),
            (1969, 12, 31)
        );
        assert_eq!(
            (ancient.year(), ancient.month(), ancient.day()),
            (-44, 3, 15)
        );
    }

    #[test]
    fn test_from_parts_matches_new() {
        assert_eq!(
            Date::from_parts(2020, 1, 2, 2 * 3600),
            Ok(date(2020, 1, 2, 2))
        );
    }

    #[test]
    fn test_from_parts_round_trips_through_getters() {
        let value = Date::from_parts(2020, 2, 29, -(9 * 3600 + 30 * 60)).expect("should be valid");

        let rebuilt = Date::from_parts(
            value.year(),
            value.month(),
            value.day(),
            value.offset_seconds(),
        )
        .expect("should be valid");

        assert_eq!(rebuilt, value);
        assert_eq!(rebuilt.offset_seconds(), value.offset_seconds());
    }

    #[test]
    fn test_from_parts_rejects_invalid_fields() {
        for (year, month, day) in [(2020, 2, 30), (2021, 2, 29), (2020, 13, 1), (2020, 1, 0)] {
            assert!(
                matches!(
                    Date::from_parts(year, month, day, 0),
                    Err(Error::InvalidDateTimeFields { .. })
                ),
                "{year}-{month}-{day} should be rejected"
            );
        }
    }

    #[test]
    fn test_from_parts_rejects_invalid_offsets() {
        for offset_seconds in [15 * 3600, 30, 100_000] {
            assert_eq!(
                Date::from_parts(2020, 1, 2, offset_seconds),
                Err(Error::InvalidTimezoneOffset {
                    seconds: offset_seconds
                })
            );
        }
    }

    #[test]
    fn test_from_parts_compares_starting_instants() {
        let a = Date::from_parts(2020, 1, 2, 12 * 3600).expect("should be valid");
        let b = Date::from_parts(2020, 1, 1, -12 * 3600).expect("should be valid");
        let later = Date::from_parts(2020, 1, 2, 0).expect("should be valid");

        assert_eq!(a, b);
        assert_eq!(hash_of(&a), hash_of(&b));
        assert!(a < later);
    }
}
