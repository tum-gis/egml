use crate::Error;
use crate::codec::geometry::aggregates::{
    deserialize_abstract_geometric_aggregate, serialize_abstract_geometric_aggregate,
    serialize_abstract_geometric_aggregate_attributes,
};
use crate::codec::geometry::primitives::{
    deserialize_point_array_property, deserialize_point_property, serialize_point_array_property,
    serialize_point_property,
};
use crate::util::{
    DeserializationConfig, GmlElement, GmlNamespace, XmlDocumentIndex, XmlFragmentWriter,
    collect_child, collect_children,
};
use egml_core::model::geometry::aggregates::{AsAbstractGeometricAggregate, MultiPoint};
use std::io::Write;

pub fn deserialize_multi_point(
    xml_document: &[u8],
    index: &XmlDocumentIndex<GmlElement>,
    config: &DeserializationConfig,
) -> Result<MultiPoint, Error> {
    let index = if index.is_truncated() {
        &XmlDocumentIndex::from_scan(xml_document, config.rescan_depth())?
    } else {
        index
    };

    let abstract_geometric_aggregate =
        deserialize_abstract_geometric_aggregate(xml_document, index, config)?;

    let point_member = collect_children(
        xml_document,
        index,
        GmlElement::PointMemberProperty,
        config,
        deserialize_point_property,
    )?;

    let point_members = collect_child(
        xml_document,
        index,
        GmlElement::PointMembersProperty,
        config,
        deserialize_point_array_property,
    )?
    .flatten();

    let mut multi_point =
        MultiPoint::from_abstract_geometric_aggregate(abstract_geometric_aggregate, point_members);
    multi_point.set_point_member(point_member);
    Ok(multi_point)
}

pub fn serialize_multi_point<W: Write>(
    multi_point: &MultiPoint,
    xml_fragment_writer: &mut XmlFragmentWriter<W>,
) -> Result<(), Error> {
    let attributes = serialize_multi_point_attributes(multi_point);

    xml_fragment_writer.write_start_event_with_attributes(
        GmlNamespace::Gml,
        GmlElement::MultiPoint,
        attributes,
    )?;

    serialize_abstract_geometric_aggregate(
        multi_point.abstract_geometric_aggregate(),
        xml_fragment_writer,
    )?;

    for member in multi_point.point_member() {
        serialize_point_property(
            member,
            xml_fragment_writer,
            GmlNamespace::Gml,
            GmlElement::PointMemberProperty,
        )?;
    }

    if let Some(members) = multi_point.point_members() {
        serialize_point_array_property(
            members,
            xml_fragment_writer,
            GmlNamespace::Gml,
            GmlElement::PointMembersProperty,
        )?;
    }

    xml_fragment_writer.write_end_event(GmlNamespace::Gml, GmlElement::MultiPoint)?;

    Ok(())
}

pub fn serialize_multi_point_attributes(multi_point: &MultiPoint) -> Vec<(String, String)> {
    serialize_abstract_geometric_aggregate_attributes(multi_point.abstract_geometric_aggregate())
}

#[cfg(test)]
mod tests {
    // Test-only convenience: builds the index the real function now
    // requires, so existing single-argument call sites below don't all
    // need to construct one by hand.
    fn deserialize(xml_document: &[u8]) -> Result<super::MultiPoint, crate::Error> {
        let index = crate::util::XmlDocumentIndex::from_scan(xml_document, None)?;
        super::deserialize_multi_point(
            xml_document,
            &index,
            &crate::util::DeserializationConfig::default(),
        )
    }

    use crate::codec::geometry::aggregates::multi_point::serialize_multi_point;
    use crate::util::{Formatting, XmlFragmentWriter};
    use egml_core::model::geometry::DirectPosition;
    use egml_core::model::geometry::aggregates::MultiPoint;
    use egml_core::model::geometry::primitives::{Point, PointArrayProperty, PointProperty};

    fn make_point(x: f64, y: f64, z: f64) -> Point {
        Point::new(DirectPosition::new(x, y, z).unwrap())
    }

    fn make_point_property(x: f64, y: f64, z: f64) -> PointProperty {
        PointProperty::from_object(make_point(x, y, z))
    }

    fn make_multi_point() -> MultiPoint {
        let p1 = make_point(678267.6213956032, 5403783.626290152, 366.96639999999996);
        let p2 = make_point(678289.06567932, 5403807.373180328, 366.99789425533834);
        let point_members = PointArrayProperty::from_objects(vec![p1, p2]);

        let mut multi_point = MultiPoint::new(Some(point_members)).unwrap();
        multi_point.set_point_member(vec![
            make_point_property(678267.6213956032, 5403783.626290152, 366.96639999999996),
            make_point_property(678289.06567932, 5403807.373180328, 366.99789425533834),
        ]);
        multi_point
    }

    #[test]
    fn deserialize_point_member_only() {
        let xml = b"<gml:MultiPoint>
            <gml:pointMember>
                <gml:Point><gml:pos>678267.6213956032 5403783.626290152 366.96639999999996</gml:pos></gml:Point>
            </gml:pointMember>
            <gml:pointMember>
                <gml:Point><gml:pos>678289.06567932 5403807.373180328 366.99789425533834</gml:pos></gml:Point>
            </gml:pointMember>
        </gml:MultiPoint>";

        let multi_point = deserialize(xml).unwrap();

        assert_eq!(multi_point.point_member().len(), 2);
        assert!(multi_point.point_members().is_none());
    }

    #[test]
    fn deserialize_point_members_only() {
        let xml = b"<gml:MultiPoint>
            <gml:pointMembers>
                <gml:Point><gml:pos>678267.6213956032 5403783.626290152 366.96639999999996</gml:pos></gml:Point>
                <gml:Point><gml:pos>678289.06567932 5403807.373180328 366.99789425533834</gml:pos></gml:Point>
            </gml:pointMembers>
        </gml:MultiPoint>";

        let multi_point = deserialize(xml).unwrap();

        assert!(multi_point.point_member().is_empty());
        assert_eq!(multi_point.point_members().unwrap().objects().len(), 2);
    }

    #[test]
    fn deserialize_both_point_member_and_point_members() {
        let xml = b"<gml:MultiPoint>
            <gml:pointMember>
                <gml:Point><gml:pos>678267.6213956032 5403783.626290152 366.96639999999996</gml:pos></gml:Point>
            </gml:pointMember>
            <gml:pointMember>
                <gml:Point><gml:pos>678289.06567932 5403807.373180328 366.99789425533834</gml:pos></gml:Point>
            </gml:pointMember>
            <gml:pointMembers>
                <gml:Point><gml:pos>678267.6213956032 5403783.626290152 366.96639999999996</gml:pos></gml:Point>
                <gml:Point><gml:pos>678289.06567932 5403807.373180328 366.99789425533834</gml:pos></gml:Point>
            </gml:pointMembers>
        </gml:MultiPoint>";

        let multi_point = deserialize(xml).unwrap();

        assert_eq!(multi_point.point_member().len(), 2);
        assert_eq!(multi_point.point_members().unwrap().objects().len(), 2);
    }

    #[test]
    fn serialize_multi_point_writes_gml_tags() {
        let multi_point = make_multi_point();

        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_multi_point(&multi_point, &mut xml_fragment_writer).expect("should serialize");
        let xml = String::from_utf8(xml_fragment_writer.into_bytes()).unwrap();

        assert!(xml.contains("<gml:MultiPoint"));
        assert!(xml.contains("<gml:pointMember"));
        assert!(xml.contains("<gml:pointMembers"));
        assert!(xml.contains("<gml:Point"));
        assert!(xml.contains("<gml:pos"));
        assert!(!xml.contains("id="));
    }

    #[test]
    fn round_trip_preserves_member_counts() {
        let original = make_multi_point();

        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_multi_point(&original, &mut xml_fragment_writer).expect("should serialize");
        let xml = String::from_utf8(xml_fragment_writer.into_bytes()).unwrap();
        let recovered = deserialize(xml.as_bytes()).unwrap();

        assert_eq!(
            recovered.point_member().len(),
            original.point_member().len()
        );
        assert_eq!(
            recovered.point_members().map(|m| m.objects().len()),
            original.point_members().map(|m| m.objects().len()),
        );
    }

    #[test]
    fn round_trip_from_xml() {
        let input_xml = b"<gml:MultiPoint>\
            <gml:pointMember><gml:Point><gml:pos srsDimension=\"3\">1 2 3</gml:pos></gml:Point></gml:pointMember>\
            <gml:pointMembers><gml:Point><gml:pos srsDimension=\"3\">4 5 6</gml:pos></gml:Point></gml:pointMembers>\
            </gml:MultiPoint>";

        let multi_point = deserialize(input_xml).unwrap();
        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_multi_point(&multi_point, &mut xml_fragment_writer).expect("should serialize");
        let output = String::from_utf8(xml_fragment_writer.into_bytes()).unwrap();

        let recovered = deserialize(output.as_bytes()).unwrap();

        assert_eq!(
            recovered.point_member().len(),
            multi_point.point_member().len()
        );
        assert_eq!(
            recovered.point_members().map(|m| m.objects().len()),
            multi_point.point_members().map(|m| m.objects().len())
        );
    }
}
