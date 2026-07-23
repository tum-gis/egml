use strum_macros::{Display, EnumIter};

/// Every concrete (non-abstract) geometry element in GML 3.2.1's
/// `gml:AbstractGeometry` substitution group
/// ([OGC 07-036 §10](https://docs.ogc.org/is/07-036/07-036.pdf)).
///
/// Curve segments (`Arc`, `Bezier`, ...) and surface patches (`PolygonPatch`,
/// `Cone`, ...) are excluded — they substitute `gml:AbstractCurveSegment`/
/// `gml:AbstractSurfacePatch`, not `gml:AbstractGeometry`, matching how this
/// crate keeps them in the separate
/// [`AbstractSurfacePatchKind`](crate::model::geometry::primitives::AbstractSurfacePatchKind)
/// family rather than [`AbstractGeometryKind`](crate::model::geometry::AbstractGeometryKind).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter, Display, Ord, PartialOrd)]
pub enum GeometryType {
    // 0-D / 1-D primitives (geometryBasic0d1d.xsd, geometryPrimitives.xsd)
    Point,
    LineString,
    Curve,
    OrientableCurve,

    // Rings (geometryBasic2d.xsd, geometryPrimitives.xsd)
    LinearRing,
    Ring,

    // Surfaces (geometryBasic2d.xsd, geometryPrimitives.xsd)
    Polygon,
    Surface,
    OrientableSurface,
    PolyhedralSurface,
    TriangulatedSurface,
    Tin,
    Shell,

    // Solids (geometryPrimitives.xsd)
    Solid,

    // Aggregates (geometryAggregates.xsd)
    MultiGeometry,
    MultiPoint,
    MultiCurve,
    MultiSurface,
    MultiSolid,

    // Complexes (geometryComplexes.xsd)
    GeometricComplex,
    CompositeCurve,
    CompositeSurface,
    CompositeSolid,

    // Implicit / grid geometries (grids.xsd)
    Grid,
    RectifiedGrid,
}
