use crate::Error;
use crate::codec::base::{deserialize_ownership_attributes, serialize_ownership_attributes};
use crate::codec::geometry::{
    deserialize_abstract_geometry_kind_for, serialize_abstract_geometry_kind,
};
use crate::util::{
    DeserializationConfig, GmlElement, GmlNamespace, XmlDocumentIndex, XmlFragmentWriter,
};
use egml_core::model::base::HasOwnershipAttributes;
use egml_core::model::geometry::{AbstractGeometryArrayProperty, AbstractGeometryKind};
use std::io::Write;

pub fn deserialize_abstract_geometry_array_property(
    xml_document: &[u8],
    index: &XmlDocumentIndex<GmlElement>,
    config: &DeserializationConfig,
) -> Result<Option<AbstractGeometryArrayProperty>, Error> {
    let index = if index.is_truncated() {
        &XmlDocumentIndex::from_scan(xml_document, config.rescan_depth())?
    } else {
        index
    };

    let ownership = deserialize_ownership_attributes(xml_document)?;

    let mut all_children: Vec<(GmlElement, &XmlDocumentIndex<GmlElement>)> = index
        .children()
        .iter()
        .flat_map(|(elem, nodes)| nodes.iter().map(move |node| (*elem, node)))
        .collect();
    all_children.sort_by_key(|(_, node)| node.range().start);

    let objects: Vec<AbstractGeometryKind> = all_children
        .iter()
        .filter_map(|(elem, node)| {
            let slice = &xml_document[node.range()];
            deserialize_abstract_geometry_kind_for(*elem, slice, node, config).transpose()
        })
        .collect::<Result<_, _>>()?;

    if objects.is_empty() {
        return Ok(None);
    }

    Ok(Some(AbstractGeometryArrayProperty::new(objects, ownership)))
}

pub fn serialize_abstract_geometry_array_property<W: Write>(
    abstract_geometry_array_property: &AbstractGeometryArrayProperty,
    xml_fragment_writer: &mut XmlFragmentWriter<W>,
    target_xml_element: GmlElement,
) -> Result<(), Error> {
    let attributes = serialize_ownership_attributes(abstract_geometry_array_property.ownership());

    xml_fragment_writer.write_start_event_with_attributes(
        GmlNamespace::Gml,
        target_xml_element,
        attributes,
    )?;

    for object in abstract_geometry_array_property.objects() {
        serialize_abstract_geometry_kind(object, xml_fragment_writer)?;
    }

    xml_fragment_writer.write_end_event(GmlNamespace::Gml, target_xml_element)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        deserialize_abstract_geometry_array_property, serialize_abstract_geometry_array_property,
    };
    use crate::util::{Formatting, GmlElement, XmlDocumentIndex, XmlFragmentWriter};
    use egml_core::model::base::HasOwnershipAttributes;
    use egml_core::model::geometry::AbstractGeometryArrayProperty;
    use egml_core::model::geometry::AbstractGeometryKind;
    use egml_core::model::geometry::DirectPosition;
    use egml_core::model::geometry::aggregates::{AbstractGeometricAggregateKind, MultiPoint};
    use egml_core::model::geometry::primitives::{
        AbstractGeometricPrimitiveKind, AbstractRingKind, AbstractSurfaceKind, LinearRing, Point,
        PointProperty, Polygon,
    };

    fn make_polygon_kind() -> AbstractGeometryKind {
        let points = vec![
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(0.0, 1.0, 0.0).unwrap(),
        ];
        let ring = AbstractRingKind::LinearRing(LinearRing::new(points).unwrap());
        let polygon = Polygon::new(Some(ring), []).unwrap();
        AbstractGeometryKind::AbstractGeometricPrimitiveKind(
            AbstractGeometricPrimitiveKind::AbstractSurfaceKind(AbstractSurfaceKind::Polygon(
                polygon,
            )),
        )
    }

    fn make_multi_point_kind() -> AbstractGeometryKind {
        let mut mp = MultiPoint::new(None).unwrap();
        mp.set_point_member(vec![PointProperty::from_object(Point::new(
            DirectPosition::new(1.0, 2.0, 3.0).unwrap(),
        ))]);
        AbstractGeometryKind::AbstractGeometricAggregateKind(
            AbstractGeometricAggregateKind::MultiPoint(mp),
        )
    }

    #[test]
    fn deserialize_with_polygons() {
        let xml = b"<gml:geometryMembers>\
            <gml:Polygon><gml:exterior><gml:LinearRing>\
            <gml:posList srsDimension=\"3\">0 0 0 1 0 0 0 1 0 0 0 0</gml:posList>\
            </gml:LinearRing></gml:exterior></gml:Polygon>\
            <gml:Polygon><gml:exterior><gml:LinearRing>\
            <gml:posList srsDimension=\"3\">1 0 0 2 0 0 1 1 0 1 0 0</gml:posList>\
            </gml:LinearRing></gml:exterior></gml:Polygon>\
            </gml:geometryMembers>";

        let index = XmlDocumentIndex::from_scan(xml, None).unwrap();
        let property = deserialize_abstract_geometry_array_property(
            xml,
            &index,
            &crate::util::DeserializationConfig::default(),
        )
        .unwrap()
        .unwrap();

        assert_eq!(property.objects().len(), 2);
        assert!(matches!(
            property.objects()[0],
            AbstractGeometryKind::AbstractGeometricPrimitiveKind(
                AbstractGeometricPrimitiveKind::AbstractSurfaceKind(AbstractSurfaceKind::Polygon(
                    _
                ))
            )
        ));
    }

    #[test]
    fn deserialize_with_multi_point() {
        let xml = b"<gml:geometryMembers>\
            <gml:MultiPoint>\
            <gml:pointMember><gml:Point><gml:pos srsDimension=\"3\">1 2 3</gml:pos></gml:Point></gml:pointMember>\
            </gml:MultiPoint>\
            </gml:geometryMembers>";

        let index = XmlDocumentIndex::from_scan(xml, None).unwrap();
        let property = deserialize_abstract_geometry_array_property(
            xml,
            &index,
            &crate::util::DeserializationConfig::default(),
        )
        .unwrap()
        .unwrap();

        assert_eq!(property.objects().len(), 1);
        assert!(matches!(
            property.objects()[0],
            AbstractGeometryKind::AbstractGeometricAggregateKind(
                AbstractGeometricAggregateKind::MultiPoint(_)
            )
        ));
    }

    #[test]
    fn deserialize_empty_returns_none() {
        let xml = b"<gml:geometryMembers/>";

        let index = XmlDocumentIndex::from_scan(xml, None).unwrap();
        let property = deserialize_abstract_geometry_array_property(
            xml,
            &index,
            &crate::util::DeserializationConfig::default(),
        )
        .unwrap();

        assert!(property.is_none());
    }

    #[test]
    fn serialize_multiple_objects() {
        let property = AbstractGeometryArrayProperty::from_objects(vec![
            make_polygon_kind(),
            make_multi_point_kind(),
        ]);
        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_abstract_geometry_array_property(
            &property,
            &mut xml_fragment_writer,
            GmlElement::PointMembersProperty,
        )
        .unwrap();
        let xml = String::from_utf8(xml_fragment_writer.into_bytes()).unwrap();

        assert!(xml.contains("<gml:Polygon"));
        assert!(xml.contains("<gml:MultiPoint"));
    }

    #[test]
    fn round_trip_polygons_preserves_count() {
        let xml = b"<gml:geometryMembers>\
            <gml:Polygon><gml:exterior><gml:LinearRing>\
            <gml:posList srsDimension=\"3\">0 0 0 1 0 0 0 1 0 0 0 0</gml:posList>\
            </gml:LinearRing></gml:exterior></gml:Polygon>\
            <gml:Polygon><gml:exterior><gml:LinearRing>\
            <gml:posList srsDimension=\"3\">1 0 0 2 0 0 1 1 0 1 0 0</gml:posList>\
            </gml:LinearRing></gml:exterior></gml:Polygon>\
            </gml:geometryMembers>";

        let index = XmlDocumentIndex::from_scan(xml, None).unwrap();
        let property = deserialize_abstract_geometry_array_property(
            xml,
            &index,
            &crate::util::DeserializationConfig::default(),
        )
        .unwrap()
        .unwrap();

        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_abstract_geometry_array_property(
            &property,
            &mut xml_fragment_writer,
            GmlElement::PointMembersProperty,
        )
        .unwrap();
        let output = String::from_utf8(xml_fragment_writer.into_bytes()).unwrap();

        let spans2 = XmlDocumentIndex::from_scan(output.as_bytes(), None).unwrap();
        let recovered = deserialize_abstract_geometry_array_property(
            output.as_bytes(),
            &spans2,
            &crate::util::DeserializationConfig::default(),
        )
        .unwrap()
        .unwrap();

        assert_eq!(recovered.objects().len(), property.objects().len());
    }

    #[test]
    fn deserialize_keeps_ownership_and_ignores_xlink_attributes() {
        // gml:ArrayAssociationType declares only gml:OwnershipAttributeGroup.
        let xml = b"<gml:geometryMembers xlink:href=\"#some-id\" xlink:title=\"Some Title\" \
            xlink:role=\"http://example.com/role\" xlink:arcrole=\"http://example.com/arcrole\" \
            xlink:show=\"new\" xlink:actuate=\"onLoad\" gml:owns=\"true\">\
            <gml:Polygon><gml:exterior><gml:LinearRing>\
            <gml:posList srsDimension=\"3\">0 0 0 1 0 0 0 1 0 0 0 0</gml:posList>\
            </gml:LinearRing></gml:exterior></gml:Polygon>\
            </gml:geometryMembers>";

        let index = XmlDocumentIndex::from_scan(xml, None).unwrap();
        let property = deserialize_abstract_geometry_array_property(
            xml,
            &index,
            &crate::util::DeserializationConfig::default(),
        )
        .unwrap()
        .unwrap();

        assert_eq!(property.objects().len(), 1);
        assert!(property.owns());
    }

    #[test]
    fn round_trip_keeps_ownership_and_drops_xlink_attributes() {
        let xml = b"<gml:geometryMembers xlink:href=\"#some-id\" xlink:title=\"Some Title\" \
            xlink:role=\"http://example.com/role\" xlink:arcrole=\"http://example.com/arcrole\" \
            xlink:show=\"new\" xlink:actuate=\"onLoad\" gml:owns=\"true\">\
            <gml:Polygon><gml:exterior><gml:LinearRing>\
            <gml:posList srsDimension=\"3\">0 0 0 1 0 0 0 1 0 0 0 0</gml:posList>\
            </gml:LinearRing></gml:exterior></gml:Polygon>\
            </gml:geometryMembers>";

        let index = XmlDocumentIndex::from_scan(xml, None).unwrap();
        let property = deserialize_abstract_geometry_array_property(
            xml,
            &index,
            &crate::util::DeserializationConfig::default(),
        )
        .unwrap()
        .unwrap();

        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_abstract_geometry_array_property(
            &property,
            &mut xml_fragment_writer,
            GmlElement::PointMembersProperty,
        )
        .unwrap();
        let output = String::from_utf8(xml_fragment_writer.into_bytes()).unwrap();

        let spans2 = XmlDocumentIndex::from_scan(output.as_bytes(), None).unwrap();
        let recovered = deserialize_abstract_geometry_array_property(
            output.as_bytes(),
            &spans2,
            &crate::util::DeserializationConfig::default(),
        )
        .unwrap()
        .unwrap();

        assert!(
            !output.contains("xlink"),
            "unexpected xlink attribute in {output}"
        );
        assert!(recovered.owns());
    }
}
