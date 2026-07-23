use crate::Error;
use crate::codec::base::{
    GmlAssociationAttributes, GmlOwnershipAttributes, serialize_association_attributes,
    serialize_ownership_attributes,
};
use crate::codec::geometry::primitives::{
    deserialize_abstract_ring_kind, serialize_abstract_ring_kind,
};
use crate::util::{Formatting, GmlElement, XmlElementSpans, XmlNode, XmlNodeContent, XmlNodeParts};
use egml_core::model::base::{HasAssociationAttributes, HasOwnershipAttributes};
use egml_core::model::geometry::primitives::AbstractRingProperty;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_abstract_ring_property(
    xml_document: &[u8],
    spans: &XmlElementSpans<GmlElement>,
) -> Result<AbstractRingProperty, Error> {
    let parsed: GmlAbstractRingProperty = de::from_reader(xml_document)?;

    let object = deserialize_abstract_ring_kind(xml_document, spans)?;

    Ok(AbstractRingProperty::new(
        object,
        parsed.association.try_into()?,
        parsed.ownership.into(),
    ))
}

pub fn serialize_abstract_ring_property(
    abstract_ring_property: &AbstractRingProperty,
    formatting: Formatting,
    target_xml_element: &'static str,
) -> Result<XmlNode, Error> {
    let mut parts = XmlNodeParts::empty();

    parts.attributes.extend(serialize_association_attributes(
        abstract_ring_property.association(),
    ));
    parts.attributes.extend(serialize_ownership_attributes(
        abstract_ring_property.ownership(),
    ));

    if let Some(object) = abstract_ring_property.object() {
        parts
            .content
            .push(XmlNodeContent::Child(serialize_abstract_ring_kind(
                object, formatting,
            )?));
    }

    Ok(XmlNode::new(target_xml_element, parts))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlAbstractRingProperty {
    #[serde(flatten)]
    pub association: GmlAssociationAttributes,
    #[serde(flatten)]
    pub ownership: GmlOwnershipAttributes,
}

#[cfg(test)]
mod tests {
    use crate::codec::geometry::primitives::{
        deserialize_abstract_ring_property, serialize_abstract_ring_property,
    };
    use crate::util::{Formatting, GmlElement, extract_xml_element_spans};
    use egml_core::model::base::{HasAssociationAttributes, HasOwnershipAttributes};
    use egml_core::model::geometry::primitives::{AbstractRingKind, AbstractRingProperty};
    use egml_core::model::xlink::{ActuateType, HRef, ShowType};

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

        let spans = extract_xml_element_spans(xml_document).expect("should extract spans");
        let mut abstract_ring_property: AbstractRingProperty =
            deserialize_abstract_ring_property(xml_document.as_ref(), &spans)
                .expect("should deserialize");
        let abstract_ring_kind = abstract_ring_property
            .take_object()
            .expect("should be there");

        let AbstractRingKind::LinearRing(linear_ring) = abstract_ring_kind else {
            panic!("expected LinearRing variant");
        };

        assert_eq!(linear_ring.points().len(), 3);
    }

    #[test]
    fn deserialize_ring_property_as_ring() {
        let xml_document = b"<gml:exterior>
   <gml:Ring>
       <gml:curveMember>
          <gml:LineString>
              <gml:pos>0.0 0.0 0.0</gml:pos>
              <gml:pos>1.0 1.0 0.0</gml:pos>
              <gml:pos>1.0 1.0 1.0</gml:pos>
              <gml:pos>0.0 0.0 0.0</gml:pos>
          </gml:LineString>
       </gml:curveMember>
    </gml:Ring>
</gml:exterior>";

        let spans = extract_xml_element_spans(xml_document).expect("should extract spans");
        let property = deserialize_abstract_ring_property(xml_document.as_ref(), &spans)
            .expect("should deserialize");
        // Ring deserialization is not yet implemented; object will be None
        assert!(property.object().is_none());
    }

    #[test]
    fn deserialize_with_full_association_and_ownership_attributes() {
        let xml_document = b"<gml:exterior xlink:href=\"#some-id\" xlink:title=\"Some Title\" \
            xlink:role=\"http://example.com/role\" xlink:arcrole=\"http://example.com/arcrole\" \
            xlink:show=\"new\" xlink:actuate=\"onLoad\" gml:owns=\"true\"/>";

        let spans = extract_xml_element_spans(xml_document).unwrap();
        let property = deserialize_abstract_ring_property(xml_document, &spans).unwrap();

        assert_eq!(property.href(), Some(&HRef::from_local("some-id")));
        assert_eq!(property.title().as_deref(), Some("Some Title"));
        assert_eq!(property.role().as_deref(), Some("http://example.com/role"));
        assert_eq!(
            property.arcrole().as_deref(),
            Some("http://example.com/arcrole")
        );
        assert_eq!(property.show(), Some(&ShowType::New));
        assert_eq!(property.actuate(), Some(&ActuateType::OnLoad));
        assert!(property.owns());
        assert!(property.object().is_none());
    }

    #[test]
    fn round_trip_preserves_full_association_and_ownership_attributes() {
        let xml_document = b"<gml:exterior xlink:href=\"#some-id\" xlink:title=\"Some Title\" \
            xlink:role=\"http://example.com/role\" xlink:arcrole=\"http://example.com/arcrole\" \
            xlink:show=\"new\" xlink:actuate=\"onLoad\" gml:owns=\"true\"/>";

        let spans = extract_xml_element_spans(xml_document).unwrap();
        let property = deserialize_abstract_ring_property(xml_document, &spans).unwrap();

        let xml_node = serialize_abstract_ring_property(
            &property,
            Formatting::Compact,
            GmlElement::ExteriorProperty.into(),
        )
        .unwrap();
        let output = xml_node.to_string(Formatting::Compact).unwrap();

        let spans2 = extract_xml_element_spans(output.as_bytes()).unwrap();
        let recovered = deserialize_abstract_ring_property(output.as_bytes(), &spans2).unwrap();

        assert_eq!(
            recovered.association(),
            property.association(),
            "association attributes did not survive the round trip; output was: {output}"
        );
        assert_eq!(recovered.ownership(), property.ownership());
    }
}
