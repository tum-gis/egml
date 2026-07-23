use crate::Error;
use crate::codec::geometry::primitives::{
    deserialize_abstract_surface, deserialize_abstract_surface_property,
    serialize_abstract_surface, serialize_abstract_surface_property,
};
use crate::util::{
    Formatting, GmlElement, XmlNode, XmlNodeContent, collect_children, extract_xml_element_spans,
};
use egml_core::model::geometry::aggregates::AggregationType;
use egml_core::model::geometry::complexes::CompositeSurface;
use egml_core::model::geometry::primitives::AsAbstractSurface;

pub fn deserialize_composite_surface(xml_document: &[u8]) -> Result<CompositeSurface, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let abstract_surface = deserialize_abstract_surface(xml_document, &spans)?;

    let surface_members = collect_children(
        xml_document,
        &spans,
        GmlElement::SurfaceMemberProperty,
        deserialize_abstract_surface_property,
    )?;

    Ok(CompositeSurface::from_abstract_surface(
        abstract_surface,
        surface_members,
        AggregationType::Array,
    )?)
}

pub fn serialize_composite_surface(
    surface: &CompositeSurface,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut parts = serialize_abstract_surface(surface.abstract_surface(), formatting)?;

    for member in surface.surface_member() {
        parts
            .content
            .push(XmlNodeContent::Child(serialize_abstract_surface_property(
                member,
                formatting,
                GmlElement::SurfaceMemberProperty.into(),
            )?));
    }

    Ok(XmlNode::new(GmlElement::CompositeSurface.into(), parts))
}

#[cfg(test)]
mod tests {
    use crate::codec::geometry::complexes::composite_surface::{
        deserialize_composite_surface, serialize_composite_surface,
    };
    use crate::util::Formatting;
    use egml_core::model::base::{AsAbstractGml, AsAbstractGmlMut};
    use egml_core::model::geometry::DirectPosition;
    use egml_core::model::geometry::aggregates::AggregationType;
    use egml_core::model::geometry::complexes::CompositeSurface;
    use egml_core::model::geometry::primitives::{
        AbstractRingKind, AbstractRingProperty, AbstractSurfaceKind, AbstractSurfaceProperty,
        LinearRing, Polygon,
    };

    fn make_composite_surface() -> CompositeSurface {
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
        CompositeSurface::new([member], AggregationType::Array).unwrap()
    }

    #[test]
    fn deserialize_composite_surface_test() {
        let xml_document = b"<gml:CompositeSurface>
            <gml:surfaceMember>
                <gml:Polygon>
                    <gml:exterior>
                        <gml:LinearRing>
                            <gml:posList>314.531 1043.46 7.14 314.531 1043.46 2.60 314.688 1043.23 2.60 314.531 1043.46 7.14</gml:posList>
                        </gml:LinearRing>
                    </gml:exterior>
                </gml:Polygon>
            </gml:surfaceMember>
            <gml:surfaceMember>
                <gml:Polygon>
                    <gml:exterior>
                        <gml:LinearRing>
                            <gml:posList>314.531 1043.46 7.14 314.688 1043.23 2.60 315.777 1041.65 2.60 314.531 1043.46 7.14</gml:posList>
                        </gml:LinearRing>
                    </gml:exterior>
                </gml:Polygon>
            </gml:surfaceMember>
            <gml:surfaceMember>
                <gml:Polygon>
                    <gml:exterior>
                        <gml:LinearRing>
                            <gml:posList>314.531 1043.46 7.14 315.777 1041.65 2.60 316.108 1041.17 7.14 314.531 1043.46 7.14</gml:posList>
                        </gml:LinearRing>
                    </gml:exterior>
                </gml:Polygon>
            </gml:surfaceMember>
        </gml:CompositeSurface>";

        let surface = deserialize_composite_surface(xml_document).expect("should deserialize");

        assert_eq!(surface.surface_member_count(), 3);
    }

    #[test]
    fn serialize_composite_surface_writes_gml_tags() {
        let surface = make_composite_surface();

        let xml_node =
            serialize_composite_surface(&surface, Formatting::Compact).expect("should serialize");
        let xml = xml_node
            .to_string(Formatting::Compact)
            .expect("to string should work");

        assert!(xml.contains("<gml:CompositeSurface"));
        assert!(xml.contains("<gml:surfaceMember"));
        assert!(xml.contains("<gml:Polygon"));
        assert!(xml.contains("<gml:exterior"));
        assert!(xml.contains("<gml:LinearRing"));
        assert!(!xml.contains("id="));
    }

    #[test]
    fn serialize_composite_surface_with_id_writes_id() {
        use egml_core::model::base::Id;

        let mut surface = make_composite_surface();
        surface.set_id(Id::try_from("test-id").unwrap());

        let xml_node =
            serialize_composite_surface(&surface, Formatting::Compact).expect("should serialize");
        let xml = xml_node
            .to_string(Formatting::Compact)
            .expect("to string should work");

        assert!(xml.contains("gml:id=\"test-id\""));
    }

    #[test]
    fn round_trip_composite_surface_preserves_member_count() {
        let surface = make_composite_surface();

        let xml_node =
            serialize_composite_surface(&surface, Formatting::Compact).expect("should serialize");
        let xml = xml_node.to_string(Formatting::Compact).unwrap();

        let recovered = deserialize_composite_surface(xml.as_bytes()).expect("should deserialize");

        assert_eq!(
            recovered.surface_member_count(),
            surface.surface_member_count()
        );
    }

    #[test]
    fn round_trip_composite_surface_from_xml() {
        let xml_document = b"<gml:CompositeSurface gml:id=\"test-id\">\
            <gml:surfaceMember><gml:Polygon><gml:exterior><gml:LinearRing>\
            <gml:posList srsDimension=\"3\">0 0 0 1 0 0 0 1 0 0 0 0</gml:posList>\
            </gml:LinearRing></gml:exterior></gml:Polygon></gml:surfaceMember>\
            </gml:CompositeSurface>";

        let surface = deserialize_composite_surface(xml_document).expect("should deserialize");
        let xml_node =
            serialize_composite_surface(&surface, Formatting::Compact).expect("should serialize");
        let output = xml_node.to_string(Formatting::Compact).unwrap();

        let recovered =
            deserialize_composite_surface(output.as_bytes()).expect("should deserialize");

        assert_eq!(
            recovered.surface_member_count(),
            surface.surface_member_count()
        );
        assert_eq!(recovered.id(), surface.id());
    }
}
