use crate::Error;
use crate::codec::base::{
    GmlAssociationAttributes, GmlOwnershipAttributes, serialize_association_attributes,
    serialize_ownership_attributes,
};
use crate::codec::geometry::primitives::{deserialize_point, serialize_point};
use crate::util::{Formatting, GmlElement, XmlElementSpans, XmlNode, XmlNodeContent, XmlNodeParts};
use egml_core::model::base::{HasAssociationAttributes, HasOwnershipAttributes};
use egml_core::model::geometry::primitives::PointProperty;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_point_property(
    xml_document: &[u8],
    spans: &XmlElementSpans<GmlElement>,
) -> Result<PointProperty, Error> {
    let parsed: GmlPointProperty = de::from_reader(xml_document)?;

    let object = spans
        .first(GmlElement::Point)
        .map(|span| deserialize_point(&xml_document[span.start..span.end]))
        .transpose()?;

    Ok(PointProperty::new(
        object,
        parsed.association.try_into()?,
        parsed.ownership.into(),
    ))
}

pub fn serialize_point_property(
    point_property: &PointProperty,
    formatting: Formatting,
    target_xml_element: &'static str,
) -> Result<XmlNode, Error> {
    let mut xml_node_parts: XmlNodeParts = XmlNodeParts::empty();

    xml_node_parts
        .attributes
        .extend(serialize_association_attributes(
            point_property.association(),
        ));
    xml_node_parts
        .attributes
        .extend(serialize_ownership_attributes(point_property.ownership()));

    if let Some(point) = point_property.object() {
        xml_node_parts
            .content
            .push(XmlNodeContent::Child(serialize_point(point, formatting)?));
    }

    Ok(XmlNode::new(target_xml_element, xml_node_parts))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlPointProperty {
    #[serde(flatten)]
    pub association: GmlAssociationAttributes,
    #[serde(flatten)]
    pub ownership: GmlOwnershipAttributes,
}

#[cfg(test)]
mod tests {
    use crate::codec::geometry::primitives::point_property::{
        deserialize_point_property, serialize_point_property,
    };
    use crate::util::{Formatting, GmlElement, XmlNode, extract_xml_element_spans};
    use egml_core::model::base::{HasAssociationAttributes, HasOwnershipAttributes};
    use egml_core::model::geometry::DirectPosition;
    use egml_core::model::geometry::primitives::{Point, PointProperty};
    use egml_core::model::xlink::{ActuateType, HRef, ShowType};

    fn make_point_property() -> PointProperty {
        PointProperty::from_object(Point::new(DirectPosition::new(1.0, 2.0, 3.0).unwrap()))
    }

    #[test]
    fn deserialize_point_property_test() {
        let xml_document = b"<gml:pointMember>
    <gml:Point>
        <gml:pos>1.0 2.0 3.0</gml:pos>
    </gml:Point>
</gml:pointMember>";

        let spans = extract_xml_element_spans(xml_document).expect("extracting spans should work");
        let property =
            deserialize_point_property(xml_document, &spans).expect("should deserialize");

        assert!(property.object().is_some());
        assert_eq!(property.object().unwrap().pos().x(), 1.0);
    }

    #[test]
    fn serialize_point_property_writes_gml_tags() {
        let property = make_point_property();

        let xml_node = serialize_point_property(
            &property,
            Formatting::Compact,
            GmlElement::PointMemberProperty.into(),
        )
        .expect("should serialize");
        let xml = xml_node
            .to_string(Formatting::Compact)
            .expect("to string should work");

        assert!(xml.contains("<gml:pointMember"));
        assert!(xml.contains("<gml:Point"));
        assert!(xml.contains("<gml:pos"));
    }

    #[test]
    fn round_trip_point_property_preserves_point() {
        let xml_document = b"<gml:pointMember>\
            <gml:Point><gml:pos srsDimension=\"3\">1 2 3</gml:pos></gml:Point>\
            </gml:pointMember>";

        let spans = extract_xml_element_spans(xml_document).expect("extracting spans should work");
        let property = deserialize_point_property(xml_document, &spans).unwrap();

        let xml_node = serialize_point_property(
            &property,
            Formatting::Compact,
            GmlElement::PointMemberProperty.into(),
        )
        .unwrap();
        let output = xml_node
            .to_string(Formatting::Compact)
            .expect("to string should work");

        let spans2 = extract_xml_element_spans(output.as_bytes()).unwrap();
        let recovered = deserialize_point_property(output.as_bytes(), &spans2).unwrap();

        assert_eq!(
            recovered.object().unwrap().pos().x(),
            property.object().unwrap().pos().x()
        );
    }

    #[test]
    fn serialize_point_property_newline_formatting() {
        let formatting = Formatting::NewLine;
        let xml_node = serialize_point_property(
            &make_point_property(),
            formatting,
            GmlElement::PointMemberProperty.into(),
        )
        .unwrap();
        let xml = xml_node.to_string(formatting).unwrap();

        assert!(
            !xml.starts_with('\n'),
            "output must not begin with a newline"
        );
        assert!(
            xml.contains('\n'),
            "NewLine formatting must produce newlines"
        );

        // All GML tags must sit at column 0 — no indentation in NewLine mode
        for line in xml.lines() {
            if line.starts_with("<gml:") || line.starts_with("</gml:") {
                assert_eq!(
                    line,
                    line.trim_start(),
                    "GML tag must not be indented in NewLine mode: {line:?}"
                );
            }
        }

        assert!(xml.contains("<gml:pointMember"));
        assert!(xml.contains("<gml:Point"));
        assert!(xml.contains("<gml:pos"));
    }

    #[test]
    fn serialize_point_property_indent_two_spaces() {
        let formatting = Formatting::Indent { char: ' ', size: 2 };
        let xml_node = serialize_point_property(
            &make_point_property(),
            formatting,
            GmlElement::PointMemberProperty.into(),
        )
        .unwrap();
        let xml = xml_node.to_string(formatting).unwrap();

        assert!(!xml.starts_with('\n'));
        assert!(xml.starts_with("<gml:pointMember")); // wrapper at depth 0

        assert!(xml.contains("\n  <gml:Point")); // depth 1 → 2 spaces
        assert!(xml.contains("\n    <gml:pos")); // depth 2 → 4 spaces
        assert!(xml.contains("\n  </gml:Point"));
        assert!(xml.contains("\n</gml:pointMember")); // closing wrapper at depth 0
    }

    #[test]
    fn serialize_point_property_indent_tabs() {
        let formatting = Formatting::Indent {
            char: '\t',
            size: 1,
        };
        let xml_node = serialize_point_property(
            &make_point_property(),
            formatting,
            GmlElement::PointMemberProperty.into(),
        )
        .unwrap();
        let xml = xml_node.to_string(formatting).unwrap();

        assert!(!xml.starts_with('\n'));
        assert!(xml.starts_with("<gml:pointMember"));

        assert!(xml.contains("\n\t<gml:Point")); // depth 1 → 1 tab
        assert!(xml.contains("\n\t\t<gml:pos")); // depth 2 → 2 tabs
        assert!(xml.contains("\n\t</gml:Point"));
        assert!(xml.contains("\n</gml:pointMember"));
    }

    #[test]
    fn deserialize_point_property_with_full_association_and_ownership_attributes() {
        let xml_document = b"<gml:pointMember xlink:href=\"#some-id\" xlink:title=\"Some Title\" \
            xlink:role=\"http://example.com/role\" xlink:arcrole=\"http://example.com/arcrole\" \
            xlink:show=\"new\" xlink:actuate=\"onLoad\" gml:owns=\"true\"/>";

        let spans = extract_xml_element_spans(xml_document).unwrap();
        let property = deserialize_point_property(xml_document, &spans).unwrap();

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
        let property = deserialize_point_property(xml_document, &spans).unwrap();

        let xml_node = serialize_point_property(
            &property,
            Formatting::Compact,
            GmlElement::PointMemberProperty.into(),
        )
        .unwrap();
        let output = xml_node.to_string(Formatting::Compact).unwrap();

        let spans2 = extract_xml_element_spans(output.as_bytes()).unwrap();
        let recovered = deserialize_point_property(output.as_bytes(), &spans2).unwrap();

        assert_eq!(
            recovered.association(),
            property.association(),
            "association attributes did not survive the round trip; output was: {output}"
        );
        assert_eq!(recovered.ownership(), property.ownership());
    }
}
