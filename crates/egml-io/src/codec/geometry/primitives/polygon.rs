use crate::Error;
use crate::codec::geometry::primitives::abstract_surface::{
    deserialize_abstract_surface, serialize_abstract_surface,
};
use crate::codec::geometry::primitives::{
    deserialize_abstract_ring_property, serialize_abstract_ring_property,
};
use crate::util::{
    Formatting, GmlElement, XmlNode, XmlNodeContent, collect_child, collect_children,
    extract_xml_element_spans,
};
use egml_core::model::geometry::primitives::{AbstractRingProperty, AsAbstractSurface, Polygon};

pub fn deserialize_polygon(xml_document: &[u8]) -> Result<Polygon, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let abstract_surface = deserialize_abstract_surface(xml_document, &spans)?;

    let exterior = collect_child(
        xml_document,
        &spans,
        GmlElement::ExteriorProperty,
        deserialize_abstract_ring_property,
    )?;
    let interior: Vec<AbstractRingProperty> = collect_children(
        xml_document,
        &spans,
        GmlElement::InteriorProperty,
        deserialize_abstract_ring_property,
    )?;

    let polygon = Polygon::from_abstract_surface(abstract_surface, exterior, interior);
    Ok(polygon)
}

pub fn serialize_polygon(polygon: &Polygon, formatting: Formatting) -> Result<XmlNode, Error> {
    let mut xml_node_parts = serialize_abstract_surface(polygon.abstract_surface(), formatting)?;

    if let Some(object) = &polygon.exterior() {
        xml_node_parts
            .content
            .push(XmlNodeContent::Child(serialize_abstract_ring_property(
                object,
                formatting,
                GmlElement::ExteriorProperty.into(),
            )?));
    }
    for prop in polygon.interior() {
        xml_node_parts
            .content
            .push(XmlNodeContent::Child(serialize_abstract_ring_property(
                prop,
                formatting,
                GmlElement::InteriorProperty.into(),
            )?));
    }

    Ok(XmlNode::new(GmlElement::Polygon.into(), xml_node_parts))
}

#[cfg(test)]
mod tests {
    use crate::codec::geometry::primitives::deserialize_polygon;
    use crate::codec::geometry::primitives::polygon::serialize_polygon;
    use crate::util::Formatting;
    use egml_core::model::common::Triangulate;
    use egml_core::model::geometry::DirectPosition;
    use egml_core::model::geometry::primitives::{
        AbstractRingKind, AbstractRingProperty, LinearRing, Polygon,
    };

    /// Real CityGML LOD2 polygon (`DEBY_LOD2_4906980_BP...K8MLz6HFMDUSAU80ScjF`): a
    /// near-circular annulus made of ~58 exterior and ~58 interior points. Regression
    /// coverage for `earcut` handling a legitimately complex real-world shape with an
    /// interior ring without failing.
    #[test]
    fn triangulate_real_world_donut_polygon() {
        let xml = br#"<gml:Polygon gml:id="PG.K8MLz6HFMDUSAU80ScjF">
          <gml:exterior>
            <gml:LinearRing gml:id="LR.i9W3NKmpTBXPNanIC7nX">
              <gml:posList>691002.452 5335973.142 517.932 691002.451 5335973.142 517.932 691002.45 5335973.143 517.931 691002.449 5335973.143 517.931 691002.449 5335973.143 517.93 691002.448 5335973.144 517.929 691002.447 5335973.144 517.928 691002.446 5335973.144 517.927 691002.446 5335973.144 517.926 691002.445 5335973.145 517.924 691002.445 5335973.145 517.923 691002.445 5335973.145 517.921 691002.444 5335973.145 517.92 691002.444 5335973.145 517.918 691002.444 5335973.145 517.916 691002.444 5335973.145 517.915 691002.444 5335973.145 517.913 691002.444 5335973.145 517.911 691002.445 5335973.145 517.91 691002.445 5335973.145 517.908 691002.446 5335973.145 517.907 691002.446 5335973.144 517.906 691002.447 5335973.144 517.904 691002.447 5335973.144 517.903 691002.448 5335973.144 517.902 691002.449 5335973.143 517.901 691002.45 5335973.143 517.901 691002.451 5335973.143 517.9 691002.452 5335973.142 517.9 691002.452 5335973.142 517.9 691002.453 5335973.141 517.9 691002.454 5335973.141 517.9 691002.455 5335973.141 517.901 691002.456 5335973.14 517.901 691002.457 5335973.14 517.902 691002.458 5335973.14 517.903 691002.458 5335973.14 517.904 691002.459 5335973.139 517.905 691002.459 5335973.139 517.906 691002.46 5335973.139 517.908 691002.46 5335973.139 517.909 691002.461 5335973.139 517.911 691002.461 5335973.138 517.912 691002.461 5335973.138 517.914 691002.461 5335973.138 517.915 691002.461 5335973.138 517.917 691002.461 5335973.138 517.919 691002.461 5335973.138 517.92 691002.461 5335973.139 517.922 691002.46 5335973.139 517.924 691002.46 5335973.139 517.925 691002.459 5335973.139 517.926 691002.459 5335973.139 517.928 691002.458 5335973.14 517.929 691002.457 5335973.14 517.93 691002.456 5335973.14 517.93 691002.456 5335973.141 517.931 691002.455 5335973.141 517.932 691002.454 5335973.141 517.932 691002.453 5335973.142 517.932 691002.453 5335973.142 517.932 691002.452 5335973.142 517.932</gml:posList>
            </gml:LinearRing>
          </gml:exterior>
          <gml:interior>
            <gml:LinearRing gml:id="LR.5IVc7zMUVEgCXQEMNjJ9">
              <gml:posList>691002.457 5335973.14 517.912 691002.457 5335973.14 517.912 691002.457 5335973.14 517.911 691002.456 5335973.14 517.91 691002.456 5335973.14 517.909 691002.455 5335973.141 517.909 691002.455 5335973.141 517.908 691002.455 5335973.141 517.908 691002.454 5335973.141 517.908 691002.454 5335973.141 517.907 691002.453 5335973.142 517.907 691002.453 5335973.142 517.907 691002.452 5335973.142 517.907 691002.452 5335973.142 517.907 691002.451 5335973.142 517.908 691002.451 5335973.142 517.908 691002.45 5335973.143 517.908 691002.45 5335973.143 517.909 691002.45 5335973.143 517.909 691002.449 5335973.143 517.91 691002.449 5335973.143 517.911 691002.449 5335973.143 517.912 691002.448 5335973.143 517.912 691002.448 5335973.144 517.913 691002.448 5335973.144 517.914 691002.448 5335973.144 517.915 691002.448 5335973.144 517.916 691002.448 5335973.144 517.917 691002.448 5335973.144 517.918 691002.448 5335973.144 517.919 691002.448 5335973.143 517.92 691002.449 5335973.143 517.92 691002.449 5335973.143 517.921 691002.449 5335973.143 517.922 691002.45 5335973.143 517.923 691002.45 5335973.143 517.923 691002.45 5335973.143 517.924 691002.451 5335973.143 517.924 691002.451 5335973.142 517.924 691002.452 5335973.142 517.925 691002.452 5335973.142 517.925 691002.453 5335973.142 517.925 691002.453 5335973.142 517.925 691002.454 5335973.141 517.925 691002.454 5335973.141 517.924 691002.455 5335973.141 517.924 691002.455 5335973.141 517.924 691002.455 5335973.141 517.923 691002.456 5335973.14 517.923 691002.456 5335973.14 517.922 691002.457 5335973.14 517.921 691002.457 5335973.14 517.92 691002.457 5335973.14 517.92 691002.457 5335973.14 517.919 691002.457 5335973.14 517.918 691002.457 5335973.14 517.917 691002.457 5335973.14 517.916 691002.457 5335973.14 517.915 691002.457 5335973.14 517.914 691002.457 5335973.14 517.913 691002.457 5335973.14 517.912</gml:posList>
            </gml:LinearRing>
          </gml:interior>
        </gml:Polygon>"#;

        let polygon = deserialize_polygon(xml).expect("should parse");
        let result = polygon.triangulate();
        assert!(
            result.is_ok(),
            "donut polygon should triangulate: {result:?}"
        );
    }

    /// Real CityGML LOD2 polygon (`DEBY_LOD2_4906980_BP...g26AvBp6PO8b9rDifVmR`): a
    /// thin near-zero-width "ridge cap" ring that traces out along a roofline and
    /// almost doubles back on itself. Regression coverage for `earcut` handling a
    /// legitimately valid (if extremely thin) sliver polygon without failing.
    #[test]
    fn triangulate_real_world_ridge_sliver_polygon() {
        let xml = br#"<gml:Polygon gml:id="PG.g26AvBp6PO8b9rDifVmR">
          <gml:exterior>
            <gml:LinearRing gml:id="LR.36K0quGoGRDubt7nC4O4">
              <gml:posList>691001.903 5335973.437 518.993 691001.907 5335973.436 519.02 691001.912 5335973.434 519.046 691001.92 5335973.431 519.073 691001.93 5335973.427 519.099 691001.942 5335973.422 519.124 691001.956 5335973.417 519.149 691001.972 5335973.41 519.173 691001.991 5335973.403 519.196 691002.011 5335973.395 519.218 691002.033 5335973.387 519.24 691002.056 5335973.377 519.26 691002.063 5335973.375 519.265 691002.057 5335973.377 519.269 691002.05 5335973.38 519.264 691002.026 5335973.389 519.244 691002.004 5335973.398 519.222 691001.983 5335973.406 519.199 691001.965 5335973.413 519.176 691001.948 5335973.42 519.152 691001.934 5335973.425 519.126 691001.922 5335973.43 519.101 691001.912 5335973.434 519.074 691001.904 5335973.437 519.048 691001.898 5335973.439 519.021 691001.894 5335973.441 518.993 691001.894 5335973.441 518.975 691001.902 5335973.438 518.975 691001.903 5335973.437 518.993</gml:posList>
            </gml:LinearRing>
          </gml:exterior>
        </gml:Polygon>"#;

        let polygon = deserialize_polygon(xml).expect("should parse");
        let result = polygon.triangulate();
        assert!(
            result.is_ok(),
            "ridge sliver polygon should triangulate: {result:?}"
        );
    }

    /// Real CityGML LOD2 polygon (`DEBY_LOD2_4906980_BP...gMfaUqQV9M84lYp4c86N`): a
    /// genuinely degenerate, zero-area "dart" ring that traces `P1->P2->P1->P4->P5->P4`
    /// (out and back on itself twice). `earcut` correctly refuses to produce any
    /// triangles for it, since it encloses no area. This is expected to error — the
    /// fix for the real-world bug this ring exposed is at the aggregate level (see
    /// `multi_surface::tests::triangulate_multi_surface_skips_degenerate_member`),
    /// which now skips members like this one instead of failing the whole
    /// `MultiSurface`.
    #[test]
    fn polygon_with_degenerate_dart_ring_fails_to_triangulate() {
        let xml = br#"<gml:Polygon gml:id="PG.gMfaUqQV9M84lYp4c86N">
          <gml:exterior>
            <gml:LinearRing gml:id="LR.C0NSX6grg36ILxFqThbQ">
              <gml:posList>691002.063 5335973.375 519.274 691002.057 5335973.377 519.269 691002.063 5335973.375 519.274 691002.067 5335973.373 519.277 691002.073 5335973.371 519.282 691002.067 5335973.373 519.277</gml:posList>
            </gml:LinearRing>
          </gml:exterior>
        </gml:Polygon>"#;

        let polygon = deserialize_polygon(xml).expect("should parse");
        let result = polygon.triangulate();
        assert!(
            result.is_err(),
            "degenerate dart ring should fail to triangulate: {result:?}"
        );
    }

    fn make_polygon(points: Vec<DirectPosition>) -> Polygon {
        let ring_kind = AbstractRingKind::LinearRing(LinearRing::new(points).unwrap());
        Polygon::new(Some(AbstractRingProperty::from_object(ring_kind)), vec![]).unwrap()
    }

    fn make_square() -> Polygon {
        make_polygon(vec![
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 1.0, 0.0).unwrap(),
            DirectPosition::new(0.0, 1.0, 0.0).unwrap(),
        ])
    }

    #[test]
    fn deserialize_polygon_with_interior_rings() {
        let xml_document = "<gml:Polygon gml:id=\"DEBY_LOD2_4959457_f5d787b1-1fee-441a-898d-0d1bab1fc83f_poly\">
                  <gml:exterior>
                    <gml:LinearRing gml:id=\"DEBY_LOD2_4959457_f5d787b1-1fee-441a-898d-0d1bab1fc83f_poly_0_\">
                      <gml:posList>690985.156 5336010.964 530.92 691004.477 5336059.877 530.92 690987.939 5336066.373 530.92 690968.654 5336017.45 530.92 690985.156 5336010.964 530.92</gml:posList>
                    </gml:LinearRing>
                  </gml:exterior>
                  <gml:interior>
                    <gml:LinearRing gml:id=\"DEBY_LOD2_4959457_f5d787b1-1fee-441a-898d-0d1bab1fc83f_poly_0_.p848qtslOqDyVvoEINYt\">
                      <gml:posList>690997.492 5336051.391 530.92 690996.582 5336049.058 530.92 690987.295 5336052.68 530.92 690988.205 5336055.013 530.92 690997.492 5336051.391 530.92</gml:posList>
                    </gml:LinearRing>
                  </gml:interior>
                  <gml:interior>
                    <gml:LinearRing gml:id=\"DEBY_LOD2_4959457_f5d787b1-1fee-441a-898d-0d1bab1fc83f_poly_0_.kRRfuBGJHBBSenuutsor\">
                      <gml:posList>690976.045 5336024.363 530.92 690976.805 5336026.31 530.92 690986.204 5336022.644 530.92 690985.444 5336020.697 530.92 690976.045 5336024.363 530.92</gml:posList>
                    </gml:LinearRing>
                  </gml:interior>
                </gml:Polygon>";

        let polygon = deserialize_polygon(xml_document.as_bytes()).expect("should deserialize");

        assert_eq!(polygon.interior().len(), 2)
    }

    #[test]
    fn serialize_polygon_writes_gml_tags() {
        let polygon = make_square();
        let xml_node = serialize_polygon(&polygon, Formatting::Compact).unwrap();
        let xml = xml_node
            .to_string(Formatting::Compact)
            .expect("should serialize");

        assert!(xml.contains("<gml:Polygon"));
        assert!(xml.contains("<gml:exterior"));
        assert!(xml.contains("<gml:LinearRing"));
        assert!(xml.contains("<gml:posList"));
        assert!(!xml.contains("id="));
    }

    #[test]
    fn round_trip_polygon_preserves_points() {
        let polygon = make_square();
        let xml_node = serialize_polygon(&polygon, Formatting::Compact).unwrap();
        let xml = xml_node
            .to_string(Formatting::Compact)
            .expect("should serialize");

        let recovered: Polygon = deserialize_polygon(xml.as_ref()).unwrap();

        let orig = polygon.exterior().unwrap().object().expect("ring missing");
        let recov = recovered
            .exterior()
            .unwrap()
            .object()
            .expect("ring missing");
        assert_eq!(orig.points().len(), recov.points().len());
    }

    #[test]
    fn round_trip_polygon_from_xml() {
        let input_xml = "<gml:Polygon>\
            <gml:exterior><gml:LinearRing><gml:posList srsDimension=\"3\">0 0 0 1 0 0 1 1 0 0 1 0 0 0 0</gml:posList></gml:LinearRing></gml:exterior>\
            </gml:Polygon>";

        let polygon = deserialize_polygon(input_xml.as_bytes()).expect("should deserialize");
        let xml_node = serialize_polygon(&polygon, Formatting::Compact).unwrap();
        let output_xml = xml_node
            .to_string(Formatting::Compact)
            .expect("should serialize");

        assert_eq!(input_xml, output_xml);
    }

    #[test]
    fn serialize_polygon_with_interior_rings() {
        let exterior_pts = vec![
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(4.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(4.0, 4.0, 0.0).unwrap(),
            DirectPosition::new(0.0, 4.0, 0.0).unwrap(),
        ];
        let interior_pts = vec![
            DirectPosition::new(1.0, 1.0, 0.0).unwrap(),
            DirectPosition::new(3.0, 1.0, 0.0).unwrap(),
            DirectPosition::new(3.0, 3.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 3.0, 0.0).unwrap(),
        ];
        let exterior = AbstractRingKind::LinearRing(LinearRing::new(exterior_pts).unwrap());
        let interior = AbstractRingKind::LinearRing(LinearRing::new(interior_pts).unwrap());
        let polygon = Polygon::new(
            Some(AbstractRingProperty::from_object(exterior)),
            vec![AbstractRingProperty::from_object(interior)],
        )
        .unwrap();

        let xml_node = serialize_polygon(&polygon, Formatting::Compact).unwrap();
        let xml = xml_node
            .to_string(Formatting::Compact)
            .expect("should serialize");

        assert!(xml.contains("<gml:interior"));
        let recovered: Polygon = deserialize_polygon(xml.as_ref()).expect("should deserialize");
        assert_eq!(recovered.interior().len(), 1);
    }
}
