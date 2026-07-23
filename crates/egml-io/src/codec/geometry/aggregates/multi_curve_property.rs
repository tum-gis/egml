use crate::Error;
use crate::codec::base::{
    GmlAssociationAttributes, GmlOwnershipAttributes, serialize_association_attributes,
    serialize_ownership_attributes,
};
use crate::codec::geometry::aggregates::{deserialize_multi_curve, serialize_multi_curve};
use crate::util::{Formatting, GmlElement, XmlElementSpans, XmlNode, XmlNodeContent, XmlNodeParts};
use egml_core::model::base::{HasAssociationAttributes, HasOwnershipAttributes};
use egml_core::model::geometry::aggregates::MultiCurveProperty;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_multi_curve_property(
    xml_document: &[u8],
    spans: &XmlElementSpans<GmlElement>,
) -> Result<MultiCurveProperty, Error> {
    let parsed: GmlMultiCurveProperty = de::from_reader(xml_document)?;

    let object = spans
        .first(GmlElement::MultiCurve)
        .map(|span| deserialize_multi_curve(&xml_document[span.start..span.end]))
        .transpose()?;

    Ok(MultiCurveProperty::new(
        object,
        parsed.association.try_into()?,
        parsed.ownership.into(),
    ))
}

pub fn serialize_multi_curve_property(
    multi_curve_property: &MultiCurveProperty,
    formatting: Formatting,
    target_xml_element: &'static str,
) -> Result<XmlNode, Error> {
    let mut xml_node_parts = XmlNodeParts::empty();

    xml_node_parts
        .attributes
        .extend(serialize_association_attributes(
            multi_curve_property.association(),
        ));
    xml_node_parts
        .attributes
        .extend(serialize_ownership_attributes(
            multi_curve_property.ownership(),
        ));

    if let Some(multi_curve) = multi_curve_property.object() {
        let raw = serialize_multi_curve(multi_curve, formatting)?.to_string(formatting)?;
        xml_node_parts.content.push(XmlNodeContent::Raw(raw));
    }

    Ok(XmlNode::new(target_xml_element, xml_node_parts))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlMultiCurveProperty {
    #[serde(flatten)]
    pub association: GmlAssociationAttributes,
    #[serde(flatten)]
    pub ownership: GmlOwnershipAttributes,
}

#[cfg(test)]
mod tests {
    use crate::codec::geometry::aggregates::multi_curve_property::{
        deserialize_multi_curve_property, serialize_multi_curve_property,
    };
    use crate::util::{Formatting, GmlElement, extract_xml_element_spans};
    use egml_core::model::base::{HasAssociationAttributes, HasOwnershipAttributes};
    use egml_core::model::geometry::DirectPosition;
    use egml_core::model::geometry::aggregates::{MultiCurve, MultiCurveProperty};
    use egml_core::model::geometry::primitives::{
        AbstractCurveKind, AbstractCurveProperty, LineString,
    };
    use egml_core::model::xlink::{ActuateType, HRef, ShowType};

    fn make_multi_curve_property() -> MultiCurveProperty {
        let points = vec![
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 1.0, 1.0).unwrap(),
            DirectPosition::new(2.0, 2.0, 2.0).unwrap(),
        ];
        let curve_kind = AbstractCurveKind::LineString(LineString::new(points).unwrap());
        let multi_curve =
            MultiCurve::new([AbstractCurveProperty::from_object(curve_kind)]).unwrap();
        MultiCurveProperty::from_object(multi_curve)
    }

    #[test]
    fn deserialize_multi_curve_property_test() {
        let xml_document = b"<gml:curveMember>
            <gml:MultiCurve>
                <gml:curveMember>
                    <gml:LineString>
                        <gml:posList srsDimension=\"3\">0 0 0 1 1 1 2 2 2</gml:posList>
                    </gml:LineString>
                </gml:curveMember>
            </gml:MultiCurve>
        </gml:curveMember>";

        let spans = extract_xml_element_spans(xml_document).expect("extracting spans should work");
        let property =
            deserialize_multi_curve_property(xml_document, &spans).expect("should deserialize");

        assert!(property.object().is_some());
        assert_eq!(property.object().unwrap().curve_member().len(), 1);
    }

    #[test]
    fn deserialize_multi_curve_property_with_xlink() {
        let xml_document = b"<gml:curveMember xlink:href=\"#some-curve-id\"/>";

        let spans = extract_xml_element_spans(xml_document).expect("extracting spans should work");
        let property =
            deserialize_multi_curve_property(xml_document, &spans).expect("should deserialize");

        assert_eq!(property.href(), Some(&HRef::from_local("some-curve-id")));
        assert!(property.object().is_none());
    }

    #[test]
    fn serialize_multi_curve_property_writes_gml_tags() {
        let property = make_multi_curve_property();

        let xml_node = serialize_multi_curve_property(
            &property,
            Formatting::Compact,
            GmlElement::CurveMemberProperty.into(),
        )
        .expect("should serialize");
        let xml = xml_node
            .to_string(Formatting::Compact)
            .expect("to string should work");

        assert!(xml.contains("<gml:curveMember"));
        assert!(xml.contains("<gml:MultiCurve"));
        assert!(xml.contains("<gml:LineString"));
        assert!(xml.contains("<gml:posList"));
    }

    #[test]
    fn round_trip_multi_curve_property_preserves_member_count() {
        let xml_document = b"<gml:curveMember>\
            <gml:MultiCurve>\
            <gml:curveMember><gml:LineString>\
            <gml:posList srsDimension=\"3\">0 0 0 1 1 1 2 2 2</gml:posList>\
            </gml:LineString></gml:curveMember>\
            </gml:MultiCurve>\
            </gml:curveMember>";

        let spans = extract_xml_element_spans(xml_document).unwrap();
        let property = deserialize_multi_curve_property(xml_document, &spans).unwrap();

        let xml_node = serialize_multi_curve_property(
            &property,
            Formatting::Compact,
            GmlElement::CurveMemberProperty.into(),
        )
        .unwrap();
        let output = xml_node.to_string(Formatting::Compact).unwrap();

        let spans2 = extract_xml_element_spans(output.as_bytes()).unwrap();
        let recovered = deserialize_multi_curve_property(output.as_bytes(), &spans2).unwrap();

        assert_eq!(
            recovered.object().unwrap().curve_member().len(),
            property.object().unwrap().curve_member().len()
        );
    }

    #[test]
    fn deserialize_with_full_association_and_ownership_attributes() {
        let xml_document = b"<gml:curveMember xlink:href=\"#some-id\" xlink:title=\"Some Title\" \
            xlink:role=\"http://example.com/role\" xlink:arcrole=\"http://example.com/arcrole\" \
            xlink:show=\"new\" xlink:actuate=\"onLoad\" gml:owns=\"true\"/>";

        let spans = extract_xml_element_spans(xml_document).unwrap();
        let property = deserialize_multi_curve_property(xml_document, &spans).unwrap();

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
        let property = deserialize_multi_curve_property(xml_document, &spans).unwrap();

        let xml_node = serialize_multi_curve_property(
            &property,
            Formatting::Compact,
            GmlElement::CurveMemberProperty.into(),
        )
        .unwrap();
        let output = xml_node.to_string(Formatting::Compact).unwrap();

        let spans2 = extract_xml_element_spans(output.as_bytes()).unwrap();
        let recovered = deserialize_multi_curve_property(output.as_bytes(), &spans2).unwrap();

        assert_eq!(
            recovered.association(),
            property.association(),
            "association attributes did not survive the round trip; output was: {output}"
        );
        assert_eq!(recovered.ownership(), property.ownership());
    }
}
