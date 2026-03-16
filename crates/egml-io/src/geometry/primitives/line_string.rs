use crate::geometry::direct_position_list::GmlDirectPositionList;
use crate::{Error, GmlDirectPosition};
use egml_core::model::base::{AsAbstractGml, AsAbstractGmlMut};
use egml_core::model::geometry::DirectPosition;
use egml_core::model::geometry::primitives::{AbstractCurve, LineString};
use quick_xml::se;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlLineString {
    #[serde(
        rename(serialize = "@gml:id", deserialize = "@id"),
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<String>,

    #[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
    pub content: Option<GmlLineStringContent>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum GmlLineStringContent {
    #[serde(rename(serialize = "gml:posList", deserialize = "posList"))]
    PosList(GmlDirectPositionList),

    #[serde(rename(serialize = "gml:pos", deserialize = "pos"))]
    Pos(Vec<GmlDirectPosition>),
}

impl TryFrom<GmlLineStringContent> for Vec<DirectPosition> {
    type Error = Error;

    fn try_from(value: GmlLineStringContent) -> Result<Self, Self::Error> {
        match value {
            GmlLineStringContent::PosList(x) => x.try_into(),
            GmlLineStringContent::Pos(x) => {
                let points: Vec<DirectPosition> = x
                    .into_iter()
                    .map(|p| p.try_into())
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(points)
            }
        }
    }
}

impl TryFrom<GmlLineString> for LineString {
    type Error = Error;

    fn try_from(value: GmlLineString) -> Result<Self, Self::Error> {
        let id = value.id.map(|id| id.try_into()).transpose()?;
        let mut abstract_curve = AbstractCurve::default();
        abstract_curve.set_id(id);

        let points: Vec<DirectPosition> = value
            .content
            .ok_or(Error::ElementNotFound("No element found".to_string()))?
            .try_into()?;

        let linear_ring = LineString::new(abstract_curve, points)?;
        Ok(linear_ring)
    }
}

impl From<&LineString> for GmlLineString {
    fn from(line: &LineString) -> Self {
        Self {
            id: line.id().map(|id| id.to_string()),
            content: Some(GmlLineStringContent::PosList(GmlDirectPositionList::from(
                line.points(),
            ))),
        }
    }
}

/// Serializes a [`LineString`] to a GML XML string.
///
/// # Errors
///
/// Returns [`Error::XmlSe`] if serialization fails.
pub fn serialize_line_string(line: &LineString) -> Result<String, Error> {
    let gml = GmlLineString::from(line);
    Ok(se::to_string_with_root("gml:LineString", &gml)?)
}

#[cfg(test)]
mod tests {
    use super::GmlLineString;
    use crate::primitives::line_string::serialize_line_string;
    use egml_core::model::geometry::DirectPosition;
    use egml_core::model::geometry::primitives::{AbstractCurve, LineString};
    use quick_xml::de;

    fn make_line_string() -> LineString {
        let points = vec![
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(2.0, 0.0, 0.0).unwrap(),
        ];
        LineString::new(AbstractCurve::default(), points).unwrap()
    }

    #[test]
    fn deserialize_line_string() {
        let xml_document = b"<gml:LineString>
                      <gml:posList srsDimension=\"3\">0.0 0.0 0.0 1.0 1.0 1.0 2.0 2.0 2.0</gml:posList>
                    </gml:LineString>";

        let parsed_geometry: GmlLineString = de::from_reader(xml_document.as_ref()).expect("");
        let line_string: LineString = parsed_geometry.try_into().unwrap();
        assert_eq!(line_string.points().len(), 3);
    }

    #[test]
    fn serialize_line_string_writes_gml_tags() {
        let line = make_line_string();
        let xml = serialize_line_string(&line).unwrap();

        assert!(xml.contains("<gml:LineString"));
        assert!(xml.contains("<gml:posList"));
        assert!(!xml.contains("id="));
    }

    #[test]
    fn round_trip_line_string_preserves_points() {
        let line = make_line_string();
        let xml = serialize_line_string(&line).unwrap();

        let gml: GmlLineString = de::from_reader(xml.as_bytes()).unwrap();
        let recovered: LineString = gml.try_into().unwrap();

        assert_eq!(recovered.points().len(), line.points().len());
        for (a, b) in recovered.points().iter().zip(line.points().iter()) {
            assert_eq!(a.x(), b.x());
            assert_eq!(a.y(), b.y());
            assert_eq!(a.z(), b.z());
        }
    }

    #[test]
    fn round_trip_line_string_from_xml() {
        let input_xml = "<gml:LineString>\
            <gml:posList srsDimension=\"3\">0 0 0 1 0 0 2 0 0</gml:posList>\
            </gml:LineString>";

        let gml: GmlLineString = de::from_reader(input_xml.as_bytes()).unwrap();
        let line: LineString = gml.try_into().unwrap();
        let output_xml = serialize_line_string(&line).unwrap();

        assert_eq!(input_xml, output_xml);
    }

    #[test]
    fn round_trip_line_string_preserves_float_precision() {
        let points = vec![
            DirectPosition::new(678058.447, 5403817.567, 424.209).unwrap(),
            DirectPosition::new(678058.275, 5403817.484, 424.209).unwrap(),
            DirectPosition::new(678058.689, 5403816.628, 424.209).unwrap(),
        ];
        let line = LineString::new(AbstractCurve::default(), points.clone()).unwrap();

        let xml = serialize_line_string(&line).unwrap();
        let gml: GmlLineString = de::from_reader(xml.as_bytes()).unwrap();
        let recovered: LineString = gml.try_into().unwrap();

        for (a, b) in recovered.points().iter().zip(points.iter()) {
            assert_eq!(a.x(), b.x());
            assert_eq!(a.y(), b.y());
            assert_eq!(a.z(), b.z());
        }
    }
}
