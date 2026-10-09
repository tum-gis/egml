use crate::Error;
use crate::codec::base::{
    deserialize_association_and_ownership_attributes,
    serialize_association_and_ownership_attributes,
};
use crate::codec::geometry::primitives::{
    deserialize_abstract_curve_kind, serialize_abstract_curve_kind,
};
use crate::util::{
    DeserializationConfig, GmlElement, XmlDocumentIndex, XmlElement, XmlFragmentWriter,
    XmlNamespace,
};
use egml_core::model::base::{HasAssociationAttributes, HasOwnershipAttributes};
use egml_core::model::geometry::primitives::AbstractCurveProperty;
use std::io::Write;

pub fn deserialize_abstract_curve_property(
    xml_document: &[u8],
    index: &XmlDocumentIndex<GmlElement>,
    config: &DeserializationConfig,
) -> Result<AbstractCurveProperty, Error> {
    let index = if index.is_truncated() {
        &XmlDocumentIndex::from_scan(xml_document, config.rescan_depth())?
    } else {
        index
    };

    let (association, ownership) = deserialize_association_and_ownership_attributes(xml_document)?;
    let object = deserialize_abstract_curve_kind(xml_document, index, config)?;

    Ok(AbstractCurveProperty::new(object, association, ownership))
}

pub fn serialize_abstract_curve_property<N: XmlNamespace, E: XmlElement, W: Write>(
    abstract_curve_property: &AbstractCurveProperty,
    xml_fragment_writer: &mut XmlFragmentWriter<W>,
    target_xml_namespace: N,
    target_xml_element: E,
) -> Result<(), Error> {
    let attributes = serialize_association_and_ownership_attributes(
        abstract_curve_property.association(),
        abstract_curve_property.ownership(),
    );

    match abstract_curve_property.object() {
        Some(abstract_curve_kind) => {
            xml_fragment_writer.write_start_event_with_attributes(
                target_xml_namespace,
                target_xml_element,
                attributes,
            )?;
            serialize_abstract_curve_kind(abstract_curve_kind, xml_fragment_writer)?;
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
    use crate::codec::geometry::primitives::{
        deserialize_abstract_curve_property, serialize_abstract_curve_property,
    };
    use crate::util::{Formatting, GmlElement, GmlNamespace, XmlDocumentIndex, XmlFragmentWriter};
    use egml_core::model::base::{HasAssociationAttributes, HasOwnershipAttributes};
    use egml_core::model::geometry::primitives::{AbstractCurveKind, AbstractCurveProperty};
    use egml_core::model::xlink::{ActuateType, HRef, ShowType};

    #[test]
    fn deserialize_with_line_string() {
        let xml_document = b"<gml:curveMember>\
            <gml:LineString><gml:posList srsDimension=\"3\">0 0 0 1 1 1 2 2 2</gml:posList></gml:LineString>\
            </gml:curveMember>";

        let index = XmlDocumentIndex::from_scan(xml_document, None).unwrap();
        let property = deserialize_abstract_curve_property(
            xml_document,
            &index,
            &crate::util::DeserializationConfig::default(),
        )
        .unwrap();

        assert!(matches!(
            property.object(),
            Some(AbstractCurveKind::LineString(_))
        ));
    }

    #[test]
    fn deserialize_with_xlink() {
        let xml_document = b"<gml:curveMember xlink:href=\"#some-id\"/>";

        let index = XmlDocumentIndex::from_scan(xml_document, None).unwrap();
        let property = deserialize_abstract_curve_property(
            xml_document,
            &index,
            &crate::util::DeserializationConfig::default(),
        )
        .unwrap();

        assert_eq!(property.href(), Some(&HRef::from_local("some-id")));
        assert!(property.object().is_none());
    }

    #[test]
    fn round_trip_line_string_preserves_points() {
        let xml_document = b"<gml:curveMember>\
            <gml:LineString><gml:posList srsDimension=\"3\">0 0 0 1 1 1 2 2 2</gml:posList></gml:LineString>\
            </gml:curveMember>";

        let index = XmlDocumentIndex::from_scan(xml_document, None).unwrap();
        let property = deserialize_abstract_curve_property(
            xml_document,
            &index,
            &crate::util::DeserializationConfig::default(),
        )
        .unwrap();

        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_abstract_curve_property(
            &property,
            &mut xml_fragment_writer,
            GmlNamespace::Gml,
            GmlElement::CurveMemberProperty,
        )
        .unwrap();
        let output = String::from_utf8(xml_fragment_writer.into_bytes()).unwrap();

        let spans2 = XmlDocumentIndex::from_scan(output.as_bytes(), None).unwrap();
        let recovered = deserialize_abstract_curve_property(
            output.as_bytes(),
            &spans2,
            &crate::util::DeserializationConfig::default(),
        )
        .unwrap();

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

        let index = XmlDocumentIndex::from_scan(xml_document, None).unwrap();
        let property = deserialize_abstract_curve_property(
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
        let property = deserialize_abstract_curve_property(
            xml_document,
            &index,
            &crate::util::DeserializationConfig::default(),
        )
        .unwrap();

        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_abstract_curve_property(
            &property,
            &mut xml_fragment_writer,
            GmlNamespace::Gml,
            GmlElement::CurveMemberProperty,
        )
        .unwrap();
        let output = String::from_utf8(xml_fragment_writer.into_bytes()).unwrap();

        let spans2 = XmlDocumentIndex::from_scan(output.as_bytes(), None).unwrap();
        let recovered = deserialize_abstract_curve_property(
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
        let property = AbstractCurveProperty::from_href(HRef::from_local("some-id"));

        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_abstract_curve_property(
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
