use crate::model::base::Property;
use crate::model::geometry::aggregates::MultiGeometry;

/// A multi-geometry held inline or by `xlink:href`, corresponding to `gml:MultiGeometryPropertyType`
/// ([OGC 07-036](https://docs.ogc.org/is/07-036/07-036.pdf), `geometryAggregates.xsd`).
pub type MultiGeometryProperty = Property<MultiGeometry>;
