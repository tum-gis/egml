use crate::Error;
use crate::util::{XmlElement, XmlFragmentWriter, XmlNamespace};
use egml_core::model::basic_types::Measure;
use quick_xml::events::Event;
use quick_xml::{Reader, XmlVersion};
use std::io::Write;

pub fn deserialize_measure(xml_document: &[u8]) -> Result<Measure, Error> {
    let mut reader = Reader::from_reader(xml_document);
    reader.config_mut().trim_text(true);

    let mut uom: Option<String> = None;

    loop {
        match reader.read_event()? {
            Event::Start(start) | Event::Empty(start) => {
                for attr in start.attributes() {
                    let attr = attr.map_err(quick_xml::Error::from)?;
                    if attr.key.local_name().as_ref() == "uom" {
                        let value = attr.normalized_value(XmlVersion::Implicit1_0)?.into_owned();
                        uom = Some(value);
                    }
                }
            }
            Event::Text(text) => {
                let raw =
                    quick_xml::escape::unescape(text.as_ref()).map_err(quick_xml::Error::from)?;
                return finish_measure(&raw, uom);
            }
            Event::CData(cdata) => {
                return finish_measure(cdata.as_ref(), uom);
            }
            Event::Eof => {
                return Err(Error::ElementNotFound("measure text content".to_string()));
            }
            _ => {}
        }
    }
}

fn finish_measure(raw: &str, uom: Option<String>) -> Result<Measure, Error> {
    let raw = raw.trim();
    let value = raw
        .parse::<f64>()
        .map_err(|_| egml_core::Error::InvalidAttributeValue {
            attribute: "measure",
            value: raw.to_string(),
        })?;
    let uom = uom.ok_or_else(|| Error::ElementNotFound("uom attribute".to_string()))?;
    Ok(Measure { uom, value })
}

pub fn serialize_measure<N: XmlNamespace, E: XmlElement, W: Write>(
    measure: &Measure,
    xml_fragment_writer: &mut XmlFragmentWriter<W>,
    target_xml_namespace: N,
    target_xml_element: E,
) -> Result<(), Error> {
    xml_fragment_writer.write_leaf_element_with_attributes(
        target_xml_namespace,
        target_xml_element,
        [("uom", measure.uom.as_str())],
        &measure.value.to_string(),
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::Formatting;

    /// Minimal `gen:value`-shaped fixture, standing in for a future
    /// `GenericsNamespace`/ADE element type.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct GenNamespace;

    impl XmlNamespace for GenNamespace {
        fn uri(&self) -> &'static str {
            "urn:x-gen"
        }

        fn default_prefix(&self) -> Option<&'static str> {
            Some("gen")
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct ValueElement;

    impl XmlElement for ValueElement {
        fn from_local_name(local_name: &str) -> Option<Self> {
            (local_name == "value").then_some(Self)
        }

        fn local_name(&self) -> &'static str {
            "value"
        }
    }

    #[test]
    fn deserialize_measure_works() {
        let xml_document = b"<gen:value uom=\"m2\">120.0</gen:value>";

        let measure = deserialize_measure(xml_document).expect("should work");

        assert_eq!(measure.uom, "m2");
        assert_eq!(measure.value, 120.0);
    }

    #[test]
    fn deserialize_measure_fails_without_uom() {
        let xml_document = b"<gen:value>120.0</gen:value>";

        let result = deserialize_measure(xml_document);

        assert!(result.is_err());
    }

    #[test]
    fn serialize_measure_works() {
        let measure = Measure {
            uom: "m2".to_string(),
            value: 120.0,
        };

        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_measure(
            &measure,
            &mut xml_fragment_writer,
            GenNamespace,
            ValueElement,
        )
        .expect("should serialize");
        let xml = String::from_utf8(xml_fragment_writer.into_bytes()).expect("valid UTF-8");

        assert_eq!(xml, r#"<gen:value uom="m2">120</gen:value>"#);
    }

    #[test]
    fn round_trip_measure() {
        let original = Measure {
            uom: "m".to_string(),
            value: 42.5,
        };

        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_measure(
            &original,
            &mut xml_fragment_writer,
            GenNamespace,
            ValueElement,
        )
        .expect("should serialize");
        let xml = xml_fragment_writer.into_bytes();

        let round_tripped = deserialize_measure(&xml).expect("should work");

        assert_eq!(round_tripped, original);
    }
}
