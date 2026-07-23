use crate::Error;
use crate::codec::geometry::aggregates::{
    deserialize_abstract_geometric_aggregate, serialize_abstract_geometric_aggregate,
};
use crate::codec::geometry::primitives::{
    deserialize_point_array_property, deserialize_point_property, serialize_point_array_property,
    serialize_point_property,
};
use crate::util::{
    Formatting, GmlElement, XmlNode, XmlNodeContent, collect_child, collect_children,
    extract_xml_element_spans,
};
use egml_core::model::geometry::aggregates::{AsAbstractGeometricAggregate, MultiPoint};

pub fn deserialize_multi_point(xml_document: &[u8]) -> Result<MultiPoint, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let abstract_geometric_aggregate =
        deserialize_abstract_geometric_aggregate(xml_document, &spans)?;

    let point_member = collect_children(
        xml_document,
        &spans,
        GmlElement::PointMemberProperty,
        deserialize_point_property,
    )?;

    let point_members = collect_child(
        xml_document,
        &spans,
        GmlElement::PointMembersProperty,
        deserialize_point_array_property,
    )?
    .flatten();

    let mut multi_point =
        MultiPoint::from_abstract_geometric_aggregate(abstract_geometric_aggregate, point_members);
    multi_point.set_point_member(point_member);
    Ok(multi_point)
}

pub fn serialize_multi_point(
    multi_point: &MultiPoint,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut parts = serialize_abstract_geometric_aggregate(
        multi_point.abstract_geometric_aggregate(),
        formatting,
    )?;

    for member in multi_point.point_member() {
        parts
            .content
            .push(XmlNodeContent::Child(serialize_point_property(
                member,
                formatting,
                GmlElement::PointMemberProperty.into(),
            )?));
    }

    if let Some(members) = multi_point.point_members() {
        parts
            .content
            .push(XmlNodeContent::Child(serialize_point_array_property(
                members,
                formatting,
                GmlElement::PointMembersProperty.into(),
            )?));
    }

    Ok(XmlNode::new(GmlElement::MultiPoint.into(), parts))
}

#[cfg(test)]
mod tests {
    use crate::codec::geometry::aggregates::multi_point::{
        deserialize_multi_point, serialize_multi_point,
    };
    use crate::util::Formatting;
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

        let multi_point = deserialize_multi_point(xml).unwrap();

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

        let multi_point = deserialize_multi_point(xml).unwrap();

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

        let multi_point = deserialize_multi_point(xml).unwrap();

        assert_eq!(multi_point.point_member().len(), 2);
        assert_eq!(multi_point.point_members().unwrap().objects().len(), 2);
    }

    #[test]
    fn serialize_multi_point_writes_gml_tags() {
        let multi_point = make_multi_point();

        let xml_node =
            serialize_multi_point(&multi_point, Formatting::Compact).expect("should serialize");
        let xml = xml_node
            .to_string(Formatting::Compact)
            .expect("to string should work");

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

        let xml_node =
            serialize_multi_point(&original, Formatting::Compact).expect("should serialize");
        let xml = xml_node.to_string(Formatting::Compact).unwrap();
        let recovered = deserialize_multi_point(xml.as_bytes()).unwrap();

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

        let multi_point = deserialize_multi_point(input_xml).unwrap();
        let xml_node =
            serialize_multi_point(&multi_point, Formatting::Compact).expect("should serialize");
        let output = xml_node.to_string(Formatting::Compact).unwrap();

        let recovered = deserialize_multi_point(output.as_bytes()).unwrap();

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
