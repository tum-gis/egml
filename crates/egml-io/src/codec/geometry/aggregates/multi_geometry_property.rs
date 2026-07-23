use crate::Error;
use crate::codec::base::{
    GmlAssociationAttributes, GmlOwnershipAttributes, serialize_association_attributes,
    serialize_ownership_attributes,
};
use crate::codec::geometry::aggregates::{deserialize_multi_geometry, serialize_multi_geometry};
use crate::util::{Formatting, GmlElement, XmlElementSpans, XmlNode, XmlNodeContent, XmlNodeParts};
use egml_core::model::base::{HasAssociationAttributes, HasOwnershipAttributes};
use egml_core::model::geometry::aggregates::MultiGeometryProperty;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_multi_geometry_property(
    xml_document: &[u8],
    spans: &XmlElementSpans<GmlElement>,
) -> Result<MultiGeometryProperty, Error> {
    let parsed: GmlMultiGeometryProperty = de::from_reader(xml_document)?;

    let object = spans
        .first(GmlElement::MultiGeometry)
        .map(|span| deserialize_multi_geometry(&xml_document[span.start..span.end]))
        .transpose()?;

    Ok(MultiGeometryProperty::new(
        object,
        parsed.association.try_into()?,
        parsed.ownership.into(),
    ))
}

pub fn serialize_multi_geometry_property(
    multi_geometry_property: &MultiGeometryProperty,
    formatting: Formatting,
    target_xml_element: &'static str,
) -> Result<XmlNode, Error> {
    let mut xml_node_parts = XmlNodeParts::empty();

    xml_node_parts
        .attributes
        .extend(serialize_association_attributes(
            multi_geometry_property.association(),
        ));
    xml_node_parts
        .attributes
        .extend(serialize_ownership_attributes(
            multi_geometry_property.ownership(),
        ));

    if let Some(multi_geometry) = multi_geometry_property.object() {
        xml_node_parts
            .content
            .push(XmlNodeContent::Child(serialize_multi_geometry(
                multi_geometry,
                formatting,
            )?));
    }

    Ok(XmlNode::new(target_xml_element, xml_node_parts))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlMultiGeometryProperty {
    #[serde(flatten)]
    pub association: GmlAssociationAttributes,
    #[serde(flatten)]
    pub ownership: GmlOwnershipAttributes,
}

#[cfg(test)]
mod tests {
    use crate::codec::geometry::aggregates::multi_geometry_property::{
        deserialize_multi_geometry_property, serialize_multi_geometry_property,
    };
    use crate::util::{Formatting, GmlElement, extract_xml_element_spans};
    use egml_core::model::base::{HasAssociationAttributes, HasOwnershipAttributes};
    use egml_core::model::geometry::AbstractGeometryKind;
    use egml_core::model::geometry::AbstractGeometryProperty;
    use egml_core::model::geometry::DirectPosition;
    use egml_core::model::geometry::aggregates::{MultiGeometry, MultiGeometryProperty};
    use egml_core::model::geometry::primitives::{
        AbstractGeometricPrimitiveKind, AbstractRingKind, AbstractRingProperty,
        AbstractSurfaceKind, LinearRing, Polygon,
    };
    use egml_core::model::xlink::{ActuateType, HRef, ShowType};

    fn make_polygon_member() -> AbstractGeometryProperty {
        let points = vec![
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(0.0, 1.0, 0.0).unwrap(),
        ];
        let ring = AbstractRingKind::LinearRing(LinearRing::new(points).unwrap());
        let polygon = Polygon::new(Some(AbstractRingProperty::from_object(ring)), []).unwrap();
        AbstractGeometryProperty::from_object(AbstractGeometryKind::AbstractGeometricPrimitiveKind(
            AbstractGeometricPrimitiveKind::AbstractSurfaceKind(AbstractSurfaceKind::Polygon(
                polygon,
            )),
        ))
    }

    fn make_multi_geometry_property() -> MultiGeometryProperty {
        let mut mg = MultiGeometry::new(None).unwrap();
        mg.set_geometry_member(vec![make_polygon_member()]);
        MultiGeometryProperty::from_object(mg)
    }

    #[test]
    fn deserialize_multi_geometry_property_test() {
        let xml_document = b"<gml:geometryMember>
            <gml:MultiGeometry>
                <gml:geometryMember>
                    <gml:Polygon>
                        <gml:exterior>
                            <gml:LinearRing>
                                <gml:posList srsDimension=\"3\">0 0 0 1 0 0 0 1 0 0 0 0</gml:posList>
                            </gml:LinearRing>
                        </gml:exterior>
                    </gml:Polygon>
                </gml:geometryMember>
            </gml:MultiGeometry>
        </gml:geometryMember>";

        let spans = extract_xml_element_spans(xml_document).expect("extracting spans should work");
        let property =
            deserialize_multi_geometry_property(xml_document, &spans).expect("should deserialize");

        assert!(property.object().is_some());
        assert_eq!(property.object().unwrap().geometry_member().len(), 1);
    }

    #[test]
    fn deserialize_multi_geometry_property_with_xlink() {
        let xml_document = b"<gml:geometryMember xlink:href=\"#some-geometry-id\"/>";

        let spans = extract_xml_element_spans(xml_document).expect("extracting spans should work");
        let property =
            deserialize_multi_geometry_property(xml_document, &spans).expect("should deserialize");

        assert_eq!(property.href(), Some(&HRef::from_local("some-geometry-id")));
        assert!(property.object().is_none());
    }

    #[test]
    fn serialize_multi_geometry_property_writes_gml_tags() {
        let property = make_multi_geometry_property();

        let xml_node = serialize_multi_geometry_property(
            &property,
            Formatting::Compact,
            GmlElement::GeometryMemberProperty.into(),
        )
        .expect("should serialize");
        let xml = xml_node
            .to_string(Formatting::Compact)
            .expect("to string should work");

        assert!(xml.contains("<gml:geometryMember"));
        assert!(xml.contains("<gml:MultiGeometry"));
        assert!(xml.contains("<gml:Polygon"));
    }

    #[test]
    fn round_trip_multi_geometry_property_preserves_member_count() {
        let xml_document = b"<gml:geometryMember>\
            <gml:MultiGeometry>\
            <gml:geometryMember><gml:Polygon><gml:exterior><gml:LinearRing>\
            <gml:posList srsDimension=\"3\">0 0 0 1 0 0 0 1 0 0 0 0</gml:posList>\
            </gml:LinearRing></gml:exterior></gml:Polygon></gml:geometryMember>\
            </gml:MultiGeometry>\
            </gml:geometryMember>";

        let spans = extract_xml_element_spans(xml_document).unwrap();
        let property = deserialize_multi_geometry_property(xml_document, &spans).unwrap();

        let xml_node = serialize_multi_geometry_property(
            &property,
            Formatting::Compact,
            GmlElement::GeometryMemberProperty.into(),
        )
        .unwrap();
        let output = xml_node.to_string(Formatting::Compact).unwrap();

        let spans2 = extract_xml_element_spans(output.as_bytes()).unwrap();
        let recovered = deserialize_multi_geometry_property(output.as_bytes(), &spans2).unwrap();

        assert_eq!(
            recovered.object().unwrap().geometry_member().len(),
            property.object().unwrap().geometry_member().len()
        );
    }

    #[test]
    fn deserialize_with_full_association_and_ownership_attributes() {
        let xml_document =
            b"<gml:geometryMember xlink:href=\"#some-id\" xlink:title=\"Some Title\" \
            xlink:role=\"http://example.com/role\" xlink:arcrole=\"http://example.com/arcrole\" \
            xlink:show=\"new\" xlink:actuate=\"onLoad\" gml:owns=\"true\"/>";

        let spans = extract_xml_element_spans(xml_document).unwrap();
        let property = deserialize_multi_geometry_property(xml_document, &spans).unwrap();

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
        let xml_document =
            b"<gml:geometryMember xlink:href=\"#some-id\" xlink:title=\"Some Title\" \
            xlink:role=\"http://example.com/role\" xlink:arcrole=\"http://example.com/arcrole\" \
            xlink:show=\"new\" xlink:actuate=\"onLoad\" gml:owns=\"true\"/>";

        let spans = extract_xml_element_spans(xml_document).unwrap();
        let property = deserialize_multi_geometry_property(xml_document, &spans).unwrap();

        let xml_node = serialize_multi_geometry_property(
            &property,
            Formatting::Compact,
            GmlElement::GeometryMemberProperty.into(),
        )
        .unwrap();
        let output = xml_node.to_string(Formatting::Compact).unwrap();

        let spans2 = extract_xml_element_spans(output.as_bytes()).unwrap();
        let recovered = deserialize_multi_geometry_property(output.as_bytes(), &spans2).unwrap();

        assert_eq!(
            recovered.association(),
            property.association(),
            "association attributes did not survive the round trip; output was: {output}"
        );
        assert_eq!(recovered.ownership(), property.ownership());
    }
}
