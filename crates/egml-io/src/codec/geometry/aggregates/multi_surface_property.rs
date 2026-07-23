use crate::Error;
use crate::codec::base::{
    GmlAssociationAttributes, GmlOwnershipAttributes, serialize_association_attributes,
    serialize_ownership_attributes,
};
use crate::codec::geometry::aggregates::{deserialize_multi_surface, serialize_multi_surface};
use crate::util::{Formatting, GmlElement, XmlElementSpans, XmlNode, XmlNodeContent, XmlNodeParts};
use egml_core::model::base::{HasAssociationAttributes, HasOwnershipAttributes};
use egml_core::model::geometry::aggregates::MultiSurfaceProperty;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_multi_surface_property(
    xml_document: &[u8],
    spans: &XmlElementSpans<GmlElement>,
) -> Result<MultiSurfaceProperty, Error> {
    let parsed: GmlMultiSurfaceProperty = de::from_reader(xml_document)?;

    let object = spans
        .first(GmlElement::MultiSurface)
        .map(|span| deserialize_multi_surface(&xml_document[span.start..span.end]))
        .transpose()?;

    Ok(MultiSurfaceProperty::new(
        object,
        parsed.association.try_into()?,
        parsed.ownership.into(),
    ))
}

pub fn serialize_multi_surface_property(
    multi_surface_property: &MultiSurfaceProperty,
    formatting: Formatting,
    target_xml_element: &'static str,
) -> Result<XmlNode, Error> {
    let mut xml_node_parts = XmlNodeParts::empty();

    xml_node_parts
        .attributes
        .extend(serialize_association_attributes(
            multi_surface_property.association(),
        ));
    xml_node_parts
        .attributes
        .extend(serialize_ownership_attributes(
            multi_surface_property.ownership(),
        ));

    if let Some(multi_surface) = multi_surface_property.object() {
        xml_node_parts
            .content
            .push(XmlNodeContent::Child(serialize_multi_surface(
                multi_surface,
                formatting,
            )?));
    }

    Ok(XmlNode::new(target_xml_element, xml_node_parts))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlMultiSurfaceProperty {
    #[serde(flatten)]
    pub association: GmlAssociationAttributes,
    #[serde(flatten)]
    pub ownership: GmlOwnershipAttributes,
}

#[cfg(test)]
mod tests {
    use crate::codec::geometry::aggregates::multi_surface_property::{
        deserialize_multi_surface_property, serialize_multi_surface_property,
    };
    use crate::util::{Formatting, GmlElement, extract_xml_element_spans};
    use egml_core::model::base::{HasAssociationAttributes, HasOwnershipAttributes};
    use egml_core::model::geometry::DirectPosition;
    use egml_core::model::geometry::aggregates::{MultiSurface, MultiSurfaceProperty};
    use egml_core::model::geometry::primitives::{
        AbstractRingKind, AbstractRingProperty, AbstractSurfaceKind, AbstractSurfaceProperty,
        LinearRing, Polygon,
    };
    use egml_core::model::xlink::{ActuateType, HRef, ShowType};

    fn make_multi_surface_property() -> MultiSurfaceProperty {
        let points = vec![
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(0.0, 1.0, 0.0).unwrap(),
        ];
        let ring_kind = AbstractRingKind::LinearRing(LinearRing::new(points).unwrap());
        let polygon = Polygon::new(Some(AbstractRingProperty::from_object(ring_kind)), []).unwrap();
        let multi_surface = MultiSurface::new([AbstractSurfaceProperty::from_object(
            AbstractSurfaceKind::Polygon(polygon),
        )])
        .unwrap();
        MultiSurfaceProperty::from_object(multi_surface)
    }

    #[test]
    fn deserialize_multi_surface_property_test() {
        let xml_document = b"<gml:surfaceMember>
            <gml:MultiSurface>
                <gml:surfaceMember>
                    <gml:Polygon>
                        <gml:exterior>
                            <gml:LinearRing>
                                <gml:posList srsDimension=\"3\">0 0 0 1 0 0 0 1 0 0 0 0</gml:posList>
                            </gml:LinearRing>
                        </gml:exterior>
                    </gml:Polygon>
                </gml:surfaceMember>
            </gml:MultiSurface>
        </gml:surfaceMember>";

        let spans = extract_xml_element_spans(xml_document).expect("extracting spans should work");
        let property =
            deserialize_multi_surface_property(xml_document, &spans).expect("should deserialize");

        assert!(property.object().is_some());
        assert_eq!(property.object().unwrap().surface_member().len(), 1);
    }

    #[test]
    fn deserialize_multi_surface_property_with_xlink() {
        let xml_document = b"<gml:surfaceMember xlink:href=\"#some-surface-id\"/>";

        let spans = extract_xml_element_spans(xml_document).expect("extracting spans should work");
        let property =
            deserialize_multi_surface_property(xml_document, &spans).expect("should deserialize");

        assert_eq!(property.href(), Some(&HRef::from_local("some-surface-id")));
        assert!(property.object().is_none());
    }

    #[test]
    fn serialize_multi_surface_property_writes_gml_tags() {
        let property = make_multi_surface_property();

        let xml_node = serialize_multi_surface_property(
            &property,
            Formatting::Compact,
            GmlElement::SurfaceMemberProperty.into(),
        )
        .expect("should serialize");
        let xml = xml_node
            .to_string(Formatting::Compact)
            .expect("to string should work");

        assert!(xml.contains("<gml:surfaceMember"));
        assert!(xml.contains("<gml:MultiSurface"));
        assert!(xml.contains("<gml:Polygon"));
        assert!(xml.contains("<gml:LinearRing"));
    }

    #[test]
    fn round_trip_multi_surface_property_preserves_member_count() {
        let xml_document = b"<gml:surfaceMember>\
            <gml:MultiSurface>\
            <gml:surfaceMember><gml:Polygon><gml:exterior><gml:LinearRing>\
            <gml:posList srsDimension=\"3\">0 0 0 1 0 0 0 1 0 0 0 0</gml:posList>\
            </gml:LinearRing></gml:exterior></gml:Polygon></gml:surfaceMember>\
            </gml:MultiSurface>\
            </gml:surfaceMember>";

        let spans = extract_xml_element_spans(xml_document).unwrap();
        let property = deserialize_multi_surface_property(xml_document, &spans).unwrap();

        let xml_node = serialize_multi_surface_property(
            &property,
            Formatting::Compact,
            GmlElement::SurfaceMemberProperty.into(),
        )
        .unwrap();
        let output = xml_node.to_string(Formatting::Compact).unwrap();

        let spans2 = extract_xml_element_spans(output.as_bytes()).unwrap();
        let recovered = deserialize_multi_surface_property(output.as_bytes(), &spans2).unwrap();

        assert_eq!(
            recovered.object().unwrap().surface_member().len(),
            property.object().unwrap().surface_member().len()
        );
    }

    #[test]
    fn deserialize_with_full_association_and_ownership_attributes() {
        let xml_document =
            b"<gml:surfaceMember xlink:href=\"#some-id\" xlink:title=\"Some Title\" \
            xlink:role=\"http://example.com/role\" xlink:arcrole=\"http://example.com/arcrole\" \
            xlink:show=\"new\" xlink:actuate=\"onLoad\" gml:owns=\"true\"/>";

        let spans = extract_xml_element_spans(xml_document).unwrap();
        let property = deserialize_multi_surface_property(xml_document, &spans).unwrap();

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
            b"<gml:surfaceMember xlink:href=\"#some-id\" xlink:title=\"Some Title\" \
            xlink:role=\"http://example.com/role\" xlink:arcrole=\"http://example.com/arcrole\" \
            xlink:show=\"new\" xlink:actuate=\"onLoad\" gml:owns=\"true\"/>";

        let spans = extract_xml_element_spans(xml_document).unwrap();
        let property = deserialize_multi_surface_property(xml_document, &spans).unwrap();

        let xml_node = serialize_multi_surface_property(
            &property,
            Formatting::Compact,
            GmlElement::SurfaceMemberProperty.into(),
        )
        .unwrap();
        let output = xml_node.to_string(Formatting::Compact).unwrap();

        let spans2 = extract_xml_element_spans(output.as_bytes()).unwrap();
        let recovered = deserialize_multi_surface_property(output.as_bytes(), &spans2).unwrap();

        assert_eq!(
            recovered.association(),
            property.association(),
            "association attributes did not survive the round trip; output was: {output}"
        );
        assert_eq!(recovered.ownership(), property.ownership());
    }
}
