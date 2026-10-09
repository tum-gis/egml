use crate::Error;
use crate::codec::geometry::primitives::abstract_geometry_primitive::{
    deserialize_abstract_geometric_primitive, serialize_abstract_geometric_primitive,
    serialize_abstract_geometric_primitive_attributes,
};
use crate::util::{DeserializationConfig, GmlElement, XmlDocumentIndex, XmlFragmentWriter};
use egml_core::model::geometry::primitives::{AbstractCurve, AsAbstractGeometricPrimitive};
use std::io::Write;

pub fn deserialize_abstract_curve(
    xml_document: &[u8],
    index: &XmlDocumentIndex<GmlElement>,
    config: &DeserializationConfig,
) -> Result<AbstractCurve, Error> {
    debug_assert!(
        !index.is_truncated(),
        "deserialize_abstract_curve received a truncated index — caller must scan to sufficient depth"
    );

    let abstract_geometric_primitive =
        deserialize_abstract_geometric_primitive(xml_document, index, config)?;
    let abstract_curve =
        AbstractCurve::from_abstract_geometric_primitive(abstract_geometric_primitive);

    Ok(abstract_curve)
}

pub fn serialize_abstract_curve<W: Write>(
    abstract_curve: &AbstractCurve,
    xml_fragment_writer: &mut XmlFragmentWriter<W>,
) -> Result<(), Error> {
    serialize_abstract_geometric_primitive(
        abstract_curve.abstract_geometric_primitive(),
        xml_fragment_writer,
    )
}

pub fn serialize_abstract_curve_attributes(
    abstract_curve: &AbstractCurve,
) -> Vec<(String, String)> {
    serialize_abstract_geometric_primitive_attributes(abstract_curve.abstract_geometric_primitive())
}
