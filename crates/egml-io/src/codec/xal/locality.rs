use crate::Error;
use crate::codec::xal::enums::{
    locality_name_type_str, locality_type_str, parse_locality_name_type, parse_locality_type,
};
use crate::util::{
    DeserializationConfig, XalElement, XalNamespace, XmlDocumentIndex, XmlFragmentWriter,
    collect_children,
};
use egml_core::model::xal::enums::{LocalityNameType, LocalityType};
use egml_core::model::xal::{Locality, LocalityName};
use quick_xml::events::{BytesStart, Event};
use quick_xml::{Reader, XmlVersion};
use std::io::Write;

pub fn deserialize_locality(
    xml_document: &[u8],
    index: &XmlDocumentIndex<XalElement>,
    config: &DeserializationConfig,
) -> Result<Locality, Error> {
    let index = if index.is_truncated() {
        &XmlDocumentIndex::from_scan(xml_document, config.rescan_depth())?
    } else {
        index
    };

    let name_elements = collect_children(
        xml_document,
        index,
        XalElement::NameElement,
        config,
        deserialize_locality_name,
    )?;

    let mut locality = Locality::new(name_elements);
    if let Some(locality_type) = read_locality_type_attribute(xml_document)? {
        locality.set_locality_type(locality_type);
    }

    Ok(locality)
}

fn read_locality_type_attribute(xml_document: &[u8]) -> Result<Option<LocalityType>, Error> {
    let mut reader = Reader::from_reader(xml_document);
    loop {
        match reader.read_event()? {
            Event::Start(start) | Event::Empty(start) => {
                for attr in start.attributes() {
                    let attr = attr.map_err(quick_xml::Error::from)?;
                    if attr.key.local_name().as_ref() != "Type" {
                        continue;
                    }
                    let value = attr.normalized_value(XmlVersion::Implicit1_0)?;
                    return Ok(parse_locality_type(value.as_ref()));
                }
                return Ok(None);
            }
            Event::Eof => return Ok(None),
            _ => {}
        }
    }
}

fn deserialize_locality_name(
    xml_document: &[u8],
    _index: &XmlDocumentIndex<XalElement>,
    _config: &DeserializationConfig,
) -> Result<LocalityName, Error> {
    let mut reader = Reader::from_reader(xml_document);
    let mut name_type = None;

    loop {
        match reader.read_event()? {
            Event::Start(start) => {
                name_type = read_name_type_attribute(&start)?;
            }
            Event::Empty(start) => {
                name_type = read_name_type_attribute(&start)?;
                return Ok(LocalityName::with_optional_name_type(
                    String::new(),
                    name_type,
                ));
            }
            Event::Text(text) => {
                let raw =
                    quick_xml::escape::unescape(text.as_ref()).map_err(quick_xml::Error::from)?;
                return Ok(LocalityName::with_optional_name_type(
                    raw.into_owned(),
                    name_type,
                ));
            }
            Event::CData(cdata) => {
                return Ok(LocalityName::with_optional_name_type(
                    cdata.as_ref().to_string(),
                    name_type,
                ));
            }
            Event::End(_) => {
                return Ok(LocalityName::with_optional_name_type(
                    String::new(),
                    name_type,
                ));
            }
            Event::Eof => {
                return Err(Error::ElementNotFound(
                    "xAL:NameElement text content".to_string(),
                ));
            }
            _ => {}
        }
    }
}

fn read_name_type_attribute(start: &BytesStart) -> Result<Option<LocalityNameType>, Error> {
    for attr in start.attributes() {
        let attr = attr.map_err(quick_xml::Error::from)?;
        if attr.key.local_name().as_ref() != "NameType" {
            continue;
        }
        let value = attr.normalized_value(XmlVersion::Implicit1_0)?;
        return Ok(parse_locality_name_type(value.as_ref()));
    }
    Ok(None)
}

pub fn serialize_locality<W: Write>(
    locality: &Locality,
    xml_fragment_writer: &mut XmlFragmentWriter<W>,
) -> Result<(), Error> {
    match locality.locality_type() {
        Some(locality_type) => xml_fragment_writer.write_start_event_with_attributes(
            XalNamespace::Xal,
            XalElement::Locality,
            [("xAL:Type", locality_type_str(locality_type))],
        )?,
        None => xml_fragment_writer.write_start_event(XalNamespace::Xal, XalElement::Locality)?,
    }

    for name_element in locality.name_elements() {
        match name_element.name_type() {
            Some(name_type) => xml_fragment_writer.write_leaf_element_with_attributes(
                XalNamespace::Xal,
                XalElement::NameElement,
                [("xAL:NameType", locality_name_type_str(name_type))],
                name_element.content(),
            )?,
            None => xml_fragment_writer.write_leaf_element_with_attributes(
                XalNamespace::Xal,
                XalElement::NameElement,
                [] as [(&str, &str); 0],
                name_element.content(),
            )?,
        }
    }

    xml_fragment_writer.write_end_event(XalNamespace::Xal, XalElement::Locality)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{deserialize_locality, serialize_locality};
    use crate::util::{
        DeserializationConfig, Formatting, XalElement, XmlDocumentIndex, XmlFragmentWriter,
    };
    use egml_core::model::xal::enums::{LocalityNameType, LocalityType};
    use egml_core::model::xal::{Locality, LocalityName};

    fn bhavani() -> Locality {
        let mut locality = Locality::new(vec![LocalityName::with_name_type(
            "Bhavani".to_string(),
            LocalityNameType::Name,
        )]);
        locality.set_locality_type(LocalityType::Town);
        locality
    }

    #[test]
    fn deserialize_locality_reads_name_elements_and_type() {
        let xml_document = br#"<xAL:Locality xAL:Type="Town"><xAL:NameElement xAL:NameType="Name">Bhavani</xAL:NameElement></xAL:Locality>"#;

        let index = XmlDocumentIndex::<XalElement>::from_scan(xml_document, None).unwrap();
        let locality =
            deserialize_locality(xml_document, &index, &DeserializationConfig::default())
                .expect("should deserialize");

        assert_eq!(locality, bhavani());
    }

    #[test]
    fn serialize_locality_writes_name_elements_and_type() {
        let mut writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_locality(&bhavani(), &mut writer).expect("should serialize");
        let xml = String::from_utf8(writer.into_bytes()).unwrap();

        assert_eq!(
            xml,
            r#"<xAL:Locality xAL:Type="Town"><xAL:NameElement xAL:NameType="Name">Bhavani</xAL:NameElement></xAL:Locality>"#
        );
    }

    #[test]
    fn round_trip_locality() {
        let original = bhavani();

        let mut writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_locality(&original, &mut writer).unwrap();
        let xml = writer.into_bytes();

        let index = XmlDocumentIndex::<XalElement>::from_scan(&xml, None).unwrap();
        let recovered =
            deserialize_locality(&xml, &index, &DeserializationConfig::default()).unwrap();

        assert_eq!(recovered, original);
    }
}
