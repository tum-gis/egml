use crate::Error;
use crate::codec::base::{
    GmlAssociationAttributes, GmlOwnershipAttributes, serialize_association_attributes,
    serialize_ownership_attributes,
};
use crate::codec::geometry::primitives::{deserialize_solid, serialize_solid};
use crate::util::{Formatting, GmlElement, XmlElementSpans, XmlNode, XmlNodeContent, XmlNodeParts};
use egml_core::model::base::{HasAssociationAttributes, HasOwnershipAttributes};
use egml_core::model::geometry::primitives::SolidProperty;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_solid_property(
    xml_document: &[u8],
    spans: &XmlElementSpans<GmlElement>,
) -> Result<SolidProperty, Error> {
    let parsed: GmlSolidProperty = de::from_reader(xml_document)?;

    let object = spans
        .first(GmlElement::Solid)
        .map(|span| deserialize_solid(&xml_document[span.start..span.end]))
        .transpose()?;

    Ok(SolidProperty::new(
        object,
        parsed.association.try_into()?,
        parsed.ownership.into(),
    ))
}

pub fn serialize_solid_property(
    solid_property: &SolidProperty,
    formatting: Formatting,
    target_xml_element: &'static str,
) -> Result<XmlNode, Error> {
    let mut xml_node_parts = XmlNodeParts::empty();

    xml_node_parts
        .attributes
        .extend(serialize_association_attributes(
            solid_property.association(),
        ));
    xml_node_parts
        .attributes
        .extend(serialize_ownership_attributes(solid_property.ownership()));

    if let Some(solid) = solid_property.object() {
        xml_node_parts
            .content
            .push(XmlNodeContent::Child(serialize_solid(solid, formatting)?));
    }

    Ok(XmlNode::new(target_xml_element, xml_node_parts))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlSolidProperty {
    #[serde(flatten)]
    pub association: GmlAssociationAttributes,
    #[serde(flatten)]
    pub ownership: GmlOwnershipAttributes,
}

#[cfg(test)]
mod tests {
    use super::{deserialize_solid_property, serialize_solid_property};
    use crate::util::{Formatting, extract_xml_element_spans};
    use egml_core::model::base::{HasAssociationAttributes, HasOwnershipAttributes};
    use egml_core::model::xlink::{ActuateType, HRef, ShowType};

    #[test]
    fn deserialize_solid_property_with_xlink() {
        let xml_document = b"<gml:solidMember xlink:href=\"#some-solid-id\"/>";

        let spans = extract_xml_element_spans(xml_document).unwrap();
        let property = deserialize_solid_property(xml_document, &spans).unwrap();

        assert_eq!(property.href(), Some(&HRef::from_local("some-solid-id")));
        assert!(property.object().is_none());
    }

    #[test]
    fn round_trip_solid_property_preserves_object() {
        let xml_document = b"<gml:solidMember>\
            <gml:Solid>\
            <gml:exterior><gml:Shell>\
            <gml:surfaceMember><gml:Polygon><gml:exterior><gml:LinearRing>\
            <gml:posList srsDimension=\"3\">0 0 0 1 0 0 0 1 0 0 0 0</gml:posList>\
            </gml:LinearRing></gml:exterior></gml:Polygon></gml:surfaceMember>\
            </gml:Shell></gml:exterior>\
            </gml:Solid>\
            </gml:solidMember>";

        let spans = extract_xml_element_spans(xml_document).unwrap();
        let property = deserialize_solid_property(xml_document, &spans).unwrap();

        let xml_node =
            serialize_solid_property(&property, Formatting::Compact, "gml:solidMember").unwrap();
        let output = xml_node.to_string(Formatting::Compact).unwrap();

        let spans2 = extract_xml_element_spans(output.as_bytes()).unwrap();
        let recovered = deserialize_solid_property(output.as_bytes(), &spans2).unwrap();

        assert!(recovered.object().is_some());
    }

    #[test]
    fn deserialize_solid_property_with_full_association_and_ownership_attributes() {
        let xml_document = b"<gml:solidMember xlink:href=\"#some-id\" xlink:title=\"Some Title\" \
            xlink:role=\"http://example.com/role\" xlink:arcrole=\"http://example.com/arcrole\" \
            xlink:show=\"new\" xlink:actuate=\"onLoad\" gml:owns=\"true\"/>";

        let spans = extract_xml_element_spans(xml_document).unwrap();
        let property = deserialize_solid_property(xml_document, &spans).unwrap();

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
        let xml_document = b"<gml:solidMember xlink:href=\"#some-id\" xlink:title=\"Some Title\" \
            xlink:role=\"http://example.com/role\" xlink:arcrole=\"http://example.com/arcrole\" \
            xlink:show=\"new\" xlink:actuate=\"onLoad\" gml:owns=\"true\"/>";

        let spans = extract_xml_element_spans(xml_document).unwrap();
        let property = deserialize_solid_property(xml_document, &spans).unwrap();

        let xml_node =
            serialize_solid_property(&property, Formatting::Compact, "gml:solidMember").unwrap();
        let output = xml_node.to_string(Formatting::Compact).unwrap();

        let spans2 = extract_xml_element_spans(output.as_bytes()).unwrap();
        let recovered = deserialize_solid_property(output.as_bytes(), &spans2).unwrap();

        assert_eq!(
            recovered.association(),
            property.association(),
            "association attributes did not survive the round trip; output was: {output}"
        );
        assert_eq!(recovered.ownership(), property.ownership());
    }
}
