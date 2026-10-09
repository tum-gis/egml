use crate::Error;
use crate::codec::base::{
    deserialize_association_and_ownership_attributes,
    serialize_association_and_ownership_attributes,
};
use crate::codec::geometry::{
    deserialize_abstract_geometry_kind, serialize_abstract_geometry_kind,
};
use crate::util::{
    DeserializationConfig, GmlElement, XmlDocumentIndex, XmlElement, XmlFragmentWriter,
    XmlNamespace,
};
use egml_core::model::base::{HasAssociationAttributes, HasOwnershipAttributes};
use egml_core::model::geometry::AbstractGeometryProperty;
use std::io::Write;

pub fn deserialize_abstract_geometry_property(
    xml_document: &[u8],
    index: &XmlDocumentIndex<GmlElement>,
    config: &DeserializationConfig,
) -> Result<AbstractGeometryProperty, Error> {
    let index = if index.is_truncated() {
        &XmlDocumentIndex::from_scan(xml_document, config.rescan_depth())?
    } else {
        index
    };
    let (association, ownership) = deserialize_association_and_ownership_attributes(xml_document)?;

    let object = deserialize_abstract_geometry_kind(xml_document, index, config)?;

    Ok(AbstractGeometryProperty::new(
        object,
        association,
        ownership,
    ))
}

pub fn serialize_abstract_geometry_property<N: XmlNamespace, E: XmlElement, W: Write>(
    abstract_geometry_property: &AbstractGeometryProperty,
    xml_fragment_writer: &mut XmlFragmentWriter<W>,
    target_xml_namespace: N,
    target_xml_element: E,
) -> Result<(), Error> {
    let attributes = serialize_association_and_ownership_attributes(
        abstract_geometry_property.association(),
        abstract_geometry_property.ownership(),
    );

    match abstract_geometry_property.object() {
        Some(object) => {
            xml_fragment_writer.write_start_event_with_attributes(
                target_xml_namespace,
                target_xml_element,
                attributes,
            )?;
            serialize_abstract_geometry_kind(object, xml_fragment_writer)?;
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
    use super::{deserialize_abstract_geometry_property, serialize_abstract_geometry_property};
    use crate::util::{Formatting, GmlElement, GmlNamespace, XmlDocumentIndex, XmlFragmentWriter};
    use egml_core::model::base::{HasAssociationAttributes, HasOwnershipAttributes};
    use egml_core::model::geometry::AbstractGeometryProperty;
    use egml_core::model::geometry::DirectPosition;
    use egml_core::model::geometry::aggregates::{AbstractGeometricAggregateKind, MultiPoint};
    use egml_core::model::geometry::primitives::{
        AbstractGeometricPrimitiveKind, AbstractRingKind, AbstractSurfaceKind, LinearRing, Point,
        PointProperty, Polygon,
    };
    use egml_core::model::xlink::{ActuateType, HRef, ShowType};

    fn make_polygon_property() -> AbstractGeometryProperty {
        let points = vec![
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(0.0, 1.0, 0.0).unwrap(),
        ];
        let ring = AbstractRingKind::LinearRing(LinearRing::new(points).unwrap());
        let polygon = Polygon::new(Some(ring), []).unwrap();
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

        let index = XmlDocumentIndex::from_scan(xml, None).unwrap();
        let property = deserialize_abstract_geometry_property(
            xml,
            &index,
            &crate::util::DeserializationConfig::default(),
        )
        .unwrap();

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

        let index = XmlDocumentIndex::from_scan(xml, None).unwrap();
        let property = deserialize_abstract_geometry_property(
            xml,
            &index,
            &crate::util::DeserializationConfig::default(),
        )
        .unwrap();

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

        let index = XmlDocumentIndex::from_scan(xml, None).unwrap();
        let property = deserialize_abstract_geometry_property(
            xml,
            &index,
            &crate::util::DeserializationConfig::default(),
        )
        .unwrap();

        assert_eq!(property.href(), Some(&HRef::from_local("some-id")));
        assert!(property.object().is_none());
    }

    #[test]
    fn serialize_polygon_property() {
        let property = make_polygon_property();
        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_abstract_geometry_property(
            &property,
            &mut xml_fragment_writer,
            GmlNamespace::Gml,
            GmlElement::Polygon,
        )
        .unwrap();
        let xml = String::from_utf8(xml_fragment_writer.into_bytes()).unwrap();

        assert!(xml.contains("<gml:Polygon"));
        assert!(xml.contains("<gml:exterior"));
        assert!(xml.contains("<gml:LinearRing"));
    }

    #[test]
    fn serialize_multi_point_property() {
        let property = make_multi_point_property();
        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_abstract_geometry_property(
            &property,
            &mut xml_fragment_writer,
            GmlNamespace::Gml,
            GmlElement::MultiPoint,
        )
        .unwrap();
        let xml = String::from_utf8(xml_fragment_writer.into_bytes()).unwrap();

        assert!(xml.contains("<gml:MultiPoint"));
        assert!(xml.contains("<gml:pointMember"));
        assert!(xml.contains("<gml:Point"));
    }

    #[test]
    fn deserialize_with_full_association_and_ownership_attributes() {
        let xml = b"<gml:geometryMember xlink:href=\"#some-id\" xlink:title=\"Some Title\" \
            xlink:role=\"http://example.com/role\" xlink:arcrole=\"http://example.com/arcrole\" \
            xlink:show=\"new\" xlink:actuate=\"onLoad\" gml:owns=\"true\"/>";

        let index = XmlDocumentIndex::from_scan(xml, None).unwrap();
        let property = deserialize_abstract_geometry_property(
            xml,
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
    fn round_trip_href_only_property() {
        let xml = b"<gml:geometryMember xlink:href=\"#some-id\"/>";

        let index = XmlDocumentIndex::from_scan(xml, None).unwrap();
        let property = deserialize_abstract_geometry_property(
            xml,
            &index,
            &crate::util::DeserializationConfig::default(),
        )
        .unwrap();
        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_abstract_geometry_property(
            &property,
            &mut xml_fragment_writer,
            GmlNamespace::Gml,
            GmlElement::Polygon,
        )
        .unwrap();
        let output = String::from_utf8(xml_fragment_writer.into_bytes()).unwrap();

        let spans2 = XmlDocumentIndex::from_scan(output.as_bytes(), None).unwrap();
        let recovered = deserialize_abstract_geometry_property(
            output.as_bytes(),
            &spans2,
            &crate::util::DeserializationConfig::default(),
        )
        .unwrap();

        assert_eq!(recovered.association(), property.association());
        assert_eq!(recovered.ownership(), property.ownership());
    }

    #[test]
    fn round_trip_preserves_full_association_and_ownership_attributes() {
        let xml = b"<gml:geometryMember xlink:href=\"#some-id\" xlink:title=\"Some Title\" \
            xlink:role=\"http://example.com/role\" xlink:arcrole=\"http://example.com/arcrole\" \
            xlink:show=\"new\" xlink:actuate=\"onLoad\" gml:owns=\"true\"/>";

        let index = XmlDocumentIndex::from_scan(xml, None).unwrap();
        let property = deserialize_abstract_geometry_property(
            xml,
            &index,
            &crate::util::DeserializationConfig::default(),
        )
        .unwrap();
        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_abstract_geometry_property(
            &property,
            &mut xml_fragment_writer,
            GmlNamespace::Gml,
            GmlElement::Polygon,
        )
        .unwrap();
        let output = String::from_utf8(xml_fragment_writer.into_bytes()).unwrap();

        let spans2 = XmlDocumentIndex::from_scan(output.as_bytes(), None).unwrap();
        let recovered = deserialize_abstract_geometry_property(
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

        let index = XmlDocumentIndex::from_scan(xml, None).unwrap();
        let property = deserialize_abstract_geometry_property(
            xml,
            &index,
            &crate::util::DeserializationConfig::default(),
        )
        .unwrap();
        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_abstract_geometry_property(
            &property,
            &mut xml_fragment_writer,
            GmlNamespace::Gml,
            GmlElement::Polygon,
        )
        .unwrap();
        let output = String::from_utf8(xml_fragment_writer.into_bytes()).unwrap();
        let wrapper = format!("<gml:geometryMember>{output}</gml:geometryMember>");

        let spans2 = XmlDocumentIndex::from_scan(wrapper.as_bytes(), None).unwrap();
        let recovered = deserialize_abstract_geometry_property(
            wrapper.as_bytes(),
            &spans2,
            &crate::util::DeserializationConfig::default(),
        )
        .unwrap();

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

    #[test]
    fn serialize_href_only_property_writes_self_closed_tag() {
        let property = AbstractGeometryProperty::from_href(HRef::from_local("some-id"));

        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_abstract_geometry_property(
            &property,
            &mut xml_fragment_writer,
            GmlNamespace::Gml,
            GmlElement::Polygon,
        )
        .unwrap();
        let xml = String::from_utf8(xml_fragment_writer.into_bytes()).unwrap();

        assert_eq!(xml, r##"<gml:Polygon xlink:href="#some-id"/>"##);
    }
}
