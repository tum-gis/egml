use crate::model::geometry::refs::AbstractGeometryKindRef;

/// Iterates over all geometries nested within a geometry, recursively,
/// borrowed through [`AbstractGeometryKindRef`].
///
/// Each container yields every nested geometry — its direct members followed by
/// those members' own nested geometries, depth-first — as a single flattened
/// stream.
///
/// Leaf geometries (e.g. [`LinearRing`](crate::model::geometry::primitives::LinearRing))
/// yield an empty iterator. Container geometries
/// (e.g. [`Polygon`](crate::model::geometry::primitives::Polygon)) yield their
/// resolved members and each member's descendants, skipping any property that
/// only carries an unresolved `xlink:href`.
pub trait IterGeometries {
    fn iter_geometries(&self) -> Box<dyn Iterator<Item = AbstractGeometryKindRef<'_>> + '_>;
}
