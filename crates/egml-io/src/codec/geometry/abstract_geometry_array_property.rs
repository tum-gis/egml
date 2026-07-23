use crate::Error;
use crate::codec::base::{
    GmlAssociationAttributes, GmlOwnershipAttributes, serialize_association_attributes,
    serialize_ownership_attributes,
};
use crate::codec::geometry::{
    deserialize_abstract_geometry_kind, serialize_abstract_geometry_kind,
};
use crate::util::{Formatting, GmlElement, XmlElementSpans, XmlNode, XmlNodeContent, XmlNodeParts};
use egml_core::model::base::{HasAssociationAttributes, HasOwnershipAttributes};
use egml_core::model::geometry::{AbstractGeometryArrayProperty, AbstractGeometryKind};
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_abstract_geometry_array_property(
    xml_document: &[u8],
    spans: &XmlElementSpans<GmlElement>,
) -> Result<Option<AbstractGeometryArrayProperty>, Error> {
    let parsed: GmlAbstractGeometryArrayProperty = de::from_reader(xml_document)?;

    let mut all_spans: Vec<(GmlElement, std::ops::Range<usize>)> = spans
        .spans()
        .iter()
        .flat_map(|(elem, ranges)| ranges.iter().map(|r| (*elem, r.clone())))
        .collect();
    all_spans.sort_by_key(|(_, r)| r.start);

    let objects: Vec<AbstractGeometryKind> = all_spans
        .iter()
        .filter_map(|(elem, span)| {
            let slice = &xml_document[span.start..span.end];
            let parent_spans = XmlElementSpans::single(*elem, slice.len());
            deserialize_abstract_geometry_kind(slice, &parent_spans).transpose()
        })
        .collect::<Result<_, _>>()?;

    if objects.is_empty() {
        return Ok(None);
    }

    Ok(Some(AbstractGeometryArrayProperty::new(
        objects,
        parsed.association.try_into()?,
        parsed.ownership.into(),
    )))
}

pub fn serialize_abstract_geometry_array_property(
    abstract_geometry_array_property: &AbstractGeometryArrayProperty,
    formatting: Formatting,
    target_xml_element: GmlElement,
) -> Result<XmlNode, Error> {
    let mut parts = XmlNodeParts::empty();

    parts.attributes.extend(serialize_association_attributes(
        abstract_geometry_array_property.association(),
    ));
    parts.attributes.extend(serialize_ownership_attributes(
        abstract_geometry_array_property.ownership(),
    ));

    for object in abstract_geometry_array_property.objects() {
        parts
            .content
            .push(XmlNodeContent::Child(serialize_abstract_geometry_kind(
                object, formatting,
            )?));
    }

    Ok(XmlNode::new(target_xml_element.into(), parts))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlAbstractGeometryArrayProperty {
    #[serde(flatten)]
    pub association: GmlAssociationAttributes,
    #[serde(flatten)]
    pub ownership: GmlOwnershipAttributes,
}

#[cfg(test)]
mod tests {
    use super::{
        deserialize_abstract_geometry_array_property, serialize_abstract_geometry_array_property,
    };
    use crate::util::{Formatting, GmlElement, extract_xml_element_spans};
    use egml_core::model::base::{HasAssociationAttributes, HasOwnershipAttributes};
    use egml_core::model::geometry::AbstractGeometryArrayProperty;
    use egml_core::model::geometry::AbstractGeometryKind;
    use egml_core::model::geometry::DirectPosition;
    use egml_core::model::geometry::aggregates::{AbstractGeometricAggregateKind, MultiPoint};
    use egml_core::model::geometry::primitives::{
        AbstractGeometricPrimitiveKind, AbstractRingKind, AbstractRingProperty,
        AbstractSurfaceKind, LinearRing, Point, PointProperty, Polygon,
    };
    use egml_core::model::xlink::{ActuateType, HRef, ShowType};

    fn make_polygon_kind() -> AbstractGeometryKind {
        let points = vec![
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(0.0, 1.0, 0.0).unwrap(),
        ];
        let ring = AbstractRingKind::LinearRing(LinearRing::new(points).unwrap());
        let polygon = Polygon::new(Some(AbstractRingProperty::from_object(ring)), []).unwrap();
        AbstractGeometryKind::AbstractGeometricPrimitiveKind(
            AbstractGeometricPrimitiveKind::AbstractSurfaceKind(AbstractSurfaceKind::Polygon(
                polygon,
            )),
        )
    }

    fn make_multi_point_kind() -> AbstractGeometryKind {
        let mut mp = MultiPoint::new(None).unwrap();
        mp.set_point_member(vec![PointProperty::from_object(Point::new(
            DirectPosition::new(1.0, 2.0, 3.0).unwrap(),
        ))]);
        AbstractGeometryKind::AbstractGeometricAggregateKind(
            AbstractGeometricAggregateKind::MultiPoint(mp),
        )
    }

    #[test]
    fn deserialize_with_polygons() {
        let xml = b"<gml:geometryMembers>\
            <gml:Polygon><gml:exterior><gml:LinearRing>\
            <gml:posList srsDimension=\"3\">0 0 0 1 0 0 0 1 0 0 0 0</gml:posList>\
            </gml:LinearRing></gml:exterior></gml:Polygon>\
            <gml:Polygon><gml:exterior><gml:LinearRing>\
            <gml:posList srsDimension=\"3\">1 0 0 2 0 0 1 1 0 1 0 0</gml:posList>\
            </gml:LinearRing></gml:exterior></gml:Polygon>\
            </gml:geometryMembers>";

        let spans = extract_xml_element_spans(xml).unwrap();
        let property = deserialize_abstract_geometry_array_property(xml, &spans)
            .unwrap()
            .unwrap();

        assert_eq!(property.objects().len(), 2);
        assert!(matches!(
            property.objects()[0],
            AbstractGeometryKind::AbstractGeometricPrimitiveKind(
                AbstractGeometricPrimitiveKind::AbstractSurfaceKind(AbstractSurfaceKind::Polygon(
                    _
                ))
            )
        ));
    }

    #[test]
    fn deserialize_with_multi_point() {
        let xml = b"<gml:geometryMembers>\
            <gml:MultiPoint>\
            <gml:pointMember><gml:Point><gml:pos srsDimension=\"3\">1 2 3</gml:pos></gml:Point></gml:pointMember>\
            </gml:MultiPoint>\
            </gml:geometryMembers>";

        let spans = extract_xml_element_spans(xml).unwrap();
        let property = deserialize_abstract_geometry_array_property(xml, &spans)
            .unwrap()
            .unwrap();

        assert_eq!(property.objects().len(), 1);
        assert!(matches!(
            property.objects()[0],
            AbstractGeometryKind::AbstractGeometricAggregateKind(
                AbstractGeometricAggregateKind::MultiPoint(_)
            )
        ));
    }

    #[test]
    fn deserialize_empty_returns_none() {
        let xml = b"<gml:geometryMembers/>";

        let spans = extract_xml_element_spans(xml).unwrap();
        let property = deserialize_abstract_geometry_array_property(xml, &spans).unwrap();

        assert!(property.is_none());
    }

    #[test]
    fn serialize_multiple_objects() {
        let property = AbstractGeometryArrayProperty::from_objects(vec![
            make_polygon_kind(),
            make_multi_point_kind(),
        ]);
        let xml_node = serialize_abstract_geometry_array_property(
            &property,
            Formatting::Compact,
            GmlElement::PointMembersProperty,
        )
        .unwrap();
        let xml = xml_node.to_string(Formatting::Compact).unwrap();

        assert!(xml.contains("<gml:Polygon"));
        assert!(xml.contains("<gml:MultiPoint"));
    }

    #[test]
    fn round_trip_polygons_preserves_count() {
        let xml = b"<gml:geometryMembers>\
            <gml:Polygon><gml:exterior><gml:LinearRing>\
            <gml:posList srsDimension=\"3\">0 0 0 1 0 0 0 1 0 0 0 0</gml:posList>\
            </gml:LinearRing></gml:exterior></gml:Polygon>\
            <gml:Polygon><gml:exterior><gml:LinearRing>\
            <gml:posList srsDimension=\"3\">1 0 0 2 0 0 1 1 0 1 0 0</gml:posList>\
            </gml:LinearRing></gml:exterior></gml:Polygon>\
            </gml:geometryMembers>";

        let spans = extract_xml_element_spans(xml).unwrap();
        let property = deserialize_abstract_geometry_array_property(xml, &spans)
            .unwrap()
            .unwrap();

        let xml_node = serialize_abstract_geometry_array_property(
            &property,
            Formatting::Compact,
            GmlElement::PointMembersProperty,
        )
        .unwrap();
        let output = xml_node.to_string(Formatting::Compact).unwrap();

        let spans2 = extract_xml_element_spans(output.as_bytes()).unwrap();
        let recovered = deserialize_abstract_geometry_array_property(output.as_bytes(), &spans2)
            .unwrap()
            .unwrap();

        assert_eq!(recovered.objects().len(), property.objects().len());
    }

    #[test]
    fn deserialize_with_full_association_and_ownership_attributes() {
        let xml = b"<gml:geometryMembers xlink:href=\"#some-id\" xlink:title=\"Some Title\" \
            xlink:role=\"http://example.com/role\" xlink:arcrole=\"http://example.com/arcrole\" \
            xlink:show=\"new\" xlink:actuate=\"onLoad\" gml:owns=\"true\">\
            <gml:Polygon><gml:exterior><gml:LinearRing>\
            <gml:posList srsDimension=\"3\">0 0 0 1 0 0 0 1 0 0 0 0</gml:posList>\
            </gml:LinearRing></gml:exterior></gml:Polygon>\
            </gml:geometryMembers>";

        let spans = extract_xml_element_spans(xml).unwrap();
        let property = deserialize_abstract_geometry_array_property(xml, &spans)
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
        let xml = b"<gml:geometryMembers xlink:href=\"#some-id\" xlink:title=\"Some Title\" \
            xlink:role=\"http://example.com/role\" xlink:arcrole=\"http://example.com/arcrole\" \
            xlink:show=\"new\" xlink:actuate=\"onLoad\" gml:owns=\"true\">\
            <gml:Polygon><gml:exterior><gml:LinearRing>\
            <gml:posList srsDimension=\"3\">0 0 0 1 0 0 0 1 0 0 0 0</gml:posList>\
            </gml:LinearRing></gml:exterior></gml:Polygon>\
            </gml:geometryMembers>";

        let spans = extract_xml_element_spans(xml).unwrap();
        let property = deserialize_abstract_geometry_array_property(xml, &spans)
            .unwrap()
            .unwrap();

        let xml_node = serialize_abstract_geometry_array_property(
            &property,
            Formatting::Compact,
            GmlElement::PointMembersProperty,
        )
        .unwrap();
        let output = xml_node.to_string(Formatting::Compact).unwrap();

        let spans2 = extract_xml_element_spans(output.as_bytes()).unwrap();
        let recovered = deserialize_abstract_geometry_array_property(output.as_bytes(), &spans2)
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
