use crate::Error;
use crate::codec::geometry::primitives::abstract_geometry_primitive::{
    deserialize_abstract_geometric_primitive, serialize_abstract_geometric_primitive,
};
use crate::util::{Formatting, GmlElement, XmlElementSpans, XmlNodeParts};
use egml_core::model::geometry::primitives::{AbstractSolid, AsAbstractGeometricPrimitive};

pub fn deserialize_abstract_solid(
    xml_document: &[u8],
    spans: &XmlElementSpans<GmlElement>,
) -> Result<AbstractSolid, Error> {
    let abstract_geometric_primitive =
        deserialize_abstract_geometric_primitive(xml_document, spans)?;
    let abstract_solid =
        AbstractSolid::from_abstract_geometric_primitive(abstract_geometric_primitive);

    Ok(abstract_solid)
}

pub fn serialize_abstract_solid(
    abstract_solid: &AbstractSolid,
    formatting: Formatting,
) -> Result<XmlNodeParts, Error> {
    serialize_abstract_geometric_primitive(
        abstract_solid.abstract_geometric_primitive(),
        formatting,
    )
}
