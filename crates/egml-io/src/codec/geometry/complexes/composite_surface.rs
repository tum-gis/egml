use crate::Error;
use crate::codec::geometry::primitives::{
    deserialize_abstract_surface, deserialize_abstract_surface_property,
    serialize_abstract_surface, serialize_abstract_surface_attributes,
    serialize_abstract_surface_property,
};
use crate::util::{
    DeserializationConfig, GmlElement, GmlNamespace, XmlDocumentIndex, XmlFragmentWriter,
    collect_children,
};
use egml_core::model::geometry::aggregates::AggregationType;
use egml_core::model::geometry::complexes::CompositeSurface;
use egml_core::model::geometry::primitives::AsAbstractSurface;
use std::io::Write;

pub fn deserialize_composite_surface(
    xml_document: &[u8],
    index: &XmlDocumentIndex<GmlElement>,
    config: &DeserializationConfig,
) -> Result<CompositeSurface, Error> {
    let index = if index.is_truncated() {
        &XmlDocumentIndex::from_scan(xml_document, config.rescan_depth())?
    } else {
        index
    };

    let abstract_surface = deserialize_abstract_surface(xml_document, index, config)?;

    let surface_members = collect_children(
        xml_document,
        index,
        GmlElement::SurfaceMemberProperty,
        config,
        deserialize_abstract_surface_property,
    )?;

    Ok(CompositeSurface::from_abstract_surface(
        abstract_surface,
        surface_members,
        AggregationType::Array,
    )?)
}

pub fn serialize_composite_surface<W: Write>(
    surface: &CompositeSurface,
    xml_fragment_writer: &mut XmlFragmentWriter<W>,
) -> Result<(), Error> {
    let attributes = serialize_composite_surface_attributes(surface);

    xml_fragment_writer.write_start_event_with_attributes(
        GmlNamespace::Gml,
        GmlElement::CompositeSurface,
        attributes,
    )?;

    serialize_abstract_surface(surface.abstract_surface(), xml_fragment_writer)?;

    for member in surface.surface_member() {
        serialize_abstract_surface_property(
            member,
            xml_fragment_writer,
            GmlNamespace::Gml,
            GmlElement::SurfaceMemberProperty,
        )?;
    }

    xml_fragment_writer.write_end_event(GmlNamespace::Gml, GmlElement::CompositeSurface)?;

    Ok(())
}

pub fn serialize_composite_surface_attributes(surface: &CompositeSurface) -> Vec<(String, String)> {
    serialize_abstract_surface_attributes(surface.abstract_surface())
}

#[cfg(test)]
mod tests {
    // Test-only convenience: builds the index the real function now
    // requires, so existing single-argument call sites below don't all
    // need to construct one by hand.
    fn deserialize(xml_document: &[u8]) -> Result<super::CompositeSurface, crate::Error> {
        let index = crate::util::XmlDocumentIndex::from_scan(xml_document, None)?;
        super::deserialize_composite_surface(
            xml_document,
            &index,
            &crate::util::DeserializationConfig::default(),
        )
    }

    use crate::codec::geometry::complexes::composite_surface::serialize_composite_surface;
    use crate::util::{Formatting, XmlFragmentWriter};
    use egml_core::model::base::{AsAbstractGml, AsAbstractGmlMut};
    use egml_core::model::geometry::DirectPosition;
    use egml_core::model::geometry::aggregates::AggregationType;
    use egml_core::model::geometry::complexes::CompositeSurface;
    use egml_core::model::geometry::primitives::{
        AbstractRingKind, AbstractSurfaceKind, AbstractSurfaceProperty, LinearRing, Polygon,
    };

    fn make_composite_surface() -> CompositeSurface {
        let ring = LinearRing::new([
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(0.0, 1.0, 0.0).unwrap(),
        ])
        .unwrap();
        let polygon = Polygon::new(Some(AbstractRingKind::LinearRing(ring)), []).unwrap();
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

        let surface = deserialize(xml_document).expect("should deserialize");

        assert_eq!(surface.surface_member_count(), 3);
    }

    #[test]
    fn serialize_composite_surface_writes_gml_tags() {
        let surface = make_composite_surface();

        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_composite_surface(&surface, &mut xml_fragment_writer).expect("should serialize");
        let xml =
            String::from_utf8(xml_fragment_writer.into_bytes()).expect("to string should work");

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

        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_composite_surface(&surface, &mut xml_fragment_writer).expect("should serialize");
        let xml =
            String::from_utf8(xml_fragment_writer.into_bytes()).expect("to string should work");

        assert!(xml.contains("gml:id=\"test-id\""));
    }

    #[test]
    fn round_trip_composite_surface_preserves_member_count() {
        let surface = make_composite_surface();

        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_composite_surface(&surface, &mut xml_fragment_writer).expect("should serialize");
        let xml =
            String::from_utf8(xml_fragment_writer.into_bytes()).expect("to string should work");

        let recovered = deserialize(xml.as_bytes()).expect("should deserialize");

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

        let surface = deserialize(xml_document).expect("should deserialize");
        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_composite_surface(&surface, &mut xml_fragment_writer).expect("should serialize");
        let output =
            String::from_utf8(xml_fragment_writer.into_bytes()).expect("to string should work");

        let recovered = deserialize(output.as_bytes()).expect("should deserialize");

        assert_eq!(
            recovered.surface_member_count(),
            surface.surface_member_count()
        );
        assert_eq!(recovered.id(), surface.id());
    }
}
