use crate::model::base::Id;
use crate::model::geometry::DirectPosition;
use std::fmt;

/// Errors returned by `egml-core` operations.
#[derive(Debug, PartialEq, Clone)]
#[non_exhaustive]
pub enum Error {
    /// Returned when a floating-point coordinate is not finite (NaN or ±infinity).
    ///
    /// `axis` names the offending component (`"x"`, `"y"`, or `"z"`); `value` is
    /// the actual non-finite number that was supplied.
    NonFiniteCoordinate { axis: &'static str, value: f64 },

    /// Returned when a collection has fewer elements than the minimum required by
    /// the GML geometry constraint.
    ///
    /// Optional fields carry progressively more context: `spec` cites the ISO
    /// clause, `id` names the offending GML object, and `detail` gives a
    /// human-readable description of the actual content that was supplied.
    TooFewElements {
        geometry: &'static str,
        minimum: usize,
        spec: Option<&'static str>,
        id: Option<Id>,
        detail: Option<String>,
    },

    /// Returned when a collection has a number of elements other than the exact
    /// count required by the GML geometry constraint.
    ///
    /// `expected` is the required count; `actual` is what was supplied.
    /// `spec` optionally cites the ISO/OGC clause that mandates the count.
    InvalidElementCount {
        geometry: &'static str,
        expected: usize,
        actual: usize,
        spec: Option<&'static str>,
    },

    /// Returned when two positions that must be distinct are identical.
    ///
    /// Applies to [`Triangle`](crate::model::geometry::primitives::Triangle)
    /// vertices ([OGC 07-036 §10.5.12.5](https://docs.ogc.org/is/07-036/07-036.pdf)).
    IdenticalPositions {
        first: DirectPosition,
        second: DirectPosition,
    },

    /// Returned when adjacent positions in a sequence are equal.
    ///
    /// `index` is the zero-based index of the first position in the duplicate
    /// pair; `position` is its value. Applies to
    /// [`LinearRing`](crate::model::geometry::primitives::LinearRing)
    /// ([OGC 07-036 §10.5.8](https://docs.ogc.org/is/07-036/07-036.pdf)) and
    /// [`LineString`](crate::model::geometry::primitives::LineString)
    /// ([OGC 07-036 §10.4.4](https://docs.ogc.org/is/07-036/07-036.pdf)).
    AdjacentDuplicatePositions {
        index: usize,
        position: DirectPosition,
    },

    /// Returned when the first and last position of a ring are equal.
    ///
    /// `position` is the repeated vertex. A `gml:LinearRing` is implicitly
    /// closed and must not include an explicit closing vertex ([OGC 07-036 §10.5.8](https://docs.ogc.org/is/07-036/07-036.pdf)).
    RepeatedClosingVertex { position: DirectPosition },

    /// Returned when an [`Envelope`](crate::model::geometry::Envelope) is constructed
    /// with a lower corner that strictly exceeds the upper corner along `axis`.
    ///
    /// `lower` and `upper` are the actual conflicting coordinate values.
    /// [OGC 07-036 §10.1.4.6](https://docs.ogc.org/is/07-036/07-036.pdf) requires `lowerCorner ≤ upperCorner` in every component.
    InvalidEnvelopeBounds {
        axis: &'static str,
        lower: f64,
        upper: f64,
    },

    /// Returned when
    /// [`Envelope::to_triangulated_surface`](crate::model::geometry::Envelope::to_triangulated_surface)
    /// is called on an envelope that is neither a surface nor a volume.
    ///
    /// `non_zero_extents` is the number of axes with non-zero extent (0 or 1).
    NotSurfaceOrVolume { non_zero_extents: u8 },

    /// Returned when [`Envelope::to_polygon`](crate::model::geometry::Envelope::to_polygon)
    /// is called on an envelope that does not have exactly two non-zero extents.
    ///
    /// `non_zero_extents` is the actual number of axes with non-zero extent.
    NotASurface { non_zero_extents: u8 },

    /// Returned when [`Envelope::to_solid`](crate::model::geometry::Envelope::to_solid)
    /// is called on an envelope that does not have all three extents non-zero.
    ///
    /// `non_zero_extents` is the actual number of axes with non-zero extent.
    NotAVolume { non_zero_extents: u8 },

    /// Returned when an empty string is used to construct a [`Id`](crate::model::base::Id).
    EmptyId,

    /// Returned when the earcut polygon triangulation algorithm produces no triangles.
    ///
    /// `context` provides additional information about which polygon or patch
    /// could not be decomposed.
    TriangulationFailed { context: String },

    /// Returned when an operation requires an exterior ring but the polygon has none.
    MissingExteriorRing,

    /// Returned when a ring property carries only an xlink:href reference and the
    /// referenced geometry object has not been resolved into an inline object.
    ///
    /// `href` is the reference value if one was present, or `None` if the property
    /// has neither an inline object nor a reference.
    UnresolvedRingReference { href: Option<String> },

    /// Returned when a surface property carries only an xlink:href reference and the
    /// referenced geometry object has not been resolved into an inline object.
    ///
    /// `href` is the reference value if one was present, or `None` if the property
    /// has neither an inline object nor a reference.
    UnresolvedSurfaceReference { href: Option<String> },

    /// Returned when a curve property carries only an xlink:href reference and the
    /// referenced geometry object has not been resolved into an inline object.
    ///
    /// `href` is the reference value if one was present, or `None` if the property
    /// has neither an inline object nor a reference.
    UnresolvedCurveReference { href: Option<String> },

    /// Returned when an operation requires an exterior shell but the solid has none.
    MissingExteriorShell,

    /// Returned when a shell property carries only an xlink:href reference and the
    /// referenced geometry object has not been resolved into an inline object.
    ///
    /// `href` is the reference value if one was present, or `None` if the property
    /// has neither an inline object nor a reference.
    UnresolvedShellReference { href: Option<String> },

    /// Returned when `triangulate` is called on a geometry type that cannot
    /// produce a surface (e.g. `Point`, `MultiCurve`).
    ///
    /// `geometry` names the concrete variant that was encountered.
    TriangulationNotSupported { geometry: &'static str },

    /// Returned when an attribute held a value that could not be parsed into
    /// its expected type (e.g. an `xlink:show`/`xlink:actuate` value outside
    /// the enumeration defined by the XLink specification).
    ///
    /// `attribute` names the attribute (e.g. `"xlink:show"`); `value` is the
    /// raw string that was supplied.
    InvalidAttributeValue {
        attribute: &'static str,
        value: String,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NonFiniteCoordinate { axis, value } => write!(
                f,
                "coordinate '{axis}' has non-finite value {value}; \
                 all GML coordinate values must be real numbers (OGC 07-036 §10.1.4.1)"
            ),
            Error::TooFewElements {
                geometry,
                minimum,
                spec,
                id,
                detail,
            } => {
                write!(f, "{geometry} requires at least {minimum} element(s)")?;
                if let Some(s) = spec {
                    write!(f, " ({s})")?;
                }
                if let Some(i) = id {
                    write!(f, " [id={i}]")?;
                }
                if let Some(d) = detail {
                    write!(f, ": {d}")?;
                }
                Ok(())
            }
            Error::InvalidElementCount {
                geometry,
                expected,
                actual,
                spec,
            } => {
                write!(
                    f,
                    "{geometry} requires exactly {expected} element(s), got {actual}"
                )?;
                if let Some(s) = spec {
                    write!(f, " ({s})")?;
                }
                Ok(())
            }
            Error::IdenticalPositions { first, second } => write!(
                f,
                "geometry contains two identical positions {first} and {second}; \
                 all positions must be distinct (OGC 07-036 §10.5.12.5)"
            ),
            Error::AdjacentDuplicatePositions { index, position } => write!(
                f,
                "adjacent duplicate positions at index {index} ({position}); \
                 consecutive coordinates must be distinct"
            ),
            Error::RepeatedClosingVertex { position } => write!(
                f,
                "linear ring has repeated closing vertex at {position}; \
                 gml:LinearRing is implicitly closed and must not include \
                 an explicit closing vertex (OGC 07-036 §10.5.8)"
            ),
            Error::InvalidEnvelopeBounds { axis, lower, upper } => write!(
                f,
                "envelope lower.{axis}={lower} exceeds upper.{axis}={upper}; \
                 lowerCorner must be ≤ upperCorner in every component (OGC 07-036 §10.1.4.6)"
            ),
            Error::NotSurfaceOrVolume { non_zero_extents } => write!(
                f,
                "envelope has {non_zero_extents} non-zero extent(s) and cannot be triangulated; \
                 requires a surface (2 non-zero extents) or volume (3 non-zero extents)"
            ),
            Error::NotASurface { non_zero_extents } => write!(
                f,
                "envelope has {non_zero_extents} non-zero extent(s); \
                 to_polygon requires exactly 2"
            ),
            Error::NotAVolume { non_zero_extents } => write!(
                f,
                "envelope has {non_zero_extents} non-zero extent(s); \
                 to_solid requires all 3 to be non-zero"
            ),
            Error::EmptyId => write!(f, "gml:id must not be empty"),
            Error::TriangulationFailed { context } => {
                write!(f, "polygon triangulation (earcut) failed: {context}")
            }
            Error::MissingExteriorRing => write!(
                f,
                "polygon has no exterior ring; \
                 operation requires a defined outer boundary (OGC 07-036 §10.5.6)"
            ),
            Error::UnresolvedRingReference { href: Some(href) } => write!(
                f,
                "ring property references '{href}' via xlink:href but the object has not been resolved"
            ),
            Error::UnresolvedRingReference { href: None } => write!(
                f,
                "ring property has neither an inline object nor an xlink:href reference"
            ),
            Error::UnresolvedSurfaceReference { href: Some(href) } => write!(
                f,
                "surface property references '{href}' via xlink:href but the object has not been resolved"
            ),
            Error::UnresolvedSurfaceReference { href: None } => write!(
                f,
                "surface property has neither an inline object nor an xlink:href reference"
            ),
            Error::UnresolvedCurveReference { href: Some(href) } => write!(
                f,
                "curve property references '{href}' via xlink:href but the object has not been resolved"
            ),
            Error::UnresolvedCurveReference { href: None } => write!(
                f,
                "curve property has neither an inline object nor an xlink:href reference"
            ),
            Error::MissingExteriorShell => write!(
                f,
                "solid has no exterior shell; \
                 operation requires a defined outer boundary (OGC 07-036 §10.6.4)"
            ),
            Error::UnresolvedShellReference { href: Some(href) } => write!(
                f,
                "shell property references '{href}' via xlink:href but the object has not been resolved"
            ),
            Error::UnresolvedShellReference { href: None } => write!(
                f,
                "shell property has neither an inline object nor an xlink:href reference"
            ),
            Error::TriangulationNotSupported { geometry } => write!(
                f,
                "triangulation is not supported for geometry type '{geometry}'"
            ),
            Error::InvalidAttributeValue { attribute, value } => {
                write!(f, "invalid value '{value}' for attribute '{attribute}'")
            }
        }
    }
}

impl std::error::Error for Error {}
