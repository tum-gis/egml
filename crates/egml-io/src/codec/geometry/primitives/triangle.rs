use crate::Error;
use crate::codec::geometry::primitives::abstract_surface_patch::{
    deserialize_abstract_surface_patch, serialize_abstract_surface_patch,
};
use crate::codec::geometry::primitives::{
    deserialize_abstract_ring_property, serialize_abstract_ring_property,
};
use crate::util::{
    Formatting, GmlElement, XmlNode, XmlNodeContent, collect_child, extract_xml_element_spans,
};
use egml_core::model::geometry::primitives::AsAbstractSurfacePatch;
use egml_core::model::geometry::primitives::Triangle;

pub fn deserialize_triangle(xml_document: &[u8]) -> Result<Triangle, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let abstract_surface_patch = deserialize_abstract_surface_patch(xml_document, &spans)?;

    let exterior = collect_child(
        xml_document,
        &spans,
        GmlElement::ExteriorProperty,
        deserialize_abstract_ring_property,
    )?
    .unwrap();

    let triangle = Triangle::from_abstract_surface_patch(abstract_surface_patch, exterior)?;
    Ok(triangle)
}

pub fn serialize_triangle(triangle: &Triangle, formatting: Formatting) -> Result<XmlNode, Error> {
    let mut xml_node_parts =
        serialize_abstract_surface_patch(triangle.abstract_surface_patch(), formatting)?;

    xml_node_parts
        .content
        .push(XmlNodeContent::Child(serialize_abstract_ring_property(
            triangle.exterior(),
            formatting,
            GmlElement::ExteriorProperty.into(),
        )?));

    Ok(XmlNode::new(GmlElement::Triangle.into(), xml_node_parts))
}

#[cfg(test)]
mod tests {
    use crate::codec::geometry::primitives::deserialize_triangle;
    use crate::codec::geometry::primitives::triangle::serialize_triangle;
    use crate::util::Formatting;
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

        let triangle: Triangle =
            deserialize_triangle(xml_document.as_ref()).expect("parsing should work");

        assert_eq!(triangle.a().x(), 354.0249938964844);
        assert_eq!(triangle.a().y(), 978.864990234375);
        assert_eq!(triangle.a().z(), 2.388849973678589);
    }

    #[test]
    fn serialize_triangle_writes_gml_tags() {
        let triangle = make_triangle();
        let xml_node = serialize_triangle(&triangle, Formatting::Compact).unwrap();
        let xml = xml_node
            .to_string(Formatting::Compact)
            .expect("serialization should work");

        assert!(xml.contains("<gml:Triangle"));
        assert!(xml.contains("<gml:exterior"));
        assert!(xml.contains("<gml:LinearRing"));
        assert!(xml.contains("<gml:posList"));
        assert!(!xml.contains("id="));
    }

    #[test]
    fn round_trip_triangle_preserves_points() {
        let triangle = make_triangle();
        let xml_node = serialize_triangle(&triangle, Formatting::Compact).unwrap();
        let xml = xml_node
            .to_string(Formatting::Compact)
            .expect("serialization should work");

        let recovered: Triangle =
            deserialize_triangle(xml.as_bytes()).expect("parsing should work");

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

        let triangle: Triangle = deserialize_triangle(input_xml.as_bytes()).unwrap();
        let output_xml_node = serialize_triangle(&triangle, Formatting::Compact).unwrap();
        let output_xml = output_xml_node.to_string(Formatting::Compact).unwrap();

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

        let xml_node = serialize_triangle(&triangle, Formatting::Compact).unwrap();
        let xml = xml_node.to_string(Formatting::Compact).unwrap();

        let recovered = deserialize_triangle(xml.as_bytes()).unwrap();

        assert_eq!(recovered.a().x(), a.x());
        assert_eq!(recovered.a().y(), a.y());
        assert_eq!(recovered.a().z(), a.z());
        assert_eq!(recovered.b().x(), b.x());
        assert_eq!(recovered.c().x(), c.x());
    }
}
