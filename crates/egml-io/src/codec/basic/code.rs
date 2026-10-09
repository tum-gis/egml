use crate::Error;
use crate::util::{XmlElement, XmlFragmentWriter, XmlNamespace};
use egml_core::model::basic_types::Code;
use quick_xml::events::{BytesStart, Event};
use quick_xml::{Reader, XmlVersion};
use std::io::Write;

pub fn deserialize_code(xml_document: &[u8]) -> Result<Code, Error> {
    let mut reader = Reader::from_reader(xml_document);

    let mut code_space: Option<String> = None;

    loop {
        match reader.read_event()? {
            Event::Start(start) => {
                code_space = read_code_space(&start)?;
            }
            Event::Empty(start) => {
                code_space = read_code_space(&start)?;
                return Ok(Code::from_parts(code_space, String::new()));
            }
            Event::Text(text) => {
                let raw =
                    quick_xml::escape::unescape(text.as_ref()).map_err(quick_xml::Error::from)?;
                return Ok(Code::from_parts(code_space, raw.into_owned()));
            }
            Event::CData(cdata) => {
                return Ok(Code::from_parts(code_space, cdata.as_ref().to_string()));
            }
            Event::End(_) => {
                return Ok(Code::from_parts(code_space, String::new()));
            }
            Event::Eof => {
                return Err(Error::ElementNotFound("code start".to_string()));
            }
            _ => {}
        }
    }
}

fn read_code_space(start: &BytesStart<'_>) -> Result<Option<String>, Error> {
    for attr in start.attributes() {
        let attr = attr.map_err(quick_xml::Error::from)?;
        if attr.key.local_name().as_ref() == "codeSpace" {
            let value = attr.normalized_value(XmlVersion::Implicit1_0)?.into_owned();
            return Ok(Some(value));
        }
    }
    Ok(None)
}

pub fn serialize_code<N: XmlNamespace, E: XmlElement, W: Write>(
    code: &Code,
    xml_fragment_writer: &mut XmlFragmentWriter<W>,
    target_xml_namespace: N,
    target_xml_element: E,
) -> Result<(), Error> {
    let attributes = code
        .code_space()
        .map(|code_space| ("codeSpace", code_space));

    xml_fragment_writer.write_leaf_element_with_attributes(
        target_xml_namespace,
        target_xml_element,
        attributes,
        code.value(),
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::{Formatting, GmlElement, GmlNamespace};
    use egml_core::model::basic_types::Code;

    #[test]
    fn deserialize_code_without_code_space() {
        let xml = b"<tran:function>2</tran:function>";

        let code = deserialize_code(xml).expect("should work");

        assert!(code.code_space().is_none());
        assert_eq!(code.value(), "2");
    }

    #[test]
    fn deserialize_code_with_code_space() {
        let xml = b"<bldg:class codeSpace=\"http://www.sig3d.org/codelists/citygml/2.0/building/2.0/_AbstractBuilding_class.xml\">1000</bldg:class>";

        let code = deserialize_code(xml).expect("should work");

        assert_eq!(
            code.code_space(),
            Some(
                "http://www.sig3d.org/codelists/citygml/2.0/building/2.0/_AbstractBuilding_class.xml"
            )
        );
        assert_eq!(code.value(), "1000");
    }

    #[test]
    fn deserialize_code_self_closing_has_empty_value() {
        let xml = b"<gml:name/>";

        let code = deserialize_code(xml).expect("should work");

        assert!(code.code_space().is_none());
        assert_eq!(code.value(), "");
    }

    #[test]
    fn deserialize_code_with_no_text_content_has_empty_value() {
        let xml = b"<gml:name></gml:name>";

        let code = deserialize_code(xml).expect("should work");

        assert_eq!(code.value(), "");
    }

    #[test]
    fn serialize_code_without_code_space() {
        let code = Code::from_parts(None::<String>, "2".to_string());
        let mut writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);

        serialize_code(
            &code,
            &mut writer,
            GmlNamespace::Gml,
            GmlElement::NameProperty,
        )
        .expect("should serialize");

        let xml = String::from_utf8(writer.into_bytes()).expect("valid UTF-8");
        assert_eq!(xml, "<gml:name>2</gml:name>");
    }

    #[test]
    fn serialize_code_with_code_space() {
        let code = Code::with_code_space("http://example.org/codes.xml", "1000");
        let mut writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);

        serialize_code(
            &code,
            &mut writer,
            GmlNamespace::Gml,
            GmlElement::NameProperty,
        )
        .expect("should serialize");

        let xml = String::from_utf8(writer.into_bytes()).expect("valid UTF-8");
        assert_eq!(
            xml,
            r#"<gml:name codeSpace="http://example.org/codes.xml">1000</gml:name>"#
        );
    }

    #[test]
    fn round_trip_without_code_space() {
        let original = Code::from_parts(None::<String>, "42".to_string());
        let mut writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_code(
            &original,
            &mut writer,
            GmlNamespace::Gml,
            GmlElement::NameProperty,
        )
        .expect("should serialize");
        let xml = writer.into_bytes();

        let round_tripped = deserialize_code(&xml).expect("should work");

        assert_eq!(round_tripped, original);
    }

    #[test]
    fn round_trip_with_code_space() {
        let original = Code::with_code_space(
            "http://www.sig3d.org/codelists/citygml/2.0/building/2.0/_AbstractBuilding_class.xml",
            "1000",
        );
        let mut writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_code(
            &original,
            &mut writer,
            GmlNamespace::Gml,
            GmlElement::NameProperty,
        )
        .expect("should serialize");
        let xml = writer.into_bytes();

        let round_tripped = deserialize_code(&xml).expect("should work");

        assert_eq!(round_tripped, original);
    }
}
