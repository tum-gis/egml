pub mod base;
mod error;
pub mod geometry;
mod ops;

pub use crate::base::*;
pub use crate::geometry::*;

#[doc(inline)]
pub use error::Error;

#[doc(inline)]
pub use ops::distance;
