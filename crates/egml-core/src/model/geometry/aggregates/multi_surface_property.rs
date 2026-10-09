use crate::model::base::Property;
use crate::model::geometry::aggregates::MultiSurface;

/// A multi-surface held inline or by `xlink:href`, corresponding to `gml:MultiSurfacePropertyType`
/// ([OGC 07-036](https://docs.ogc.org/is/07-036/07-036.pdf), `geometryAggregates.xsd`).
pub type MultiSurfaceProperty = Property<MultiSurface>;
