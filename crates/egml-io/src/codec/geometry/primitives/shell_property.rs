use crate::Error;
use crate::codec::base::{
    GmlAssociationAttributes, GmlOwnershipAttributes, serialize_association_attributes,
    serialize_ownership_attributes,
};
use crate::codec::geometry::primitives::{deserialize_shell, serialize_shell};
use crate::util::{Formatting, GmlElement, XmlElementSpans, XmlNode, XmlNodeContent, XmlNodeParts};
use egml_core::model::base::{HasAssociationAttributes, HasOwnershipAttributes};
use egml_core::model::geometry::primitives::ShellProperty;
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_shell_property(
    xml_document: &[u8],
    spans: &XmlElementSpans<GmlElement>,
) -> Result<ShellProperty, Error> {
    let parsed: GmlShellProperty = de::from_reader(xml_document)?;

    let object = spans
        .first(GmlElement::Shell)
        .map(|span| deserialize_shell(&xml_document[span.start..span.end]))
        .transpose()?;

    Ok(ShellProperty::new(
        object,
        parsed.association.try_into()?,
        parsed.ownership.into(),
    ))
}

pub fn serialize_shell_property(
    shell_property: &ShellProperty,
    formatting: Formatting,
    target_xml_element: &'static str,
) -> Result<XmlNode, Error> {
    let mut parts = XmlNodeParts::empty();

    parts.attributes.extend(serialize_association_attributes(
        shell_property.association(),
    ));
    parts
        .attributes
        .extend(serialize_ownership_attributes(shell_property.ownership()));

    if let Some(shell) = shell_property.object() {
        parts
            .content
            .push(XmlNodeContent::Child(serialize_shell(shell, formatting)?));
    }

    Ok(XmlNode::new(target_xml_element, parts))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlShellProperty {
    #[serde(flatten)]
    pub association: GmlAssociationAttributes,
    #[serde(flatten)]
    pub ownership: GmlOwnershipAttributes,
}

#[cfg(test)]
mod tests {
    use crate::codec::geometry::primitives::shell_property::{
        deserialize_shell_property, serialize_shell_property,
    };
    use crate::util::{Formatting, GmlElement, extract_xml_element_spans};
    use egml_core::model::base::{HasAssociationAttributes, HasOwnershipAttributes};
    use egml_core::model::geometry::DirectPosition;
    use egml_core::model::geometry::primitives::{
        AbstractRingKind, AbstractRingProperty, AbstractSurfaceKind, AbstractSurfaceProperty,
        LinearRing, Polygon, Shell, ShellProperty,
    };
    use egml_core::model::xlink::{ActuateType, HRef, ShowType};

    fn make_shell_property() -> ShellProperty {
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
            [],
        )
        .unwrap();
        let member = AbstractSurfaceProperty::from_object(AbstractSurfaceKind::Polygon(polygon));
        ShellProperty::from_object(Shell::new([member]).unwrap())
    }

    #[test]
    fn deserialize_shell_property_test() {
        let xml_document = b"<gml:exterior>
    <gml:Shell>
        <gml:surfaceMember>
            <gml:Polygon>
                <gml:exterior>
                    <gml:LinearRing>
                        <gml:posList srsDimension=\"3\">0 0 0 1 0 0 0 1 0 0 0 0</gml:posList>
                    </gml:LinearRing>
                </gml:exterior>
            </gml:Polygon>
        </gml:surfaceMember>
    </gml:Shell>
</gml:exterior>";

        let spans = extract_xml_element_spans(xml_document).expect("extracting spans should work");
        let property =
            deserialize_shell_property(xml_document, &spans).expect("should deserialize");

        assert!(property.object().is_some());
        assert_eq!(property.object().unwrap().members().len(), 1);
    }

    #[test]
    fn deserialize_shell_property_with_xlink() {
        let xml_document = b"<gml:exterior xlink:href=\"#some-shell-id\"/>";

        let spans = extract_xml_element_spans(xml_document).expect("extracting spans should work");
        let property =
            deserialize_shell_property(xml_document, &spans).expect("should deserialize");

        assert_eq!(property.href(), Some(&HRef::from_local("some-shell-id")));
        assert!(property.object().is_none());
    }

    #[test]
    fn serialize_shell_property_writes_gml_tags() {
        let property = make_shell_property();

        let xml_node = serialize_shell_property(
            &property,
            Formatting::Compact,
            GmlElement::ExteriorProperty.into(),
        )
        .expect("should serialize");
        let xml = xml_node
            .to_string(Formatting::Compact)
            .expect("to string should work");

        assert!(xml.contains("<gml:exterior"));
        assert!(xml.contains("<gml:Shell"));
        assert!(xml.contains("<gml:surfaceMember"));
        assert!(xml.contains("<gml:Polygon"));
    }

    #[test]
    fn round_trip_shell_property_preserves_members() {
        let xml_document = b"<gml:exterior>\
            <gml:Shell>\
            <gml:surfaceMember><gml:Polygon><gml:exterior><gml:LinearRing>\
            <gml:posList srsDimension=\"3\">0 0 0 1 0 0 0 1 0 0 0 0</gml:posList>\
            </gml:LinearRing></gml:exterior></gml:Polygon></gml:surfaceMember>\
            </gml:Shell>\
            </gml:exterior>";

        let spans = extract_xml_element_spans(xml_document).expect("extracting spans should work");
        let property = deserialize_shell_property(xml_document, &spans).unwrap();

        let xml_node = serialize_shell_property(
            &property,
            Formatting::Compact,
            GmlElement::ExteriorProperty.into(),
        )
        .unwrap();
        let output = xml_node.to_string(Formatting::Compact).unwrap();

        let spans2 = extract_xml_element_spans(output.as_bytes()).unwrap();
        let recovered = deserialize_shell_property(output.as_bytes(), &spans2).unwrap();

        assert_eq!(
            recovered.object().unwrap().members().len(),
            property.object().unwrap().members().len()
        );
    }

    #[test]
    fn deserialize_shell_property_with_full_association_and_ownership_attributes() {
        let xml_document = b"<gml:exterior xlink:href=\"#some-id\" xlink:title=\"Some Title\" \
            xlink:role=\"http://example.com/role\" xlink:arcrole=\"http://example.com/arcrole\" \
            xlink:show=\"new\" xlink:actuate=\"onLoad\" gml:owns=\"true\"/>";

        let spans = extract_xml_element_spans(xml_document).unwrap();
        let property = deserialize_shell_property(xml_document, &spans).unwrap();

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
        let xml_document = b"<gml:exterior xlink:href=\"#some-id\" xlink:title=\"Some Title\" \
            xlink:role=\"http://example.com/role\" xlink:arcrole=\"http://example.com/arcrole\" \
            xlink:show=\"new\" xlink:actuate=\"onLoad\" gml:owns=\"true\"/>";

        let spans = extract_xml_element_spans(xml_document).unwrap();
        let property = deserialize_shell_property(xml_document, &spans).unwrap();

        let xml_node = serialize_shell_property(
            &property,
            Formatting::Compact,
            GmlElement::ExteriorProperty.into(),
        )
        .unwrap();
        let output = xml_node.to_string(Formatting::Compact).unwrap();

        let spans2 = extract_xml_element_spans(output.as_bytes()).unwrap();
        let recovered = deserialize_shell_property(output.as_bytes(), &spans2).unwrap();

        assert_eq!(
            recovered.association(),
            property.association(),
            "association attributes did not survive the round trip; output was: {output}"
        );
        assert_eq!(recovered.ownership(), property.ownership());
    }
}
