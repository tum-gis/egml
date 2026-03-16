//! Core primitives and operations for processing GML (Geography Markup Language) data.
//!
//! `egml-core` implements a subset of [ISO 19136 GML 3.2](https://www.ogc.org/standards/gml)
//! focused on 3-D geometry. It provides the geometry model, bounding-box computations,
//! triangulation, and distance queries needed to work with GML-based datasets.
//!
//! # Module overview
//!
//! | Module | Contents |
//! |--------|----------|
//! | [`model::base`] | Root GML types: [`AbstractGml`](model::base::AbstractGml), [`Id`](model::base::Id) |
//! | [`model::basic`] | Primitive GML scalar types: [`Code`](model::basic::Code), [`Measure`](model::basic::Measure) |
//! | [`model::feature`] | Abstract GML feature class |
//! | [`model::geometry`] | Full geometry hierarchy: primitives, aggregates, and complexes |
//! | [`util::plane`] | Plane in R³ — point + unit normal |
//! | [`util::triangulate`] | Earcut-based polygon triangulation |
//!
//! # Geometry hierarchy
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
//!
//! # Quick-start example
//!
//! ```rust
//! use egml_core::model::geometry::DirectPosition;
//! use egml_core::model::geometry::primitives::{AbstractRing, LinearRing};
//! use egml_core::model::geometry::Envelope;
//!
//! let pts = vec![
//!     DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
//!     DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
//!     DirectPosition::new(1.0, 1.0, 0.0).unwrap(),
//!     DirectPosition::new(0.0, 1.0, 0.0).unwrap(),
//! ];
//! let ring = LinearRing::new(AbstractRing::default(), pts).unwrap();
//! let envelope = Envelope::from_points(ring.points()).unwrap();
//! assert!(envelope.size_x() > 0.0);
//! ```

mod error;
pub mod model;
mod ops;
pub mod util;

#[doc(inline)]
pub use error::Error;

#[doc(inline)]
pub use ops::distance;
