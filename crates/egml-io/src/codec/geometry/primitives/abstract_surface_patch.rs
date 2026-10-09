use crate::Error;
use crate::codec::abstract_object::{
    deserialize_abstract_object, serialize_abstract_object, serialize_abstract_object_attributes,
};
use crate::util::{DeserializationConfig, GmlElement, XmlDocumentIndex, XmlFragmentWriter};
use egml_core::model::AsAbstractObject;
use egml_core::model::geometry::primitives::AbstractSurfacePatch;
use std::io::Write;

pub fn deserialize_abstract_surface_patch(
    xml_document: &[u8],
    index: &XmlDocumentIndex<GmlElement>,
    config: &DeserializationConfig,
) -> Result<AbstractSurfacePatch, Error> {
    debug_assert!(
        !index.is_truncated(),
        "deserialize_abstract_surface_patch received a truncated index — caller must scan to sufficient depth"
    );

    let abstract_object = deserialize_abstract_object(xml_document, index, config)?;
    let abstract_surface_patch = AbstractSurfacePatch::from_abstract_object(abstract_object);

    Ok(abstract_surface_patch)
}

pub fn serialize_abstract_surface_patch<W: Write>(
    abstract_surface_patch: &AbstractSurfacePatch,
    xml_fragment_writer: &mut XmlFragmentWriter<W>,
) -> Result<(), Error> {
    serialize_abstract_object(
        abstract_surface_patch.abstract_object(),
        xml_fragment_writer,
    )?;

    Ok(())
}

pub fn serialize_abstract_surface_patch_attributes(
    abstract_surface_patch: &AbstractSurfacePatch,
) -> Vec<(String, String)> {
    serialize_abstract_object_attributes(abstract_surface_patch.abstract_object())
}
