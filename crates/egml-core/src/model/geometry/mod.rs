//! GML geometry model (ISO 19136 §10).
//!
//! This module implements the GML 3.2 geometry type hierarchy in three
//! sub-modules, plus two foundational types used throughout:
//!
//! | Item | Description |
//! |------|-------------|
//! | [`DirectPosition`] | 3-D coordinate in a CRS |
//! | [`Envelope`] | Axis-aligned bounding box |
//! | [`primitives`] | 0-D to 3-D geometry primitives |
//! | [`aggregates`] | Multi-geometry collections (MultiCurve, MultiSurface) |
//! | [`complexes`] | Topology-aware geometry complexes (CompositeSurface) |
//!
//! # Type hierarchy
//!
//! ```text
//! AbstractGeometry
//! └── AbstractGeometricPrimitive
//!     ├── Point
//!     ├── AbstractCurve  →  LineString
//!     ├── AbstractSurface  →  Surface, TriangulatedSurface, Polygon, Triangle
//!     └── AbstractSolid  →  Solid
//! AbstractGeometricAggregate
//!     ├── MultiCurve
//!     └── MultiSurface
//! CompositeSurface
//! ```

mod abstract_geometry;
pub mod aggregates;
pub mod complexes;
mod direct_position;
mod envelope;
pub mod primitives;

pub use self::abstract_geometry::*;
pub use self::direct_position::*;
pub use self::envelope::*;
