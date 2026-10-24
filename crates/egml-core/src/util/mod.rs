//! Internal geometry utilities.
//!
//! | Module | Contents |
//! |--------|----------|
//! | `distance` | Planar point-to-segment distance helpers (crate-internal) |
//! | [`plane`] | Plane in R³ defined by a point and a unit normal vector |
//! | [`triangulate`] | Earcut-based polygon-to-triangle decomposition |

pub(crate) mod distance;
pub mod plane;
pub mod triangulate;
