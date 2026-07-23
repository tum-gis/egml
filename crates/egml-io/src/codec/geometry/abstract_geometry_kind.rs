use crate::Error;
use crate::codec::geometry::aggregates::{
    deserialize_abstract_geometric_aggregate_kind, serialize_abstract_geometric_aggregate_kind,
};
use crate::codec::geometry::primitives::{
    deserialize_abstract_geometric_primitive_kind, serialize_abstract_geometric_primitive_kind,
};
use crate::util::{Formatting, GmlElement, XmlElementSpans, XmlNode};
use egml_core::model::geometry::AbstractGeometryKind;

pub fn deserialize_abstract_geometry_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans<GmlElement>,
) -> Result<Option<AbstractGeometryKind>, Error> {
    if let Some(x) = deserialize_abstract_geometric_aggregate_kind(xml_document, spans)? {
        return Ok(Some(x.into()));
    }

    if let Some(x) = deserialize_abstract_geometric_primitive_kind(xml_document, spans)? {
        return Ok(Some(x.into()));
    }

    Ok(None)
}

pub fn serialize_abstract_geometry_kind(
    abstract_geometry_kind: &AbstractGeometryKind,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    match abstract_geometry_kind {
        AbstractGeometryKind::AbstractGeometricAggregateKind(x) => {
            serialize_abstract_geometric_aggregate_kind(x, formatting)
        }
        AbstractGeometryKind::AbstractGeometricPrimitiveKind(x) => {
            serialize_abstract_geometric_primitive_kind(x, formatting)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{deserialize_abstract_geometry_kind, serialize_abstract_geometry_kind};
    use crate::util::{Formatting, extract_xml_element_spans};
    use egml_core::model::geometry::AbstractGeometryKind;
    use egml_core::model::geometry::DirectPosition;
    use egml_core::model::geometry::aggregates::{
        AbstractGeometricAggregateKind, MultiGeometry, MultiPoint,
    };
    use egml_core::model::geometry::primitives::{
        AbstractGeometricPrimitiveKind, AbstractRingKind, AbstractRingProperty,
        AbstractSurfaceKind, LinearRing, Point, PointProperty, Polygon,
    };

    fn make_polygon_kind() -> AbstractGeometryKind {
        let points = vec![
            DirectPosition::new(-0.057, 0.057, 0.4).unwrap(),
            DirectPosition::new(-0.182, 0.182, 0.428).unwrap(),
            DirectPosition::new(0.0, 0.258, 0.428).unwrap(),
            DirectPosition::new(0.0, 0.081, 0.4).unwrap(),
        ];
        let ring = AbstractRingKind::LinearRing(LinearRing::new(points).unwrap());
        let polygon = Polygon::new(Some(AbstractRingProperty::from_object(ring)), []).unwrap();
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
    fn deserialize_polygon_as_geometry_kind() {
        let xml = b"<gml:someParent>\
            <gml:Polygon>\
            <gml:exterior><gml:LinearRing>\
            <gml:posList srsDimension=\"3\">0 0 0 1 0 0 0 1 0 0 0 0</gml:posList>\
            </gml:LinearRing></gml:exterior>\
            </gml:Polygon>\
            </gml:someParent>";

        let spans = extract_xml_element_spans(xml).unwrap();
        let kind = deserialize_abstract_geometry_kind(xml, &spans)
            .unwrap()
            .unwrap();

        assert!(matches!(
            kind,
            AbstractGeometryKind::AbstractGeometricPrimitiveKind(
                AbstractGeometricPrimitiveKind::AbstractSurfaceKind(AbstractSurfaceKind::Polygon(
                    _
                ))
            )
        ));
    }

    #[test]
    fn deserialize_multi_point_as_geometry_kind() {
        let xml = b"<gml:someParent>\
            <gml:MultiPoint>\
            <gml:pointMember><gml:Point><gml:pos srsDimension=\"3\">1 2 3</gml:pos></gml:Point></gml:pointMember>\
            </gml:MultiPoint>\
            </gml:someParent>";

        let spans = extract_xml_element_spans(xml).unwrap();
        let kind = deserialize_abstract_geometry_kind(xml, &spans)
            .unwrap()
            .unwrap();

        assert!(matches!(
            kind,
            AbstractGeometryKind::AbstractGeometricAggregateKind(
                AbstractGeometricAggregateKind::MultiPoint(_)
            )
        ));
    }

    #[test]
    fn deserialize_returns_none_when_no_geometry() {
        let xml = b"<gml:someParent/>";

        let spans = extract_xml_element_spans(xml).unwrap();
        let kind = deserialize_abstract_geometry_kind(xml, &spans).unwrap();

        assert!(kind.is_none());
    }

    #[test]
    fn serialize_polygon_geometry_kind() {
        let kind = make_polygon_kind();
        let xml_node = serialize_abstract_geometry_kind(&kind, Formatting::Compact).unwrap();
        let xml = xml_node.to_string(Formatting::Compact).unwrap();

        assert!(xml.contains("<gml:Polygon"));
        assert!(xml.contains("<gml:exterior"));
        assert!(xml.contains("<gml:LinearRing"));
    }

    #[test]
    fn serialize_polygon_geometry_kind_newline_formatting() {
        let kind = make_polygon_kind();
        let xml_node = serialize_abstract_geometry_kind(&kind, Formatting::NewLine).unwrap();
        let xml = xml_node.to_string(Formatting::NewLine).unwrap();

        assert!(
            xml.contains('\n'),
            "NewLine formatting must produce newlines"
        );
        assert!(
            !xml.starts_with('\n'),
            "output must not begin with a newline"
        );

        // Compact mode must produce no newlines (sanity baseline)
        let compact_xml = serialize_abstract_geometry_kind(&kind, Formatting::Compact)
            .unwrap()
            .to_string(Formatting::Compact)
            .unwrap();
        assert!(!compact_xml.contains('\n'));

        // In NewLine mode every GML tag must sit at column 0 — no indentation
        for line in xml.lines() {
            if line.starts_with("<gml:") || line.starts_with("</gml:") {
                assert_eq!(
                    line,
                    line.trim_start(),
                    "GML tag must not be indented in NewLine mode: {line:?}"
                );
            }
        }

        assert!(xml.contains("<gml:Polygon"));
        assert!(xml.contains("<gml:exterior"));
        assert!(xml.contains("<gml:LinearRing"));
    }

    #[test]
    fn serialize_polygon_geometry_kind_indent_two_spaces() {
        let formatting = Formatting::Indent { char: ' ', size: 2 };
        let kind = make_polygon_kind();
        let xml = serialize_abstract_geometry_kind(&kind, formatting)
            .unwrap()
            .to_string(formatting)
            .unwrap();

        // Root element at column 0 — no leading newline
        assert!(!xml.starts_with('\n'));
        assert!(xml.starts_with("<gml:Polygon"));

        // Each nesting level indented by size * depth spaces
        assert!(xml.contains("\n  <gml:exterior")); // depth 1 → 2 spaces
        assert!(xml.contains("\n    <gml:LinearRing")); // depth 2 → 4 spaces
        assert!(xml.contains("\n    </gml:LinearRing"));
        assert!(xml.contains("\n  </gml:exterior"));
        assert!(xml.contains("\n</gml:Polygon")); // depth 0 → no indent
    }

    #[test]
    fn serialize_polygon_geometry_kind_indent_tabs() {
        let formatting = Formatting::Indent {
            char: '\t',
            size: 1,
        };
        let kind = make_polygon_kind();
        let xml = serialize_abstract_geometry_kind(&kind, formatting)
            .unwrap()
            .to_string(formatting)
            .unwrap();

        assert!(!xml.starts_with('\n'));
        assert!(xml.starts_with("<gml:Polygon"));

        assert!(xml.contains("\n\t<gml:exterior")); // depth 1 → 1 tab
        assert!(xml.contains("\n\t\t<gml:LinearRing")); // depth 2 → 2 tabs
        assert!(xml.contains("\n\t\t</gml:LinearRing"));
        assert!(xml.contains("\n\t</gml:exterior"));
        assert!(xml.contains("\n</gml:Polygon"));
    }

    #[test]
    fn serialize_multi_point_geometry_kind() {
        let kind = make_multi_point_kind();
        let xml_node = serialize_abstract_geometry_kind(&kind, Formatting::Compact).unwrap();
        let xml = xml_node.to_string(Formatting::Compact).unwrap();

        assert!(xml.contains("<gml:MultiPoint"));
        assert!(xml.contains("<gml:pointMember"));
        assert!(xml.contains("<gml:Point"));
    }

    #[test]
    fn round_trip_polygon_geometry_kind() {
        let kind = make_polygon_kind();
        let xml_node = serialize_abstract_geometry_kind(&kind, Formatting::Compact).unwrap();
        let xml = xml_node.to_string(Formatting::Compact).unwrap();

        let wrapper = format!("<gml:parent>{xml}</gml:parent>");
        let spans = extract_xml_element_spans(wrapper.as_bytes()).unwrap();
        let recovered = deserialize_abstract_geometry_kind(wrapper.as_bytes(), &spans)
            .unwrap()
            .unwrap();

        assert!(matches!(
            recovered,
            AbstractGeometryKind::AbstractGeometricPrimitiveKind(
                AbstractGeometricPrimitiveKind::AbstractSurfaceKind(AbstractSurfaceKind::Polygon(
                    _
                ))
            )
        ));
    }

    #[test]
    fn deserialize_nested_multi_geometry() {
        let xml = b"<gml:parent>\
            <gml:MultiGeometry srsName=\"urn:ogc:def:crs:EPSG::25832\" srsDimension=\"3\" gml:id=\"fme-gen-1b18fb8f-dbee-460c-bd8e-4e3229066ea2\">\
              <gml:geometryMember>\
                <gml:MultiGeometry>\
                  <gml:geometryMember>\
                    <gml:Polygon>\
                      <gml:exterior><gml:LinearRing>\
                        <gml:posList>1.1102230246251565E-16 -0.08104761325536736 0.40000000000000024 5.551115123125783E-17 -0.25873011317535366 0.42868290603009473 -0.18294981752345554 -0.1829498175234554 0.42868290603009473 -0.057309316931854926 -0.057309316931854926 0.40000000000000024 1.1102230246251565E-16 -0.08104761325536736 0.40000000000000024</gml:posList>\
                      </gml:LinearRing></gml:exterior>\
                    </gml:Polygon>\
                  </gml:geometryMember>\
                  <gml:geometryMember>\
                    <gml:Polygon>\
                      <gml:exterior><gml:LinearRing>\
                        <gml:posList>-0.057309316931854926 -0.057309316931854926 0.40000000000000024 -0.18294981752345554 -0.1829498175234554 0.42868290603009473 -0.25873011317535366 0.0 0.42868290603009473 -0.0810476132553673 0.0 0.40000000000000024 -0.057309316931854926 -0.057309316931854926 0.40000000000000024</gml:posList>\
                      </gml:LinearRing></gml:exterior>\
                    </gml:Polygon>\
                  </gml:geometryMember>\
                  <gml:geometryMember>\
                    <gml:Polygon>\
                      <gml:exterior><gml:LinearRing>\
                        <gml:posList>-0.24026196252347953 0.24026196252347976 0.49950174129794134 -0.24748737341529176 0.2474873734152918 0.6403170178482882 5.551115123125783E-17 0.3500000000000002 0.6403170178482882 5.551115123125783E-17 0.3397817259230812 0.49950174129794134 -0.24026196252347953 0.24026196252347976 0.49950174129794134</gml:posList>\
                      </gml:LinearRing></gml:exterior>\
                    </gml:Polygon>\
                  </gml:geometryMember>\
                </gml:MultiGeometry>\
              </gml:geometryMember>\
            </gml:MultiGeometry>\
            </gml:parent>";

        let spans = extract_xml_element_spans(xml).unwrap();
        let kind = deserialize_abstract_geometry_kind(xml, &spans)
            .unwrap()
            .unwrap();

        let AbstractGeometryKind::AbstractGeometricAggregateKind(
            AbstractGeometricAggregateKind::MultiGeometry(outer),
        ) = kind
        else {
            panic!("expected outer MultiGeometry");
        };

        assert_eq!(outer.geometry_member().len(), 1);

        let inner_kind = outer.geometry_member()[0].object().unwrap();
        let AbstractGeometryKind::AbstractGeometricAggregateKind(
            AbstractGeometricAggregateKind::MultiGeometry(inner),
        ) = inner_kind
        else {
            panic!("expected inner MultiGeometry");
        };

        assert_eq!(inner.geometry_member().len(), 3);
        assert!(inner.geometry_member().iter().all(|m| matches!(
            m.object().unwrap(),
            AbstractGeometryKind::AbstractGeometricPrimitiveKind(
                AbstractGeometricPrimitiveKind::AbstractSurfaceKind(AbstractSurfaceKind::Polygon(
                    _
                ))
            )
        )));
    }

    #[test]
    fn round_trip_multi_point_geometry_kind() {
        let kind = make_multi_point_kind();
        let xml_node = serialize_abstract_geometry_kind(&kind, Formatting::Compact).unwrap();
        let xml = xml_node.to_string(Formatting::Compact).unwrap();

        let wrapper = format!("<gml:parent>{xml}</gml:parent>");
        let spans = extract_xml_element_spans(wrapper.as_bytes()).unwrap();
        let recovered = deserialize_abstract_geometry_kind(wrapper.as_bytes(), &spans)
            .unwrap()
            .unwrap();

        assert!(matches!(
            recovered,
            AbstractGeometryKind::AbstractGeometricAggregateKind(
                AbstractGeometricAggregateKind::MultiPoint(_)
            )
        ));
    }
}
