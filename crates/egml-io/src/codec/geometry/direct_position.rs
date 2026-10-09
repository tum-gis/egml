use crate::Error;
use crate::util::{XmlElement, XmlFragmentWriter, XmlNamespace};
use egml_core::model::geometry::DirectPosition;
use quick_xml::events::{BytesStart, Event};
use quick_xml::{Reader, XmlVersion};
use std::io::Write;

/// Parses a `gml:pos` fragment (e.g. `<gml:pos srsDimension="3">1 2 3</gml:pos>`)
/// directly via [`quick_xml::Reader`].
///
/// Takes the reader positioned just before the element's `Start`/`Empty`
/// event rather than owning it, so it can be called on a reader shared with
/// (and left positioned for) surrounding parsing code. The caller is
/// responsible for configuring the reader (e.g. `trim_text`).
///
/// # Errors
///
/// Returns an error if the upcoming XML is not well-formed, `srsDimension`
/// is present and not `3`, the element has no text content, or the text
/// content does not contain exactly 3 whitespace-separated `f64` values.
pub fn deserialize_direct_position(reader: &mut Reader<&[u8]>) -> Result<DirectPosition, Error> {
    loop {
        match reader.read_event()? {
            Event::Start(start) | Event::Empty(start) => {
                return direct_position_from_start(&start, reader);
            }
            Event::Eof => {
                return Err(Error::ElementNotFound("pos text content".to_string()));
            }
            _ => {}
        }
    }
}

/// Parses a `gml:DirectPositionType`-shaped element whose `Start`/`Empty`
/// event has already been read (e.g. by a caller dispatching on the
/// element's name, such as `gml:lowerCorner` vs. `gml:upperCorner`). Reads
/// `start`'s `srsDimension` attribute, then the following text content
/// from `reader`.
pub(crate) fn direct_position_from_start(
    start: &BytesStart<'_>,
    reader: &mut Reader<&[u8]>,
) -> Result<DirectPosition, Error> {
    let mut srs_dimension: Option<u32> = None;
    for attr in start.attributes() {
        let attr = attr.map_err(quick_xml::Error::from)?;
        if attr.key.local_name().as_ref() == "srsDimension" {
            let value = attr.normalized_value(XmlVersion::Implicit1_0)?.into_owned();
            let dimension =
                value
                    .parse::<u32>()
                    .map_err(|_| egml_core::Error::InvalidAttributeValue {
                        attribute: "srsDimension",
                        value,
                    })?;
            srs_dimension = Some(dimension);
        }
    }

    loop {
        match reader.read_event()? {
            Event::Text(text) => {
                let raw =
                    quick_xml::escape::unescape(text.as_ref()).map_err(quick_xml::Error::from)?;
                return finish_direct_position(&raw, srs_dimension);
            }
            Event::CData(cdata) => {
                return finish_direct_position(cdata.as_ref(), srs_dimension);
            }
            Event::End(_) => {
                return finish_direct_position("", srs_dimension);
            }
            Event::Eof => {
                return Err(Error::ElementNotFound("pos text content".to_string()));
            }
            _ => {}
        }
    }
}

fn finish_direct_position(raw: &str, srs_dimension: Option<u32>) -> Result<DirectPosition, Error> {
    if srs_dimension.unwrap_or(3) != 3 {
        return Err(Error::UnsupportedDimension {
            found: srs_dimension.unwrap_or(0),
        });
    }

    let values: Vec<f64> = raw
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    if values.len() != 3 {
        return Err(Error::InvalidCoordinateCount {
            count: values.len(),
        });
    }

    let position = DirectPosition::new(values[0], values[1], values[2])?;
    Ok(position)
}

pub fn serialize_direct_position<N: XmlNamespace, E: XmlElement, W: Write>(
    direct_position: &DirectPosition,
    xml_fragment_writer: &mut XmlFragmentWriter<W>,
    target_xml_namespace: N,
    target_xml_element: E,
) -> Result<(), Error> {
    let content = format!(
        "{} {} {}",
        direct_position.x(),
        direct_position.y(),
        direct_position.z()
    );

    xml_fragment_writer.write_leaf_element_with_attributes(
        target_xml_namespace,
        target_xml_element,
        [("srsDimension", "3")],
        &content,
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{deserialize_direct_position, serialize_direct_position};
    use crate::Error;
    use crate::util::{Formatting, GmlElement, GmlNamespace, XmlFragmentWriter};
    use egml_core::model::geometry::DirectPosition;
    use quick_xml::Reader;

    #[test]
    fn deserialize_direct_position_ignores_unrelated_attributes() {
        let xml_document = b"<gml:pos srsDimension=\"3\" gml:id=\"UUID_6b33ecfa-6e08-4e8e-a4b5-e1d06540faf0\">678000.9484065345 5403659.060043676 417.3802376791456</gml:pos>";
        let mut reader = reader_for(xml_document);

        let direct_position = deserialize_direct_position(&mut reader).expect("should work");

        assert_eq!(direct_position.x(), 678000.9484065345);
        assert_eq!(direct_position.y(), 5403659.060043676);
        assert_eq!(direct_position.z(), 417.3802376791456);
    }

    fn reader_for(xml_document: &[u8]) -> Reader<&[u8]> {
        let mut reader = Reader::from_reader(xml_document);
        reader.config_mut().trim_text(true);
        reader
    }

    #[test]
    fn deserialize_direct_position_works() {
        let xml_document = b"<gml:pos srsDimension=\"3\">678000.9484065345 5403659.060043676 417.3802376791456</gml:pos>";
        let mut reader = reader_for(xml_document);

        let direct_position = deserialize_direct_position(&mut reader).expect("should work");

        assert_eq!(direct_position.x(), 678000.9484065345);
        assert_eq!(direct_position.y(), 5403659.060043676);
        assert_eq!(direct_position.z(), 417.3802376791456);
    }

    #[test]
    fn deserialize_direct_position_without_srs_dimension_defaults_to_3() {
        let xml_document = b"<gml:pos>1 2 3</gml:pos>";
        let mut reader = reader_for(xml_document);

        let direct_position = deserialize_direct_position(&mut reader).expect("should work");

        assert_eq!(direct_position.x(), 1.0);
        assert_eq!(direct_position.y(), 2.0);
        assert_eq!(direct_position.z(), 3.0);
    }

    #[test]
    fn deserialize_direct_position_fails_with_unsupported_dimension() {
        let xml_document = b"<gml:pos srsDimension=\"2\">1 2</gml:pos>";
        let mut reader = reader_for(xml_document);

        let result = deserialize_direct_position(&mut reader);

        assert!(matches!(
            result,
            Err(Error::UnsupportedDimension { found: 2 })
        ));
    }

    #[test]
    fn deserialize_direct_position_fails_with_wrong_coordinate_count() {
        let xml_document = b"<gml:pos>1 2</gml:pos>";
        let mut reader = reader_for(xml_document);

        let result = deserialize_direct_position(&mut reader);

        assert!(matches!(
            result,
            Err(Error::InvalidCoordinateCount { count: 2 })
        ));
    }

    #[test]
    fn serialize_direct_position_writes_srs_dimension_and_values() {
        let position = DirectPosition::new(1.0, 2.0, 3.0).unwrap();
        let mut writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);

        serialize_direct_position(&position, &mut writer, GmlNamespace::Gml, GmlElement::Pos)
            .expect("should serialize");

        let xml = String::from_utf8(writer.into_bytes()).expect("valid UTF-8");
        assert_eq!(xml, r#"<gml:pos srsDimension="3">1 2 3</gml:pos>"#);
    }

    #[test]
    fn round_trip_direct_position() {
        let original =
            DirectPosition::new(678000.9484065345, 5403659.060043676, 417.3802376791456).unwrap();
        let mut writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_direct_position(&original, &mut writer, GmlNamespace::Gml, GmlElement::Pos)
            .expect("should serialize");
        let xml = writer.into_bytes();
        let mut reader = reader_for(&xml);

        let recovered = deserialize_direct_position(&mut reader).expect("should deserialize");

        assert_eq!(recovered.x(), original.x());
        assert_eq!(recovered.y(), original.y());
        assert_eq!(recovered.z(), original.z());
    }
}
