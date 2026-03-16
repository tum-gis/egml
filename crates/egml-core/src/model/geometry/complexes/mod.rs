//! Topology-aware geometry complexes.
//!
//! Unlike aggregates, complexes impose topological constraints on their
//! members.  [`CompositeSurface`] requires that all constituent surface
//! patches share edges coherently, forming a single connected manifold.

mod composite_surface;

pub use composite_surface::*;
