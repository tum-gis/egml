use crate::Error;
use crate::codec::base::{
    GmlAssociationAttributes, GmlOwnershipAttributes, serialize_association_attributes,
    serialize_ownership_attributes,
};
use crate::codec::geometry::primitives::{
    deserialize_abstract_curve_kind, serialize_abstract_curve_kind,
};
use crate::util::{Formatting, GmlElement, XmlElementSpans, XmlNode, XmlNodeContent, XmlNodeParts};
use egml_core::model::base::{HasAssociationAttributes, HasOwnershipAttributes};
use egml_core::model::geometry::primitives::AbstractCurveProperty;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_abstract_curve_property(
    xml_document: &[u8],
    spans: &XmlElementSpans<GmlElement>,
) -> Result<AbstractCurveProperty, Error> {
    let parsed: GmlAbstractCurveProperty = de::from_reader(xml_document)?;

    let object = deserialize_abstract_curve_kind(xml_document, spans)?;

    Ok(AbstractCurveProperty::new(
        object,
        parsed.association.try_into()?,
        parsed.ownership.into(),
    ))
}

pub fn serialize_abstract_curve_property(
    abstract_curve_property: &AbstractCurveProperty,
    formatting: Formatting,
    target_xml_element: &'static str,
) -> Result<XmlNode, Error> {
    let mut parts = XmlNodeParts::empty();

    parts.attributes.extend(serialize_association_attributes(
        abstract_curve_property.association(),
    ));
    parts.attributes.extend(serialize_ownership_attributes(
        abstract_curve_property.ownership(),
    ));

    if let Some(abstract_curve_kind) = abstract_curve_property.object() {
        parts
            .content
            .push(XmlNodeContent::Child(serialize_abstract_curve_kind(
                abstract_curve_kind,
                formatting,
            )?));
    }

    Ok(XmlNode::new(target_xml_element, parts))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlAbstractCurveProperty {
    #[serde(flatten)]
    pub association: GmlAssociationAttributes,
    #[serde(flatten)]
    pub ownership: GmlOwnershipAttributes,
}

#[cfg(test)]
mod tests {
    use crate::codec::geometry::primitives::{
        deserialize_abstract_curve_property, serialize_abstract_curve_property,
    };
    use crate::util::{Formatting, GmlElement, extract_xml_element_spans};
    use egml_core::model::base::{HasAssociationAttributes, HasOwnershipAttributes};
    use egml_core::model::geometry::primitives::AbstractCurveKind;
    use egml_core::model::xlink::{ActuateType, HRef, ShowType};

    #[test]
    fn deserialize_with_line_string() {
        let xml_document = b"<gml:curveMember>\
            <gml:LineString><gml:posList srsDimension=\"3\">0 0 0 1 1 1 2 2 2</gml:posList></gml:LineString>\
            </gml:curveMember>";

        let spans = extract_xml_element_spans(xml_document).unwrap();
        let property = deserialize_abstract_curve_property(xml_document, &spans).unwrap();

        assert!(matches!(
            property.object(),
            Some(AbstractCurveKind::LineString(_))
        ));
    }

    #[test]
    fn deserialize_with_xlink() {
        let xml_document = b"<gml:curveMember xlink:href=\"#some-id\"/>";

        let spans = extract_xml_element_spans(xml_document).unwrap();
        let property = deserialize_abstract_curve_property(xml_document, &spans).unwrap();

        assert_eq!(property.href(), Some(&HRef::from_local("some-id")));
        assert!(property.object().is_none());
    }

    #[test]
    fn round_trip_line_string_preserves_points() {
        let xml_document = b"<gml:curveMember>\
            <gml:LineString><gml:posList srsDimension=\"3\">0 0 0 1 1 1 2 2 2</gml:posList></gml:LineString>\
            </gml:curveMember>";

        let spans = extract_xml_element_spans(xml_document).unwrap();
        let property = deserialize_abstract_curve_property(xml_document, &spans).unwrap();

        let xml_node = serialize_abstract_curve_property(
            &property,
            Formatting::Compact,
            GmlElement::CurveMemberProperty.into(),
        )
        .unwrap();
        let output = xml_node.to_string(Formatting::Compact).unwrap();

        let spans2 = extract_xml_element_spans(output.as_bytes()).unwrap();
        let recovered = deserialize_abstract_curve_property(output.as_bytes(), &spans2).unwrap();

        assert!(matches!(
            recovered.object(),
            Some(AbstractCurveKind::LineString(_))
        ));
    }

    #[test]
    fn deserialize_with_full_association_and_ownership_attributes() {
        let xml_document = b"<gml:curveMember xlink:href=\"#some-id\" xlink:title=\"Some Title\" \
            xlink:role=\"http://example.com/role\" xlink:arcrole=\"http://example.com/arcrole\" \
            xlink:show=\"new\" xlink:actuate=\"onLoad\" gml:owns=\"true\"/>";

        let spans = extract_xml_element_spans(xml_document).unwrap();
        let property = deserialize_abstract_curve_property(xml_document, &spans).unwrap();

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
        let xml_document = b"<gml:curveMember xlink:href=\"#some-id\" xlink:title=\"Some Title\" \
            xlink:role=\"http://example.com/role\" xlink:arcrole=\"http://example.com/arcrole\" \
            xlink:show=\"new\" xlink:actuate=\"onLoad\" gml:owns=\"true\"/>";

        let spans = extract_xml_element_spans(xml_document).unwrap();
        let property = deserialize_abstract_curve_property(xml_document, &spans).unwrap();

        let xml_node = serialize_abstract_curve_property(
            &property,
            Formatting::Compact,
            GmlElement::CurveMemberProperty.into(),
        )
        .unwrap();
        let output = xml_node.to_string(Formatting::Compact).unwrap();

        let spans2 = extract_xml_element_spans(output.as_bytes()).unwrap();
        let recovered = deserialize_abstract_curve_property(output.as_bytes(), &spans2).unwrap();

        assert_eq!(
            recovered.association(),
            property.association(),
            "association attributes did not survive the round trip; output was: {output}"
        );
        assert_eq!(recovered.ownership(), property.ownership());
    }
}
