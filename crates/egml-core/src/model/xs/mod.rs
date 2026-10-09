//! XML Schema built-in datatypes.
//!
//! These types come from [XSD 1.1 Part 2](https://www.w3.org/TR/xmlschema11-2/),
//! not from GML itself, but GML application schemas such as CityGML use them
//! directly (e.g. `creationDate`, `dateOfConstruction`).
//!
//! | Type | XSD counterpart | Description |
//! |------|----------------|-------------|
//! | [`Date`] | `xs:date` | A calendar date with a timezone |
//! | [`DateTime`] | `xs:dateTime` | A date and time of day with a timezone |
//!
//! XSD makes the timezone optional, but both types always carry one; a
//! missing timezone is filled in when the data is read. Offsets are limited
//! to what XSD can write: whole minutes within ±14:00. Equality, ordering and
//! hashing compare instants.
//!
//! Parsing and writing the lexical forms is done by `egml-io`.

mod date;
mod date_time;
mod offset;

pub use date::*;
pub use date_time::*;
