use crate::model::base::Property;
use crate::model::geometry::primitives::Point;

/// A point held inline or by `xlink:href`, corresponding to `gml:PointPropertyType`
/// ([OGC 07-036](https://docs.ogc.org/is/07-036/07-036.pdf), `geometryBasic0d1d.xsd`).
pub type PointProperty = Property<Point>;
