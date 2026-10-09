use crate::Error;
use crate::codec::geometry::abstract_geometry::{
    deserialize_abstract_geometry, serialize_abstract_geometry,
    serialize_abstract_geometry_attributes,
};
use crate::util::{DeserializationConfig, GmlElement, XmlDocumentIndex, XmlFragmentWriter};
use egml_core::model::geometry::AsAbstractGeometry;
use egml_core::model::geometry::aggregates::AbstractGeometricAggregate;
use std::io::Write;

pub fn deserialize_abstract_geometric_aggregate(
    xml_document: &[u8],
    index: &XmlDocumentIndex<GmlElement>,
    config: &DeserializationConfig,
) -> Result<AbstractGeometricAggregate, Error> {
    debug_assert!(
        !index.is_truncated(),
        "deserialize_abstract_geometric_aggregate received a truncated index — caller must scan to sufficient depth"
    );

    let abstract_geometry = deserialize_abstract_geometry(xml_document, index, config)?;
    let abstract_geometric_aggregate =
        AbstractGeometricAggregate::from_abstract_geometry(abstract_geometry);

    Ok(abstract_geometric_aggregate)
}

pub fn serialize_abstract_geometric_aggregate<W: Write>(
    abstract_geometric_aggregate: &AbstractGeometricAggregate,
    xml_fragment_writer: &mut XmlFragmentWriter<W>,
) -> Result<(), Error> {
    serialize_abstract_geometry(
        abstract_geometric_aggregate.abstract_geometry(),
        xml_fragment_writer,
    )
}

pub fn serialize_abstract_geometric_aggregate_attributes(
    abstract_geometric_aggregate: &AbstractGeometricAggregate,
) -> Vec<(String, String)> {
    serialize_abstract_geometry_attributes(abstract_geometric_aggregate.abstract_geometry())
}
