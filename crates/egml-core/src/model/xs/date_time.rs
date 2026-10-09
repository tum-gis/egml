use crate::Error;
use crate::model::xs::offset;
use chrono::{Datelike, FixedOffset, NaiveDate, NaiveDateTime, Timelike, Utc};

/// A date and time of day with a timezone.
///
/// Corresponds to `xs:dateTime` in [XSD 1.1 Part 2 §3.3.7](https://www.w3.org/TR/xmlschema11-2/#dateTime).
/// XSD makes the timezone optional, but this type always carries one.
///
/// Equality, ordering and hashing compare instants, so
/// `03:04:05+02:00 == 01:04:05Z`.
///
/// # Examples
///
/// ```rust
/// use chrono::{FixedOffset, NaiveDate};
/// use egml_core::model::xs::DateTime;
///
/// let local = NaiveDate::from_ymd_opt(2020, 1, 2)
///     .unwrap()
///     .and_hms_opt(3, 4, 5)
///     .unwrap();
/// let cet = FixedOffset::east_opt(3_600).unwrap();
///
/// let date_time = DateTime::from_local(local, cet).unwrap();
/// assert_eq!(date_time.local(), local);
/// assert_eq!(date_time.offset(), cet);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DateTime {
    value: chrono::DateTime<FixedOffset>,
}

impl DateTime {
    /// Wraps a [`chrono::DateTime`].
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidTimezoneOffset`] if the offset is not a whole
    /// number of minutes within `-14:00..=+14:00`.
    pub fn new(value: chrono::DateTime<FixedOffset>) -> Result<Self, Error> {
        offset::validate_offset(*value.offset())?;
        Ok(Self { value })
    }

    /// Creates a date-time from its local wall-clock value and offset east of UTC.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidTimezoneOffset`] if `offset` is invalid, or
    /// [`Error::DateTimeOutOfRange`] if the value is not representable.
    pub fn from_local(local: NaiveDateTime, offset: FixedOffset) -> Result<Self, Error> {
        let offset = offset::validate_offset(offset)?;
        let value = offset::localize(local, offset)?;
        Ok(Self { value })
    }

    /// Creates a date-time from its local fields and offset, without chrono
    /// types.
    ///
    /// `offset_seconds` is the offset east of UTC (local minus UTC). A leap
    /// second is given as `second == 59` with `nanosecond` of one second or
    /// more, as in chrono.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidDateTimeFields`] if the fields do not form a
    /// valid date and time (e.g. February 30 or hour 24),
    /// [`Error::InvalidTimezoneOffset`] if `offset_seconds` is not a whole
    /// number of minutes within `-14:00..=+14:00`, or
    /// [`Error::DateTimeOutOfRange`] if the value is not representable.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use egml_core::model::xs::DateTime;
    ///
    /// let date_time = DateTime::from_parts(2020, 1, 2, 3, 4, 5, 0, 3_600).unwrap();
    /// assert_eq!(date_time.hour(), 3);
    /// assert_eq!(date_time.offset_seconds(), 3_600);
    ///
    /// assert!(DateTime::from_parts(2020, 2, 30, 0, 0, 0, 0, 0).is_err());
    /// ```
    // The flat signature lets language bindings map this method one to one.
    #[allow(clippy::too_many_arguments)]
    pub fn from_parts(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
        nanosecond: u32,
        offset_seconds: i32,
    ) -> Result<Self, Error> {
        let local = NaiveDate::from_ymd_opt(year, month, day)
            .and_then(|x| x.and_hms_nano_opt(hour, minute, second, nanosecond))
            .ok_or_else(|| Error::InvalidDateTimeFields {
                value: format!(
                    "year={year}, month={month}, day={day}, hour={hour}, minute={minute}, \
                     second={second}, nanosecond={nanosecond}"
                ),
            })?;
        let offset = offset::offset_from_seconds(offset_seconds)?;
        Self::from_local(local, offset)
    }

    /// Returns the wrapped [`chrono::DateTime`].
    pub fn as_chrono(&self) -> &chrono::DateTime<FixedOffset> {
        &self.value
    }

    /// Returns the local wall-clock date and time.
    pub fn local(&self) -> NaiveDateTime {
        self.value.naive_local()
    }

    /// Returns the offset east of UTC.
    pub fn offset(&self) -> FixedOffset {
        *self.value.offset()
    }

    /// Returns the local year; negative for years before 1 CE, with 0 for
    /// 1 BCE (proleptic Gregorian, as in XSD 1.1).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use egml_core::model::xs::DateTime;
    ///
    /// let date_time = DateTime::from_parts(-44, 3, 15, 12, 0, 0, 0, 0).unwrap();
    /// assert_eq!(date_time.year(), -44);
    /// ```
    pub fn year(&self) -> i32 {
        self.value.year()
    }

    /// Returns the local month, starting from 1.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use egml_core::model::xs::DateTime;
    ///
    /// let date_time = DateTime::from_parts(2020, 12, 31, 0, 0, 0, 0, 0).unwrap();
    /// assert_eq!(date_time.month(), 12);
    /// ```
    pub fn month(&self) -> u32 {
        self.value.month()
    }

    /// Returns the local day of the month, starting from 1.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use egml_core::model::xs::DateTime;
    ///
    /// let date_time = DateTime::from_parts(2020, 12, 31, 0, 0, 0, 0, 0).unwrap();
    /// assert_eq!(date_time.day(), 31);
    /// ```
    pub fn day(&self) -> u32 {
        self.value.day()
    }

    /// Returns the local hour, from 0 to 23.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use egml_core::model::xs::DateTime;
    ///
    /// // 23:30 at -05:00 stays 23 locally, although it is the next day in UTC.
    /// let date_time = DateTime::from_parts(2020, 1, 2, 23, 30, 0, 0, -5 * 3_600).unwrap();
    /// assert_eq!(date_time.hour(), 23);
    /// ```
    pub fn hour(&self) -> u32 {
        self.value.hour()
    }

    /// Returns the local minute, from 0 to 59.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use egml_core::model::xs::DateTime;
    ///
    /// let date_time = DateTime::from_parts(2020, 1, 2, 3, 4, 5, 0, 0).unwrap();
    /// assert_eq!(date_time.minute(), 4);
    /// ```
    pub fn minute(&self) -> u32 {
        self.value.minute()
    }

    /// Returns the local second, from 0 to 59. A leap second reports 59; see
    /// [`DateTime::nanosecond`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use egml_core::model::xs::DateTime;
    ///
    /// let date_time = DateTime::from_parts(2020, 1, 2, 3, 4, 5, 0, 0).unwrap();
    /// assert_eq!(date_time.second(), 5);
    /// ```
    pub fn second(&self) -> u32 {
        self.value.second()
    }

    /// Returns the fraction of the second in nanoseconds. Values of one second
    /// or more (up to 1 999 999 999) mark a leap second.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use egml_core::model::xs::DateTime;
    ///
    /// let date_time = DateTime::from_parts(2020, 1, 2, 3, 4, 5, 250_000_000, 0).unwrap();
    /// assert_eq!(date_time.nanosecond(), 250_000_000);
    ///
    /// let leap_second = DateTime::from_parts(2016, 12, 31, 23, 59, 59, 1_000_000_000, 0).unwrap();
    /// assert_eq!(leap_second.nanosecond(), 1_000_000_000);
    /// ```
    pub fn nanosecond(&self) -> u32 {
        self.value.nanosecond()
    }

    /// Returns the offset east of UTC in seconds (local minus UTC).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use egml_core::model::xs::DateTime;
    ///
    /// let date_time = DateTime::from_parts(2020, 1, 2, 3, 4, 5, 0, -5 * 3_600).unwrap();
    /// assert_eq!(date_time.offset_seconds(), -18_000);
    /// ```
    pub fn offset_seconds(&self) -> i32 {
        self.offset().local_minus_utc()
    }
}

impl From<chrono::DateTime<Utc>> for DateTime {
    fn from(value: chrono::DateTime<Utc>) -> Self {
        Self {
            value: value.fixed_offset(),
        }
    }
}

impl TryFrom<chrono::DateTime<FixedOffset>> for DateTime {
    type Error = Error;

    /// # Errors
    ///
    /// Returns [`Error::InvalidTimezoneOffset`] if the offset lies outside the
    /// range permitted by XSD.
    fn try_from(value: chrono::DateTime<FixedOffset>) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<DateTime> for chrono::DateTime<FixedOffset> {
    fn from(value: DateTime) -> Self {
        value.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, TimeZone};
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    fn hash_of(value: &DateTime) -> u64 {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish()
    }

    fn naive(y: i32, mo: u32, d: u32, h: u32, mi: u32, s: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(y, mo, d)
            .unwrap()
            .and_hms_opt(h, mi, s)
            .unwrap()
    }

    fn hours_east(hours: i32) -> FixedOffset {
        FixedOffset::east_opt(hours * 3600).unwrap()
    }

    fn date_time(local: NaiveDateTime, hours: i32) -> DateTime {
        DateTime::from_local(local, hours_east(hours)).expect("should be valid")
    }

    #[test]
    fn test_from_local() {
        let value = date_time(naive(2020, 1, 2, 3, 4, 5), 2);

        assert_eq!(value.local(), naive(2020, 1, 2, 3, 4, 5));
        assert_eq!(value.offset(), hours_east(2));
    }

    #[test]
    fn test_new_rejects_invalid_offset() {
        let value = hours_east(15)
            .from_local_datetime(&naive(2020, 1, 2, 3, 4, 5))
            .unwrap();

        assert_eq!(
            DateTime::new(value),
            Err(Error::InvalidTimezoneOffset { seconds: 15 * 3600 })
        );
    }

    #[test]
    fn test_from_local_rejects_offset_with_seconds() {
        assert_eq!(
            DateTime::from_local(
                naive(2020, 1, 2, 3, 4, 5),
                FixedOffset::east_opt(30).unwrap()
            ),
            Err(Error::InvalidTimezoneOffset { seconds: 30 })
        );
    }

    #[test]
    fn test_equal_instants_with_different_offsets() {
        let a = date_time(naive(2020, 1, 2, 3, 4, 5), 2);
        let b = date_time(naive(2020, 1, 2, 1, 4, 5), 0);

        assert_eq!(a, b);
        assert_eq!(hash_of(&a), hash_of(&b));
    }

    #[test]
    fn test_ordering_uses_instants() {
        let earlier = date_time(naive(2020, 1, 2, 3, 0, 0), 2);
        let later = date_time(naive(2020, 1, 2, 2, 0, 0), 0);

        assert!(earlier < later);
    }

    #[test]
    fn test_chrono_conversions() {
        let value: DateTime = Utc.with_ymd_and_hms(2020, 1, 2, 3, 4, 5).unwrap().into();
        assert_eq!(value.offset(), hours_east(0));

        let chrono_value: chrono::DateTime<FixedOffset> = value.into();
        assert_eq!(chrono_value.to_rfc3339(), "2020-01-02T03:04:05+00:00");
        assert_eq!(DateTime::try_from(chrono_value), Ok(value));
    }

    fn from_parts(fields: (i32, u32, u32, u32, u32, u32, u32), offset_seconds: i32) -> DateTime {
        let (year, month, day, hour, minute, second, nanosecond) = fields;
        DateTime::from_parts(
            year,
            month,
            day,
            hour,
            minute,
            second,
            nanosecond,
            offset_seconds,
        )
        .expect("should be valid")
    }

    #[test]
    fn test_getters_return_local_fields() {
        let value = from_parts((2020, 1, 2, 3, 4, 5, 6), -5 * 3600);

        assert_eq!(value.year(), 2020);
        assert_eq!(value.month(), 1);
        assert_eq!(value.day(), 2);
        assert_eq!(value.hour(), 3);
        assert_eq!(value.minute(), 4);
        assert_eq!(value.second(), 5);
        assert_eq!(value.nanosecond(), 6);
        assert_eq!(value.offset_seconds(), -5 * 3600);
    }

    #[test]
    fn test_getters_before_1970() {
        let late_1969 = from_parts((1969, 12, 31, 23, 59, 59, 0), 3600);
        let ancient = from_parts((-44, 3, 15, 12, 0, 0, 0), 0);

        assert_eq!(
            (late_1969.year(), late_1969.month(), late_1969.day()),
            (1969, 12, 31)
        );
        assert_eq!(late_1969.hour(), 23);
        assert_eq!(
            (ancient.year(), ancient.month(), ancient.day()),
            (-44, 3, 15)
        );
    }

    #[test]
    fn test_from_parts_matches_from_local() {
        assert_eq!(
            from_parts((2020, 1, 2, 3, 4, 5, 0), 2 * 3600),
            date_time(naive(2020, 1, 2, 3, 4, 5), 2)
        );
    }

    #[test]
    fn test_from_parts_round_trips_through_getters() {
        let value = from_parts((2017, 6, 30, 23, 59, 58, 123_456_789), 9 * 3600 + 30 * 60);

        let rebuilt = from_parts(
            (
                value.year(),
                value.month(),
                value.day(),
                value.hour(),
                value.minute(),
                value.second(),
                value.nanosecond(),
            ),
            value.offset_seconds(),
        );

        assert_eq!(rebuilt, value);
        assert_eq!(rebuilt.offset_seconds(), value.offset_seconds());
    }

    #[test]
    fn test_from_parts_accepts_leap_second() {
        let value = from_parts((2016, 12, 31, 23, 59, 59, 1_500_000_000), 0);

        assert_eq!(value.second(), 59);
        assert_eq!(value.nanosecond(), 1_500_000_000);
    }

    #[test]
    fn test_from_parts_rejects_invalid_fields() {
        for (year, month, day, hour, minute, second, nanosecond) in [
            (2020, 2, 30, 0, 0, 0, 0),
            (2020, 13, 1, 0, 0, 0, 0),
            (2020, 1, 2, 24, 0, 0, 0),
            (2020, 1, 2, 3, 60, 0, 0),
            (2020, 1, 2, 3, 4, 60, 0),
            (2020, 1, 2, 3, 4, 5, 2_000_000_000),
        ] {
            let result =
                DateTime::from_parts(year, month, day, hour, minute, second, nanosecond, 0);
            assert!(
                matches!(result, Err(Error::InvalidDateTimeFields { .. })),
                "{year}-{month}-{day} {hour}:{minute}:{second}.{nanosecond} should be rejected"
            );
        }
    }

    #[test]
    fn test_from_parts_rejects_invalid_offsets() {
        for offset_seconds in [15 * 3600, -15 * 3600, 30, 100_000, i32::MIN] {
            assert_eq!(
                DateTime::from_parts(2020, 1, 2, 3, 4, 5, 0, offset_seconds),
                Err(Error::InvalidTimezoneOffset {
                    seconds: offset_seconds
                })
            );
        }
    }

    #[test]
    fn test_from_parts_compares_instants() {
        let in_cet = from_parts((2020, 1, 2, 4, 0, 0, 0), 3600);
        let in_utc = from_parts((2020, 1, 2, 3, 0, 0, 0), 0);
        let later = from_parts((2020, 1, 2, 3, 0, 1, 0), 0);

        assert_eq!(in_cet, in_utc);
        assert_eq!(hash_of(&in_cet), hash_of(&in_utc));
        assert!(in_cet < later);
    }
}
