//! Re-export façade for the `egml` workspace.
//!
//! This crate bundles [`egml-core`], [`egml-io`], and [`egml-transform`] under a
//! single convenient namespace so downstream users only need one dependency.
//!
//! | Re-export path | Source crate | Purpose |
//! |----------------|-------------|---------|
//! | `egml::model::…` | `egml-core` | Geometry types, envelopes, IDs |
//! | `egml::util::…` | `egml-core` | Plane and triangulation utilities |
//! | `egml::io::…` | `egml-io` | XML parsing of GML documents |
//! | `egml::transform::…` | `egml-transform` | Geometric transforms (in development) |
//! | `egml::Error` | `egml-core` | Core error type |
//!
//! # Quick-start example
//!
//! ```rust
//! use egml::model::geometry::DirectPosition;
//! use egml::model::geometry::Envelope;
//!
//! let pos = DirectPosition::new(1.0, 2.0, 3.0).unwrap();
//! let env = Envelope::from_points(&[pos]).unwrap();
//! assert_eq!(env.lower_corner(), env.upper_corner());
//! ```
//!

pub use egml_core::{Error, model, util};

pub use egml_io as io;

pub use egml_transform as transform;
