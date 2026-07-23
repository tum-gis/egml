use crate::Error;
use crate::codec::geometry::aggregates::{
    deserialize_abstract_geometric_aggregate, serialize_abstract_geometric_aggregate,
};
use crate::codec::geometry::primitives::{
    deserialize_abstract_curve_property, serialize_abstract_curve_property,
};
use crate::util::{
    Formatting, GmlElement, XmlNode, XmlNodeContent, collect_children, extract_xml_element_spans,
};
use egml_core::model::geometry::aggregates::{AsAbstractGeometricAggregate, MultiCurve};

pub fn deserialize_multi_curve(xml_document: &[u8]) -> Result<MultiCurve, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let abstract_geometric_aggregate =
        deserialize_abstract_geometric_aggregate(xml_document, &spans)?;

    let surface_members = collect_children(
        xml_document,
        &spans,
        GmlElement::CurveMemberProperty,
        deserialize_abstract_curve_property,
    )?;

    Ok(MultiCurve::from_abstract_geometric_aggregate(
        abstract_geometric_aggregate,
        surface_members,
    )?)
}

pub fn serialize_multi_curve(
    multi_curve: &MultiCurve,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut parts = serialize_abstract_geometric_aggregate(
        multi_curve.abstract_geometric_aggregate(),
        formatting,
    )?;

    for member in multi_curve.curve_member() {
        parts
            .content
            .push(XmlNodeContent::Child(serialize_abstract_curve_property(
                member,
                formatting,
                GmlElement::CurveMemberProperty.into(),
            )?));
    }

    Ok(XmlNode::new(GmlElement::MultiCurve.into(), parts))
}

#[cfg(test)]
mod tests {
    use crate::codec::geometry::aggregates::multi_curve::{
        deserialize_multi_curve, serialize_multi_curve,
    };
    use crate::util::Formatting;
    use egml_core::model::geometry::DirectPosition;
    use egml_core::model::geometry::aggregates::MultiCurve;
    use egml_core::model::geometry::primitives::LineString;
    use egml_core::model::geometry::primitives::{AbstractCurveKind, AbstractCurveProperty};

    fn make_multi_curve() -> MultiCurve {
        let points = vec![
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 1.0, 1.0).unwrap(),
            DirectPosition::new(2.0, 2.0, 2.0).unwrap(),
        ];
        let curve_kind = AbstractCurveKind::LineString(LineString::new(points).unwrap());
        MultiCurve::new([AbstractCurveProperty::from_object(curve_kind)]).unwrap()
    }

    #[test]
    fn test_deserialize_multi_curve() {
        let xml_document = b"<gml:MultiCurve>
                  <gml:curveMember>
                    <gml:LineString>
                      <gml:posList srsDimension=\"3\">0.0 0.0 0.0 1.0 1.0 1.0 2.0 2.0 2.0</gml:posList>
                    </gml:LineString>
                  </gml:curveMember>
                </gml:MultiCurve>";

        let multi_curve: MultiCurve = deserialize_multi_curve(xml_document.as_ref()).unwrap();
        assert_eq!(multi_curve.curve_member().len(), 1);
    }

    #[test]
    fn serialize_multi_curve_writes_gml_tags() {
        let multi_curve = make_multi_curve();
        let xml_node = serialize_multi_curve(&multi_curve, Formatting::Compact).unwrap();
        let xml = xml_node.to_string(Formatting::Compact).unwrap();

        assert!(xml.contains("<gml:MultiCurve"));
        assert!(xml.contains("<gml:curveMember"));
        assert!(xml.contains("<gml:LineString"));
        assert!(xml.contains("<gml:posList"));
        assert!(!xml.contains("id="));
    }

    #[test]
    fn round_trip_multi_curve_preserves_member_count() {
        let multi_curve = make_multi_curve();
        let xml_node = serialize_multi_curve(&multi_curve, Formatting::Compact).unwrap();
        let xml = xml_node.to_string(Formatting::Compact).unwrap();

        let recovered = deserialize_multi_curve(xml.as_bytes()).unwrap();

        assert_eq!(
            recovered.curve_member().len(),
            multi_curve.curve_member().len()
        );
    }

    #[test]
    fn round_trip_multi_curve_from_xml() {
        let input_xml = "<gml:MultiCurve gml:id=\"test-id\">\
            <gml:curveMember><gml:LineString><gml:posList srsDimension=\"3\">0 0 0 1 1 1 2 2 2</gml:posList></gml:LineString></gml:curveMember>\
            </gml:MultiCurve>";

        let multi_curve: MultiCurve = deserialize_multi_curve(input_xml.as_bytes()).unwrap();

        let output_xml_node = serialize_multi_curve(&multi_curve, Formatting::Compact).unwrap();
        let output_xml = output_xml_node.to_string(Formatting::Compact).unwrap();

        assert_eq!(input_xml, output_xml);
    }
}
