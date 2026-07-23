use crate::Error;
use crate::codec::geometry::aggregates::{
    deserialize_multi_curve, deserialize_multi_geometry, deserialize_multi_point,
    deserialize_multi_surface, serialize_multi_curve, serialize_multi_geometry,
    serialize_multi_point, serialize_multi_surface,
};
use crate::util::{Formatting, GmlElement, XmlElementSpans, XmlNode};
use egml_core::model::geometry::aggregates::AbstractGeometricAggregateKind;

pub fn deserialize_abstract_geometric_aggregate_kind(
    xml_document: &[u8],
    spans: &XmlElementSpans<GmlElement>,
) -> Result<Option<AbstractGeometricAggregateKind>, Error> {
    if let Some(span) = spans.first(GmlElement::MultiCurve) {
        let multi_curve = deserialize_multi_curve(&xml_document[span.start..span.end])?;
        return Ok(Some(multi_curve.into()));
    }

    if let Some(span) = spans.first(GmlElement::MultiGeometry) {
        let multi_geometry = deserialize_multi_geometry(&xml_document[span.start..span.end])?;
        return Ok(Some(multi_geometry.into()));
    }

    if let Some(span) = spans.first(GmlElement::MultiPoint) {
        let multi_point = deserialize_multi_point(&xml_document[span.start..span.end])?;
        return Ok(Some(multi_point.into()));
    }

    if let Some(span) = spans.first(GmlElement::MultiSurface) {
        let multi_surface = deserialize_multi_surface(&xml_document[span.start..span.end])?;
        return Ok(Some(multi_surface.into()));
    }

    Ok(None)
}

pub fn serialize_abstract_geometric_aggregate_kind(
    abstract_geometric_aggregate_kind: &AbstractGeometricAggregateKind,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    match abstract_geometric_aggregate_kind {
        AbstractGeometricAggregateKind::MultiCurve(x) => serialize_multi_curve(x, formatting),
        AbstractGeometricAggregateKind::MultiGeometry(x) => serialize_multi_geometry(x, formatting),
        AbstractGeometricAggregateKind::MultiPoint(x) => serialize_multi_point(x, formatting),
        AbstractGeometricAggregateKind::MultiSurface(x) => serialize_multi_surface(x, formatting),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        deserialize_abstract_geometric_aggregate_kind, serialize_abstract_geometric_aggregate_kind,
    };
    use crate::util::{Formatting, extract_xml_element_spans};
    use egml_core::model::geometry::DirectPosition;
    use egml_core::model::geometry::aggregates::{
        AbstractGeometricAggregateKind, MultiCurve, MultiPoint, MultiSurface,
    };
    use egml_core::model::geometry::primitives::{
        AbstractCurveKind, AbstractCurveProperty, AbstractRingKind, AbstractRingProperty,
        AbstractSurfaceKind, AbstractSurfaceProperty, LineString, LinearRing, Point, PointProperty,
        Polygon,
    };

    fn make_multi_point() -> MultiPoint {
        let mut mp = MultiPoint::new(None).unwrap();
        mp.set_point_member(vec![PointProperty::from_object(Point::new(
            DirectPosition::new(1.0, 2.0, 3.0).unwrap(),
        ))]);
        mp
    }

    fn make_multi_curve() -> MultiCurve {
        let points = vec![
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 1.0, 1.0).unwrap(),
            DirectPosition::new(2.0, 2.0, 2.0).unwrap(),
        ];
        MultiCurve::new([AbstractCurveProperty::from_object(
            AbstractCurveKind::LineString(LineString::new(points).unwrap()),
        )])
        .unwrap()
    }

    fn make_multi_surface() -> MultiSurface {
        let points = vec![
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(0.0, 1.0, 0.0).unwrap(),
        ];
        let ring = AbstractRingKind::LinearRing(LinearRing::new(points).unwrap());
        let polygon = Polygon::new(Some(AbstractRingProperty::from_object(ring)), []).unwrap();
        MultiSurface::new([AbstractSurfaceProperty::from_object(
            AbstractSurfaceKind::Polygon(polygon),
        )])
        .unwrap()
    }

    #[test]
    fn deserialize_multi_point_kind() {
        let xml = b"<gml:someParent>\
            <gml:MultiPoint>\
            <gml:pointMember><gml:Point><gml:pos srsDimension=\"3\">1 2 3</gml:pos></gml:Point></gml:pointMember>\
            </gml:MultiPoint>\
            </gml:someParent>";

        let spans = extract_xml_element_spans(xml).unwrap();
        let kind = deserialize_abstract_geometric_aggregate_kind(xml, &spans)
            .unwrap()
            .unwrap();

        assert!(matches!(
            kind,
            AbstractGeometricAggregateKind::MultiPoint(_)
        ));
    }

    #[test]
    fn deserialize_multi_curve_kind() {
        let xml = b"<gml:someParent>\
            <gml:MultiCurve>\
            <gml:curveMember><gml:LineString><gml:posList srsDimension=\"3\">0 0 0 1 1 1 2 2 2</gml:posList></gml:LineString></gml:curveMember>\
            </gml:MultiCurve>\
            </gml:someParent>";

        let spans = extract_xml_element_spans(xml).unwrap();
        let kind = deserialize_abstract_geometric_aggregate_kind(xml, &spans)
            .unwrap()
            .unwrap();

        assert!(matches!(
            kind,
            AbstractGeometricAggregateKind::MultiCurve(_)
        ));
    }

    #[test]
    fn deserialize_multi_surface_kind() {
        let xml = b"<gml:someParent>\
            <gml:MultiSurface>\
            <gml:surfaceMember><gml:Polygon><gml:exterior><gml:LinearRing>\
            <gml:posList srsDimension=\"3\">0 0 0 1 0 0 0 1 0 0 0 0</gml:posList>\
            </gml:LinearRing></gml:exterior></gml:Polygon></gml:surfaceMember>\
            </gml:MultiSurface>\
            </gml:someParent>";

        let spans = extract_xml_element_spans(xml).unwrap();
        let kind = deserialize_abstract_geometric_aggregate_kind(xml, &spans)
            .unwrap()
            .unwrap();

        assert!(matches!(
            kind,
            AbstractGeometricAggregateKind::MultiSurface(_)
        ));
    }

    #[test]
    fn deserialize_returns_none_when_no_aggregate() {
        let xml = b"<gml:someParent/>";

        let spans = extract_xml_element_spans(xml).unwrap();
        let kind = deserialize_abstract_geometric_aggregate_kind(xml, &spans).unwrap();

        assert!(kind.is_none());
    }

    #[test]
    fn serialize_multi_point_kind() {
        let kind = AbstractGeometricAggregateKind::MultiPoint(make_multi_point());
        let xml_node =
            serialize_abstract_geometric_aggregate_kind(&kind, Formatting::Compact).unwrap();
        let xml = xml_node.to_string(Formatting::Compact).unwrap();

        assert!(xml.contains("<gml:MultiPoint"));
        assert!(xml.contains("<gml:pointMember"));
        assert!(xml.contains("<gml:Point"));
    }

    #[test]
    fn serialize_multi_curve_kind() {
        let kind = AbstractGeometricAggregateKind::MultiCurve(make_multi_curve());
        let xml_node =
            serialize_abstract_geometric_aggregate_kind(&kind, Formatting::Compact).unwrap();
        let xml = xml_node.to_string(Formatting::Compact).unwrap();

        assert!(xml.contains("<gml:MultiCurve"));
        assert!(xml.contains("<gml:curveMember"));
        assert!(xml.contains("<gml:LineString"));
    }

    #[test]
    fn serialize_multi_surface_kind() {
        let kind = AbstractGeometricAggregateKind::MultiSurface(make_multi_surface());
        let xml_node =
            serialize_abstract_geometric_aggregate_kind(&kind, Formatting::Compact).unwrap();
        let xml = xml_node.to_string(Formatting::Compact).unwrap();

        assert!(xml.contains("<gml:MultiSurface"));
        assert!(xml.contains("<gml:surfaceMember"));
        assert!(xml.contains("<gml:Polygon"));
    }

    #[test]
    fn round_trip_multi_point_kind() {
        let kind = AbstractGeometricAggregateKind::MultiPoint(make_multi_point());
        let xml_node =
            serialize_abstract_geometric_aggregate_kind(&kind, Formatting::Compact).unwrap();
        let xml = xml_node.to_string(Formatting::Compact).unwrap();

        let wrapper = format!("<gml:parent>{xml}</gml:parent>");
        let spans = extract_xml_element_spans(wrapper.as_bytes()).unwrap();
        let recovered = deserialize_abstract_geometric_aggregate_kind(wrapper.as_bytes(), &spans)
            .unwrap()
            .unwrap();

        assert!(matches!(
            recovered,
            AbstractGeometricAggregateKind::MultiPoint(_)
        ));
    }

    #[test]
    fn round_trip_multi_surface_kind() {
        let kind = AbstractGeometricAggregateKind::MultiSurface(make_multi_surface());
        let xml_node =
            serialize_abstract_geometric_aggregate_kind(&kind, Formatting::Compact).unwrap();
        let xml = xml_node.to_string(Formatting::Compact).unwrap();

        let wrapper = format!("<gml:parent>{xml}</gml:parent>");
        let spans = extract_xml_element_spans(wrapper.as_bytes()).unwrap();
        let recovered = deserialize_abstract_geometric_aggregate_kind(wrapper.as_bytes(), &spans)
            .unwrap()
            .unwrap();

        assert!(matches!(
            recovered,
            AbstractGeometricAggregateKind::MultiSurface(_)
        ));
    }
}
