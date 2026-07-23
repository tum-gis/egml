use crate::Error;
use crate::codec::geometry::primitives::abstract_geometry_primitive::{
    deserialize_abstract_geometric_primitive, serialize_abstract_geometric_primitive,
};
use crate::util::{Formatting, GmlElement, XmlElementSpans, XmlNodeParts};
use egml_core::model::geometry::primitives::{AbstractSurface, AsAbstractGeometricPrimitive};

pub fn deserialize_abstract_surface(
    xml_document: &[u8],
    spans: &XmlElementSpans<GmlElement>,
) -> Result<AbstractSurface, Error> {
    let abstract_geometric_primitive =
        deserialize_abstract_geometric_primitive(xml_document, spans)?;
    let abstract_surface =
        AbstractSurface::from_abstract_geometric_primitive(abstract_geometric_primitive);

    Ok(abstract_surface)
}

pub fn serialize_abstract_surface(
    abstract_surface: &AbstractSurface,
    formatting: Formatting,
) -> Result<XmlNodeParts, Error> {
    serialize_abstract_geometric_primitive(
        abstract_surface.abstract_geometric_primitive(),
        formatting,
    )
}
