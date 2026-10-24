use crate::Error;
use crate::codec::base::{
    deserialize_abstract_gml, serialize_abstract_gml, serialize_abstract_gml_attributes,
};
use crate::codec::xs::any_uri_from_lexical;
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

    let srs_name = attributes
        .get(&GmlAttribute::SrsName)
        .map(|s| any_uri_from_lexical(s));
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

#[cfg(test)]
mod tests {
    use super::*;
    use egml_core::model::xs::AnyUri;

    fn deserialize(xml_document: &[u8]) -> AbstractGeometry {
        let index = XmlDocumentIndex::<GmlElement>::from_scan(xml_document, Some(1)).unwrap();
        deserialize_abstract_geometry(xml_document, &index, &DeserializationConfig::default())
            .expect("should deserialize")
    }

    #[test]
    fn deserialize_reads_srs_attributes() {
        let geometry = deserialize(
            b"<gml:MultiSurface srsName=\"EPSG:25832\" srsDimension=\"3\"></gml:MultiSurface>",
        );

        assert_eq!(geometry.srs_name().map(AnyUri::as_str), Some("EPSG:25832"));
        assert_eq!(geometry.srs_dimension(), Some(3));
    }

    #[test]
    fn deserialize_collapses_srs_name_whitespace() {
        let geometry =
            deserialize(b"<gml:MultiSurface srsName=\" EPSG:25832  \"></gml:MultiSurface>");

        assert_eq!(geometry.srs_name().map(AnyUri::as_str), Some("EPSG:25832"));
    }

    #[test]
    fn serialize_attributes_writes_srs_name() {
        let mut geometry = AbstractGeometry::new();
        geometry.set_srs_name("EPSG:25832");

        let attributes = serialize_abstract_geometry_attributes(&geometry);

        assert!(attributes.contains(&("srsName".to_string(), "EPSG:25832".to_string())));
    }
}
