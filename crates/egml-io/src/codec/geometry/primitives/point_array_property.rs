use crate::Error;
use crate::codec::base::{
    GmlAssociationAttributes, GmlOwnershipAttributes, serialize_association_attributes,
    serialize_ownership_attributes,
};
use crate::codec::geometry::primitives::{deserialize_point, serialize_point};
use crate::util::{
    Formatting, GmlElement, XmlElementSpans, XmlNode, XmlNodeContent, XmlNodeParts,
    collect_children_simple,
};
use egml_core::model::base::{HasAssociationAttributes, HasOwnershipAttributes};
use egml_core::model::geometry::primitives::PointArrayProperty;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_point_array_property(
    xml_document: &[u8],
    spans: &XmlElementSpans<GmlElement>,
) -> Result<Option<PointArrayProperty>, Error> {
    let parsed: GmlPointArrayProperty = de::from_reader(xml_document)?;

    let objects =
        collect_children_simple(xml_document, spans, GmlElement::Point, deserialize_point)?;

    if objects.is_empty() {
        return Ok(None);
    }

    Ok(Some(PointArrayProperty::new(
        objects,
        parsed.association.try_into()?,
        parsed.ownership.into(),
    )))
}

pub fn serialize_point_array_property(
    point_array_property: &PointArrayProperty,
    formatting: Formatting,
    target_xml_element: &'static str,
) -> Result<XmlNode, Error> {
    let mut xml_node_parts = XmlNodeParts::empty();

    xml_node_parts
        .attributes
        .extend(serialize_association_attributes(
            point_array_property.association(),
        ));
    xml_node_parts
        .attributes
        .extend(serialize_ownership_attributes(
            point_array_property.ownership(),
        ));

    for point in point_array_property.objects() {
        xml_node_parts
            .content
            .push(XmlNodeContent::Child(serialize_point(point, formatting)?));
    }

    Ok(XmlNode::new(target_xml_element, xml_node_parts))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlPointArrayProperty {
    #[serde(flatten)]
    pub association: GmlAssociationAttributes,
    #[serde(flatten)]
    pub ownership: GmlOwnershipAttributes,
}

#[cfg(test)]
mod tests {
    use crate::codec::geometry::primitives::point_array_property::{
        deserialize_point_array_property, serialize_point_array_property,
    };
    use crate::util::{Formatting, GmlElement, extract_xml_element_spans};
    use egml_core::model::base::{HasAssociationAttributes, HasOwnershipAttributes};
    use egml_core::model::geometry::DirectPosition;
    use egml_core::model::geometry::primitives::{Point, PointArrayProperty};
    use egml_core::model::xlink::{ActuateType, HRef, ShowType};

    fn make_point_array_property() -> PointArrayProperty {
        PointArrayProperty::from_objects(vec![
            Point::new(DirectPosition::new(1.0, 2.0, 3.0).unwrap()),
            Point::new(DirectPosition::new(4.0, 5.0, 6.0).unwrap()),
        ])
    }

    #[test]
    fn deserialize_point_array_property_test() {
        let xml_document = b"<gml:pointMembers>
    <gml:Point>
        <gml:pos>1.0 2.0 3.0</gml:pos>
    </gml:Point>
    <gml:Point>
        <gml:pos>11.0 12.0 13.0</gml:pos>
    </gml:Point>
</gml:pointMembers>";

        let spans = extract_xml_element_spans(xml_document).expect("extracting spans should work");
        let property = deserialize_point_array_property(xml_document, &spans)
            .expect("should deserialize")
            .expect("should be some");

        assert_eq!(property.objects().len(), 2);
        assert_eq!(property.objects()[0].pos().x(), 1.0);
        assert_eq!(property.objects()[1].pos().x(), 11.0);
    }

    #[test]
    fn serialize_point_array_property_writes_gml_tags() {
        let property = make_point_array_property();

        let xml_node = serialize_point_array_property(
            &property,
            Formatting::Compact,
            GmlElement::PointMembersProperty.into(),
        )
        .expect("should serialize");
        let xml = xml_node
            .to_string(Formatting::Compact)
            .expect("to string should work");

        assert!(xml.contains("<gml:pointMembers"));
        assert!(xml.contains("<gml:Point"));
        assert!(xml.contains("<gml:pos"));
        assert_eq!(xml.matches("<gml:Point").count(), 2);
    }

    #[test]
    fn round_trip_point_array_property_preserves_points() {
        let xml_document = b"<gml:pointMembers>\
            <gml:Point><gml:pos srsDimension=\"3\">1 2 3</gml:pos></gml:Point>\
            <gml:Point><gml:pos srsDimension=\"3\">4 5 6</gml:pos></gml:Point>\
            </gml:pointMembers>";

        let spans = extract_xml_element_spans(xml_document).expect("extracting spans should work");
        let property = deserialize_point_array_property(xml_document, &spans)
            .unwrap()
            .unwrap();

        let xml_node = serialize_point_array_property(
            &property,
            Formatting::Compact,
            GmlElement::PointMembersProperty.into(),
        )
        .unwrap();
        let output = xml_node.to_string(Formatting::Compact).unwrap();

        let spans2 = extract_xml_element_spans(output.as_bytes()).unwrap();
        let recovered = deserialize_point_array_property(output.as_bytes(), &spans2)
            .unwrap()
            .unwrap();

        assert_eq!(recovered.objects().len(), 2);
        assert_eq!(
            recovered.objects()[0].pos().x(),
            property.objects()[0].pos().x()
        );
        assert_eq!(
            recovered.objects()[1].pos().x(),
            property.objects()[1].pos().x()
        );
    }

    #[test]
    fn deserialize_with_full_association_and_ownership_attributes() {
        let xml_document = b"<gml:pointMembers xlink:href=\"#some-id\" xlink:title=\"Some Title\" \
            xlink:role=\"http://example.com/role\" xlink:arcrole=\"http://example.com/arcrole\" \
            xlink:show=\"new\" xlink:actuate=\"onLoad\" gml:owns=\"true\">\
            <gml:Point><gml:pos srsDimension=\"3\">1 2 3</gml:pos></gml:Point>\
            </gml:pointMembers>";

        let spans = extract_xml_element_spans(xml_document).unwrap();
        let property = deserialize_point_array_property(xml_document, &spans)
            .unwrap()
            .unwrap();

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
    }

    #[test]
    fn round_trip_preserves_full_association_and_ownership_attributes() {
        let xml_document = b"<gml:pointMembers xlink:href=\"#some-id\" xlink:title=\"Some Title\" \
            xlink:role=\"http://example.com/role\" xlink:arcrole=\"http://example.com/arcrole\" \
            xlink:show=\"new\" xlink:actuate=\"onLoad\" gml:owns=\"true\">\
            <gml:Point><gml:pos srsDimension=\"3\">1 2 3</gml:pos></gml:Point>\
            </gml:pointMembers>";

        let spans = extract_xml_element_spans(xml_document).unwrap();
        let property = deserialize_point_array_property(xml_document, &spans)
            .unwrap()
            .unwrap();

        let xml_node = serialize_point_array_property(
            &property,
            Formatting::Compact,
            GmlElement::PointMembersProperty.into(),
        )
        .unwrap();
        let output = xml_node.to_string(Formatting::Compact).unwrap();

        let spans2 = extract_xml_element_spans(output.as_bytes()).unwrap();
        let recovered = deserialize_point_array_property(output.as_bytes(), &spans2)
            .unwrap()
            .unwrap();

        assert_eq!(
            recovered.association(),
            property.association(),
            "association attributes did not survive the round trip; output was: {output}"
        );
        assert_eq!(recovered.ownership(), property.ownership());
    }
}
