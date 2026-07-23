use crate::Error;
use crate::codec::geometry::primitives::abstract_geometry_primitive::{
    deserialize_abstract_geometric_primitive, serialize_abstract_geometric_primitive,
};
use crate::util::{Formatting, GmlElement, XmlElementSpans, XmlNodeParts};
use egml_core::model::geometry::primitives::{AbstractCurve, AsAbstractGeometricPrimitive};

pub fn deserialize_abstract_curve(
    xml_document: &[u8],
    spans: &XmlElementSpans<GmlElement>,
) -> Result<AbstractCurve, Error> {
    let abstract_geometric_primitive =
        deserialize_abstract_geometric_primitive(xml_document, spans)?;
    let abstract_curve =
        AbstractCurve::from_abstract_geometric_primitive(abstract_geometric_primitive);

    Ok(abstract_curve)
}

pub fn serialize_abstract_curve(
    abstract_curve: &AbstractCurve,
    formatting: Formatting,
) -> Result<XmlNodeParts, Error> {
    serialize_abstract_geometric_primitive(
        abstract_curve.abstract_geometric_primitive(),
        formatting,
    )
}
