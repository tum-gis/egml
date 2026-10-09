use crate::Error;
use crate::util::{XmlElement, XmlFragmentWriter, XmlNamespace};
use egml_core::model::geometry::DirectPosition;
use quick_xml::events::{BytesStart, Event};
use quick_xml::{Reader, XmlVersion};
use std::io::Write;

/// Parses a `gml:posList` fragment (e.g. `<gml:posList srsDimension="3">1 2 3 4 5
/// 6</gml:posList>`) directly via [`quick_xml::Reader`], without going through
/// serde.
///
/// Takes the reader positioned just before the element's `Start`/`Empty`
/// event rather than owning it, so it can be called on a reader shared with
/// (and left positioned for) surrounding parsing code.
///
/// # Errors
///
/// Returns an error if the upcoming XML is not well-formed, `srsDimension`
/// is present and not `3`, or the text content's whitespace-separated `f64`
/// values are not a multiple of 3.
pub fn deserialize_direct_position_list(
    reader: &mut Reader<&[u8]>,
) -> Result<Vec<DirectPosition>, Error> {
    loop {
        match reader.read_event()? {
            Event::Start(start) | Event::Empty(start) => {
                return direct_position_list_from_start(&start, reader);
            }
            Event::Eof => {
                return Err(Error::ElementNotFound("posList text content".to_string()));
            }
            _ => {}
        }
    }
}

/// Parses a `gml:DirectPositionListType`-shaped element whose `Start`/`Empty`
/// event has already been read. Reads `start`'s `srsDimension` attribute,
/// then the following text content from `reader`.
pub(crate) fn direct_position_list_from_start(
    start: &BytesStart<'_>,
    reader: &mut Reader<&[u8]>,
) -> Result<Vec<DirectPosition>, Error> {
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
                return finish_direct_position_list(&raw, srs_dimension);
            }
            Event::CData(cdata) => {
                return finish_direct_position_list(cdata.as_ref(), srs_dimension);
            }
            Event::End(_) => {
                return finish_direct_position_list("", srs_dimension);
            }
            Event::Eof => {
                return Err(Error::ElementNotFound("posList text content".to_string()));
            }
            _ => {}
        }
    }
}

fn finish_direct_position_list(
    raw: &str,
    srs_dimension: Option<u32>,
) -> Result<Vec<DirectPosition>, Error> {
    if srs_dimension.unwrap_or(3) != 3 {
        return Err(Error::UnsupportedDimension {
            found: srs_dimension.unwrap_or(0),
        });
    }

    let values: Vec<f64> = raw
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    if !values.len().is_multiple_of(3) {
        return Err(Error::InvalidCoordinateCount {
            count: values.len(),
        });
    }

    let mut points = Vec::with_capacity(values.len() / 3);
    for chunk in values.as_chunks::<3>().0 {
        points.push(DirectPosition::new(chunk[0], chunk[1], chunk[2])?);
    }

    Ok(points)
}

pub fn serialize_direct_position_list<N: XmlNamespace, E: XmlElement, W: Write>(
    points: &[DirectPosition],
    xml_fragment_writer: &mut XmlFragmentWriter<W>,
    target_xml_namespace: N,
    target_xml_element: E,
) -> Result<(), Error> {
    let content = points
        .iter()
        .flat_map(|p| [p.x(), p.y(), p.z()])
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(" ");

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
    use super::{deserialize_direct_position_list, serialize_direct_position_list};
    use crate::Error;
    use crate::util::{Formatting, GmlElement, GmlNamespace, XmlFragmentWriter};
    use egml_core::model::geometry::DirectPosition;
    use quick_xml::Reader;

    fn reader_for(xml_document: &[u8]) -> Reader<&[u8]> {
        let mut reader = Reader::from_reader(xml_document);
        reader.config_mut().trim_text(true);
        reader
    }

    #[test]
    fn deserialize_direct_position_list_works() {
        let xml_document = b"<gml:posList srsDimension=\"3\">350.54400634765625 972.9130249023438 0.11999999731779099 350.5414201635045 968.6025425887852 0.11999999731779099 350.54400634765625 968.6025096366793 0.11999999731779099 350.54400634765625 972.9130249023438 0.11999999731779099</gml:posList>";
        let mut reader = reader_for(xml_document);

        let points = deserialize_direct_position_list(&mut reader).expect("should work");

        assert_eq!(points.len(), 4);
    }

    #[test]
    fn deserialize_direct_position_list_without_srs_dimension_defaults_to_3() {
        let xml_document = b"<gml:posList>1 2 3 4 5 6</gml:posList>";
        let mut reader = reader_for(xml_document);

        let points = deserialize_direct_position_list(&mut reader).expect("should work");

        assert_eq!(points.len(), 2);
    }

    #[test]
    fn deserialize_direct_position_list_fails_with_unsupported_dimension() {
        let xml_document = b"<gml:posList srsDimension=\"2\">1 2 3 4</gml:posList>";
        let mut reader = reader_for(xml_document);

        let result = deserialize_direct_position_list(&mut reader);

        assert!(matches!(
            result,
            Err(Error::UnsupportedDimension { found: 2 })
        ));
    }

    #[test]
    fn deserialize_direct_position_list_fails_with_wrong_coordinate_count() {
        let xml_document = b"<gml:posList>1 2 3 4</gml:posList>";
        let mut reader = reader_for(xml_document);

        let result = deserialize_direct_position_list(&mut reader);

        assert!(matches!(
            result,
            Err(Error::InvalidCoordinateCount { count: 4 })
        ));
    }

    #[test]
    fn serialize_direct_position_list_writes_srs_dimension_and_values() {
        let points = vec![
            DirectPosition::new(1.0, 2.0, 3.0).unwrap(),
            DirectPosition::new(4.0, 5.0, 6.0).unwrap(),
        ];
        let mut writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);

        serialize_direct_position_list(
            &points,
            &mut writer,
            GmlNamespace::Gml,
            GmlElement::PosListProperty,
        )
        .expect("should serialize");

        let xml = String::from_utf8(writer.into_bytes()).expect("valid UTF-8");
        assert_eq!(
            xml,
            r#"<gml:posList srsDimension="3">1 2 3 4 5 6</gml:posList>"#
        );
    }

    #[test]
    fn round_trip_direct_position_list() {
        let points = vec![
            DirectPosition::new(678000.9484065345, 5403659.060043676, 417.3802376791456).unwrap(),
            DirectPosition::new(1.0, 2.0, 3.0).unwrap(),
        ];
        let mut writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_direct_position_list(
            &points,
            &mut writer,
            GmlNamespace::Gml,
            GmlElement::PosListProperty,
        )
        .expect("should serialize");
        let xml = writer.into_bytes();
        let mut reader = reader_for(&xml);

        let recovered = deserialize_direct_position_list(&mut reader).expect("should deserialize");

        assert_eq!(recovered.len(), points.len());
        for (a, b) in recovered.iter().zip(points.iter()) {
            assert_eq!(a.x(), b.x());
            assert_eq!(a.y(), b.y());
            assert_eq!(a.z(), b.z());
        }
    }
}
