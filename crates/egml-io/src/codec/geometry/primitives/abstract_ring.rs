use crate::Error;
use crate::codec::geometry::primitives::abstract_curve::{
    deserialize_abstract_curve, serialize_abstract_curve, serialize_abstract_curve_attributes,
};
use crate::util::{DeserializationConfig, GmlElement, XmlDocumentIndex, XmlFragmentWriter};
use egml_core::model::geometry::primitives::{AbstractRing, AsAbstractCurve};
use std::io::Write;

pub fn deserialize_abstract_ring(
    xml_document: &[u8],
    index: &XmlDocumentIndex<GmlElement>,
    config: &DeserializationConfig,
) -> Result<AbstractRing, Error> {
    debug_assert!(
        !index.is_truncated(),
        "deserialize_abstract_ring received a truncated index — caller must scan to sufficient depth"
    );

    let abstract_curve = deserialize_abstract_curve(xml_document, index, config)?;
    let abstract_ring = AbstractRing::from_abstract_curve(abstract_curve);

    Ok(abstract_ring)
}

pub fn serialize_abstract_ring<W: Write>(
    abstract_ring: &AbstractRing,
    xml_fragment_writer: &mut XmlFragmentWriter<W>,
) -> Result<(), Error> {
    serialize_abstract_curve(abstract_ring.abstract_curve(), xml_fragment_writer)
}

pub fn serialize_abstract_ring_attributes(abstract_ring: &AbstractRing) -> Vec<(String, String)> {
    serialize_abstract_curve_attributes(abstract_ring.abstract_curve())
}
