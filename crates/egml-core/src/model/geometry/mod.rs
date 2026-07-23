//! GML geometry model ([OGC 07-036 §10](https://docs.ogc.org/is/07-036/07-036.pdf)).
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
mod abstract_geometry_array_property;
mod abstract_geometry_kind;
mod abstract_geometry_property;
pub mod aggregates;
pub mod complexes;
mod direct_position;
mod direct_position_list;
mod envelope;
pub mod primitives;
pub mod refs;

pub use self::abstract_geometry::*;
pub use self::abstract_geometry_array_property::*;
pub use self::abstract_geometry_kind::*;
pub use self::abstract_geometry_property::*;
pub use self::direct_position::*;
pub use self::direct_position_list::*;
pub use self::envelope::*;
