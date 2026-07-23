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
mod abstract_geometric_aggregate_kind;
mod aggregation_type;
mod multi_curve;
mod multi_curve_property;
mod multi_geometry;
mod multi_geometry_property;
mod multi_point;
mod multi_point_property;
mod multi_surface;
mod multi_surface_property;
pub mod refs;

pub use abstract_geometric_aggregate::*;
pub use abstract_geometric_aggregate_kind::*;
pub use aggregation_type::*;
pub use multi_curve::*;
pub use multi_curve_property::*;
pub use multi_geometry::*;
pub use multi_geometry_property::*;
pub use multi_point::*;
pub use multi_point_property::*;
pub use multi_surface::*;
pub use multi_surface_property::*;
