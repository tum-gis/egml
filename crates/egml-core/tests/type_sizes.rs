//! Guards against accidental growth of hot model types.
//!
//! `AbstractGml` and `AbstractGeometry` are embedded in every geometry object,
//! so every byte here is multiplied by the number of geometries in a dataset.

use egml_core::model::base::{AbstractGml, Id};
use egml_core::model::geometry::{AbstractGeometry, Envelope};
use std::mem::size_of;

#[test]
fn id_is_a_thin_string() {
    assert!(size_of::<Id>() <= 16, "Id: {}", size_of::<Id>());
}

#[test]
fn base_metadata_stays_compact() {
    assert!(
        size_of::<AbstractGml>() <= 32,
        "AbstractGml: {}",
        size_of::<AbstractGml>()
    );
    assert!(
        size_of::<AbstractGeometry>() <= 40,
        "AbstractGeometry: {}",
        size_of::<AbstractGeometry>()
    );
}

#[test]
fn rings_and_shells_are_stored_without_wrapper() {
    use egml_core::model::geometry::primitives::{Polygon, Solid};

    assert!(
        size_of::<Polygon>() <= 128,
        "Polygon: {}",
        size_of::<Polygon>()
    );
    assert!(size_of::<Solid>() <= 160, "Solid: {}", size_of::<Solid>());
}

#[test]
fn property_attributes_cost_one_pointer() {
    use egml_core::model::base::Property;
    use egml_core::model::geometry::primitives::Point;

    assert_eq!(
        size_of::<Property<Point>>(),
        size_of::<Option<Point>>() + size_of::<usize>()
    );
}

#[test]
fn envelope_stays_compact() {
    // Two corners (48 B) + `Option<AnyUri>` srs_name (16 B) + `Option<u8>` srs_dimension.
    assert!(
        size_of::<Envelope>() <= 72,
        "Envelope: {}",
        size_of::<Envelope>()
    );
}

#[test]
fn point_sequences_are_boxed_slices() {
    use egml_core::model::geometry::primitives::{LineString, LinearRing};

    // `AbstractGeometry` (40 B) + `Box<[DirectPosition]>` (16 B), no capacity field.
    assert!(
        size_of::<LinearRing>() <= 56,
        "LinearRing: {}",
        size_of::<LinearRing>()
    );
    assert!(
        size_of::<LineString>() <= 56,
        "LineString: {}",
        size_of::<LineString>()
    );
}

#[test]
fn any_uri_is_a_thin_string() {
    use egml_core::model::xs::AnyUri;

    assert!(size_of::<AnyUri>() <= 16, "AnyUri: {}", size_of::<AnyUri>());
    assert_eq!(size_of::<Option<AnyUri>>(), size_of::<AnyUri>());
}

#[test]
fn code_stays_compact() {
    use egml_core::model::basic_types::Code;

    // `Option<AnyUri>` code_space (16 B) + `String` value (24 B).
    assert!(size_of::<Code>() <= 40, "Code: {}", size_of::<Code>());
}
