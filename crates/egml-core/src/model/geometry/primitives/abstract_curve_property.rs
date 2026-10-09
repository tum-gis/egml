use crate::model::base::Property;
use crate::model::geometry::primitives::AbstractCurveKind;

/// A curve held inline or by `xlink:href`, corresponding to `gml:CurvePropertyType`
/// ([OGC 07-036](https://docs.ogc.org/is/07-036/07-036.pdf), `geometryBasic0d1d.xsd`).
pub type AbstractCurveProperty = Property<AbstractCurveKind>;
