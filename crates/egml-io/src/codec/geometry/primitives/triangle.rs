use crate::Error;
use crate::codec::geometry::primitives::abstract_surface_patch::{
    deserialize_abstract_surface_patch, serialize_abstract_surface_patch,
    serialize_abstract_surface_patch_attributes,
};
use crate::codec::geometry::primitives::{
    deserialize_abstract_ring_property, serialize_abstract_ring_property,
};
use crate::util::{
    DeserializationConfig, GmlElement, GmlNamespace, XmlDocumentIndex, XmlFragmentWriter,
    collect_child,
};
use egml_core::model::geometry::primitives::AsAbstractSurfacePatch;
use egml_core::model::geometry::primitives::Triangle;
use std::io::Write;

pub fn deserialize_triangle(
    xml_document: &[u8],
    index: &XmlDocumentIndex<GmlElement>,
    config: &DeserializationConfig,
) -> Result<Triangle, Error> {
    let index = if index.is_truncated() {
        &XmlDocumentIndex::from_scan(xml_document, config.rescan_depth())?
    } else {
        index
    };

    let abstract_surface_patch = deserialize_abstract_surface_patch(xml_document, index, config)?;

    let exterior = collect_child(
        xml_document,
        index,
        GmlElement::ExteriorProperty,
        config,
        deserialize_abstract_ring_property,
    )?
    .unwrap();

    let triangle = Triangle::from_abstract_surface_patch(abstract_surface_patch, exterior)?;
    Ok(triangle)
}

pub fn serialize_triangle<W: Write>(
    triangle: &Triangle,
    xml_fragment_writer: &mut XmlFragmentWriter<W>,
) -> Result<(), Error> {
    let attributes = serialize_triangle_attributes(triangle);

    xml_fragment_writer.write_start_event_with_attributes(
        GmlNamespace::Gml,
        GmlElement::Triangle,
        attributes,
    )?;

    serialize_abstract_surface_patch(triangle.abstract_surface_patch(), xml_fragment_writer)?;

    serialize_abstract_ring_property(
        triangle.exterior(),
        xml_fragment_writer,
        GmlNamespace::Gml,
        GmlElement::ExteriorProperty,
    )?;

    xml_fragment_writer.write_end_event(GmlNamespace::Gml, GmlElement::Triangle)?;

    Ok(())
}

pub fn serialize_triangle_attributes(triangle: &Triangle) -> Vec<(String, String)> {
    serialize_abstract_surface_patch_attributes(triangle.abstract_surface_patch())
}

#[cfg(test)]
mod tests {
    // Test-only convenience: builds the index the real function now
    // requires, so existing single-argument call sites below don't all
    // need to construct one by hand.
    fn deserialize(xml_document: &[u8]) -> Result<super::Triangle, crate::Error> {
        let index = crate::util::XmlDocumentIndex::from_scan(xml_document, None)?;
        super::deserialize_triangle(
            xml_document,
            &index,
            &crate::util::DeserializationConfig::default(),
        )
    }

    use crate::codec::geometry::primitives::triangle::serialize_triangle;
    use crate::util::{Formatting, XmlFragmentWriter};
    use egml_core::model::geometry::DirectPosition;
    use egml_core::model::geometry::primitives::Triangle;

    fn make_triangle() -> Triangle {
        let a = DirectPosition::new(1.0, 0.0, 0.0).unwrap();
        let b = DirectPosition::new(0.0, 1.0, 0.0).unwrap();
        let c = DirectPosition::new(0.0, 0.0, 1.0).unwrap();
        Triangle::from_points(a, b, c).unwrap()
    }

    #[test]
    fn deserialize_triangle_test() {
        let xml_document = b"
    <gml:Triangle>
        <gml:exterior>
            <gml:LinearRing>
                <gml:posList>354.0249938964844 978.864990234375 2.388849973678589 355.39898681640625 978.8480224609375 2.388849973678589 355.3919982910156 978.8480224609375 2.1084799766540527 354.0249938964844 978.864990234375 2.388849973678589</gml:posList>
            </gml:LinearRing>
        </gml:exterior>
    </gml:Triangle>";

        let triangle: Triangle = deserialize(xml_document.as_ref()).expect("parsing should work");

        assert_eq!(triangle.a().x(), 354.0249938964844);
        assert_eq!(triangle.a().y(), 978.864990234375);
        assert_eq!(triangle.a().z(), 2.388849973678589);
    }

    #[test]
    fn serialize_triangle_writes_gml_tags() {
        let triangle = make_triangle();
        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_triangle(&triangle, &mut xml_fragment_writer).unwrap();
        let xml = String::from_utf8(xml_fragment_writer.into_bytes()).expect("valid UTF-8");

        assert!(xml.contains("<gml:Triangle"));
        assert!(xml.contains("<gml:exterior"));
        assert!(xml.contains("<gml:LinearRing"));
        assert!(xml.contains("<gml:posList"));
        assert!(!xml.contains("id="));
    }

    #[test]
    fn round_trip_triangle_preserves_points() {
        let triangle = make_triangle();
        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_triangle(&triangle, &mut xml_fragment_writer).unwrap();
        let xml = String::from_utf8(xml_fragment_writer.into_bytes()).expect("valid UTF-8");

        let recovered: Triangle = deserialize(xml.as_bytes()).expect("parsing should work");

        assert_eq!(recovered.a().x(), triangle.a().x());
        assert_eq!(recovered.a().y(), triangle.a().y());
        assert_eq!(recovered.a().z(), triangle.a().z());
        assert_eq!(recovered.b().x(), triangle.b().x());
        assert_eq!(recovered.b().y(), triangle.b().y());
        assert_eq!(recovered.b().z(), triangle.b().z());
        assert_eq!(recovered.c().x(), triangle.c().x());
        assert_eq!(recovered.c().y(), triangle.c().y());
        assert_eq!(recovered.c().z(), triangle.c().z());
    }

    #[test]
    fn round_trip_triangle_from_xml() {
        let input_xml = "<gml:Triangle>\
            <gml:exterior><gml:LinearRing><gml:posList srsDimension=\"3\">1 0 0 0 1 0 0 0 1 1 0 0</gml:posList></gml:LinearRing></gml:exterior>\
            </gml:Triangle>";

        let triangle: Triangle = deserialize(input_xml.as_bytes()).unwrap();
        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_triangle(&triangle, &mut xml_fragment_writer).unwrap();
        let output_xml = String::from_utf8(xml_fragment_writer.into_bytes()).unwrap();

        assert_eq!(input_xml, output_xml);
    }

    #[test]
    fn round_trip_triangle_preserves_float_precision() {
        let a =
            DirectPosition::new(354.0249938964844, 978.864990234375, 2.388849973678589).unwrap();
        let b =
            DirectPosition::new(355.39898681640625, 978.8480224609375, 2.388849973678589).unwrap();
        let c =
            DirectPosition::new(355.3919982910156, 978.8480224609375, 2.1084799766540527).unwrap();
        let triangle = Triangle::from_points(a, b, c).unwrap();

        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_triangle(&triangle, &mut xml_fragment_writer).unwrap();
        let xml = String::from_utf8(xml_fragment_writer.into_bytes()).unwrap();

        let recovered = deserialize(xml.as_bytes()).unwrap();

        assert_eq!(recovered.a().x(), a.x());
        assert_eq!(recovered.a().y(), a.y());
        assert_eq!(recovered.a().z(), a.z());
        assert_eq!(recovered.b().x(), b.x());
        assert_eq!(recovered.c().x(), c.x());
    }
}
