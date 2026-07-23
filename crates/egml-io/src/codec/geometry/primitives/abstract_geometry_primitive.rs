use crate::Error;
use crate::codec::geometry::abstract_geometry::{
    deserialize_abstract_geometry, serialize_abstract_geometry,
};
use crate::util::{Formatting, GmlElement, XmlElementSpans, XmlNodeParts};
use egml_core::model::geometry::AsAbstractGeometry;
use egml_core::model::geometry::primitives::AbstractGeometricPrimitive;

pub fn deserialize_abstract_geometric_primitive(
    xml_document: &[u8],
    spans: &XmlElementSpans<GmlElement>,
) -> Result<AbstractGeometricPrimitive, Error> {
    let abstract_geometry = deserialize_abstract_geometry(xml_document, spans)?;
    let abstract_geometric_primitive =
        AbstractGeometricPrimitive::from_abstract_geometry(abstract_geometry);

    Ok(abstract_geometric_primitive)
}

pub fn serialize_abstract_geometric_primitive(
    abstract_geometric_primitive: &AbstractGeometricPrimitive,
    formatting: Formatting,
) -> Result<XmlNodeParts, Error> {
    serialize_abstract_geometry(abstract_geometric_primitive.abstract_geometry(), formatting)
}
