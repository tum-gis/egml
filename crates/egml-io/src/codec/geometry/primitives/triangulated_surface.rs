use crate::Error;
use crate::codec::geometry::primitives::{deserialize_surface, serialize_surface};
use crate::util::{Formatting, GmlElement, XmlNode};
use egml_core::model::geometry::primitives::{AsSurface, TriangulatedSurface};

pub fn deserialize_triangulated_surface(xml_document: &[u8]) -> Result<TriangulatedSurface, Error> {
    let surface = deserialize_surface(xml_document)?;

    let triangulated_surface = TriangulatedSurface::new(surface)?;
    Ok(triangulated_surface)
}

pub fn serialize_triangulated_surface(
    triangulated_surface: &TriangulatedSurface,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut xml_node = serialize_surface(triangulated_surface.surface(), formatting)?;
    xml_node.name = GmlElement::TriangulatedSurface.into();

    Ok(xml_node)
}

#[cfg(test)]
mod tests {
    use crate::codec::geometry::primitives::serialize_triangulated_surface;
    use crate::codec::geometry::primitives::triangulated_surface::deserialize_triangulated_surface;
    use crate::util::Formatting;
    use egml_core::model::geometry::DirectPosition;
    use egml_core::model::geometry::primitives::{AsSurface, Triangle, TriangulatedSurface};

    fn make_triangulated_surface() -> TriangulatedSurface {
        let t1 = Triangle::from_points(
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(0.0, 1.0, 0.0).unwrap(),
        )
        .unwrap();
        let t2 = Triangle::from_points(
            DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 1.0, 0.0).unwrap(),
            DirectPosition::new(0.0, 1.0, 0.0).unwrap(),
        )
        .unwrap();
        TriangulatedSurface::from_triangles(vec![t1, t2]).unwrap()
    }

    #[test]
    fn deserialize_triangulated_surface_with_three_triangles() {
        let xml_document = b"
        <gml:TriangulatedSurface srsDimension=\"3\">
          <gml:patches>
            <gml:Triangle>
              <gml:exterior>
                <gml:LinearRing>
                  <gml:posList>-76.10530090332031 3.262645959854126 -0.023333795368671417 -76.19206237792969 3.5082428455352783 0.0 -77.13458251953125 3.1704795360565186 0.0 -76.10530090332031 3.262645959854126 -0.023333795368671417</gml:posList>
                </gml:LinearRing>
              </gml:exterior>
            </gml:Triangle>
            <gml:Triangle>
              <gml:exterior>
                <gml:LinearRing>
                  <gml:posList>-76.10530090332031 3.262645959854126 -0.023333795368671417 -77.13458251953125 3.1704795360565186 0.0 -77.04559326171875 2.925680637359619 -0.023333795368671417 -76.10530090332031 3.262645959854126 -0.023333795368671417</gml:posList>
                </gml:LinearRing>
              </gml:exterior>
            </gml:Triangle>
            <gml:Triangle>
              <gml:exterior>
                <gml:LinearRing>
                  <gml:posList>-66.17540740966797 6.251502513885498 -0.023333795368671417 -66.23863220214844 6.504178047180176 0.0 -71.25069427490234 5.123663902282715 0.0 -66.17540740966797 6.251502513885498 -0.023333795368671417</gml:posList>
                </gml:LinearRing>
              </gml:exterior>
            </gml:Triangle>
          </gml:patches>
        </gml:TriangulatedSurface>";

        let triangulated_surface: TriangulatedSurface =
            deserialize_triangulated_surface(xml_document).unwrap();

        assert_eq!(triangulated_surface.patches_len(), 3);
    }

    #[test]
    fn deserialize_deprecated_triangulated_surface_with_three_triangles() {
        let xml_document = b"
        <gml:TriangulatedSurface srsDimension=\"3\">
          <gml:trianglePatches>
            <gml:Triangle>
              <gml:exterior>
                <gml:LinearRing>
                  <gml:posList>-76.10530090332031 3.262645959854126 -0.023333795368671417 -76.19206237792969 3.5082428455352783 0.0 -77.13458251953125 3.1704795360565186 0.0 -76.10530090332031 3.262645959854126 -0.023333795368671417</gml:posList>
                </gml:LinearRing>
              </gml:exterior>
            </gml:Triangle>
          </gml:trianglePatches>
        </gml:TriangulatedSurface>";

        let triangulated_surface: TriangulatedSurface =
            deserialize_triangulated_surface(xml_document).unwrap();

        assert_eq!(triangulated_surface.patches_len(), 1);
    }

    #[test]
    fn serialize_triangulated_surface_writes_gml_tags() {
        let surface = make_triangulated_surface();
        let xml_node = serialize_triangulated_surface(&surface, Formatting::Compact).unwrap();
        let xml = xml_node.to_string(Formatting::Compact).unwrap();

        assert!(xml.contains("<gml:TriangulatedSurface"));
        assert!(xml.contains("<gml:patches"));
        assert!(xml.contains("<gml:Triangle"));
        assert!(xml.contains("<gml:posList"));
        assert!(!xml.contains("id="));
    }

    #[test]
    fn round_trip_triangulated_surface_preserves_patch_count() {
        let surface = make_triangulated_surface();
        let xml_node = serialize_triangulated_surface(&surface, Formatting::Compact).unwrap();
        let xml = xml_node.to_string(Formatting::Compact).unwrap();

        let recovered = deserialize_triangulated_surface(xml.as_bytes()).unwrap();

        assert_eq!(recovered.patches_len(), surface.patches_len());
    }

    #[test]
    fn round_trip_triangulated_surface_from_xml() {
        let input_xml = "<gml:TriangulatedSurface gml:id=\"my-id\">\
            <gml:patches>\
            <gml:Triangle><gml:exterior><gml:LinearRing><gml:posList srsDimension=\"3\">0 0 0 1 0 0 0 1 0 0 0 0</gml:posList></gml:LinearRing></gml:exterior></gml:Triangle>\
            <gml:Triangle><gml:exterior><gml:LinearRing><gml:posList srsDimension=\"3\">1 0 0 1 1 0 0 1 0 1 0 0</gml:posList></gml:LinearRing></gml:exterior></gml:Triangle>\
            </gml:patches>\
            </gml:TriangulatedSurface>";

        let triangulated_surface: TriangulatedSurface =
            deserialize_triangulated_surface(input_xml.as_ref()).unwrap();
        let output_xml_node =
            serialize_triangulated_surface(&triangulated_surface, Formatting::Compact).unwrap();
        let output_xml = output_xml_node.to_string(Formatting::Compact).unwrap();

        assert_eq!(input_xml, output_xml);
    }

    #[test]
    fn round_trip_triangulated_surface_preserves_float_precision() {
        let t1 = Triangle::from_points(
            DirectPosition::new(-76.10530090332031, 3.262645959854126, -0.023333795368671417)
                .unwrap(),
            DirectPosition::new(-76.19206237792969, 3.5082428455352783, 0.0).unwrap(),
            DirectPosition::new(-77.13458251953125, 3.1704795360565186, 0.0).unwrap(),
        )
        .unwrap();
        let surface = TriangulatedSurface::from_triangles(vec![t1]).unwrap();

        let xml_node = serialize_triangulated_surface(&surface, Formatting::Compact).unwrap();
        let xml = xml_node.to_string(Formatting::Compact).unwrap();
        let recovered = deserialize_triangulated_surface(xml.as_bytes()).unwrap();

        let orig_triangles = surface.triangles();
        let rec_triangles = recovered.triangles();
        assert_eq!(orig_triangles.len(), rec_triangles.len());
        assert_eq!(orig_triangles[0].a().x(), rec_triangles[0].a().x());
        assert_eq!(orig_triangles[0].a().y(), rec_triangles[0].a().y());
        assert_eq!(orig_triangles[0].a().z(), rec_triangles[0].a().z());
    }
}
