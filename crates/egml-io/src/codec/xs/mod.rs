//! Codecs for XML Schema built-in datatypes (`xs:anyURI`, `xs:date`,
//! `xs:dateTime`, `xs:double`, `xs:integer`) used by GML application schemas.

mod any_uri;
mod date;
mod date_time;
mod double;
mod integer;
mod lexical;

pub use any_uri::*;
pub use date::*;
pub use date_time::*;
pub(crate) use double::{format_double, parse_double};
pub use integer::*;
pub(crate) use lexical::collapse_whitespace;
