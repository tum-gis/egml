use crate::Error;
use crate::codec::geometry::abstract_geometry::{
    deserialize_abstract_geometry, serialize_abstract_geometry,
};
use crate::util::{Formatting, GmlElement, XmlElementSpans, XmlNodeParts};
use egml_core::model::geometry::AsAbstractGeometry;
use egml_core::model::geometry::aggregates::AbstractGeometricAggregate;

pub fn deserialize_abstract_geometric_aggregate(
    xml_document: &[u8],
    spans: &XmlElementSpans<GmlElement>,
) -> Result<AbstractGeometricAggregate, Error> {
    let abstract_geometry = deserialize_abstract_geometry(xml_document, spans)?;
    let abstract_geometric_aggregate =
        AbstractGeometricAggregate::from_abstract_geometry(abstract_geometry);

    Ok(abstract_geometric_aggregate)
}

pub fn serialize_abstract_geometric_aggregate(
    abstract_geometric_aggregate: &AbstractGeometricAggregate,
    formatting: Formatting,
) -> Result<XmlNodeParts, Error> {
    serialize_abstract_geometry(abstract_geometric_aggregate.abstract_geometry(), formatting)
}
