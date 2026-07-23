use crate::Error;
use crate::codec::base::{deserialize_abstract_gml, serialize_abstract_gml};
use crate::util::{Formatting, GmlElement, XmlElementSpans, XmlNodeParts};
use egml_core::model::base::AsAbstractGml;
use egml_core::model::geometry::{AbstractGeometry, AsAbstractGeometry, AsAbstractGeometryMut};
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_abstract_geometry(
    xml_document: &[u8],
    spans: &XmlElementSpans<GmlElement>,
) -> Result<AbstractGeometry, Error> {
    let abstract_gml = deserialize_abstract_gml(xml_document, spans)?;
    let mut abstract_geometry = AbstractGeometry::from_abstract_gml(abstract_gml);

    let parsed: GmlAbstractGeometry = de::from_reader(xml_document)?;
    abstract_geometry.set_srs_name_opt(parsed.srs_name);
    abstract_geometry.set_srs_dimension_opt(parsed.srs_dimension);

    Ok(abstract_geometry)
}

pub fn serialize_abstract_geometry(
    abstract_geometry: &AbstractGeometry,
    formatting: Formatting,
) -> Result<XmlNodeParts, Error> {
    let mut xml_node_parts = serialize_abstract_gml(abstract_geometry.abstract_gml(), formatting)?;

    if let Some(srs_name) = &abstract_geometry.srs_name() {
        xml_node_parts
            .attributes
            .push(("srsName".to_string(), srs_name.to_string()));
    }

    if let Some(srs_dimension) = &abstract_geometry.srs_dimension() {
        xml_node_parts
            .attributes
            .push(("srsDimension".to_string(), srs_dimension.to_string()));
    }

    Ok(xml_node_parts)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlAbstractGeometry {
    #[serde(rename(deserialize = "@srsName"), skip_serializing)]
    pub srs_name: Option<String>,

    #[serde(rename(deserialize = "@srsDimension"), skip_serializing)]
    pub srs_dimension: Option<u32>,
}

impl From<&AbstractGeometry> for GmlAbstractGeometry {
    fn from(item: &AbstractGeometry) -> Self {
        Self {
            srs_name: item.srs_name().cloned(),
            srs_dimension: item.srs_dimension(),
        }
    }
}
