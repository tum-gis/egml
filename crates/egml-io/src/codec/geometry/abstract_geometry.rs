use crate::Error;
use crate::codec::base::{
    deserialize_abstract_gml, serialize_abstract_gml, serialize_abstract_gml_attributes,
};
use crate::util::{
    DeserializationConfig, GmlAttribute, GmlElement, XmlDocumentIndex, XmlElement,
    XmlFragmentWriter, deserialize_gml_attributes,
};
use egml_core::model::base::AsAbstractGml;
use egml_core::model::geometry::{AbstractGeometry, AsAbstractGeometry, AsAbstractGeometryMut};
use std::io::Write;

pub fn deserialize_abstract_geometry(
    xml_document: &[u8],
    index: &XmlDocumentIndex<GmlElement>,
    config: &DeserializationConfig,
) -> Result<AbstractGeometry, Error> {
    debug_assert!(
        !index.is_truncated(),
        "deserialize_abstract_geometry received a truncated index — caller must scan to sufficient depth"
    );

    let abstract_gml = deserialize_abstract_gml(xml_document, index, config)?;
    let mut abstract_geometry = AbstractGeometry::from_abstract_gml(abstract_gml);

    let attributes = deserialize_gml_attributes(xml_document)?;
    let srs_dimension: Option<u32> = attributes
        .get(&GmlAttribute::SrsDimension)
        .map(|s| {
            s.parse()
                .map_err(|_| egml_core::Error::InvalidAttributeValue {
                    attribute: "srsDimension",
                    value: s.clone(),
                })
        })
        .transpose()?;
    abstract_geometry.set_srs_dimension_opt(srs_dimension);

    let srs_name: Option<String> = attributes.get(&GmlAttribute::SrsName).cloned();
    abstract_geometry.set_srs_name_opt(srs_name);

    Ok(abstract_geometry)
}

pub fn serialize_abstract_geometry<W: Write>(
    abstract_geometry: &AbstractGeometry,
    xml_fragment_writer: &mut XmlFragmentWriter<W>,
) -> Result<(), Error> {
    serialize_abstract_gml(abstract_geometry.abstract_gml(), xml_fragment_writer)
}

pub fn serialize_abstract_geometry_attributes(
    abstract_geometry: &AbstractGeometry,
) -> Vec<(String, String)> {
    let mut attributes = serialize_abstract_gml_attributes(abstract_geometry.abstract_gml());

    if let Some(srs_dimension) = &abstract_geometry.srs_dimension() {
        attributes.push((
            GmlAttribute::SrsDimension.local_name().to_string(),
            srs_dimension.to_string(),
        ));
    }

    if let Some(srs_name) = &abstract_geometry.srs_name() {
        attributes.push((
            GmlAttribute::SrsName.local_name().to_string(),
            srs_name.to_string(),
        ));
    }

    attributes
}
