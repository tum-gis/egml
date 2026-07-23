use crate::Error;
use crate::codec::base::{
    GmlAssociationAttributes, GmlOwnershipAttributes, serialize_association_attributes,
    serialize_ownership_attributes,
};
use crate::codec::geometry::aggregates::{deserialize_multi_point, serialize_multi_point};
use crate::util::{Formatting, GmlElement, XmlElementSpans, XmlNode, XmlNodeContent, XmlNodeParts};
use egml_core::model::base::{HasAssociationAttributes, HasOwnershipAttributes};
use egml_core::model::geometry::aggregates::MultiPointProperty;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_multi_point_property(
    xml_document: &[u8],
    spans: &XmlElementSpans<GmlElement>,
) -> Result<MultiPointProperty, Error> {
    let parsed: GmlMultiPointProperty = de::from_reader(xml_document)?;

    let object = spans
        .first(GmlElement::MultiPoint)
        .map(|span| deserialize_multi_point(&xml_document[span.start..span.end]))
        .transpose()?;

    Ok(MultiPointProperty::new(
        object,
        parsed.association.try_into()?,
        parsed.ownership.into(),
    ))
}

pub fn serialize_multi_point_property(
    multi_point_property: &MultiPointProperty,
    formatting: Formatting,
    target_xml_element: &'static str,
) -> Result<XmlNode, Error> {
    let mut xml_node_parts = XmlNodeParts::empty();

    xml_node_parts
        .attributes
        .extend(serialize_association_attributes(
            multi_point_property.association(),
        ));
    xml_node_parts
        .attributes
        .extend(serialize_ownership_attributes(
            multi_point_property.ownership(),
        ));

    if let Some(multi_point) = multi_point_property.object() {
        let raw = serialize_multi_point(multi_point, formatting)?.to_string(formatting)?;
        xml_node_parts.content.push(XmlNodeContent::Raw(raw));
    }

    Ok(XmlNode::new(target_xml_element, xml_node_parts))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlMultiPointProperty {
    #[serde(flatten)]
    pub association: GmlAssociationAttributes,
    #[serde(flatten)]
    pub ownership: GmlOwnershipAttributes,
}

#[cfg(test)]
mod tests {
    use crate::codec::geometry::aggregates::multi_point_property::{
        deserialize_multi_point_property, serialize_multi_point_property,
    };
    use crate::util::{Formatting, GmlElement, extract_xml_element_spans};
    use egml_core::model::base::{HasAssociationAttributes, HasOwnershipAttributes};
    use egml_core::model::geometry::DirectPosition;
    use egml_core::model::geometry::aggregates::{MultiPoint, MultiPointProperty};
    use egml_core::model::geometry::primitives::{Point, PointProperty};
    use egml_core::model::xlink::{ActuateType, HRef, ShowType};

    fn make_multi_point_property() -> MultiPointProperty {
        let mut multi_point = MultiPoint::new(None).unwrap();
        multi_point.set_point_member(vec![
            PointProperty::from_object(Point::new(DirectPosition::new(1.0, 2.0, 3.0).unwrap())),
            PointProperty::from_object(Point::new(DirectPosition::new(4.0, 5.0, 6.0).unwrap())),
        ]);
        MultiPointProperty::from_object(multi_point)
    }

    #[test]
    fn deserialize_multi_point_property_test() {
        let xml_document = b"<gml:pointMember>
            <gml:MultiPoint>
                <gml:pointMember>
                    <gml:Point><gml:pos srsDimension=\"3\">1 2 3</gml:pos></gml:Point>
                </gml:pointMember>
                <gml:pointMember>
                    <gml:Point><gml:pos srsDimension=\"3\">4 5 6</gml:pos></gml:Point>
                </gml:pointMember>
            </gml:MultiPoint>
        </gml:pointMember>";

        let spans = extract_xml_element_spans(xml_document).expect("extracting spans should work");
        let property =
            deserialize_multi_point_property(xml_document, &spans).expect("should deserialize");

        assert!(property.object().is_some());
        assert_eq!(property.object().unwrap().point_member().len(), 2);
    }

    #[test]
    fn deserialize_multi_point_property_with_xlink() {
        let xml_document = b"<gml:pointMember xlink:href=\"#some-point-id\"/>";

        let spans = extract_xml_element_spans(xml_document).expect("extracting spans should work");
        let property =
            deserialize_multi_point_property(xml_document, &spans).expect("should deserialize");

        assert_eq!(property.href(), Some(&HRef::from_local("some-point-id")));
        assert!(property.object().is_none());
    }

    #[test]
    fn serialize_multi_point_property_writes_gml_tags() {
        let property = make_multi_point_property();

        let xml_node = serialize_multi_point_property(
            &property,
            Formatting::Compact,
            GmlElement::PointMemberProperty.into(),
        )
        .expect("should serialize");
        let xml = xml_node
            .to_string(Formatting::Compact)
            .expect("to string should work");

        assert!(xml.contains("<gml:pointMember"));
        assert!(xml.contains("<gml:MultiPoint"));
        assert!(xml.contains("<gml:Point"));
        assert!(xml.contains("<gml:pos"));
    }

    #[test]
    fn round_trip_multi_point_property_preserves_member_count() {
        let xml_document = b"<gml:pointMember>\
            <gml:MultiPoint>\
            <gml:pointMember><gml:Point>\
            <gml:pos srsDimension=\"3\">1 2 3</gml:pos>\
            </gml:Point></gml:pointMember>\
            </gml:MultiPoint>\
            </gml:pointMember>";

        let spans = extract_xml_element_spans(xml_document).unwrap();
        let property = deserialize_multi_point_property(xml_document, &spans).unwrap();

        let xml_node = serialize_multi_point_property(
            &property,
            Formatting::Compact,
            GmlElement::PointMemberProperty.into(),
        )
        .unwrap();
        let output = xml_node.to_string(Formatting::Compact).unwrap();

        let spans2 = extract_xml_element_spans(output.as_bytes()).unwrap();
        let recovered = deserialize_multi_point_property(output.as_bytes(), &spans2).unwrap();

        assert_eq!(
            recovered.object().unwrap().point_member().len(),
            property.object().unwrap().point_member().len()
        );
    }

    #[test]
    fn deserialize_with_full_association_and_ownership_attributes() {
        let xml_document = b"<gml:pointMember xlink:href=\"#some-id\" xlink:title=\"Some Title\" \
            xlink:role=\"http://example.com/role\" xlink:arcrole=\"http://example.com/arcrole\" \
            xlink:show=\"new\" xlink:actuate=\"onLoad\" gml:owns=\"true\"/>";

        let spans = extract_xml_element_spans(xml_document).unwrap();
        let property = deserialize_multi_point_property(xml_document, &spans).unwrap();

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
        let xml_document = b"<gml:pointMember xlink:href=\"#some-id\" xlink:title=\"Some Title\" \
            xlink:role=\"http://example.com/role\" xlink:arcrole=\"http://example.com/arcrole\" \
            xlink:show=\"new\" xlink:actuate=\"onLoad\" gml:owns=\"true\"/>";

        let spans = extract_xml_element_spans(xml_document).unwrap();
        let property = deserialize_multi_point_property(xml_document, &spans).unwrap();

        let xml_node = serialize_multi_point_property(
            &property,
            Formatting::Compact,
            GmlElement::PointMemberProperty.into(),
        )
        .unwrap();
        let output = xml_node.to_string(Formatting::Compact).unwrap();

        let spans2 = extract_xml_element_spans(output.as_bytes()).unwrap();
        let recovered = deserialize_multi_point_property(output.as_bytes(), &spans2).unwrap();

        assert_eq!(
            recovered.association(),
            property.association(),
            "association attributes did not survive the round trip; output was: {output}"
        );
        assert_eq!(recovered.ownership(), property.ownership());
    }
}
