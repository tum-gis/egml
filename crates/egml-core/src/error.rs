use crate::model::base::Id;
use thiserror::Error;

/// Errors returned by `egml-core` operations.
#[derive(Error, Debug, Eq, PartialEq, Hash, Clone)]
pub enum Error {
    /// Returned when a floating-point coordinate is not finite (NaN or ±infinity).
    ///
    /// The inner string names the offending coordinate component: `"x"`, `"y"`, or `"z"`.
    /// GML requires all coordinate values in a `DirectPosition` to be real numbers
    /// (ISO 19136 §9.4 `DirectPositionType`).
    #[error(
        "coordinate '{0}' has a non-finite value (NaN or ±infinity); all GML coordinate values must be real numbers (ISO 19136 §9.4)"
    )]
    NonFiniteCoordinate(&'static str),

    /// Returned when a collection has fewer elements than the minimum required by
    /// the GML geometry constraint.
    ///
    /// The inner string is a human-readable description of the violated constraint,
    /// e.g. `"LinearRing requires at least 3 positions (ISO 19136 §10.5.12)"`.
    #[error(
        "{geometry} requires at least {minimum} elements ({spec:?}) [id={id:?}] [message={message:?}]"
    )]
    TooFewElements {
        geometry: &'static str,
        minimum: usize,
        spec: Option<&'static str>,
        id: Option<Id>,
        message: Option<String>,
    },

    /// Returned when a collection contains a number of elements that is not
    /// accepted by the operation (e.g. a `Triangle` not given exactly 3 points).
    ///
    /// The inner string describes which collection or argument was invalid.
    #[error("invalid number of elements: {0}")]
    WrongElementCount(&'static str),

    /// Returned when an operation requires a non-empty input but received an empty one.
    ///
    /// The inner string names the offending parameter or context (e.g. `"solid"`,
    /// `"multi curve"`).
    #[error("'{0}' must not be empty")]
    EmptyCollection(&'static str),

    /// Returned when a geometry contains two identical positions where all positions
    /// must be distinct (e.g. all three vertices of a [`Triangle`](crate::model::geometry::primitives::Triangle)
    /// must be different, per ISO 19136 §10.5.9).
    #[error(
        "geometry contains two identical positions; all positions must be distinct (ISO 19136 §10.5.9)"
    )]
    IdenticalPositions,

    /// Returned when adjacent positions in a sequence are equal and the geometry type
    /// requires all consecutive positions to differ.
    ///
    /// Applies to [`LinearRing`](crate::model::geometry::primitives::LinearRing)
    /// (ISO 19136 §10.5.12) and [`LineString`](crate::model::geometry::primitives::LineString)
    /// (ISO 19136 §10.4.4).
    #[error(
        "sequence contains adjacent duplicate positions; consecutive coordinates must be distinct (ISO 19136 §10.4.3)"
    )]
    AdjacentDuplicatePositions,

    /// Returned when the first and last position of a ring are equal.
    ///
    /// A `gml:LinearRing` is implicitly closed: the geometry engine connects the
    /// last position back to the first automatically.  An explicit repeated closing
    /// vertex is therefore redundant and invalid (ISO 19136 §10.5.12).
    #[error(
        "linear ring has matching first and last positions; gml:LinearRing is implicitly closed and must not include an explicit closing vertex (ISO 19136 §10.5.12)"
    )]
    RepeatedClosingVertex,

    /// Returned when an [`Envelope`](crate::model::geometry::Envelope) is constructed
    /// with a lower corner that is strictly greater than the upper corner in one or
    /// more coordinate components.
    ///
    /// The inner string names the offending component (`"x"`, `"y"`, or `"z"`).
    /// ISO 19136 §10.1.4 requires `lowerCorner ≤ upperCorner` in every component.
    #[error(
        "envelope lowerCorner.{0} exceeds upperCorner.{0}; each component of lowerCorner must be ≤ the corresponding upperCorner component (ISO 19136 §10.1.4)"
    )]
    InvalidEnvelopeBounds(&'static str),

    /// Returned when the earcut polygon triangulation algorithm produces no triangles.
    ///
    /// The inner string provides additional context about the failure (e.g. which
    /// polygon or patch could not be decomposed).
    #[error("polygon triangulation (earcut) failed: {0}")]
    TriangulationFailed(&'static str),
}
