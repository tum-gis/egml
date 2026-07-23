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
use egml_core::model::geometry::AbstractGeometryProperty;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_abstract_geometry_property(
    xml_document: &[u8],
    spans: &XmlElementSpans<GmlElement>,
) -> Result<AbstractGeometryProperty, Error> {
    let parsed: GmlAbstractGeometryProperty = de::from_reader(xml_document)?;

    let object = deserialize_abstract_geometry_kind(xml_document, spans)?;

    Ok(AbstractGeometryProperty::new(
        object,
        parsed.association.try_into()?,
        parsed.ownership.into(),
    ))
}

pub fn serialize_abstract_geometry_property(
    abstract_geometry_property: &AbstractGeometryProperty,
    formatting: Formatting,
    target_xml_element: &'static str,
) -> Result<XmlNode, Error> {
    let mut xml_node_parts = XmlNodeParts::empty();

    xml_node_parts
        .attributes
        .extend(serialize_association_attributes(
            abstract_geometry_property.association(),
        ));
    xml_node_parts
        .attributes
        .extend(serialize_ownership_attributes(
            abstract_geometry_property.ownership(),
        ));

    if let Some(object) = abstract_geometry_property.object() {
        xml_node_parts
            .content
            .push(XmlNodeContent::Child(serialize_abstract_geometry_kind(
                object, formatting,
            )?));
    }

    Ok(XmlNode::new(target_xml_element, xml_node_parts))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlAbstractGeometryProperty {
    #[serde(flatten)]
    pub association: GmlAssociationAttributes,
    #[serde(flatten)]
    pub ownership: GmlOwnershipAttributes,
}

#[cfg(test)]
mod tests {
    use super::{deserialize_abstract_geometry_property, serialize_abstract_geometry_property};
    use crate::util::{Formatting, GmlElement, extract_xml_element_spans};
    use egml_core::model::base::{HasAssociationAttributes, HasOwnershipAttributes};
    use egml_core::model::geometry::AbstractGeometryProperty;
    use egml_core::model::geometry::DirectPosition;
    use egml_core::model::geometry::aggregates::{AbstractGeometricAggregateKind, MultiPoint};
    use egml_core::model::geometry::primitives::{
        AbstractGeometricPrimitiveKind, AbstractRingKind, AbstractRingProperty,
        AbstractSurfaceKind, LinearRing, Point, PointProperty, Polygon,
    };
    use egml_core::model::xlink::{ActuateType, HRef, ShowType};

    fn make_polygon_property() -> AbstractGeometryProperty {
        let points = vec![
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(0.0, 1.0, 0.0).unwrap(),
        ];
        let ring = AbstractRingKind::LinearRing(LinearRing::new(points).unwrap());
        let polygon = Polygon::new(Some(AbstractRingProperty::from_object(ring)), []).unwrap();
        use egml_core::model::geometry::AbstractGeometryKind;
        use egml_core::model::geometry::primitives::AbstractGeometricPrimitiveKind;
        AbstractGeometryProperty::from_object(AbstractGeometryKind::AbstractGeometricPrimitiveKind(
            AbstractGeometricPrimitiveKind::AbstractSurfaceKind(AbstractSurfaceKind::Polygon(
                polygon,
            )),
        ))
    }

    fn make_multi_point_property() -> AbstractGeometryProperty {
        use egml_core::model::geometry::AbstractGeometryKind;
        let mut mp = MultiPoint::new(None).unwrap();
        mp.set_point_member(vec![PointProperty::from_object(Point::new(
            DirectPosition::new(1.0, 2.0, 3.0).unwrap(),
        ))]);
        AbstractGeometryProperty::from_object(AbstractGeometryKind::AbstractGeometricAggregateKind(
            AbstractGeometricAggregateKind::MultiPoint(mp),
        ))
    }

    #[test]
    fn deserialize_with_polygon() {
        let xml = b"<gml:geometryMember>\
            <gml:Polygon><gml:exterior><gml:LinearRing>\
            <gml:posList srsDimension=\"3\">0 0 0 1 0 0 0 1 0 0 0 0</gml:posList>\
            </gml:LinearRing></gml:exterior></gml:Polygon>\
            </gml:geometryMember>";

        let spans = extract_xml_element_spans(xml).unwrap();
        let property = deserialize_abstract_geometry_property(xml, &spans).unwrap();

        assert!(matches!(
            property.object(),
            Some(
                egml_core::model::geometry::AbstractGeometryKind::AbstractGeometricPrimitiveKind(
                    AbstractGeometricPrimitiveKind::AbstractSurfaceKind(
                        AbstractSurfaceKind::Polygon(_)
                    )
                )
            )
        ));
    }

    #[test]
    fn deserialize_with_multi_point() {
        let xml = b"<gml:geometryMember>\
            <gml:MultiPoint>\
            <gml:pointMember><gml:Point><gml:pos srsDimension=\"3\">1 2 3</gml:pos></gml:Point></gml:pointMember>\
            </gml:MultiPoint>\
            </gml:geometryMember>";

        let spans = extract_xml_element_spans(xml).unwrap();
        let property = deserialize_abstract_geometry_property(xml, &spans).unwrap();

        assert!(matches!(
            property.object(),
            Some(
                egml_core::model::geometry::AbstractGeometryKind::AbstractGeometricAggregateKind(
                    AbstractGeometricAggregateKind::MultiPoint(_)
                )
            )
        ));
    }

    #[test]
    fn deserialize_with_xlink() {
        let xml = b"<gml:geometryMember xlink:href=\"#some-id\"/>";

        let spans = extract_xml_element_spans(xml).unwrap();
        let property = deserialize_abstract_geometry_property(xml, &spans).unwrap();

        assert_eq!(property.href(), Some(&HRef::from_local("some-id")));
        assert!(property.object().is_none());
    }

    #[test]
    fn serialize_polygon_property() {
        let property = make_polygon_property();
        let xml_node = serialize_abstract_geometry_property(
            &property,
            Formatting::Compact,
            GmlElement::Polygon.into(),
        )
        .unwrap();
        let xml = xml_node.to_string(Formatting::Compact).unwrap();

        assert!(xml.contains("<gml:Polygon"));
        assert!(xml.contains("<gml:exterior"));
        assert!(xml.contains("<gml:LinearRing"));
    }

    #[test]
    fn serialize_multi_point_property() {
        let property = make_multi_point_property();
        let xml_node = serialize_abstract_geometry_property(
            &property,
            Formatting::Compact,
            GmlElement::MultiPoint.into(),
        )
        .unwrap();
        let xml = xml_node.to_string(Formatting::Compact).unwrap();

        assert!(xml.contains("<gml:MultiPoint"));
        assert!(xml.contains("<gml:pointMember"));
        assert!(xml.contains("<gml:Point"));
    }

    #[test]
    fn deserialize_with_full_association_and_ownership_attributes() {
        let xml = b"<gml:geometryMember xlink:href=\"#some-id\" xlink:title=\"Some Title\" \
            xlink:role=\"http://example.com/role\" xlink:arcrole=\"http://example.com/arcrole\" \
            xlink:show=\"new\" xlink:actuate=\"onLoad\" gml:owns=\"true\"/>";

        let spans = extract_xml_element_spans(xml).unwrap();
        let property = deserialize_abstract_geometry_property(xml, &spans).unwrap();

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
    fn round_trip_href_only_property() {
        let xml = b"<gml:geometryMember xlink:href=\"#some-id\"/>";

        let spans = extract_xml_element_spans(xml).unwrap();
        let property = deserialize_abstract_geometry_property(xml, &spans).unwrap();
        let xml_node = serialize_abstract_geometry_property(
            &property,
            Formatting::Compact,
            GmlElement::Polygon.into(),
        )
        .unwrap();
        let output = xml_node.to_string(Formatting::Compact).unwrap();

        let spans2 = extract_xml_element_spans(output.as_bytes()).unwrap();
        let recovered = deserialize_abstract_geometry_property(output.as_bytes(), &spans2).unwrap();

        assert_eq!(recovered.association(), property.association());
        assert_eq!(recovered.ownership(), property.ownership());
    }

    #[test]
    fn round_trip_preserves_full_association_and_ownership_attributes() {
        let xml = b"<gml:geometryMember xlink:href=\"#some-id\" xlink:title=\"Some Title\" \
            xlink:role=\"http://example.com/role\" xlink:arcrole=\"http://example.com/arcrole\" \
            xlink:show=\"new\" xlink:actuate=\"onLoad\" gml:owns=\"true\"/>";

        let spans = extract_xml_element_spans(xml).unwrap();
        let property = deserialize_abstract_geometry_property(xml, &spans).unwrap();
        let xml_node = serialize_abstract_geometry_property(
            &property,
            Formatting::Compact,
            GmlElement::Polygon.into(),
        )
        .unwrap();
        let output = xml_node.to_string(Formatting::Compact).unwrap();

        let spans2 = extract_xml_element_spans(output.as_bytes()).unwrap();
        let recovered = deserialize_abstract_geometry_property(output.as_bytes(), &spans2).unwrap();

        assert_eq!(
            recovered.association(),
            property.association(),
            "association attributes did not survive the round trip; output was: {output}"
        );
        assert_eq!(
            recovered.ownership(),
            property.ownership(),
            "ownership attributes did not survive the round trip; output was: {output}"
        );
    }

    #[test]
    fn round_trip_polygon_property() {
        let xml = b"<gml:geometryMember>\
            <gml:Polygon><gml:exterior><gml:LinearRing>\
            <gml:posList srsDimension=\"3\">0 0 0 1 0 0 0 1 0 0 0 0</gml:posList>\
            </gml:LinearRing></gml:exterior></gml:Polygon>\
            </gml:geometryMember>";

        let spans = extract_xml_element_spans(xml).unwrap();
        let property = deserialize_abstract_geometry_property(xml, &spans).unwrap();
        let xml_node = serialize_abstract_geometry_property(
            &property,
            Formatting::Compact,
            GmlElement::Polygon.into(),
        )
        .unwrap();
        let output = xml_node.to_string(Formatting::Compact).unwrap();
        let wrapper = format!("<gml:geometryMember>{output}</gml:geometryMember>");

        let spans2 = extract_xml_element_spans(wrapper.as_bytes()).unwrap();
        let recovered =
            deserialize_abstract_geometry_property(wrapper.as_bytes(), &spans2).unwrap();

        assert!(matches!(
            recovered.object(),
            Some(
                egml_core::model::geometry::AbstractGeometryKind::AbstractGeometricPrimitiveKind(
                    AbstractGeometricPrimitiveKind::AbstractSurfaceKind(
                        AbstractSurfaceKind::Polygon(_)
                    )
                )
            )
        ));
    }
}
