use crate::Error;
use crate::codec::base::{
    GmlAssociationAttributes, GmlOwnershipAttributes, serialize_association_attributes,
    serialize_ownership_attributes,
};
use crate::codec::geometry::primitives::{deserialize_linear_ring, serialize_linear_ring};
use crate::util::{Formatting, GmlElement, XmlElementSpans, XmlNode, XmlNodeContent, XmlNodeParts};
use egml_core::model::base::{HasAssociationAttributes, HasOwnershipAttributes};
use egml_core::model::geometry::primitives::LinearRingProperty;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_linear_ring_property(
    xml_document: &[u8],
    spans: &XmlElementSpans<GmlElement>,
) -> Result<LinearRingProperty, Error> {
    let parsed: GmlLinearRingProperty = de::from_reader(xml_document)?;

    let object = spans
        .first(GmlElement::LinearRing)
        .map(|span| deserialize_linear_ring(&xml_document[span.start..span.end]))
        .transpose()?;

    Ok(LinearRingProperty::new(
        object,
        parsed.association.try_into()?,
        parsed.ownership.into(),
    ))
}

pub fn serialize_linear_ring_property(
    linear_ring_property: &LinearRingProperty,
    formatting: Formatting,
    target_xml_element: &'static str,
) -> Result<XmlNode, Error> {
    let mut xml_node_parts = XmlNodeParts::empty();

    xml_node_parts
        .attributes
        .extend(serialize_association_attributes(
            linear_ring_property.association(),
        ));
    xml_node_parts
        .attributes
        .extend(serialize_ownership_attributes(
            linear_ring_property.ownership(),
        ));

    if let Some(ring) = linear_ring_property.object() {
        xml_node_parts
            .content
            .push(XmlNodeContent::Child(serialize_linear_ring(
                ring, formatting,
            )?));
    }

    Ok(XmlNode::new(target_xml_element, xml_node_parts))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlLinearRingProperty {
    #[serde(flatten)]
    pub association: GmlAssociationAttributes,
    #[serde(flatten)]
    pub ownership: GmlOwnershipAttributes,
}

#[cfg(test)]
mod tests {
    use crate::codec::geometry::primitives::linear_ring_property::{
        deserialize_linear_ring_property, serialize_linear_ring_property,
    };
    use crate::util::{Formatting, GmlElement, extract_xml_element_spans};
    use egml_core::model::geometry::DirectPosition;
    use egml_core::model::geometry::primitives::{LinearRing, LinearRingProperty};

    fn make_linear_ring_property() -> LinearRingProperty {
        LinearRingProperty::from_object(
            LinearRing::new([
                DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
                DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
                DirectPosition::new(0.0, 1.0, 0.0).unwrap(),
            ])
            .unwrap(),
        )
    }

    #[test]
    fn deserialize_linear_ring_property_test() {
        let xml_document = b"<gml:exterior>
    <gml:LinearRing>
        <gml:posList srsDimension=\"3\">0 0 0 1 0 0 0 1 0 0 0 0</gml:posList>
    </gml:LinearRing>
</gml:exterior>";

        let spans = extract_xml_element_spans(xml_document).expect("extracting spans should work");
        let property =
            deserialize_linear_ring_property(xml_document, &spans).expect("should deserialize");

        assert!(property.object().is_some());
        assert_eq!(property.object().unwrap().points().len(), 3);
    }

    #[test]
    fn serialize_linear_ring_property_writes_gml_tags() {
        let property = make_linear_ring_property();

        let xml_node = serialize_linear_ring_property(
            &property,
            Formatting::Compact,
            GmlElement::ExteriorProperty.into(),
        )
        .expect("should serialize");
        let xml = xml_node
            .to_string(Formatting::Compact)
            .expect("to string should work");

        assert!(xml.contains("<gml:exterior"));
        assert!(xml.contains("<gml:LinearRing"));
        assert!(xml.contains("<gml:posList"));
    }

    #[test]
    fn round_trip_linear_ring_property_preserves_points() {
        let xml_document = b"<gml:exterior>\
            <gml:LinearRing><gml:posList srsDimension=\"3\">0 0 0 1 0 0 0 1 0 0 0 0</gml:posList></gml:LinearRing>\
            </gml:exterior>";

        let spans = extract_xml_element_spans(xml_document).expect("extracting spans should work");
        let property = deserialize_linear_ring_property(xml_document, &spans).unwrap();

        let xml_node = serialize_linear_ring_property(
            &property,
            Formatting::Compact,
            GmlElement::ExteriorProperty.into(),
        )
        .unwrap();
        let output = xml_node.to_string(Formatting::Compact).unwrap();

        let spans2 = extract_xml_element_spans(output.as_bytes()).unwrap();
        let recovered = deserialize_linear_ring_property(output.as_bytes(), &spans2).unwrap();

        assert_eq!(
            recovered.object().unwrap().points().len(),
            property.object().unwrap().points().len()
        );
    }
}
