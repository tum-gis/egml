//! Borrowed views over the geometry primitive `Kind` enums.
//!
//! Each `*Ref<'a>` type mirrors the shape of its owned `*Kind` counterpart
//! one-to-one, holding `&'a` references (or nested `Ref` types) instead of
//! owned values. This lets code that needs to treat heterogeneous nested
//! geometry types uniformly — e.g. a resolver indexing geometries by
//! `gml:id`, or a recursive visitor — do so without cloning.

mod abstract_curve_kind_ref;
mod abstract_geometric_primitive_kind_ref;
mod abstract_ring_kind_ref;
mod abstract_solid_kind_ref;
mod abstract_surface_kind_ref;
mod surface_kind_ref;

pub use abstract_curve_kind_ref::*;
pub use abstract_geometric_primitive_kind_ref::*;
pub use abstract_ring_kind_ref::*;
pub use abstract_solid_kind_ref::*;
pub use abstract_surface_kind_ref::*;
pub use surface_kind_ref::*;
