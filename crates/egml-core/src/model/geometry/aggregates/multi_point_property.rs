use crate::model::base::Property;
use crate::model::geometry::aggregates::MultiPoint;

/// A multi-point held inline or by `xlink:href`, corresponding to `gml:MultiPointPropertyType`
/// ([OGC 07-036](https://docs.ogc.org/is/07-036/07-036.pdf), `geometryAggregates.xsd`).
pub type MultiPointProperty = Property<MultiPoint>;
