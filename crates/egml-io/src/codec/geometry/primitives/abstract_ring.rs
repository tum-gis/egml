use crate::Error;
use crate::codec::geometry::primitives::abstract_curve::{
    deserialize_abstract_curve, serialize_abstract_curve,
};
use crate::util::{Formatting, GmlElement, XmlElementSpans, XmlNodeParts};
use egml_core::model::geometry::primitives::{AbstractRing, AsAbstractCurve};

pub fn deserialize_abstract_ring(
    xml_document: &[u8],
    spans: &XmlElementSpans<GmlElement>,
) -> Result<AbstractRing, Error> {
    let abstract_curve = deserialize_abstract_curve(xml_document, spans)?;
    let abstract_ring = AbstractRing::from_abstract_curve(abstract_curve);

    Ok(abstract_ring)
}

pub fn serialize_abstract_ring(
    abstract_ring: &AbstractRing,
    formatting: Formatting,
) -> Result<XmlNodeParts, Error> {
    serialize_abstract_curve(abstract_ring.abstract_curve(), formatting)
}
