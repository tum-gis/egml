use crate::Error;
use crate::codec::geometry::primitives::abstract_geometry_primitive::{
    deserialize_abstract_geometric_primitive, serialize_abstract_geometric_primitive,
    serialize_abstract_geometric_primitive_attributes,
};
use crate::util::{DeserializationConfig, GmlElement, XmlDocumentIndex, XmlFragmentWriter};
use egml_core::model::geometry::primitives::{AbstractSurface, AsAbstractGeometricPrimitive};
use std::io::Write;

pub fn deserialize_abstract_surface(
    xml_document: &[u8],
    index: &XmlDocumentIndex<GmlElement>,
    config: &DeserializationConfig,
) -> Result<AbstractSurface, Error> {
    debug_assert!(
        !index.is_truncated(),
        "deserialize_abstract_surface received a truncated index — caller must scan to sufficient depth"
    );

    let abstract_geometric_primitive =
        deserialize_abstract_geometric_primitive(xml_document, index, config)?;
    let abstract_surface =
        AbstractSurface::from_abstract_geometric_primitive(abstract_geometric_primitive);

    Ok(abstract_surface)
}

pub fn serialize_abstract_surface<W: Write>(
    abstract_surface: &AbstractSurface,
    xml_fragment_writer: &mut XmlFragmentWriter<W>,
) -> Result<(), Error> {
    serialize_abstract_geometric_primitive(
        abstract_surface.abstract_geometric_primitive(),
        xml_fragment_writer,
    )
}

pub fn serialize_abstract_surface_attributes(
    abstract_surface: &AbstractSurface,
) -> Vec<(String, String)> {
    serialize_abstract_geometric_primitive_attributes(
        abstract_surface.abstract_geometric_primitive(),
    )
}
