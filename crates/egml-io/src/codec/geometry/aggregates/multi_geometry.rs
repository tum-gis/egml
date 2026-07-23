use crate::Error;
use crate::codec::geometry::aggregates::{
    deserialize_abstract_geometric_aggregate, serialize_abstract_geometric_aggregate,
};
use crate::codec::geometry::{
    deserialize_abstract_geometry_array_property, deserialize_abstract_geometry_property,
    serialize_abstract_geometry_array_property, serialize_abstract_geometry_property,
};
use crate::util::{
    Formatting, GmlElement, XmlNode, XmlNodeContent, collect_child, collect_children,
    extract_xml_element_spans,
};
use egml_core::model::geometry::aggregates::{AsAbstractGeometricAggregate, MultiGeometry};

pub fn deserialize_multi_geometry(xml_document: &[u8]) -> Result<MultiGeometry, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let abstract_geometric_aggregate =
        deserialize_abstract_geometric_aggregate(xml_document, &spans)?;

    let geometry_member = collect_children(
        xml_document,
        &spans,
        GmlElement::GeometryMemberProperty,
        deserialize_abstract_geometry_property,
    )?;

    let geometry_members = collect_child(
        xml_document,
        &spans,
        GmlElement::GeometryMembersProperty,
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

pub fn serialize_multi_geometry(
    multi_geometry: &MultiGeometry,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut xml_node_parts = serialize_abstract_geometric_aggregate(
        multi_geometry.abstract_geometric_aggregate(),
        formatting,
    )?;

    for member in multi_geometry.geometry_member() {
        xml_node_parts
            .content
            .push(XmlNodeContent::Child(serialize_abstract_geometry_property(
                member,
                formatting,
                GmlElement::GeometryMemberProperty.into(),
            )?));
    }

    if let Some(members) = multi_geometry.geometry_members() {
        xml_node_parts.content.push(XmlNodeContent::Child(
            serialize_abstract_geometry_array_property(
                members,
                formatting,
                GmlElement::GeometryMembersProperty,
            )?,
        ));
    }

    Ok(XmlNode::new(
        GmlElement::MultiGeometry.into(),
        xml_node_parts,
    ))
}

#[cfg(test)]
mod tests {
    use super::{deserialize_multi_geometry, serialize_multi_geometry};
    use crate::util::Formatting;
    use egml_core::model::geometry::AbstractGeometryKind;
    use egml_core::model::geometry::AbstractGeometryProperty;
    use egml_core::model::geometry::DirectPosition;
    use egml_core::model::geometry::aggregates::MultiGeometry;
    use egml_core::model::geometry::primitives::{
        AbstractGeometricPrimitiveKind, AbstractRingKind, AbstractRingProperty,
        AbstractSurfaceKind, LinearRing, Polygon,
    };

    fn make_polygon_member() -> AbstractGeometryProperty {
        let points = vec![
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(0.0, 1.0, 0.0).unwrap(),
        ];
        let ring = AbstractRingKind::LinearRing(LinearRing::new(points).unwrap());
        let polygon = Polygon::new(Some(AbstractRingProperty::from_object(ring)), []).unwrap();
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

        let mg = deserialize_multi_geometry(xml).unwrap();

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

        let mg = deserialize_multi_geometry(xml).unwrap();

        assert!(mg.geometry_member().is_empty());
        assert_eq!(mg.geometry_members().unwrap().objects().len(), 2);
    }

    #[test]
    fn serialize_writes_gml_tags() {
        let mg = make_multi_geometry();
        let xml_node = serialize_multi_geometry(&mg, Formatting::Compact).unwrap();
        let xml = xml_node.to_string(Formatting::Compact).unwrap();

        assert!(xml.contains("<gml:MultiGeometry"));
        assert!(xml.contains("<gml:geometryMember"));
        assert!(xml.contains("<gml:Polygon"));
    }

    #[test]
    fn round_trip_geometry_member_preserves_count() {
        let original = make_multi_geometry();
        let xml_node = serialize_multi_geometry(&original, Formatting::Compact).unwrap();
        let xml = xml_node.to_string(Formatting::Compact).unwrap();
        let recovered = deserialize_multi_geometry(xml.as_bytes()).unwrap();

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

        let first = deserialize_multi_geometry(xml).unwrap();
        let xml_node = serialize_multi_geometry(&first, Formatting::Compact).unwrap();
        let output = xml_node.to_string(Formatting::Compact).unwrap();
        let second = deserialize_multi_geometry(output.as_bytes()).unwrap();

        assert_eq!(
            second.geometry_member().len(),
            first.geometry_member().len()
        );
    }
}
