use crate::Error;
use crate::codec::base::{
    deserialize_association_and_ownership_attributes,
    serialize_association_and_ownership_attributes,
};
use crate::codec::geometry::aggregates::multi_curve::deserialize_multi_curve;
use crate::codec::geometry::aggregates::serialize_multi_curve;
use crate::util::{
    DeserializationConfig, GmlElement, XmlDocumentIndex, XmlElement, XmlFragmentWriter,
    XmlNamespace,
};
use egml_core::model::base::{HasAssociationAttributes, HasOwnershipAttributes};
use egml_core::model::geometry::aggregates::MultiCurveProperty;
use std::io::Write;

pub fn deserialize_multi_curve_property(
    xml_document: &[u8],
    index: &XmlDocumentIndex<GmlElement>,
    config: &DeserializationConfig,
) -> Result<MultiCurveProperty, Error> {
    let index = if index.is_truncated() {
        &XmlDocumentIndex::from_scan(xml_document, config.rescan_depth())?
    } else {
        index
    };

    let (association, ownership) = deserialize_association_and_ownership_attributes(xml_document)?;

    let object = index
        .first(GmlElement::MultiCurve)
        .map(|node| deserialize_multi_curve(&xml_document[node.range()], node, config))
        .transpose()?;

    Ok(MultiCurveProperty::new(object, association, ownership))
}

pub fn serialize_multi_curve_property<N: XmlNamespace, E: XmlElement, W: Write>(
    multi_curve_property: &MultiCurveProperty,
    xml_fragment_writer: &mut XmlFragmentWriter<W>,
    target_xml_namespace: N,
    target_xml_element: E,
) -> Result<(), Error> {
    let attributes = serialize_association_and_ownership_attributes(
        multi_curve_property.association(),
        multi_curve_property.ownership(),
    );

    match multi_curve_property.object() {
        Some(multi_curve) => {
            xml_fragment_writer.write_start_event_with_attributes(
                target_xml_namespace,
                target_xml_element,
                attributes,
            )?;
            serialize_multi_curve(multi_curve, xml_fragment_writer)?;
            xml_fragment_writer.write_end_event(target_xml_namespace, target_xml_element)?;
        }
        None => {
            xml_fragment_writer.write_empty_element_with_attributes(
                target_xml_namespace,
                target_xml_element,
                attributes,
            )?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::codec::geometry::aggregates::multi_curve_property::{
        deserialize_multi_curve_property, serialize_multi_curve_property,
    };
    use crate::util::{Formatting, GmlElement, GmlNamespace, XmlDocumentIndex, XmlFragmentWriter};
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

        let index =
            XmlDocumentIndex::from_scan(xml_document, None).expect("extracting spans should work");
        let property = deserialize_multi_curve_property(
            xml_document,
            &index,
            &crate::util::DeserializationConfig::default(),
        )
        .expect("should deserialize");

        assert!(property.object().is_some());
        assert_eq!(property.object().unwrap().curve_member().len(), 1);
    }

    #[test]
    fn deserialize_multi_curve_property_with_xlink() {
        let xml_document = b"<gml:curveMember xlink:href=\"#some-curve-id\"/>";

        let index =
            XmlDocumentIndex::from_scan(xml_document, None).expect("extracting spans should work");
        let property = deserialize_multi_curve_property(
            xml_document,
            &index,
            &crate::util::DeserializationConfig::default(),
        )
        .expect("should deserialize");

        assert_eq!(property.href(), Some(&HRef::from_local("some-curve-id")));
        assert!(property.object().is_none());
    }

    #[test]
    fn serialize_multi_curve_property_writes_gml_tags() {
        let property = make_multi_curve_property();

        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_multi_curve_property(
            &property,
            &mut xml_fragment_writer,
            GmlNamespace::Gml,
            GmlElement::CurveMemberProperty,
        )
        .expect("should serialize");
        let xml = String::from_utf8(xml_fragment_writer.into_bytes()).expect("valid UTF-8");

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

        let index = XmlDocumentIndex::from_scan(xml_document, None).unwrap();
        let property = deserialize_multi_curve_property(
            xml_document,
            &index,
            &crate::util::DeserializationConfig::default(),
        )
        .unwrap();

        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_multi_curve_property(
            &property,
            &mut xml_fragment_writer,
            GmlNamespace::Gml,
            GmlElement::CurveMemberProperty,
        )
        .unwrap();
        let output = String::from_utf8(xml_fragment_writer.into_bytes()).unwrap();

        let spans2 = XmlDocumentIndex::from_scan(output.as_bytes(), None).unwrap();
        let recovered = deserialize_multi_curve_property(
            output.as_bytes(),
            &spans2,
            &crate::util::DeserializationConfig::default(),
        )
        .unwrap();

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

        let index = XmlDocumentIndex::from_scan(xml_document, None).unwrap();
        let property = deserialize_multi_curve_property(
            xml_document,
            &index,
            &crate::util::DeserializationConfig::default(),
        )
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
        assert!(property.object().is_none());
    }

    #[test]
    fn round_trip_preserves_full_association_and_ownership_attributes() {
        let xml_document = b"<gml:curveMember xlink:href=\"#some-id\" xlink:title=\"Some Title\" \
            xlink:role=\"http://example.com/role\" xlink:arcrole=\"http://example.com/arcrole\" \
            xlink:show=\"new\" xlink:actuate=\"onLoad\" gml:owns=\"true\"/>";

        let index = XmlDocumentIndex::from_scan(xml_document, None).unwrap();
        let property = deserialize_multi_curve_property(
            xml_document,
            &index,
            &crate::util::DeserializationConfig::default(),
        )
        .unwrap();

        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_multi_curve_property(
            &property,
            &mut xml_fragment_writer,
            GmlNamespace::Gml,
            GmlElement::CurveMemberProperty,
        )
        .unwrap();
        let output = String::from_utf8(xml_fragment_writer.into_bytes()).unwrap();

        let spans2 = XmlDocumentIndex::from_scan(output.as_bytes(), None).unwrap();
        let recovered = deserialize_multi_curve_property(
            output.as_bytes(),
            &spans2,
            &crate::util::DeserializationConfig::default(),
        )
        .unwrap();

        assert_eq!(
            recovered.association(),
            property.association(),
            "association attributes did not survive the round trip; output was: {output}"
        );
        assert_eq!(recovered.ownership(), property.ownership());
    }

    #[test]
    fn serialize_href_only_property_writes_self_closed_tag() {
        let property = MultiCurveProperty::from_href(HRef::from_local("some-id"));

        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_multi_curve_property(
            &property,
            &mut xml_fragment_writer,
            GmlNamespace::Gml,
            GmlElement::CurveMemberProperty,
        )
        .unwrap();
        let xml = String::from_utf8(xml_fragment_writer.into_bytes()).unwrap();

        assert_eq!(xml, r##"<gml:curveMember xlink:href="#some-id"/>"##);
    }
}
