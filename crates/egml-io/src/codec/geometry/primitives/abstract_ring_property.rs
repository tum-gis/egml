use crate::Error;
use crate::codec::geometry::primitives::{
    deserialize_abstract_ring_kind, serialize_abstract_ring_kind,
};
use crate::util::{
    DeserializationConfig, GmlElement, XmlDocumentIndex, XmlElement, XmlFragmentWriter,
    XmlNamespace,
};
use egml_core::model::geometry::primitives::AbstractRingKind;
use std::io::Write;

/// Deserializes a `gml:AbstractRingPropertyType` element (e.g. `gml:exterior`)
/// into the ring it wraps.
///
/// The property type carries no attributes and its ring is mandatory.
///
/// # Errors
///
/// Returns [`Error::MissingLinearRing`] if the element contains no supported ring.
pub fn deserialize_abstract_ring_property(
    xml_document: &[u8],
    index: &XmlDocumentIndex<GmlElement>,
    config: &DeserializationConfig,
) -> Result<AbstractRingKind, Error> {
    let index = if index.is_truncated() {
        &XmlDocumentIndex::from_scan(xml_document, config.rescan_depth())?
    } else {
        index
    };

    deserialize_abstract_ring_kind(xml_document, index, config)?.ok_or(Error::MissingLinearRing)
}

/// Serializes `ring` wrapped in a `gml:AbstractRingPropertyType` element.
pub fn serialize_abstract_ring_property<N: XmlNamespace, E: XmlElement, W: Write>(
    ring: &AbstractRingKind,
    xml_fragment_writer: &mut XmlFragmentWriter<W>,
    target_xml_namespace: N,
    target_xml_element: E,
) -> Result<(), Error> {
    xml_fragment_writer.write_start_event(target_xml_namespace, target_xml_element)?;
    serialize_abstract_ring_kind(ring, xml_fragment_writer)?;
    xml_fragment_writer.write_end_event(target_xml_namespace, target_xml_element)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::Error;
    use crate::codec::geometry::primitives::{
        deserialize_abstract_ring_property, serialize_abstract_ring_property,
    };
    use crate::util::{
        DeserializationConfig, Formatting, GmlElement, GmlNamespace, XmlDocumentIndex,
        XmlFragmentWriter,
    };
    use egml_core::model::geometry::primitives::AbstractRingKind;

    fn deserialize(xml_document: &[u8]) -> Result<AbstractRingKind, Error> {
        let index = XmlDocumentIndex::from_scan(xml_document, None).expect("should index");
        deserialize_abstract_ring_property(xml_document, &index, &DeserializationConfig::default())
    }

    fn serialize(ring: &AbstractRingKind) -> String {
        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_abstract_ring_property(
            ring,
            &mut xml_fragment_writer,
            GmlNamespace::Gml,
            GmlElement::ExteriorProperty,
        )
        .expect("should serialize");
        String::from_utf8(xml_fragment_writer.into_bytes()).expect("valid UTF-8")
    }

    #[test]
    fn deserialize_ring_property_as_linear_ring() {
        let xml_document = b"<gml:exterior>
   <gml:LinearRing>
      <gml:pos>0.0 0.0 0.0</gml:pos>
      <gml:pos>1.0 1.0 0.0</gml:pos>
      <gml:pos>1.0 1.0 1.0</gml:pos>
      <gml:pos>0.0 0.0 0.0</gml:pos>
   </gml:LinearRing>
</gml:exterior>";

        let AbstractRingKind::LinearRing(linear_ring) =
            deserialize(xml_document).expect("should deserialize")
        else {
            panic!("expected LinearRing variant");
        };

        assert_eq!(linear_ring.points().len(), 3);
    }

    #[test]
    fn deserialize_unsupported_ring_is_an_error() {
        // gml:Ring deserialization is not yet implemented.
        let xml_document = b"<gml:exterior><gml:Ring><gml:curveMember><gml:LineString>\
            <gml:pos>0 0 0</gml:pos><gml:pos>1 1 0</gml:pos><gml:pos>1 1 1</gml:pos>\
            <gml:pos>0 0 0</gml:pos></gml:LineString></gml:curveMember></gml:Ring></gml:exterior>";

        assert!(matches!(
            deserialize(xml_document),
            Err(Error::MissingLinearRing)
        ));
    }

    #[test]
    fn deserialize_empty_property_is_an_error() {
        assert!(matches!(
            deserialize(b"<gml:exterior/>"),
            Err(Error::MissingLinearRing)
        ));
    }

    #[test]
    fn deserialize_ignores_xlink_and_ownership_attributes() {
        // gml:AbstractRingPropertyType declares neither attribute group; tolerate them in input.
        let xml_document = b"<gml:exterior xlink:href=\"#some-id\" xlink:title=\"Some Title\" gml:owns=\"true\"><gml:LinearRing>\
            <gml:posList srsDimension=\"3\">0 0 0 1 0 0 0 1 0 0 0 0</gml:posList>\
            </gml:LinearRing></gml:exterior>";

        let ring = deserialize(xml_document).expect("should deserialize");

        assert_eq!(ring.points().len(), 3);
        assert!(!serialize(&ring).contains("xlink"));
    }

    #[test]
    fn round_trip_preserves_ring() {
        let xml_document = b"<gml:exterior><gml:LinearRing>\
            <gml:posList srsDimension=\"3\">0 0 0 1 0 0 0 1 0 0 0 0</gml:posList>\
            </gml:LinearRing></gml:exterior>";
        let ring = deserialize(xml_document).expect("should deserialize");

        let output = serialize(&ring);

        assert!(output.starts_with("<gml:exterior><gml:LinearRing"));
        assert_eq!(
            deserialize(output.as_bytes()).expect("should deserialize"),
            ring
        );
    }
}
