use crate::Error;
use crate::codec::base::{
    deserialize_association_and_ownership_attributes,
    serialize_association_and_ownership_attributes,
};
use crate::codec::geometry::primitives::point::deserialize_point;
use crate::codec::geometry::primitives::serialize_point;
use crate::util::{
    DeserializationConfig, GmlElement, XmlDocumentIndex, XmlElement, XmlFragmentWriter,
    XmlNamespace,
};
use egml_core::model::base::{HasAssociationAttributes, HasOwnershipAttributes};
use egml_core::model::geometry::primitives::PointProperty;
use std::io::Write;

pub fn deserialize_point_property(
    xml_document: &[u8],
    index: &XmlDocumentIndex<GmlElement>,
    config: &DeserializationConfig,
) -> Result<PointProperty, Error> {
    let index = if index.is_truncated() {
        &XmlDocumentIndex::from_scan(xml_document, config.rescan_depth())?
    } else {
        index
    };

    let (association, ownership) = deserialize_association_and_ownership_attributes(xml_document)?;

    let object = index
        .first(GmlElement::Point)
        .map(|node| deserialize_point(&xml_document[node.range()], node, config))
        .transpose()?;

    Ok(PointProperty::new(object, association, ownership))
}

pub fn serialize_point_property<N: XmlNamespace, E: XmlElement, W: Write>(
    point_property: &PointProperty,
    xml_fragment_writer: &mut XmlFragmentWriter<W>,
    target_xml_namespace: N,
    target_xml_element: E,
) -> Result<(), Error> {
    let attributes = serialize_association_and_ownership_attributes(
        point_property.association(),
        point_property.ownership(),
    );

    match point_property.object() {
        Some(point) => {
            xml_fragment_writer.write_start_event_with_attributes(
                target_xml_namespace,
                target_xml_element,
                attributes,
            )?;
            serialize_point(point, xml_fragment_writer)?;
            xml_fragment_writer.write_end_event(target_xml_namespace, target_xml_element)?;
        }
        None => {
            xml_fragment_writer.write_empty_element_with_attributes(
                target_xml_namespace,
                target_xml_element,
                attributes,
            )?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::codec::geometry::primitives::point_property::{
        deserialize_point_property, serialize_point_property,
    };
    use crate::util::{Formatting, GmlElement, GmlNamespace, XmlDocumentIndex, XmlFragmentWriter};
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

        let index =
            XmlDocumentIndex::from_scan(xml_document, None).expect("extracting spans should work");
        let property = deserialize_point_property(
            xml_document,
            &index,
            &crate::util::DeserializationConfig::default(),
        )
        .expect("should deserialize");

        assert!(property.object().is_some());
        assert_eq!(property.object().unwrap().pos().x(), 1.0);
    }

    #[test]
    fn serialize_point_property_writes_gml_tags() {
        let property = make_point_property();

        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_point_property(
            &property,
            &mut xml_fragment_writer,
            GmlNamespace::Gml,
            GmlElement::PointMemberProperty,
        )
        .expect("should serialize");
        let xml = String::from_utf8(xml_fragment_writer.into_bytes()).unwrap();

        assert!(xml.contains("<gml:pointMember"));
        assert!(xml.contains("<gml:Point"));
        assert!(xml.contains("<gml:pos"));
    }

    #[test]
    fn round_trip_point_property_preserves_point() {
        let xml_document = b"<gml:pointMember>\
            <gml:Point><gml:pos srsDimension=\"3\">1 2 3</gml:pos></gml:Point>\
            </gml:pointMember>";

        let index =
            XmlDocumentIndex::from_scan(xml_document, None).expect("extracting spans should work");
        let property = deserialize_point_property(
            xml_document,
            &index,
            &crate::util::DeserializationConfig::default(),
        )
        .unwrap();

        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_point_property(
            &property,
            &mut xml_fragment_writer,
            GmlNamespace::Gml,
            GmlElement::PointMemberProperty,
        )
        .unwrap();
        let output = xml_fragment_writer.into_bytes();

        let spans2 = XmlDocumentIndex::from_scan(&output, None).unwrap();
        let recovered = deserialize_point_property(
            &output,
            &spans2,
            &crate::util::DeserializationConfig::default(),
        )
        .unwrap();

        assert_eq!(
            recovered.object().unwrap().pos().x(),
            property.object().unwrap().pos().x()
        );
    }

    #[test]
    fn serialize_point_property_newline_formatting() {
        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::NewLine);
        serialize_point_property(
            &make_point_property(),
            &mut xml_fragment_writer,
            GmlNamespace::Gml,
            GmlElement::PointMemberProperty,
        )
        .unwrap();
        let xml = String::from_utf8(xml_fragment_writer.into_bytes()).unwrap();

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
        let formatting = Formatting::Indent {
            char: b' ',
            size: 2,
        };
        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(formatting);
        serialize_point_property(
            &make_point_property(),
            &mut xml_fragment_writer,
            GmlNamespace::Gml,
            GmlElement::PointMemberProperty,
        )
        .unwrap();
        let xml = String::from_utf8(xml_fragment_writer.into_bytes()).unwrap();

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
            char: b'\t',
            size: 1,
        };
        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(formatting);
        serialize_point_property(
            &make_point_property(),
            &mut xml_fragment_writer,
            GmlNamespace::Gml,
            GmlElement::PointMemberProperty,
        )
        .unwrap();
        let xml = String::from_utf8(xml_fragment_writer.into_bytes()).unwrap();

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

        let index = XmlDocumentIndex::from_scan(xml_document, None).unwrap();
        let property = deserialize_point_property(
            xml_document,
            &index,
            &crate::util::DeserializationConfig::default(),
        )
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
        assert!(property.object().is_none());
    }

    #[test]
    fn round_trip_preserves_full_association_and_ownership_attributes() {
        let xml_document = b"<gml:pointMember xlink:href=\"#some-id\" xlink:title=\"Some Title\" \
            xlink:role=\"http://example.com/role\" xlink:arcrole=\"http://example.com/arcrole\" \
            xlink:show=\"new\" xlink:actuate=\"onLoad\" gml:owns=\"true\"/>";

        let index = XmlDocumentIndex::from_scan(xml_document, None).unwrap();
        let property = deserialize_point_property(
            xml_document,
            &index,
            &crate::util::DeserializationConfig::default(),
        )
        .unwrap();

        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_point_property(
            &property,
            &mut xml_fragment_writer,
            GmlNamespace::Gml,
            GmlElement::PointMemberProperty,
        )
        .unwrap();
        let output = String::from_utf8(xml_fragment_writer.into_bytes()).unwrap();

        let spans2 = XmlDocumentIndex::from_scan(output.as_bytes(), None).unwrap();
        let recovered = deserialize_point_property(
            output.as_bytes(),
            &spans2,
            &crate::util::DeserializationConfig::default(),
        )
        .unwrap();

        assert_eq!(
            recovered.association(),
            property.association(),
            "association attributes did not survive the round trip; output was: {output}"
        );
        assert_eq!(recovered.ownership(), property.ownership());
    }

    #[test]
    fn serialize_href_only_property_writes_self_closed_tag() {
        let property = PointProperty::from_href(HRef::from_local("some-id"));

        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_point_property(
            &property,
            &mut xml_fragment_writer,
            GmlNamespace::Gml,
            GmlElement::PointMemberProperty,
        )
        .unwrap();
        let xml = String::from_utf8(xml_fragment_writer.into_bytes()).unwrap();

        assert_eq!(xml, r##"<gml:pointMember xlink:href="#some-id"/>"##);
    }
}
