use crate::Error;
use crate::codec::geometry::aggregates::{
    deserialize_abstract_geometric_aggregate, serialize_abstract_geometric_aggregate,
};
use crate::codec::geometry::primitives::{
    deserialize_abstract_surface_property, serialize_abstract_surface_property,
};
use crate::util::{
    Formatting, GmlElement, XmlNode, XmlNodeContent, collect_children_lenient,
    extract_xml_element_spans,
};
use egml_core::model::geometry::aggregates::{AsAbstractGeometricAggregate, MultiSurface};
use tracing::debug;

pub fn deserialize_multi_surface(xml_document: &[u8]) -> Result<MultiSurface, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let abstract_geometric_aggregate =
        deserialize_abstract_geometric_aggregate(xml_document, &spans)?;

    let (surface_members, skipped) = collect_children_lenient(
        xml_document,
        &spans,
        GmlElement::SurfaceMemberProperty,
        deserialize_abstract_surface_property,
    );
    if !skipped.is_empty() {
        debug!(
            count = skipped.len(),
            "MultiSurface: dropped invalid surfaceMember(s)"
        );
    }

    Ok(MultiSurface::from_abstract_geometric_aggregate(
        abstract_geometric_aggregate,
        surface_members,
    )?)
}

pub fn serialize_multi_surface(
    multi_surface: &MultiSurface,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut xml_node_parts = serialize_abstract_geometric_aggregate(
        multi_surface.abstract_geometric_aggregate(),
        formatting,
    )?;

    for member in multi_surface.surface_member() {
        xml_node_parts
            .content
            .push(XmlNodeContent::Child(serialize_abstract_surface_property(
                member,
                formatting,
                GmlElement::SurfaceMemberProperty.into(),
            )?));
    }

    Ok(XmlNode::new(
        GmlElement::MultiSurface.into(),
        xml_node_parts,
    ))
}

#[cfg(test)]
mod tests {
    use crate::codec::geometry::aggregates::multi_surface::{
        deserialize_multi_surface, serialize_multi_surface,
    };
    use crate::util::Formatting;
    use egml_core::model::base::{AsAbstractGml, AsAbstractGmlMut};
    use egml_core::model::geometry::DirectPosition;
    use egml_core::model::geometry::aggregates::MultiSurface;
    use egml_core::model::geometry::primitives::{
        AbstractRingKind, AbstractRingProperty, AbstractSurfaceKind, AbstractSurfaceProperty,
        LinearRing, Polygon,
    };

    fn make_multi_surface() -> MultiSurface {
        let points = vec![
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(0.0, 1.0, 0.0).unwrap(),
        ];
        let ring_kind = AbstractRingKind::LinearRing(LinearRing::new(points).unwrap());
        let polygon = Polygon::new(Some(AbstractRingProperty::from_object(ring_kind)), []).unwrap();
        MultiSurface::new([AbstractSurfaceProperty::from_object(
            AbstractSurfaceKind::Polygon(polygon),
        )])
        .unwrap()
    }

    #[test]
    fn deserialize_multi_surface_with_composite_surface_member() {
        let xml_document = b"
            <gml:MultiSurface srsDimension=\"3\">
                <gml:surfaceMember>
                    <gml:CompositeSurface>
                        <gml:surfaceMember>
                            <gml:Polygon>
                                <gml:exterior>
                                    <gml:LinearRing>
                                        <gml:posList>314.531005859375 1043.4599609375 7.144559860229492 314.531005859375 1043.4599609375 2.6047298908233643 314.68798828125 1043.22998046875 2.6047298908233643 314.531005859375 1043.4599609375 7.144559860229492</gml:posList>
                                    </gml:LinearRing>
                                </gml:exterior>
                            </gml:Polygon>
                        </gml:surfaceMember>
                        <gml:surfaceMember>
                            <gml:Polygon>
                                <gml:exterior>
                                    <gml:LinearRing>
                                        <gml:posList>314.531005859375 1043.4599609375 7.144559860229492 314.68798828125 1043.22998046875 2.6047298908233643 315.7770080566406 1041.6500244140625 2.6047298908233643 314.531005859375 1043.4599609375 7.144559860229492</gml:posList>
                                    </gml:LinearRing>
                                </gml:exterior>
                            </gml:Polygon>
                        </gml:surfaceMember>
                        <gml:surfaceMember>
                            <gml:Polygon>
                                <gml:exterior>
                                    <gml:LinearRing>
                                        <gml:posList>314.531005859375 1043.4599609375 7.144559860229492 315.7770080566406 1041.6500244140625 2.6047298908233643 316.1080017089844 1041.1700439453125 7.144559860229492 314.531005859375 1043.4599609375 7.144559860229492</gml:posList>
                                    </gml:LinearRing>
                                </gml:exterior>
                            </gml:Polygon>
                        </gml:surfaceMember>
                    </gml:CompositeSurface>
                </gml:surfaceMember>
            </gml:MultiSurface>";

        let result = deserialize_multi_surface(xml_document).unwrap();
        assert_eq!(result.surface_member().len(), 1);
    }

    #[test]
    fn deserialize_multi_surface_with_one_polygon_member() {
        let xml_document = b"<gml:MultiSurface gml:id=\"UUID_6b33ecfa-6e08-4e8e-a4b5-e1d06540faf0\">
              <gml:surfaceMember>
                <gml:Polygon gml:id=\"UUID_efb8f6a5-82fa-4b21-8709-c1d93ed1e595\">
                  <gml:exterior>
                    <gml:LinearRing>
                      <gml:posList srsDimension=\"3\">678009.7116291433 5403638.313338383 417.3480034550211 678012.5609078613 5403634.960884141 417.34658523466385 678013.7892528991 5403636.004867206 417.51938733855997 678010.9399743223 5403639.357321232 417.5208051908512 678009.7116291433 5403638.313338383 417.3480034550211</gml:posList>
                    </gml:LinearRing>
                  </gml:exterior>
                </gml:Polygon>
              </gml:surfaceMember>
            </gml:MultiSurface>";

        let result = deserialize_multi_surface(xml_document).unwrap();
        assert_eq!(result.surface_member().len(), 1);
    }

    #[test]
    fn deserialize_multi_surface_with_duplicate_ring_points() {
        let xml_document = b"<gml:MultiSurface srsName=\"EPSG:25832\" srsDimension=\"3\">
              <gml:surfaceMember>
                <gml:Polygon gml:id=\"4018133_PG.3nRTCd4XPu47PsAAUyNv\">
                  <gml:exterior>
                    <gml:LinearRing gml:id=\"4018133_LR.lHfcvQUrKVl08ifcH6eO\">
                      <gml:posList>678105.792 5403815.554 369.98523 678105.792 5403815.555 367.67323 678106.047 5403815.125 367.67323 678106.047 5403815.125 367.67323 678106.047 5403815.125 367.67323 678106.047 5403815.124 369.98523 678105.792 5403815.554 369.98523</gml:posList>
                    </gml:LinearRing>
                  </gml:exterior>
                </gml:Polygon>
              </gml:surfaceMember>
            </gml:MultiSurface>";

        let result = deserialize_multi_surface(xml_document).unwrap();
        assert_eq!(result.surface_member().len(), 1);
    }

    #[test]
    fn deserialize_multi_surface_with_interior_rings() {
        let xml_document = b"
            <gml:MultiSurface srsName=\"EPSG:25832\" srsDimension=\"3\">
              <gml:surfaceMember>
                <gml:Polygon gml:id=\"4018106_PG.dKY9ug9ol2tsxL5bLAPz\">
                  <gml:exterior>
                    <gml:LinearRing gml:id=\"4018106_LR.Wqmtl1E6Yz3eVJkuGjsK\">
                      <gml:posList>678097.805 5403801.433 367.40123 678092.938 5403810.139 367.40123 678092.938 5403810.139 370.87623 678092.032 5403811.76 370.87623 678092.032 5403811.76 377.09023 678097.805 5403801.433 377.09023 678097.805 5403801.433 367.40123</gml:posList>
                    </gml:LinearRing>
                  </gml:exterior>
                  <gml:interior>
                    <gml:LinearRing>
                      <gml:posList>678096.88 5403803.088 374.90623 678097.403 5403802.152 374.90623 678097.403 5403802.152 376.19923 678096.88 5403803.088 376.19923 678096.88 5403803.088 374.90623</gml:posList>
                    </gml:LinearRing>
                  </gml:interior>
                </gml:Polygon>
              </gml:surfaceMember>
            </gml:MultiSurface>";

        let result = deserialize_multi_surface(xml_document).unwrap();
        assert_eq!(result.surface_member().len(), 1);
    }

    #[test]
    fn deserialize_multi_surface_without_id() {
        let xml_document = b"<gml:MultiSurface>
              <gml:surfaceMember>
                <gml:Polygon>
                  <gml:exterior>
                    <gml:LinearRing>
                      <gml:posList srsDimension=\"3\">678009.7116291433 5403638.313338383 417.3480034550211 678012.5609078613 5403634.960884141 417.34658523466385 678013.7892528991 5403636.004867206 417.51938733855997 678010.9399743223 5403639.357321232 417.5208051908512 678009.7116291433 5403638.313338383 417.3480034550211</gml:posList>
                    </gml:LinearRing>
                  </gml:exterior>
                </gml:Polygon>
              </gml:surfaceMember>
            </gml:MultiSurface>";

        let result = deserialize_multi_surface(xml_document).unwrap();
        assert_eq!(result.surface_member().len(), 1);
    }

    #[test]
    fn deserialize_multi_surface_skips_invalid_polygon_member() {
        let xml_document = b"<gml:MultiSurface srsDimension=\"3\">
              <gml:surfaceMember>
                <gml:Polygon>
                  <gml:exterior>
                    <gml:LinearRing>
                      <gml:posList>0 0 0 1 0 0</gml:posList>
                    </gml:LinearRing>
                  </gml:exterior>
                </gml:Polygon>
              </gml:surfaceMember>
              <gml:surfaceMember>
                <gml:Polygon>
                  <gml:exterior>
                    <gml:LinearRing>
                      <gml:posList>0 0 0 1 0 0 0 1 0 0 0 0</gml:posList>
                    </gml:LinearRing>
                  </gml:exterior>
                </gml:Polygon>
              </gml:surfaceMember>
            </gml:MultiSurface>";

        let result = deserialize_multi_surface(xml_document).unwrap();
        assert_eq!(result.surface_member().len(), 1);
    }

    #[test]
    fn deserialize_multi_surface_with_polygon_patches() {
        let xml_document = b"<gml:MultiSurface srsDimension=\"3\">
    <gml:surfaceMember>
        <gml:Surface>
            <gml:patches>
                <gml:PolygonPatch>
                    <gml:exterior>
                        <gml:LinearRing>
                            <gml:posList>350.54400634765625 972.9130249023438 0.11999999731779099 350.5414201635045 968.6025425887852 0.11999999731779099 350.54400634765625 968.6025096366793 0.11999999731779099 350.54400634765625 972.9130249023438 0.11999999731779099</gml:posList>
                        </gml:LinearRing>
                    </gml:exterior>
                </gml:PolygonPatch>
            </gml:patches>
        </gml:Surface>
    </gml:surfaceMember>
    <gml:surfaceMember>
        <gml:Surface>
            <gml:patches>
                <gml:PolygonPatch>
                    <gml:exterior>
                        <gml:LinearRing>
                            <gml:posList>350.54400634765625 972.9130249023438 0.11999999731779099 350.364013671875 968.6048030300961 0.11999999731779099 350.5414201635045 968.6025425887852 0.11999999731779099 350.54400634765625 972.9130249023438 0.11999999731779099</gml:posList>
                        </gml:LinearRing>
                    </gml:exterior>
                </gml:PolygonPatch>
            </gml:patches>
        </gml:Surface>
    </gml:surfaceMember>
</gml:MultiSurface>";

        let result = deserialize_multi_surface(xml_document).unwrap();
        assert_eq!(result.surface_member().len(), 2);
    }

    /// Regression test for a real CityGML LOD2 export (`DEBY_LOD2_4906980`): one
    /// exterior ring is a genuinely degenerate zero-area "dart"
    /// (`P1->P2->P1->P4->P5->P4`, tracing out and back on itself), which `earcut`
    /// correctly refuses to triangulate (see
    /// `polygon::tests::polygon_with_degenerate_dart_ring_fails_to_triangulate`
    /// for that in isolation). `MultiSurface::triangulate` skips members like this
    /// one instead of failing the whole aggregate, and now surfaces the skip via
    /// [`egml_core::model::common::Triangulation::skipped`] instead of only logging it.
    #[test]
    fn triangulate_multi_surface_skips_degenerate_member() {
        use egml_core::model::common::Triangulate;

        let xml_document = b"<gml:MultiSurface srsName=\"EPSG:25832\" srsDimension=\"3\">
              <gml:surfaceMember>
                <gml:Polygon gml:id=\"DEBY_LOD2_4906980_BP.utxbGmDKX0NjrFxFteS4_PG.k7bDoCvW0sUy0jLSPKmd\">
                  <gml:exterior>
                    <gml:LinearRing gml:id=\"DEBY_LOD2_4906980_BP.utxbGmDKX0NjrFxFteS4_LR.rRl2hgY3Vm4oEQdZlXZ1\">
                      <gml:posList>691002.434 5335973.189 518.253 691002.179 5335973.289 518.253 691002.179 5335973.29 518.925 691002.434 5335973.19 518.925 691002.434 5335973.189 518.253</gml:posList>
                    </gml:LinearRing>
                  </gml:exterior>
                </gml:Polygon>
              </gml:surfaceMember>
              <gml:surfaceMember>
                <gml:Polygon gml:id=\"DEBY_LOD2_4906980_BP.utxbGmDKX0NjrFxFteS4_PG.gMfaUqQV9M84lYp4c86N\">
                  <gml:exterior>
                    <gml:LinearRing gml:id=\"DEBY_LOD2_4906980_BP.utxbGmDKX0NjrFxFteS4_LR.C0NSX6grg36ILxFqThbQ\">
                      <gml:posList>691002.063 5335973.375 519.274 691002.057 5335973.377 519.269 691002.063 5335973.375 519.274 691002.067 5335973.373 519.277 691002.073 5335973.371 519.282 691002.067 5335973.373 519.277</gml:posList>
                    </gml:LinearRing>
                  </gml:exterior>
                </gml:Polygon>
              </gml:surfaceMember>
            </gml:MultiSurface>";

        let multi_surface = deserialize_multi_surface(xml_document).expect("should deserialize");
        assert_eq!(multi_surface.surface_member().len(), 2);

        let triangulation = multi_surface
            .triangulate()
            .expect("should triangulate despite one degenerate member");
        assert!(!triangulation.surface().triangles().is_empty());
        assert_eq!(triangulation.skipped().len(), 1);
    }

    #[test]
    fn serialize_multi_surface_writes_gml_tags() {
        let multi_surface = make_multi_surface();

        let xml_node =
            serialize_multi_surface(&multi_surface, Formatting::Compact).expect("should serialize");
        let xml = xml_node
            .to_string(Formatting::Compact)
            .expect("to string should work");

        assert!(xml.contains("<gml:MultiSurface"));
        assert!(xml.contains("<gml:surfaceMember"));
        assert!(xml.contains("<gml:Polygon"));
        assert!(xml.contains("<gml:exterior"));
        assert!(xml.contains("<gml:LinearRing"));
        assert!(!xml.contains("id="));
    }

    #[test]
    fn serialize_multi_surface_with_id_writes_id() {
        use egml_core::model::base::Id;

        let mut multi_surface = make_multi_surface();
        multi_surface.set_id(Id::try_from("test-id").unwrap());

        let xml_node =
            serialize_multi_surface(&multi_surface, Formatting::Compact).expect("should serialize");
        let xml = xml_node
            .to_string(Formatting::Compact)
            .expect("to string should work");

        assert!(xml.contains("gml:id=\"test-id\""));
    }

    #[test]
    fn round_trip_multi_surface_preserves_member_count() {
        let multi_surface = make_multi_surface();

        let xml_node =
            serialize_multi_surface(&multi_surface, Formatting::Compact).expect("should serialize");
        let xml = xml_node.to_string(Formatting::Compact).unwrap();
        let recovered = deserialize_multi_surface(xml.as_bytes()).unwrap();

        assert_eq!(
            recovered.surface_member().len(),
            multi_surface.surface_member().len()
        );
    }

    #[test]
    fn round_trip_multi_surface_from_xml() {
        let input_xml = b"<gml:MultiSurface gml:id=\"test-id\">\
            <gml:surfaceMember><gml:Polygon><gml:exterior><gml:LinearRing>\
            <gml:posList srsDimension=\"3\">0 0 0 1 0 0 0 1 0 0 0 0</gml:posList>\
            </gml:LinearRing></gml:exterior></gml:Polygon></gml:surfaceMember>\
            </gml:MultiSurface>";

        let multi_surface = deserialize_multi_surface(input_xml).expect("should deserialize");
        let xml_node =
            serialize_multi_surface(&multi_surface, Formatting::Compact).expect("should serialize");
        let output = xml_node.to_string(Formatting::Compact).unwrap();

        let recovered =
            deserialize_multi_surface(output.as_bytes()).expect("should deserialize recovered");

        assert_eq!(
            recovered.surface_member().len(),
            multi_surface.surface_member().len()
        );
        assert_eq!(recovered.id(), multi_surface.id());
    }
}
