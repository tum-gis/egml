use crate::model::base::ArrayProperty;
use crate::model::geometry::primitives::Point;

/// Inline members, corresponding to `gml:PointArrayPropertyType`
/// ([OGC 07-036](https://docs.ogc.org/is/07-036/07-036.pdf), `geometryBasic0d1d.xsd`).
pub type PointArrayProperty = ArrayProperty<Point>;
