use crate::model::base::Property;
use crate::model::geometry::primitives::AbstractSurfaceKind;

/// A surface held inline or by `xlink:href`, corresponding to `gml:SurfacePropertyType`
/// ([OGC 07-036](https://docs.ogc.org/is/07-036/07-036.pdf), `geometryBasic2d.xsd`).
pub type AbstractSurfaceProperty = Property<AbstractSurfaceKind>;
