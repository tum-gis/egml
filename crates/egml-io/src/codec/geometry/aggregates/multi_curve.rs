use crate::Error;
use crate::codec::geometry::aggregates::{
    deserialize_abstract_geometric_aggregate, serialize_abstract_geometric_aggregate,
    serialize_abstract_geometric_aggregate_attributes,
};
use crate::codec::geometry::primitives::{
    deserialize_abstract_curve_property, serialize_abstract_curve_property,
};
use crate::util::{
    DeserializationConfig, GmlElement, GmlNamespace, XmlDocumentIndex, XmlFragmentWriter,
    collect_children,
};
use egml_core::model::geometry::aggregates::{AsAbstractGeometricAggregate, MultiCurve};
use std::io::Write;

pub fn deserialize_multi_curve(
    xml_document: &[u8],
    index: &XmlDocumentIndex<GmlElement>,
    config: &DeserializationConfig,
) -> Result<MultiCurve, Error> {
    let index = if index.is_truncated() {
        &XmlDocumentIndex::from_scan(xml_document, config.rescan_depth())?
    } else {
        index
    };

    let abstract_geometric_aggregate =
        deserialize_abstract_geometric_aggregate(xml_document, index, config)?;

    let surface_members = collect_children(
        xml_document,
        index,
        GmlElement::CurveMemberProperty,
        config,
        deserialize_abstract_curve_property,
    )?;

    Ok(MultiCurve::from_abstract_geometric_aggregate(
        abstract_geometric_aggregate,
        surface_members,
    )?)
}

pub fn serialize_multi_curve<W: Write>(
    multi_curve: &MultiCurve,
    xml_fragment_writer: &mut XmlFragmentWriter<W>,
) -> Result<(), Error> {
    let attributes = serialize_multi_curve_attributes(multi_curve);

    xml_fragment_writer.write_start_event_with_attributes(
        GmlNamespace::Gml,
        GmlElement::MultiCurve,
        attributes,
    )?;

    serialize_abstract_geometric_aggregate(
        multi_curve.abstract_geometric_aggregate(),
        xml_fragment_writer,
    )?;

    for member in multi_curve.curve_member() {
        serialize_abstract_curve_property(
            member,
            xml_fragment_writer,
            GmlNamespace::Gml,
            GmlElement::CurveMemberProperty,
        )?;
    }

    xml_fragment_writer.write_end_event(GmlNamespace::Gml, GmlElement::MultiCurve)?;

    Ok(())
}

pub fn serialize_multi_curve_attributes(multi_curve: &MultiCurve) -> Vec<(String, String)> {
    serialize_abstract_geometric_aggregate_attributes(multi_curve.abstract_geometric_aggregate())
}

#[cfg(test)]
mod tests {
    // Test-only convenience: builds the index the real function now
    // requires, so existing single-argument call sites below don't all
    // need to construct one by hand.
    fn deserialize(xml_document: &[u8]) -> Result<super::MultiCurve, crate::Error> {
        let index = crate::util::XmlDocumentIndex::from_scan(xml_document, None)?;
        super::deserialize_multi_curve(
            xml_document,
            &index,
            &crate::util::DeserializationConfig::default(),
        )
    }

    use crate::codec::geometry::aggregates::multi_curve::serialize_multi_curve;
    use crate::util::{Formatting, XmlFragmentWriter};
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

        let multi_curve: MultiCurve = deserialize(xml_document.as_ref()).unwrap();
        assert_eq!(multi_curve.curve_member().len(), 1);
    }

    #[test]
    fn serialize_multi_curve_writes_gml_tags() {
        let multi_curve = make_multi_curve();
        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_multi_curve(&multi_curve, &mut xml_fragment_writer).unwrap();
        let xml = String::from_utf8(xml_fragment_writer.into_bytes()).unwrap();

        assert!(xml.contains("<gml:MultiCurve"));
        assert!(xml.contains("<gml:curveMember"));
        assert!(xml.contains("<gml:LineString"));
        assert!(xml.contains("<gml:posList"));
        assert!(!xml.contains("id="));
    }

    #[test]
    fn round_trip_multi_curve_preserves_member_count() {
        let multi_curve = make_multi_curve();
        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_multi_curve(&multi_curve, &mut xml_fragment_writer).unwrap();
        let xml = String::from_utf8(xml_fragment_writer.into_bytes()).unwrap();

        let recovered = deserialize(xml.as_bytes()).unwrap();

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

        let multi_curve: MultiCurve = deserialize(input_xml.as_bytes()).unwrap();

        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_multi_curve(&multi_curve, &mut xml_fragment_writer).unwrap();
        let output_xml = String::from_utf8(xml_fragment_writer.into_bytes()).unwrap();

        assert_eq!(input_xml, output_xml);
    }
}
