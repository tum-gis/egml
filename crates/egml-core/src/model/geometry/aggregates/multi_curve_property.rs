use crate::model::base::Property;
use crate::model::geometry::aggregates::MultiCurve;

/// A multi-curve held inline or by `xlink:href`, corresponding to `gml:MultiCurvePropertyType`
/// ([OGC 07-036](https://docs.ogc.org/is/07-036/07-036.pdf), `geometryAggregates.xsd`).
pub type MultiCurveProperty = Property<MultiCurve>;
