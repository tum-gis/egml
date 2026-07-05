use crate::Error;
use crate::primitives::GmlCurveProperty;
use egml_core::model::base::{AsAbstractGml, AsAbstractGmlMut};
use egml_core::model::geometry::aggregates::MultiCurve;
use egml_core::model::geometry::primitives::CurveProperty;
use egml_core::model::geometry::{AsAbstractGeometry, AsAbstractGeometryMut};
use quick_xml::{DeError, de, se};
use serde::{Deserialize, Serialize};
use std::io::BufRead;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlMultiCurve {
    #[serde(
        rename(serialize = "@gml:id", deserialize = "@id"),
        skip_serializing_if = "Option::is_none"
    )]
    id: Option<String>,

    #[serde(rename = "@srsDimension", skip_serializing_if = "Option::is_none")]
    srs_dimension: Option<u32>,

    #[serde(
        rename(serialize = "gml:curveMember", deserialize = "curveMember"),
        default
    )]
    members: Vec<GmlCurveProperty>,
}

impl TryFrom<GmlMultiCurve> for MultiCurve {
    type Error = Error;

    fn try_from(item: GmlMultiCurve) -> Result<Self, Self::Error> {
        let id = item.id.map(|id| id.try_into()).transpose()?;
        let curve_members: Vec<CurveProperty> = item
            .members
            .into_iter()
            .map(|x| x.try_into())
            .collect::<Result<Vec<CurveProperty>, Error>>()?;

        let mut multi_curve = MultiCurve::new(curve_members)?;
        multi_curve.set_id(id);
        multi_curve.set_srs_dimension(item.srs_dimension);
        Ok(multi_curve)
    }
}

impl From<&MultiCurve> for GmlMultiCurve {
    fn from(item: &MultiCurve) -> Self {
        Self {
            id: item.id().map(|id| id.to_string()),
            srs_dimension: item.srs_dimension(),
            members: item
                .curve_member()
                .iter()
                .map(GmlCurveProperty::from)
                .collect(),
        }
    }
}

pub fn deserialize_multi_curve<R: BufRead>(reader: R) -> Result<MultiCurve, Error> {
    let parsed_geometry: Result<GmlMultiCurve, DeError> = de::from_reader(reader);
    parsed_geometry?.try_into()
}

/// Serializes a [`MultiCurve`] to a GML XML string.
///
/// # Errors
///
/// Returns [`Error::XmlSe`] if serialization fails.
pub fn serialize_multi_curve(multi_curve: &MultiCurve) -> Result<String, Error> {
    let gml = GmlMultiCurve::from(multi_curve);
    Ok(se::to_string_with_root("gml:MultiCurve", &gml)?)
}

#[cfg(test)]
mod tests {
    use super::GmlMultiCurve;
    use crate::aggregates::multi_curve::{deserialize_multi_curve, serialize_multi_curve};
    use egml_core::model::geometry::DirectPosition;
    use egml_core::model::geometry::aggregates::MultiCurve;
    use egml_core::model::geometry::primitives::LineString;
    use egml_core::model::geometry::primitives::{CurveKind, CurveProperty};
    use quick_xml::de;

    fn make_multi_curve() -> MultiCurve {
        let points = vec![
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 1.0, 1.0).unwrap(),
            DirectPosition::new(2.0, 2.0, 2.0).unwrap(),
        ];
        let curve_kind = CurveKind::LineString(LineString::new(points).unwrap());
        MultiCurve::new([CurveProperty::new(curve_kind)]).unwrap()
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

        let parsed_geometry: GmlMultiCurve = de::from_reader(xml_document.as_ref()).expect("");
        let multi_curve: MultiCurve = parsed_geometry.try_into().unwrap();
        assert_eq!(multi_curve.curve_member().len(), 1);
    }

    #[test]
    fn serialize_multi_curve_writes_gml_tags() {
        let multi_curve = make_multi_curve();
        let xml = serialize_multi_curve(&multi_curve).unwrap();

        assert!(xml.contains("<gml:MultiCurve"));
        assert!(xml.contains("<gml:curveMember"));
        assert!(xml.contains("<gml:LineString"));
        assert!(xml.contains("<gml:posList"));
        assert!(!xml.contains("id="));
    }

    #[test]
    fn round_trip_multi_curve_preserves_member_count() {
        let multi_curve = make_multi_curve();
        let xml = serialize_multi_curve(&multi_curve).unwrap();
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

        let gml: GmlMultiCurve = de::from_reader(input_xml.as_bytes()).unwrap();
        let multi_curve: MultiCurve = gml.try_into().unwrap();
        let output_xml = serialize_multi_curve(&multi_curve).unwrap();

        assert_eq!(input_xml, output_xml);
    }
}
