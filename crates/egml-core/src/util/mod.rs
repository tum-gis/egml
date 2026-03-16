//! Internal geometry utilities.
//!
//! | Module | Contents |
//! |--------|----------|
//! | [`plane`] | Plane in R³ defined by a point and a unit normal vector |
//! | [`triangulate`] | Earcut-based polygon-to-triangle decomposition |

pub mod plane;
pub mod triangulate;
