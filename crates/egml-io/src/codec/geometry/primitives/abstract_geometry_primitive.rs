use crate::Error;
use crate::codec::geometry::abstract_geometry::{
    deserialize_abstract_geometry, serialize_abstract_geometry,
    serialize_abstract_geometry_attributes,
};
use crate::util::{DeserializationConfig, GmlElement, XmlDocumentIndex, XmlFragmentWriter};
use egml_core::model::geometry::AsAbstractGeometry;
use egml_core::model::geometry::primitives::AbstractGeometricPrimitive;
use std::io::Write;

pub fn deserialize_abstract_geometric_primitive(
    xml_document: &[u8],
    index: &XmlDocumentIndex<GmlElement>,
    config: &DeserializationConfig,
) -> Result<AbstractGeometricPrimitive, Error> {
    debug_assert!(
        !index.is_truncated(),
        "deserialize_abstract_geometric_primitive received a truncated index — caller must scan to sufficient depth"
    );

    let abstract_geometry = deserialize_abstract_geometry(xml_document, index, config)?;
    let abstract_geometric_primitive =
        AbstractGeometricPrimitive::from_abstract_geometry(abstract_geometry);

    Ok(abstract_geometric_primitive)
}

pub fn serialize_abstract_geometric_primitive<W: Write>(
    abstract_geometric_primitive: &AbstractGeometricPrimitive,
    xml_fragment_writer: &mut XmlFragmentWriter<W>,
) -> Result<(), Error> {
    serialize_abstract_geometry(
        abstract_geometric_primitive.abstract_geometry(),
        xml_fragment_writer,
    )
}

pub fn serialize_abstract_geometric_primitive_attributes(
    abstract_geometric_primitive: &AbstractGeometricPrimitive,
) -> Vec<(String, String)> {
    serialize_abstract_geometry_attributes(abstract_geometric_primitive.abstract_geometry())
}
