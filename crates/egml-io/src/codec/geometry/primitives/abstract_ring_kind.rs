use crate::Error;
use crate::codec::geometry::primitives::{deserialize_linear_ring, serialize_linear_ring};
use crate::util::{Formatting, GmlElement, XmlElementSpans, XmlNode};
use egml_core::model::geometry::primitives::AbstractRingKind;

pub fn deserialize_abstract_ring_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans<GmlElement>,
) -> Result<Option<AbstractRingKind>, Error> {
    if let Some(span) = spans.first(GmlElement::LinearRing) {
        let linear_ring = deserialize_linear_ring(&xml_document[span.start..span.end])?;
        return Ok(Some(linear_ring.into()));
    }

    /*if let Some(span) = spans.first(GmlElement::Ring) {
        let xml_document_selection = &xml_document[span.start..span.end];
        let spans = extract_xml_element_spans(xml_document_selection)?;
        let abstract_ring_kind = deserialize_ring_kind(xml_document_selection, &spans)?;
        if let Some(abstract_ring_kind) = abstract_ring_kind {
            return Ok(Some(abstract_ring_kind));
        }
    }*/

    Ok(None)
}

pub fn serialize_abstract_ring_kind(
    abstract_ring_kind: &AbstractRingKind,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    match abstract_ring_kind {
        AbstractRingKind::LinearRing(x) => serialize_linear_ring(x, formatting),
        AbstractRingKind::AbstractRingKind(x) => serialize_abstract_ring_kind(x, formatting),
    }
}

#[cfg(test)]
mod tests {
    use crate::codec::geometry::primitives::deserialize_abstract_ring_kind;
    use crate::util::extract_xml_element_spans;

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

        let spans = extract_xml_element_spans(xml_document).expect("should work");
        let abstract_ring_kind = deserialize_abstract_ring_kind(xml_document, &spans).unwrap();
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

        let spans = extract_xml_element_spans(xml_document).expect("should work");
        let result = deserialize_abstract_ring_kind(xml_document, &spans).unwrap();
        println!("{:?}", result);
        //let gml_linear_ring = result.unwrap().linear_ring.expect("should work");
        //let linear_ring: LinearRing = gml_linear_ring.try_into().unwrap();

        //assert_eq!(linear_ring.points().len(), 3);
    }
}
