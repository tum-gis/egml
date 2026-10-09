use crate::Error;
use crate::codec::geometry::primitives::linear_ring::deserialize_linear_ring;
use crate::codec::geometry::primitives::serialize_linear_ring;
use crate::util::{DeserializationConfig, GmlElement, XmlDocumentIndex, XmlFragmentWriter};
use egml_core::model::geometry::primitives::AbstractRingKind;
use std::io::Write;

pub fn deserialize_abstract_ring_kind(
    xml_document: &[u8],
    index: &XmlDocumentIndex<GmlElement>,
    config: &DeserializationConfig,
) -> Result<Option<AbstractRingKind>, Error> {
    if let Some(node) = index.first(GmlElement::LinearRing) {
        let linear_ring = deserialize_linear_ring(&xml_document[node.range()], node, config)?;
        return Ok(Some(linear_ring.into()));
    }

    /*if let Some(span) = spans.first(GmlElement::Ring) {
        let xml_document_selection = &xml_document[span.start..span.end];
        let index = XmlDocumentIndex::from_scan(xml_document_selection, None)?;
        let abstract_ring_kind = deserialize_ring_kind(xml_document_selection, &spans)?;
        if let Some(abstract_ring_kind) = abstract_ring_kind {
            return Ok(Some(abstract_ring_kind));
        }
    }*/

    Ok(None)
}

/// Deserializes `node` as an [`AbstractRingKind`] given that `element` is
/// already known to be `node`'s own type (the caller found it by iterating
/// a parent's `children()`, so it doesn't need to search for it again via
/// [`XmlDocumentIndex::first`]). Returns `None` for any `element` outside
/// this vocabulary, exactly like [`deserialize_abstract_ring_kind`] does
/// when no candidate matches, rather than panicking on unexpected input.
pub fn deserialize_abstract_ring_kind_for(
    element: GmlElement,
    xml_document: &[u8],
    node: &XmlDocumentIndex<GmlElement>,
    config: &DeserializationConfig,
) -> Result<Option<AbstractRingKind>, Error> {
    if element == GmlElement::LinearRing {
        let linear_ring = deserialize_linear_ring(xml_document, node, config)?;
        return Ok(Some(linear_ring.into()));
    }

    Ok(None)
}

pub fn serialize_abstract_ring_kind<W: Write>(
    abstract_ring_kind: &AbstractRingKind,
    xml_fragment_writer: &mut XmlFragmentWriter<W>,
) -> Result<(), Error> {
    match abstract_ring_kind {
        AbstractRingKind::LinearRing(x) => serialize_linear_ring(x, xml_fragment_writer),
        AbstractRingKind::AbstractRingKind(x) => {
            serialize_abstract_ring_kind(x, xml_fragment_writer)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::codec::geometry::primitives::deserialize_abstract_ring_kind;
    use crate::util::XmlDocumentIndex;

    #[test]
    fn deserialize_ring_kind_as_linear_ring() {
        let xml_document = b"<gml:surfaceMember>
        <gml:LinearRing>
      <gml:pos>0.0 0.0 0.0</gml:pos>
      <gml:pos>1.0 1.0 0.0</gml:pos>
      <gml:pos>1.0 1.0 1.0</gml:pos>
      <gml:pos>0.0 0.0 0.0</gml:pos>
   </gml:LinearRing>
   </gml:surfaceMember>";

        let index = XmlDocumentIndex::from_scan(xml_document, None).expect("should work");
        let abstract_ring_kind = deserialize_abstract_ring_kind(
            xml_document,
            &index,
            &crate::util::DeserializationConfig::default(),
        )
        .unwrap();
        assert!(abstract_ring_kind.is_some());
        let abstract_ring_kind = abstract_ring_kind.unwrap();
        assert_eq!(abstract_ring_kind.points().len(), 3);
    }

    #[test]
    fn deserialize_ring_kind_as_ring() {
        let xml_document = b"<gml:Ring>
       <gml:curveMember>
          <gml:LineString>
              <gml:pos>0.0 0.0 0.0</gml:pos>
              <gml:pos>1.0 1.0 0.0</gml:pos>
              <gml:pos>1.0 1.0 1.0</gml:pos>
              <gml:pos>0.0 0.0 0.0</gml:pos>
          </gml:LineString>
       </gml:curveMember>
    </gml:Ring>";

        let index = XmlDocumentIndex::from_scan(xml_document, None).expect("should work");
        let result = deserialize_abstract_ring_kind(
            xml_document,
            &index,
            &crate::util::DeserializationConfig::default(),
        )
        .unwrap();

        // gml:Ring (a ring built from curve members) is not supported yet, so it is
        // skipped rather than deserialized.
        assert_eq!(result, None);
    }
}
