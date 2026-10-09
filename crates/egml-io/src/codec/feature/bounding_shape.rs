use crate::Error;
use crate::codec::geometry::{deserialize_envelope, serialize_envelope};
use crate::util::{
    DeserializationConfig, GmlElement, GmlNamespace, XmlDocumentIndex, XmlFragmentWriter,
};
use egml_core::model::feature::BoundingShape;
use egml_core::model::geometry::Envelope;
use std::io::Write;

pub fn deserialize_bounding_shape(
    xml_document: &[u8],
    index: &XmlDocumentIndex<GmlElement>,
    config: &DeserializationConfig,
) -> Result<BoundingShape, Error> {
    let index = if index.is_truncated() {
        &XmlDocumentIndex::from_scan(xml_document, config.rescan_depth())?
    } else {
        index
    };

    let envelope: Option<Envelope> = index
        .first(GmlElement::Envelope)
        .map(|x| deserialize_envelope(&xml_document[x.range()]))
        .transpose()?;

    Ok(BoundingShape::new_unchecked(envelope, None))
}

pub fn serialize_bounding_shape<W: Write>(
    bounding_shape: &BoundingShape,
    xml_fragment_writer: &mut XmlFragmentWriter<W>,
) -> Result<(), Error> {
    xml_fragment_writer.write_start_event(GmlNamespace::Gml, GmlElement::BoundedByProperty)?;

    if let Some(envelope) = bounding_shape.envelope() {
        serialize_envelope(envelope, xml_fragment_writer)?;
    }

    xml_fragment_writer.write_end_event(GmlNamespace::Gml, GmlElement::BoundedByProperty)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    // Test-only convenience: builds the index the real function now
    // requires, so existing single-argument call sites below don't all
    // need to construct one by hand.
    fn deserialize(xml_document: &[u8]) -> Result<super::BoundingShape, crate::Error> {
        let index = crate::util::XmlDocumentIndex::from_scan(xml_document, None)?;
        super::deserialize_bounding_shape(
            xml_document,
            &index,
            &crate::util::DeserializationConfig::default(),
        )
    }

    use super::serialize_bounding_shape;
    use crate::util::{Formatting, XmlFragmentWriter};
    use egml_core::model::geometry::DirectPosition;

    #[test]
    fn deserialize_bounding_shape_without_envelope() {
        let xml_document = b"<gml:boundedBy/>";

        let bounding_shape = deserialize(xml_document).expect("should deserialize");

        assert!(bounding_shape.envelope().is_none());
    }

    #[test]
    fn deserialize_bounding_shape_with_empty_children() {
        let xml_document = b"<gml:boundedBy></gml:boundedBy>";

        let bounding_shape = deserialize(xml_document).expect("should deserialize");

        assert!(bounding_shape.envelope().is_none());
    }

    #[test]
    fn deserialize_bounding_shape_with_envelope() {
        let xml_document = b"<gml:boundedBy>
<gml:Envelope srsDimension=\"3\">
<gml:lowerCorner>1 2 3</gml:lowerCorner>
<gml:upperCorner>11 12 13</gml:upperCorner>
</gml:Envelope>
</gml:boundedBy>";

        let bounding_shape = deserialize(xml_document).expect("should deserialize");

        let envelope = bounding_shape.envelope().expect("should have envelope");
        assert_eq!(envelope.lower_corner().x(), 1.0);
        assert_eq!(envelope.upper_corner().x(), 11.0);
    }

    #[test]
    fn round_trip_bounding_shape() {
        let lower = DirectPosition::new(1.0, 2.0, 3.0).unwrap();
        let upper = DirectPosition::new(11.0, 12.0, 13.0).unwrap();
        let envelope = egml_core::model::geometry::Envelope::new(lower, upper).unwrap();
        let original =
            egml_core::model::feature::BoundingShape::new_unchecked(Some(envelope), None);

        let mut writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_bounding_shape(&original, &mut writer).expect("should serialize");
        let xml = writer.into_bytes();

        let recovered = deserialize(&xml).expect("should deserialize");

        let recovered_envelope = recovered.envelope().expect("should have envelope");
        let original_envelope = original.envelope().expect("should have envelope");
        assert_eq!(
            recovered_envelope.lower_corner().x(),
            original_envelope.lower_corner().x()
        );
        assert_eq!(
            recovered_envelope.upper_corner().x(),
            original_envelope.upper_corner().x()
        );
    }
}
