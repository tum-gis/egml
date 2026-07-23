use crate::Error;
use crate::util::{Formatting, GmlElement, XmlElementSpans, XmlNodeParts};
use egml_core::model::AbstractObject;

pub fn deserialize_abstract_object(
    _xml_document: &[u8],
    _spans: &XmlElementSpans<GmlElement>,
) -> Result<AbstractObject, Error> {
    let abstract_object = AbstractObject::default();

    Ok(abstract_object)
}

pub fn serialize_abstract_object(
    _abstract_object: &AbstractObject,
    _formatting: Formatting,
) -> Result<XmlNodeParts, Error> {
    Ok(XmlNodeParts::empty())
}
