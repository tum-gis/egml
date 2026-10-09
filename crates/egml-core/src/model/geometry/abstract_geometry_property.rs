use crate::model::base::Property;
use crate::model::geometry::AbstractGeometryKind;

/// A geometry held inline or by `xlink:href`, corresponding to `gml:GeometryPropertyType`
/// ([OGC 07-036](https://docs.ogc.org/is/07-036/07-036.pdf), `geometryBasic0d1d.xsd`).
pub type AbstractGeometryProperty = Property<AbstractGeometryKind>;
