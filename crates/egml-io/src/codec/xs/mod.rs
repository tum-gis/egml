//! Codecs for XML Schema built-in datatypes (`xs:date`, `xs:dateTime`,
//! `xs:integer`) used by GML application schemas.

mod date;
mod date_time;
mod integer;
mod lexical;

pub use date::*;
pub use date_time::*;
pub use integer::*;
