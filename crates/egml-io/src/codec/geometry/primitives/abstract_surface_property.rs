use crate::Error;
use crate::codec::base::{
    GmlAssociationAttributes, GmlOwnershipAttributes, serialize_association_attributes,
    serialize_ownership_attributes,
};
use crate::codec::geometry::primitives::{
    deserialize_abstract_surface_kind, serialize_abstract_surface_kind,
};
use crate::util::{Formatting, GmlElement, XmlElementSpans, XmlNode, XmlNodeContent, XmlNodeParts};
use egml_core::model::base::{HasAssociationAttributes, HasOwnershipAttributes};
use egml_core::model::geometry::primitives::AbstractSurfaceProperty;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_abstract_surface_property(
    xml_document: &[u8],
    spans: &XmlElementSpans<GmlElement>,
) -> Result<AbstractSurfaceProperty, Error> {
    let parsed: GmlAbstractSurfaceProperty = de::from_reader(xml_document)?;

    let object = deserialize_abstract_surface_kind(xml_document, spans)?;

    Ok(AbstractSurfaceProperty::new(
        object,
        parsed.association.try_into()?,
        parsed.ownership.into(),
    ))
}

pub fn serialize_abstract_surface_property(
    abstract_surface_property: &AbstractSurfaceProperty,
    formatting: Formatting,
    target_xml_element: &'static str,
) -> Result<XmlNode, Error> {
    let mut xml_node_parts = XmlNodeParts::empty();

    xml_node_parts
        .attributes
        .extend(serialize_association_attributes(
            abstract_surface_property.association(),
        ));
    xml_node_parts
        .attributes
        .extend(serialize_ownership_attributes(
            abstract_surface_property.ownership(),
        ));

    if let Some(abstract_surface_kind) = abstract_surface_property.object() {
        xml_node_parts
            .content
            .push(XmlNodeContent::Child(serialize_abstract_surface_kind(
                abstract_surface_kind,
                formatting,
            )?));
    }

    Ok(XmlNode::new(target_xml_element, xml_node_parts))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlAbstractSurfaceProperty {
    #[serde(flatten)]
    pub association: GmlAssociationAttributes,
    #[serde(flatten)]
    pub ownership: GmlOwnershipAttributes,
}

#[cfg(test)]
mod tests {
    use crate::codec::geometry::primitives::abstract_surface_property::{
        deserialize_abstract_surface_property, serialize_abstract_surface_property,
    };
    use crate::util::{Formatting, GmlElement, extract_xml_element_spans};
    use egml_core::model::base::HasAssociationAttributes;
    use egml_core::model::geometry::DirectPosition;
    use egml_core::model::geometry::primitives::{
        AbstractRingKind, AbstractRingProperty, AbstractSurfaceKind, AbstractSurfaceProperty,
        LinearRing, Polygon,
    };
    use egml_core::model::xlink::HRef;

    fn make_surface_property() -> AbstractSurfaceProperty {
        let ring = LinearRing::new([
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(0.0, 1.0, 0.0).unwrap(),
        ])
        .unwrap();
        let polygon = Polygon::new(
            Some(AbstractRingProperty::from_object(
                AbstractRingKind::LinearRing(ring),
            )),
            vec![],
        )
        .unwrap();
        AbstractSurfaceProperty::from_object(AbstractSurfaceKind::Polygon(polygon))
    }

    #[test]
    fn deserialize_abstract_surface_property_with_polygon() {
        let xml_document = b"<gml:surfaceMember>
    <gml:Polygon>
        <gml:exterior>
            <gml:LinearRing>
                <gml:posList srsDimension=\"3\">0 0 0 1 0 0 0 1 0 0 0 0</gml:posList>
            </gml:LinearRing>
        </gml:exterior>
    </gml:Polygon>
</gml:surfaceMember>";

        let spans = extract_xml_element_spans(xml_document).expect("extracting spans should work");
        let property = deserialize_abstract_surface_property(xml_document, &spans)
            .expect("should deserialize");

        assert!(matches!(
            property.object(),
            Some(AbstractSurfaceKind::Polygon(_))
        ));
    }

    #[test]
    fn deserialize_abstract_surface_property_with_xlink() {
        let xml_document = b"<gml:surfaceMember xlink:href=\"#some-surface-id\"/>";

        let spans = extract_xml_element_spans(xml_document).expect("extracting spans should work");
        let property = deserialize_abstract_surface_property(xml_document, &spans)
            .expect("should deserialize");

        assert_eq!(property.href(), Some(&HRef::from_local("some-surface-id")));
        assert!(property.object().is_none());
    }

    #[test]
    fn serialize_abstract_surface_property_writes_gml_tags() {
        let property = make_surface_property();

        let xml_node = serialize_abstract_surface_property(
            &property,
            Formatting::Compact,
            GmlElement::SurfaceMemberProperty.into(),
        )
        .expect("should serialize");
        let xml = xml_node
            .to_string(Formatting::Compact)
            .expect("to string should work");

        assert!(xml.contains("<gml:surfaceMember"));
        assert!(xml.contains("<gml:Polygon"));
        assert!(xml.contains("<gml:exterior"));
        assert!(xml.contains("<gml:LinearRing"));
        assert!(xml.contains("<gml:posList"));
    }

    #[test]
    fn round_trip_abstract_surface_property_preserves_polygon() {
        let xml_document = b"<gml:surfaceMember>\
            <gml:Polygon><gml:exterior><gml:LinearRing>\
            <gml:posList srsDimension=\"3\">0 0 0 1 0 0 0 1 0 0 0 0</gml:posList>\
            </gml:LinearRing></gml:exterior></gml:Polygon>\
            </gml:surfaceMember>";

        let spans = extract_xml_element_spans(xml_document).expect("extracting spans should work");
        let property = deserialize_abstract_surface_property(xml_document, &spans).unwrap();

        let xml_node = serialize_abstract_surface_property(
            &property,
            Formatting::Compact,
            GmlElement::SurfaceMemberProperty.into(),
        )
        .unwrap();
        let output = xml_node.to_string(Formatting::Compact).unwrap();

        let spans2 = extract_xml_element_spans(output.as_bytes()).unwrap();
        let recovered = deserialize_abstract_surface_property(output.as_bytes(), &spans2).unwrap();

        assert!(matches!(
            recovered.object(),
            Some(AbstractSurfaceKind::Polygon(_))
        ));
    }
}
