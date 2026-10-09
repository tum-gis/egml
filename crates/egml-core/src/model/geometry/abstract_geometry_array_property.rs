use crate::model::base::ArrayProperty;
use crate::model::geometry::AbstractGeometryKind;

/// Inline members, corresponding to `gml:GeometryArrayPropertyType`
/// ([OGC 07-036](https://docs.ogc.org/is/07-036/07-036.pdf), `geometryBasic0d1d.xsd`).
pub type AbstractGeometryArrayProperty = ArrayProperty<AbstractGeometryKind>;
