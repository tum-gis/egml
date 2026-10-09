use crate::model::base::Property;
use crate::model::geometry::primitives::Solid;

/// A solid held inline or by `xlink:href`, corresponding to `gml:SolidPropertyType`
/// ([OGC 07-036](https://docs.ogc.org/is/07-036/07-036.pdf), `geometryPrimitives.xsd`).
pub type SolidProperty = Property<Solid>;
