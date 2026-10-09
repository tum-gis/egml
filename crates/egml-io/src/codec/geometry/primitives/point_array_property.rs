use crate::Error;
use crate::codec::base::{deserialize_ownership_attributes, serialize_ownership_attributes};
use crate::codec::geometry::primitives::point::deserialize_point;
use crate::codec::geometry::primitives::serialize_point;
use crate::util::{
    DeserializationConfig, GmlElement, XmlDocumentIndex, XmlElement, XmlFragmentWriter,
    XmlNamespace, collect_children,
};
use egml_core::model::base::HasOwnershipAttributes;
use egml_core::model::geometry::primitives::PointArrayProperty;
use std::io::Write;

pub fn deserialize_point_array_property(
    xml_document: &[u8],
    index: &XmlDocumentIndex<GmlElement>,
    config: &DeserializationConfig,
) -> Result<Option<PointArrayProperty>, Error> {
    let index = if index.is_truncated() {
        &XmlDocumentIndex::from_scan(xml_document, config.rescan_depth())?
    } else {
        index
    };

    let ownership = deserialize_ownership_attributes(xml_document)?;
    let objects = collect_children(
        xml_document,
        index,
        GmlElement::Point,
        config,
        deserialize_point,
    )?;

    if objects.is_empty() {
        return Ok(None);
    }

    Ok(Some(PointArrayProperty::new(objects, ownership)))
}

pub fn serialize_point_array_property<N: XmlNamespace, E: XmlElement, W: Write>(
    point_array_property: &PointArrayProperty,
    xml_fragment_writer: &mut XmlFragmentWriter<W>,
    target_xml_namespace: N,
    target_xml_element: E,
) -> Result<(), Error> {
    let attributes = serialize_ownership_attributes(point_array_property.ownership());

    xml_fragment_writer.write_start_event_with_attributes(
        target_xml_namespace,
        target_xml_element,
        attributes,
    )?;

    for point in point_array_property.objects() {
        serialize_point(point, xml_fragment_writer)?;
    }

    xml_fragment_writer.write_end_event(target_xml_namespace, target_xml_element)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::codec::geometry::primitives::point_array_property::{
        deserialize_point_array_property, serialize_point_array_property,
    };
    use crate::util::{Formatting, GmlElement, GmlNamespace, XmlDocumentIndex, XmlFragmentWriter};
    use egml_core::model::base::HasOwnershipAttributes;
    use egml_core::model::geometry::DirectPosition;
    use egml_core::model::geometry::primitives::{Point, PointArrayProperty};

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

        let index =
            XmlDocumentIndex::from_scan(xml_document, None).expect("extracting spans should work");
        let property = deserialize_point_array_property(
            xml_document,
            &index,
            &crate::util::DeserializationConfig::default(),
        )
        .expect("should deserialize")
        .expect("should be some");

        assert_eq!(property.objects().len(), 2);
        assert_eq!(property.objects()[0].pos().x(), 1.0);
        assert_eq!(property.objects()[1].pos().x(), 11.0);
    }

    #[test]
    fn serialize_point_array_property_writes_gml_tags() {
        let property = make_point_array_property();

        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_point_array_property(
            &property,
            &mut xml_fragment_writer,
            GmlNamespace::Gml,
            GmlElement::PointMembersProperty,
        )
        .expect("should serialize");
        let xml = String::from_utf8(xml_fragment_writer.into_bytes()).expect("valid UTF-8");

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

        let index =
            XmlDocumentIndex::from_scan(xml_document, None).expect("extracting spans should work");
        let property = deserialize_point_array_property(
            xml_document,
            &index,
            &crate::util::DeserializationConfig::default(),
        )
        .unwrap()
        .unwrap();

        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_point_array_property(
            &property,
            &mut xml_fragment_writer,
            GmlNamespace::Gml,
            GmlElement::PointMembersProperty,
        )
        .unwrap();
        let output = String::from_utf8(xml_fragment_writer.into_bytes()).unwrap();

        let spans2 = XmlDocumentIndex::from_scan(output.as_bytes(), None).unwrap();
        let recovered = deserialize_point_array_property(
            output.as_bytes(),
            &spans2,
            &crate::util::DeserializationConfig::default(),
        )
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
    fn deserialize_keeps_ownership_and_ignores_xlink_attributes() {
        // gml:ArrayAssociationType declares only gml:OwnershipAttributeGroup.
        let xml_document = b"<gml:pointMembers xlink:href=\"#some-id\" xlink:title=\"Some Title\" \
            xlink:role=\"http://example.com/role\" xlink:arcrole=\"http://example.com/arcrole\" \
            xlink:show=\"new\" xlink:actuate=\"onLoad\" gml:owns=\"true\">\
            <gml:Point><gml:pos srsDimension=\"3\">1 2 3</gml:pos></gml:Point>\
            </gml:pointMembers>";

        let index = XmlDocumentIndex::from_scan(xml_document, None).unwrap();
        let property = deserialize_point_array_property(
            xml_document,
            &index,
            &crate::util::DeserializationConfig::default(),
        )
        .unwrap()
        .unwrap();

        assert_eq!(property.objects().len(), 1);
        assert!(property.owns());
    }

    #[test]
    fn round_trip_keeps_ownership_and_drops_xlink_attributes() {
        let xml_document = b"<gml:pointMembers xlink:href=\"#some-id\" xlink:title=\"Some Title\" \
            xlink:role=\"http://example.com/role\" xlink:arcrole=\"http://example.com/arcrole\" \
            xlink:show=\"new\" xlink:actuate=\"onLoad\" gml:owns=\"true\">\
            <gml:Point><gml:pos srsDimension=\"3\">1 2 3</gml:pos></gml:Point>\
            </gml:pointMembers>";

        let index = XmlDocumentIndex::from_scan(xml_document, None).unwrap();
        let property = deserialize_point_array_property(
            xml_document,
            &index,
            &crate::util::DeserializationConfig::default(),
        )
        .unwrap()
        .unwrap();

        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_point_array_property(
            &property,
            &mut xml_fragment_writer,
            GmlNamespace::Gml,
            GmlElement::PointMembersProperty,
        )
        .unwrap();
        let output = String::from_utf8(xml_fragment_writer.into_bytes()).unwrap();

        let spans2 = XmlDocumentIndex::from_scan(output.as_bytes(), None).unwrap();
        let recovered = deserialize_point_array_property(
            output.as_bytes(),
            &spans2,
            &crate::util::DeserializationConfig::default(),
        )
        .unwrap()
        .unwrap();

        assert!(
            !output.contains("xlink"),
            "unexpected xlink attribute in {output}"
        );
        assert!(recovered.owns());
    }
}
