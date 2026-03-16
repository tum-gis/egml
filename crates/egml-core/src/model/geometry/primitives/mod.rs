//! Geometry primitives from 0-D (point) to 3-D (solid).
//!
//! Each concrete type corresponds to a named GML element:
//!
//! | Rust type | GML element | Dimension |
//! |-----------|------------|-----------|
//! | [`Point`] | `gml:Point` | 0-D |
//! | [`LineString`] | `gml:LineString` | 1-D |
//! | [`LinearRing`] | `gml:LinearRing` | 1-D (closed) |
//! | [`Polygon`] | `gml:Polygon` | 2-D |
//! | [`Triangle`] | `gml:Triangle` | 2-D |
//! | [`Surface`] | `gml:Surface` | 2-D (patched) |
//! | [`TriangulatedSurface`] | `gml:TriangulatedSurface` | 2-D |
//! | [`Solid`] | `gml:Solid` | 3-D |
//!
//! Abstract base traits and supporting property types are also re-exported from
//! this module.

mod abstract_curve;
pub mod abstract_geometric_primitive;
mod abstract_ring;
mod abstract_ring_property;
mod abstract_solid;
mod abstract_surface;
mod abstract_surface_patch;
mod line_string;
pub mod linear_ring;
mod point;
pub mod polygon;
mod polygon_patch;
pub mod solid;
mod surface;
mod surface_interpolation;
mod surface_patch_array_property;
pub mod surface_property;
pub mod triangle;
pub mod triangulated_surface;

pub use abstract_curve::*;
pub use abstract_geometric_primitive::*;
pub use abstract_ring::*;
pub use abstract_ring_property::*;
pub use abstract_solid::*;
pub use abstract_surface::*;
pub use abstract_surface_patch::*;
pub use line_string::*;
pub use linear_ring::*;
pub use point::*;
pub use polygon::*;
pub use polygon_patch::*;
pub use solid::*;
pub use surface::*;
pub use surface_patch_array_property::*;
pub use surface_property::*;
pub use triangle::*;
pub use triangulated_surface::*;
