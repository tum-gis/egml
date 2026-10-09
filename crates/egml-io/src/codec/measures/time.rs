use crate::Error;
use crate::util::{XmlElement, XmlFragmentWriter, XmlNamespace};
use egml_core::model::measures::Time;
use quick_xml::events::Event;
use quick_xml::{Reader, XmlVersion};
use std::io::Write;

/// Parses a `gml:TimeType` fragment (e.g. `<time uom="s">3.0</time>`)
/// directly via [`quick_xml::Reader`].
///
/// Takes the reader positioned just before the element's `Start`/`Empty`
/// event rather than owning it, so it can be called on a reader shared with
/// (and left positioned for) surrounding parsing code. The caller is
/// responsible for configuring the reader (e.g. `trim_text`).
///
/// # Errors
///
/// Returns an error if the upcoming XML is not well-formed, the element has
/// no `uom` attribute, the element has no text content, or the text content
/// is not a valid `f64`.
pub fn deserialize_time(reader: &mut Reader<&[u8]>) -> Result<Time, Error> {
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
                return finish_time(&raw, uom);
            }
            Event::CData(cdata) => {
                return finish_time(cdata.as_ref(), uom);
            }
            Event::Eof => {
                return Err(Error::ElementNotFound("time text content".to_string()));
            }
            _ => {}
        }
    }
}

fn finish_time(raw: &str, uom: Option<String>) -> Result<Time, Error> {
    let raw = raw.trim();
    let value = raw
        .parse::<f64>()
        .map_err(|_| egml_core::Error::InvalidAttributeValue {
            attribute: "time",
            value: raw.to_string(),
        })?;
    let uom = uom.ok_or_else(|| Error::ElementNotFound("uom attribute".to_string()))?;
    Ok(Time::new(value, uom))
}

pub fn serialize_time<N: XmlNamespace, E: XmlElement, W: Write>(
    time: &Time,
    xml_fragment_writer: &mut XmlFragmentWriter<W>,
    target_xml_namespace: N,
    target_xml_element: E,
) -> Result<(), Error> {
    xml_fragment_writer.write_leaf_element_with_attributes(
        target_xml_namespace,
        target_xml_element,
        [("uom", time.uom())],
        &time.value().to_string(),
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::{Formatting, GmlElement, GmlNamespace};
    use egml_core::model::measures::Time;

    fn reader_for(xml_document: &[u8]) -> Reader<&[u8]> {
        let mut reader = Reader::from_reader(xml_document);
        reader.config_mut().trim_text(true);
        reader
    }

    #[test]
    fn deserialize_time_works() {
        let xml_document = b"<time uom=\"s\">3.0</time>";
        let mut reader = reader_for(xml_document);

        let time = deserialize_time(&mut reader).expect("should work");

        assert_eq!(time.uom(), "s");
        assert_eq!(time.value(), 3.0);
    }

    #[test]
    fn deserialize_time_fails_without_uom() {
        let xml_document = b"<time>3.0</time>";
        let mut reader = reader_for(xml_document);

        let result = deserialize_time(&mut reader);

        assert!(result.is_err());
    }

    #[test]
    fn serialize_time_works() {
        let time = Time::new(3.0, "s");
        let mut writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);

        serialize_time(
            &time,
            &mut writer,
            GmlNamespace::Gml,
            GmlElement::NameProperty,
        )
        .expect("should serialize");

        let xml = String::from_utf8(writer.into_bytes()).expect("valid UTF-8");
        assert_eq!(xml, "<gml:name uom=\"s\">3</gml:name>");
    }

    #[test]
    fn round_trip_time() {
        let original = Time::new(42.5, "s");
        let mut writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_time(
            &original,
            &mut writer,
            GmlNamespace::Gml,
            GmlElement::NameProperty,
        )
        .expect("should serialize");
        let xml = writer.into_bytes();
        let mut reader = reader_for(&xml);

        let round_tripped = deserialize_time(&mut reader).expect("should work");

        assert_eq!(round_tripped, original);
    }
}
