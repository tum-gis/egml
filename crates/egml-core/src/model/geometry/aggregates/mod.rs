//! Multi-geometry collections.
//!
//! Aggregates group independent geometry primitives of the same kind without
//! any topological constraints between members.
//!
//! | Rust type | GML element | Aggregates |
//! |-----------|------------|------------|
//! | [`MultiCurve`] | `gml:MultiCurve` | [`LineString`](super::primitives::LineString) members |
//! | [`MultiSurface`] | `gml:MultiSurface` | [`Surface`](super::primitives::Surface) members |
//!
//! The [`AggregationType`] enum qualifies how elements within an aggregate
//! relate to one another (bag, set, sequence, array, or record).

mod abstract_geometric_aggregate;
mod aggregation_type;
mod multi_curve;
mod multi_surface;

pub use abstract_geometric_aggregate::*;
pub use aggregation_type::*;
pub use multi_curve::*;
pub use multi_surface::*;
