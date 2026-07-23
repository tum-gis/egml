use crate::Error;
use crate::codec::abstract_object::{deserialize_abstract_object, serialize_abstract_object};
use crate::util::{Formatting, GmlElement, XmlElementSpans, XmlNodeParts};
use egml_core::model::AsAbstractObject;
use egml_core::model::geometry::primitives::AbstractSurfacePatch;

pub fn deserialize_abstract_surface_patch(
    xml_document: &[u8],
    spans: &XmlElementSpans<GmlElement>,
) -> Result<AbstractSurfacePatch, Error> {
    let abstract_object = deserialize_abstract_object(xml_document, spans)?;
    let abstract_surface_patch = AbstractSurfacePatch::from_abstract_object(abstract_object);

    Ok(abstract_surface_patch)
}

pub fn serialize_abstract_surface_patch(
    abstract_surface_patch: &AbstractSurfacePatch,
    formatting: Formatting,
) -> Result<XmlNodeParts, Error> {
    let xml_node_parts =
        serialize_abstract_object(abstract_surface_patch.abstract_object(), formatting)?;

    Ok(xml_node_parts)
}
