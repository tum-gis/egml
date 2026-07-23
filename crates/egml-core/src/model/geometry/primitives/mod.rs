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
mod abstract_curve_kind;
mod abstract_curve_property;
pub mod abstract_geometric_primitive;
mod abstract_geometric_primitive_kind;
mod abstract_ring;
mod abstract_ring_kind;
mod abstract_ring_property;
mod abstract_solid;
mod abstract_solid_kind;
mod abstract_surface;
mod abstract_surface_kind;
mod abstract_surface_patch;
mod abstract_surface_patch_array_property;
mod abstract_surface_patch_kind;
mod abstract_surface_property;
mod line_string;
mod linear_ring;
mod linear_ring_property;
mod point;
mod point_array_property;
mod point_property;
mod polygon;
mod polygon_patch;
pub mod refs;
mod shell;
mod shell_property;
mod solid;
mod solid_property;
mod surface;
mod surface_interpolation;
pub mod surface_kind;
mod triangle;
mod triangulated_surface;

pub use abstract_curve::*;
pub use abstract_curve_kind::*;
pub use abstract_curve_property::*;
pub use abstract_geometric_primitive::*;
pub use abstract_geometric_primitive_kind::*;
pub use abstract_ring::*;
pub use abstract_ring_kind::*;
pub use abstract_ring_property::*;
pub use abstract_solid::*;
pub use abstract_solid_kind::*;
pub use abstract_surface::*;
pub use abstract_surface_kind::*;
pub use abstract_surface_patch::*;
pub use abstract_surface_patch_array_property::*;
pub use abstract_surface_patch_kind::*;
pub use abstract_surface_property::*;
pub use line_string::*;
pub use linear_ring::*;
pub use linear_ring_property::*;
pub use point::*;
pub use point_array_property::*;
pub use point_property::*;
pub use polygon::*;
pub use polygon_patch::*;
pub use shell::*;
pub use shell_property::*;
pub use solid::*;
pub use solid_property::*;
pub use surface::*;
pub use surface_kind::*;
pub use triangle::*;
pub use triangulated_surface::*;
