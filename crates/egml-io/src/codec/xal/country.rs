use crate::Error;
use crate::codec::xal::enums::{country_name_type_str, parse_country_name_type};
use crate::util::{
    DeserializationConfig, XalElement, XalNamespace, XmlDocumentIndex, XmlFragmentWriter,
    collect_children,
};
use egml_core::model::xal::enums::CountryNameType;
use egml_core::model::xal::{Country, CountryName};
use quick_xml::events::{BytesStart, Event};
use quick_xml::{Reader, XmlVersion};
use std::io::Write;

pub fn deserialize_country(
    xml_document: &[u8],
    index: &XmlDocumentIndex<XalElement>,
    config: &DeserializationConfig,
) -> Result<Country, Error> {
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
        deserialize_name_element,
    )?;

    Ok(Country::new(name_elements))
}

fn deserialize_name_element(
    xml_document: &[u8],
    _index: &XmlDocumentIndex<XalElement>,
    _config: &DeserializationConfig,
) -> Result<CountryName, Error> {
    let mut reader = Reader::from_reader(xml_document);
    let mut name_type = None;

    loop {
        match reader.read_event()? {
            Event::Start(start) => {
                name_type = read_name_type_attribute(&start)?;
            }
            Event::Empty(start) => {
                name_type = read_name_type_attribute(&start)?;
                return Ok(CountryName::with_optional_name_type(
                    String::new(),
                    name_type,
                ));
            }
            Event::Text(text) => {
                let raw =
                    quick_xml::escape::unescape(text.as_ref()).map_err(quick_xml::Error::from)?;
                return Ok(CountryName::with_optional_name_type(
                    raw.into_owned(),
                    name_type,
                ));
            }
            Event::CData(cdata) => {
                return Ok(CountryName::with_optional_name_type(
                    cdata.as_ref().to_string(),
                    name_type,
                ));
            }
            Event::End(_) => {
                return Ok(CountryName::with_optional_name_type(
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

fn read_name_type_attribute(start: &BytesStart) -> Result<Option<CountryNameType>, Error> {
    for attr in start.attributes() {
        let attr = attr.map_err(quick_xml::Error::from)?;
        if attr.key.local_name().as_ref() != "NameType" {
            continue;
        }
        let value = attr.normalized_value(XmlVersion::Implicit1_0)?;
        return Ok(parse_country_name_type(value.as_ref()));
    }
    Ok(None)
}

pub fn serialize_country<W: Write>(
    country: &Country,
    xml_fragment_writer: &mut XmlFragmentWriter<W>,
) -> Result<(), Error> {
    xml_fragment_writer.write_start_event(XalNamespace::Xal, XalElement::Country)?;

    for name_element in country.name_elements() {
        match name_element.name_type() {
            Some(name_type) => xml_fragment_writer.write_leaf_element_with_attributes(
                XalNamespace::Xal,
                XalElement::NameElement,
                [("xAL:NameType", country_name_type_str(name_type))],
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

    xml_fragment_writer.write_end_event(XalNamespace::Xal, XalElement::Country)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{deserialize_country, serialize_country};
    use crate::util::{
        DeserializationConfig, Formatting, XalElement, XmlDocumentIndex, XmlFragmentWriter,
    };
    use egml_core::model::xal::enums::CountryNameType;
    use egml_core::model::xal::{Country, CountryName};

    #[test]
    fn deserialize_country_reads_name_elements() {
        let xml_document =
            br#"<xAL:Country><xAL:NameElement xAL:NameType="Name">Germany</xAL:NameElement></xAL:Country>"#;

        let index = XmlDocumentIndex::<XalElement>::from_scan(xml_document, None).unwrap();
        let country = deserialize_country(xml_document, &index, &DeserializationConfig::default())
            .expect("should deserialize");

        assert_eq!(country.name_elements().len(), 1);
        assert_eq!(country.name_elements()[0].content(), "Germany");
        assert_eq!(
            country.name_elements()[0].name_type(),
            Some(CountryNameType::Name)
        );
    }

    #[test]
    fn serialize_country_writes_name_elements() {
        let name_element =
            CountryName::with_name_type("Germany".to_string(), CountryNameType::Name);
        let country = Country::new(vec![name_element]);

        let mut writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_country(&country, &mut writer).expect("should serialize");
        let xml = String::from_utf8(writer.into_bytes()).unwrap();

        assert_eq!(
            xml,
            r#"<xAL:Country><xAL:NameElement xAL:NameType="Name">Germany</xAL:NameElement></xAL:Country>"#
        );
    }

    #[test]
    fn round_trip_country() {
        let name_element =
            CountryName::with_name_type("Germany".to_string(), CountryNameType::Name);
        let original = Country::new(vec![name_element]);

        let mut writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_country(&original, &mut writer).unwrap();
        let xml = writer.into_bytes();

        let index = XmlDocumentIndex::<XalElement>::from_scan(&xml, None).unwrap();
        let recovered =
            deserialize_country(&xml, &index, &DeserializationConfig::default()).unwrap();

        assert_eq!(recovered, original);
    }
}
