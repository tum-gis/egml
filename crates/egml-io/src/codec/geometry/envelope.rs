use crate::Error;
use crate::codec::geometry::direct_position::direct_position_from_start;
use crate::codec::geometry::serialize_direct_position;
use crate::util::{GmlElement, GmlNamespace, XmlFragmentWriter};
use egml_core::model::geometry::{DirectPosition, Envelope};
use quick_xml::events::Event;
use quick_xml::{Reader, XmlVersion};
use std::io::Write;

pub fn deserialize_envelope(xml_document: &[u8]) -> Result<Envelope, Error> {
    let mut reader = Reader::from_reader(xml_document);

    let start = loop {
        match reader.read_event()? {
            Event::Start(start) => break start,
            Event::Eof => {
                return Err(Error::ElementNotFound("Envelope start".to_string()));
            }
            _ => {}
        }
    };

    let mut srs_name: Option<String> = None;
    let mut srs_dimension: Option<u32> = None;

    for attr in start.attributes() {
        let attr = attr.map_err(quick_xml::Error::from)?;
        let value = attr.normalized_value(XmlVersion::Implicit1_0)?.into_owned();
        match attr.key.local_name().as_ref() {
            "srsName" => srs_name = Some(value),
            "srsDimension" => {
                let dimension =
                    value
                        .parse::<u32>()
                        .map_err(|_| egml_core::Error::InvalidAttributeValue {
                            attribute: "srsDimension",
                            value,
                        })?;
                srs_dimension = Some(dimension);
            }
            _ => {}
        }
    }

    if srs_dimension.unwrap_or(3) != 3 {
        return Err(Error::UnsupportedDimension {
            found: srs_dimension.unwrap_or(0),
        });
    }

    let mut lower_corner: Option<DirectPosition> = None;
    let mut upper_corner: Option<DirectPosition> = None;

    while lower_corner.is_none() || upper_corner.is_none() {
        match reader.read_event()? {
            Event::Start(start) => match start.local_name().as_ref() {
                "lowerCorner" if lower_corner.is_none() => {
                    lower_corner = Some(direct_position_from_start(&start, &mut reader)?);
                }
                "upperCorner" if upper_corner.is_none() => {
                    upper_corner = Some(direct_position_from_start(&start, &mut reader)?);
                }
                _ => {
                    reader.read_to_end(start.name())?;
                }
            },
            Event::Empty(start) => match start.local_name().as_ref() {
                "lowerCorner" if lower_corner.is_none() => {
                    lower_corner = Some(direct_position_from_start(&start, &mut reader)?);
                }
                "upperCorner" if upper_corner.is_none() => {
                    upper_corner = Some(direct_position_from_start(&start, &mut reader)?);
                }
                _ => {}
            },
            Event::Eof => break,
            // Skips Text/End/Comment/etc., including each corner's own leftover End tag
            // (direct_position_from_start reads up to a corner's text but not past it).
            _ => {}
        }
    }

    let lower_corner =
        lower_corner.ok_or_else(|| Error::ElementNotFound("gml:lowerCorner".to_string()))?;
    let upper_corner =
        upper_corner.ok_or_else(|| Error::ElementNotFound("gml:upperCorner".to_string()))?;

    let mut envelope = Envelope::new(lower_corner, upper_corner)?;
    envelope.set_srs_name_opt(srs_name);
    envelope.set_srs_dimension_opt(srs_dimension);
    Ok(envelope)
}

pub fn serialize_envelope<W: Write>(
    envelope: &Envelope,
    xml_fragment_writer: &mut XmlFragmentWriter<W>,
) -> Result<(), Error> {
    let mut attributes = Vec::new();
    if let Some(srs_name) = envelope.srs_name() {
        attributes.push(("srsName", srs_name));
    }
    let srs_dimension_string = envelope.srs_dimension().map(|d| d.to_string());
    if let Some(srs_dimension_string) = &srs_dimension_string {
        attributes.push(("srsDimension", srs_dimension_string.as_str()));
    }
    xml_fragment_writer.write_start_event_with_attributes(
        GmlNamespace::Gml,
        GmlElement::Envelope,
        attributes,
    )?;

    serialize_direct_position(
        envelope.lower_corner(),
        xml_fragment_writer,
        GmlNamespace::Gml,
        GmlElement::LowerCornerProperty,
    )?;
    serialize_direct_position(
        envelope.upper_corner(),
        xml_fragment_writer,
        GmlNamespace::Gml,
        GmlElement::UpperCornerProperty,
    )?;

    xml_fragment_writer.write_end_event(GmlNamespace::Gml, GmlElement::Envelope)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{deserialize_envelope, serialize_envelope};
    use crate::Error;
    use crate::util::{Formatting, XmlFragmentWriter};
    use egml_core::model::geometry::{DirectPosition, Envelope};

    fn make_envelope() -> Envelope {
        let lower = DirectPosition::new(1.0, 2.0, 3.0).unwrap();
        let upper = DirectPosition::new(11.0, 12.0, 13.0).unwrap();
        Envelope::new(lower, upper).unwrap()
    }

    #[test]
    fn serialize_envelope_writes_corners() {
        let envelope = make_envelope();
        let mut writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);

        serialize_envelope(&envelope, &mut writer).expect("should serialize");

        let xml = String::from_utf8(writer.into_bytes()).expect("valid UTF-8");
        assert_eq!(
            xml,
            r#"<gml:Envelope><gml:lowerCorner srsDimension="3">1 2 3</gml:lowerCorner><gml:upperCorner srsDimension="3">11 12 13</gml:upperCorner></gml:Envelope>"#
        );
    }

    #[test]
    fn serialize_envelope_writes_srs_name_and_dimension() {
        let mut envelope = make_envelope();
        envelope.set_srs_name("urn:ogc:def:crs:EPSG::25832");
        envelope.set_srs_dimension(3);
        let mut writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);

        serialize_envelope(&envelope, &mut writer).expect("should serialize");

        let xml = String::from_utf8(writer.into_bytes()).expect("valid UTF-8");
        assert!(xml.contains(r#"srsName="urn:ogc:def:crs:EPSG::25832""#));
        assert!(xml.contains(r#"srsDimension="3""#));
    }

    #[test]
    fn deserialize_envelope_works() {
        let xml_document = b"<gml:Envelope srsDimension=\"3\">
<gml:lowerCorner>1 2 3</gml:lowerCorner>
<gml:upperCorner>11 12 13</gml:upperCorner>
</gml:Envelope>";
        let envelope = deserialize_envelope(xml_document).expect("should deserialize");

        assert_eq!(envelope.lower_corner().x(), 1.0);
        assert_eq!(envelope.lower_corner().y(), 2.0);
        assert_eq!(envelope.lower_corner().z(), 3.0);
        assert_eq!(envelope.upper_corner().x(), 11.0);
        assert_eq!(envelope.upper_corner().y(), 12.0);
        assert_eq!(envelope.upper_corner().z(), 13.0);
    }

    #[test]
    fn deserialize_envelope_reads_srs_name_and_dimension() {
        let xml_document =
            br#"<gml:Envelope srsName="urn:ogc:def:crs:EPSG::25832" srsDimension="3">
<gml:lowerCorner>1 2 3</gml:lowerCorner>
<gml:upperCorner>11 12 13</gml:upperCorner>
</gml:Envelope>"#;
        let envelope = deserialize_envelope(xml_document).expect("should deserialize");

        assert_eq!(envelope.srs_name(), Some("urn:ogc:def:crs:EPSG::25832"));
        assert_eq!(envelope.srs_dimension(), Some(3));
    }

    #[test]
    fn deserialize_envelope_fails_with_unsupported_dimension() {
        let xml_document = b"<gml:Envelope srsDimension=\"2\">
<gml:lowerCorner>1 2</gml:lowerCorner>
<gml:upperCorner>11 12</gml:upperCorner>
</gml:Envelope>";
        let result = deserialize_envelope(xml_document);

        assert!(matches!(
            result,
            Err(Error::UnsupportedDimension { found: 2 })
        ));
    }

    #[test]
    fn deserialize_envelope_accepts_upper_corner_before_lower_corner() {
        let xml_document = b"<gml:Envelope srsDimension=\"3\">
<gml:upperCorner>11 12 13</gml:upperCorner>
<gml:lowerCorner>1 2 3</gml:lowerCorner>
</gml:Envelope>";
        let envelope = deserialize_envelope(xml_document).expect("should deserialize");

        assert_eq!(envelope.lower_corner().x(), 1.0);
        assert_eq!(envelope.lower_corner().y(), 2.0);
        assert_eq!(envelope.lower_corner().z(), 3.0);
        assert_eq!(envelope.upper_corner().x(), 11.0);
        assert_eq!(envelope.upper_corner().y(), 12.0);
        assert_eq!(envelope.upper_corner().z(), 13.0);
    }

    #[test]
    fn deserialize_envelope_ignores_unknown_child_elements() {
        let xml_document = b"<gml:Envelope srsDimension=\"3\">
<gml:extraneous><gml:nested>ignored</gml:nested></gml:extraneous>
<gml:lowerCorner>1 2 3</gml:lowerCorner>
<gml:upperCorner>11 12 13</gml:upperCorner>
</gml:Envelope>";
        let envelope = deserialize_envelope(xml_document).expect("should deserialize");

        assert_eq!(envelope.lower_corner().x(), 1.0);
        assert_eq!(envelope.upper_corner().x(), 11.0);
    }

    #[test]
    fn deserialize_envelope_fails_with_missing_corner() {
        let xml_document = b"<gml:Envelope srsDimension=\"3\">
<gml:lowerCorner>1 2 3</gml:lowerCorner>
</gml:Envelope>";
        let result = deserialize_envelope(xml_document);

        assert!(matches!(
            result,
            Err(Error::ElementNotFound(name)) if name == "gml:upperCorner"
        ));
    }

    #[test]
    fn round_trip_envelope() {
        let mut original = make_envelope();
        original.set_srs_name("urn:ogc:def:crs:EPSG::25832");
        original.set_srs_dimension(3);
        let mut writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_envelope(&original, &mut writer).expect("should serialize");
        let xml = writer.into_bytes();
        let recovered = deserialize_envelope(&xml).expect("should deserialize");

        assert_eq!(recovered.lower_corner().x(), original.lower_corner().x());
        assert_eq!(recovered.lower_corner().y(), original.lower_corner().y());
        assert_eq!(recovered.lower_corner().z(), original.lower_corner().z());
        assert_eq!(recovered.upper_corner().x(), original.upper_corner().x());
        assert_eq!(recovered.upper_corner().y(), original.upper_corner().y());
        assert_eq!(recovered.upper_corner().z(), original.upper_corner().z());
        assert_eq!(recovered.srs_name(), original.srs_name());
        assert_eq!(recovered.srs_dimension(), original.srs_dimension());
    }
}
