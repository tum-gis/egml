use crate::Error;
use crate::codec::geometry::aggregates::{
    deserialize_abstract_geometric_aggregate, serialize_abstract_geometric_aggregate,
    serialize_abstract_geometric_aggregate_attributes,
};
use crate::codec::geometry::{
    deserialize_abstract_geometry_array_property, deserialize_abstract_geometry_property,
    serialize_abstract_geometry_array_property, serialize_abstract_geometry_property,
};
use crate::util::{
    DeserializationConfig, GmlElement, GmlNamespace, XmlDocumentIndex, XmlFragmentWriter,
    collect_child, collect_children,
};
use egml_core::model::geometry::aggregates::{AsAbstractGeometricAggregate, MultiGeometry};
use std::io::Write;

pub fn deserialize_multi_geometry(
    xml_document: &[u8],
    index: &XmlDocumentIndex<GmlElement>,
    config: &DeserializationConfig,
) -> Result<MultiGeometry, Error> {
    let index = if index.is_truncated() {
        &XmlDocumentIndex::from_scan(xml_document, config.rescan_depth())?
    } else {
        index
    };

    let abstract_geometric_aggregate =
        deserialize_abstract_geometric_aggregate(xml_document, index, config)?;

    let geometry_member = collect_children(
        xml_document,
        index,
        GmlElement::GeometryMemberProperty,
        config,
        deserialize_abstract_geometry_property,
    )?;

    let geometry_members = collect_child(
        xml_document,
        index,
        GmlElement::GeometryMembersProperty,
        config,
        deserialize_abstract_geometry_array_property,
    )?
    .flatten();

    let mut multi_geometry = MultiGeometry::from_abstract_geometric_aggregate(
        abstract_geometric_aggregate,
        geometry_members,
    );
    multi_geometry.set_geometry_member(geometry_member);
    Ok(multi_geometry)
}

pub fn serialize_multi_geometry<W: Write>(
    multi_geometry: &MultiGeometry,
    xml_fragment_writer: &mut XmlFragmentWriter<W>,
) -> Result<(), Error> {
    let attributes = serialize_multi_geometry_attributes(multi_geometry);

    xml_fragment_writer.write_start_event_with_attributes(
        GmlNamespace::Gml,
        GmlElement::MultiGeometry,
        attributes,
    )?;

    serialize_abstract_geometric_aggregate(
        multi_geometry.abstract_geometric_aggregate(),
        xml_fragment_writer,
    )?;

    for member in multi_geometry.geometry_member() {
        serialize_abstract_geometry_property(
            member,
            xml_fragment_writer,
            GmlNamespace::Gml,
            GmlElement::GeometryMemberProperty,
        )?;
    }

    if let Some(members) = multi_geometry.geometry_members() {
        serialize_abstract_geometry_array_property(
            members,
            xml_fragment_writer,
            GmlElement::GeometryMembersProperty,
        )?;
    }

    xml_fragment_writer.write_end_event(GmlNamespace::Gml, GmlElement::MultiGeometry)?;

    Ok(())
}

pub fn serialize_multi_geometry_attributes(
    multi_geometry: &MultiGeometry,
) -> Vec<(String, String)> {
    serialize_abstract_geometric_aggregate_attributes(multi_geometry.abstract_geometric_aggregate())
}

#[cfg(test)]
mod tests {
    // Test-only convenience: builds the index the real function now
    // requires, so existing single-argument call sites below don't all
    // need to construct one by hand.
    fn deserialize(xml_document: &[u8]) -> Result<super::MultiGeometry, crate::Error> {
        let index = crate::util::XmlDocumentIndex::from_scan(xml_document, None)?;
        super::deserialize_multi_geometry(
            xml_document,
            &index,
            &crate::util::DeserializationConfig::default(),
        )
    }

    use super::serialize_multi_geometry;
    use crate::util::{Formatting, XmlFragmentWriter};
    use egml_core::model::geometry::AbstractGeometryKind;
    use egml_core::model::geometry::AbstractGeometryProperty;
    use egml_core::model::geometry::DirectPosition;
    use egml_core::model::geometry::aggregates::MultiGeometry;
    use egml_core::model::geometry::primitives::{
        AbstractGeometricPrimitiveKind, AbstractRingKind, AbstractSurfaceKind, LinearRing, Polygon,
    };

    fn make_polygon_member() -> AbstractGeometryProperty {
        let points = vec![
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(0.0, 1.0, 0.0).unwrap(),
        ];
        let ring = AbstractRingKind::LinearRing(LinearRing::new(points).unwrap());
        let polygon = Polygon::new(Some(ring), []).unwrap();
        AbstractGeometryProperty::from_object(AbstractGeometryKind::AbstractGeometricPrimitiveKind(
            AbstractGeometricPrimitiveKind::AbstractSurfaceKind(AbstractSurfaceKind::Polygon(
                polygon,
            )),
        ))
    }

    fn make_multi_geometry() -> MultiGeometry {
        let mut mg = MultiGeometry::new(None).unwrap();
        mg.set_geometry_member(vec![make_polygon_member(), make_polygon_member()]);
        mg
    }

    #[test]
    fn deserialize_geometry_members() {
        let xml = b"<gml:MultiGeometry>\
            <gml:geometryMember>\
            <gml:Polygon><gml:exterior><gml:LinearRing>\
            <gml:posList srsDimension=\"3\">0 0 0 1 0 0 0 1 0 0 0 0</gml:posList>\
            </gml:LinearRing></gml:exterior></gml:Polygon>\
            </gml:geometryMember>\
            <gml:geometryMember>\
            <gml:Polygon><gml:exterior><gml:LinearRing>\
            <gml:posList srsDimension=\"3\">1 0 0 2 0 0 1 1 0 1 0 0</gml:posList>\
            </gml:LinearRing></gml:exterior></gml:Polygon>\
            </gml:geometryMember>\
            </gml:MultiGeometry>";

        let mg = deserialize(xml).unwrap();

        assert_eq!(mg.geometry_member().len(), 2);
        assert!(mg.geometry_members().is_none());
    }

    #[test]
    fn deserialize_geometry_members_array() {
        let xml = b"<gml:MultiGeometry>\
            <gml:geometryMembers>\
            <gml:Polygon><gml:exterior><gml:LinearRing>\
            <gml:posList srsDimension=\"3\">0 0 0 1 0 0 0 1 0 0 0 0</gml:posList>\
            </gml:LinearRing></gml:exterior></gml:Polygon>\
            <gml:Polygon><gml:exterior><gml:LinearRing>\
            <gml:posList srsDimension=\"3\">1 0 0 2 0 0 1 1 0 1 0 0</gml:posList>\
            </gml:LinearRing></gml:exterior></gml:Polygon>\
            </gml:geometryMembers>\
            </gml:MultiGeometry>";

        let mg = deserialize(xml).unwrap();

        assert!(mg.geometry_member().is_empty());
        assert_eq!(mg.geometry_members().unwrap().objects().len(), 2);
    }

    #[test]
    fn serialize_writes_gml_tags() {
        let mg = make_multi_geometry();
        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_multi_geometry(&mg, &mut xml_fragment_writer).unwrap();
        let xml = String::from_utf8(xml_fragment_writer.into_bytes()).unwrap();

        assert!(xml.contains("<gml:MultiGeometry"));
        assert!(xml.contains("<gml:geometryMember"));
        assert!(xml.contains("<gml:Polygon"));
    }

    #[test]
    fn round_trip_geometry_member_preserves_count() {
        let original = make_multi_geometry();
        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_multi_geometry(&original, &mut xml_fragment_writer).unwrap();
        let xml = String::from_utf8(xml_fragment_writer.into_bytes()).unwrap();
        let recovered = deserialize(xml.as_bytes()).unwrap();

        assert_eq!(
            recovered.geometry_member().len(),
            original.geometry_member().len()
        );
    }

    #[test]
    fn round_trip_from_xml() {
        let xml = b"<gml:MultiGeometry>\
            <gml:geometryMember>\
            <gml:Polygon><gml:exterior><gml:LinearRing>\
            <gml:posList srsDimension=\"3\">0 0 0 1 0 0 0 1 0 0 0 0</gml:posList>\
            </gml:LinearRing></gml:exterior></gml:Polygon>\
            </gml:geometryMember>\
            </gml:MultiGeometry>";

        let first = deserialize(xml).unwrap();
        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_multi_geometry(&first, &mut xml_fragment_writer).unwrap();
        let output = String::from_utf8(xml_fragment_writer.into_bytes()).unwrap();

        let second = deserialize(output.as_bytes()).unwrap();

        assert_eq!(
            second.geometry_member().len(),
            first.geometry_member().len()
        );
    }
}
