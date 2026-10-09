use crate::Error;
use crate::util::{XmlElement, XmlFragmentWriter, XmlNamespace};
use egml_core::model::measures::Speed;
use quick_xml::events::Event;
use quick_xml::{Reader, XmlVersion};
use std::io::Write;

/// Parses a `gml:SpeedType` fragment (e.g. `<speed uom="m/s">2.5</speed>`)
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
pub fn deserialize_speed(reader: &mut Reader<&[u8]>) -> Result<Speed, Error> {
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
                return finish_speed(&raw, uom);
            }
            Event::CData(cdata) => {
                return finish_speed(cdata.as_ref(), uom);
            }
            Event::Eof => {
                return Err(Error::ElementNotFound("speed text content".to_string()));
            }
            _ => {}
        }
    }
}

fn finish_speed(raw: &str, uom: Option<String>) -> Result<Speed, Error> {
    let raw = raw.trim();
    let value = raw
        .parse::<f64>()
        .map_err(|_| egml_core::Error::InvalidAttributeValue {
            attribute: "speed",
            value: raw.to_string(),
        })?;
    let uom = uom.ok_or_else(|| Error::ElementNotFound("uom attribute".to_string()))?;
    Ok(Speed::new(value, uom))
}

pub fn serialize_speed<N: XmlNamespace, E: XmlElement, W: Write>(
    speed: &Speed,
    xml_fragment_writer: &mut XmlFragmentWriter<W>,
    target_xml_namespace: N,
    target_xml_element: E,
) -> Result<(), Error> {
    xml_fragment_writer.write_leaf_element_with_attributes(
        target_xml_namespace,
        target_xml_element,
        [("uom", speed.uom())],
        &speed.value().to_string(),
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::{Formatting, GmlElement, GmlNamespace};
    use egml_core::model::measures::Speed;

    fn reader_for(xml_document: &[u8]) -> Reader<&[u8]> {
        let mut reader = Reader::from_reader(xml_document);
        reader.config_mut().trim_text(true);
        reader
    }

    #[test]
    fn deserialize_speed_works() {
        let xml_document = b"<speed uom=\"m/s\">2.5</speed>";
        let mut reader = reader_for(xml_document);

        let speed = deserialize_speed(&mut reader).expect("should work");

        assert_eq!(speed.uom(), "m/s");
        assert_eq!(speed.value(), 2.5);
    }

    #[test]
    fn deserialize_speed_fails_without_uom() {
        let xml_document = b"<speed>2.5</speed>";
        let mut reader = reader_for(xml_document);

        let result = deserialize_speed(&mut reader);

        assert!(result.is_err());
    }

    #[test]
    fn serialize_speed_works() {
        let speed = Speed::new(2.5, "m/s");
        let mut writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);

        serialize_speed(
            &speed,
            &mut writer,
            GmlNamespace::Gml,
            GmlElement::NameProperty,
        )
        .expect("should serialize");

        let xml = String::from_utf8(writer.into_bytes()).expect("valid UTF-8");
        assert_eq!(xml, "<gml:name uom=\"m/s\">2.5</gml:name>");
    }

    #[test]
    fn round_trip_speed() {
        let original = Speed::new(42.5, "m/s");
        let mut writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_speed(
            &original,
            &mut writer,
            GmlNamespace::Gml,
            GmlElement::NameProperty,
        )
        .expect("should serialize");
        let xml = writer.into_bytes();
        let mut reader = reader_for(&xml);

        let round_tripped = deserialize_speed(&mut reader).expect("should work");

        assert_eq!(round_tripped, original);
    }
}
