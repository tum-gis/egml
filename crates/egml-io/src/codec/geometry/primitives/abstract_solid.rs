use crate::Error;
use crate::codec::geometry::primitives::abstract_geometry_primitive::{
    deserialize_abstract_geometric_primitive, serialize_abstract_geometric_primitive,
    serialize_abstract_geometric_primitive_attributes,
};
use crate::util::{DeserializationConfig, GmlElement, XmlDocumentIndex, XmlFragmentWriter};
use egml_core::model::geometry::primitives::{AbstractSolid, AsAbstractGeometricPrimitive};
use std::io::Write;

pub fn deserialize_abstract_solid(
    xml_document: &[u8],
    index: &XmlDocumentIndex<GmlElement>,
    config: &DeserializationConfig,
) -> Result<AbstractSolid, Error> {
    debug_assert!(
        !index.is_truncated(),
        "deserialize_abstract_solid received a truncated index — caller must scan to sufficient depth"
    );

    let abstract_geometric_primitive =
        deserialize_abstract_geometric_primitive(xml_document, index, config)?;
    let abstract_solid =
        AbstractSolid::from_abstract_geometric_primitive(abstract_geometric_primitive);

    Ok(abstract_solid)
}

pub fn serialize_abstract_solid<W: Write>(
    abstract_solid: &AbstractSolid,
    xml_fragment_writer: &mut XmlFragmentWriter<W>,
) -> Result<(), Error> {
    serialize_abstract_geometric_primitive(
        abstract_solid.abstract_geometric_primitive(),
        xml_fragment_writer,
    )
}

pub fn serialize_abstract_solid_attributes(
    abstract_solid: &AbstractSolid,
) -> Vec<(String, String)> {
    serialize_abstract_geometric_primitive_attributes(abstract_solid.abstract_geometric_primitive())
}
