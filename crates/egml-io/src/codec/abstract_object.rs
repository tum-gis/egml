use crate::Error;
use crate::util::{DeserializationConfig, GmlElement, XmlDocumentIndex, XmlFragmentWriter};
use egml_core::model::AbstractObject;
use std::io::Write;

pub fn deserialize_abstract_object(
    _xml_document: &[u8],
    _index: &XmlDocumentIndex<GmlElement>,
    _config: &DeserializationConfig,
) -> Result<AbstractObject, Error> {
    let abstract_object = AbstractObject::default();

    Ok(abstract_object)
}

pub fn serialize_abstract_object<W: Write>(
    _abstract_object: &AbstractObject,
    _xml_fragment_writer: &mut XmlFragmentWriter<W>,
) -> Result<(), Error> {
    Ok(())
}

pub fn serialize_abstract_object_attributes(
    _abstract_object: &AbstractObject,
) -> Vec<(String, String)> {
    Vec::new()
}
