use crate::Error;
use crate::codec::geometry::GmlDirectPosition;
use crate::codec::geometry::direct_position_list::GmlDirectPositionList;
use crate::codec::geometry::primitives::abstract_curve::{
    deserialize_abstract_curve, serialize_abstract_curve,
};
use crate::util::{
    Formatting, GmlElement, XmlNode, XmlNodeContent, dedup_adjacent_positions,
    extract_xml_element_spans, serialize_inner,
};
use egml_core::model::geometry::DirectPosition;
use egml_core::model::geometry::primitives::{AsAbstractCurve, LineString};
use quick_xml::de;
use serde::{Deserialize, Serialize};

pub fn deserialize_line_string(xml_document: &[u8]) -> Result<LineString, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let abstract_curve = deserialize_abstract_curve(xml_document, &spans)?;

    let parsed: GmlLineString = de::from_reader(xml_document)?;
    let mut points: Vec<DirectPosition> = parsed.content.unwrap().try_into()?;
    dedup_adjacent_positions(&mut points, "LineString");

    let line_string = LineString::from_abstract_curve(abstract_curve, points)?;
    Ok(line_string)
}

pub fn serialize_line_string(
    line_string: &LineString,
    formatting: Formatting,
) -> Result<XmlNode, Error> {
    let mut xml_node_parts = serialize_abstract_curve(line_string.abstract_curve(), formatting)?;

    if let Some(raw) = serialize_inner(GmlLineString::from(line_string), formatting)? {
        xml_node_parts.content.push(XmlNodeContent::Raw(raw));
    }

    Ok(XmlNode::new(GmlElement::LineString.into(), xml_node_parts))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlLineString {
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
        let points: Vec<DirectPosition> = value
            .content
            .ok_or(Error::ElementNotFound("No element found".to_string()))?
            .try_into()?;

        let line_string = LineString::new(points)?;
        Ok(line_string)
    }
}

impl From<&LineString> for GmlLineString {
    fn from(line: &LineString) -> Self {
        Self {
            content: Some(GmlLineStringContent::PosList(GmlDirectPositionList::from(
                line.points(),
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::GmlLineString;
    use crate::codec::geometry::primitives::line_string::serialize_line_string;
    use crate::codec::geometry::primitives::{deserialize_linear_ring, serialize_linear_ring};
    use crate::util::{Formatting, extract_xml_element_spans};
    use egml_core::model::geometry::DirectPosition;
    use egml_core::model::geometry::primitives::LineString;
    use quick_xml::de;

    fn make_line_string() -> LineString {
        let points = vec![
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(2.0, 0.0, 0.0).unwrap(),
        ];
        LineString::new(points).unwrap()
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
    fn deserialize_line_string_repairs_adjacent_duplicate_points() {
        let xml_document = b"<gml:LineString>
                      <gml:posList srsDimension=\"3\">0 0 0 1 0 0 1 0 0 2 0 0</gml:posList>
                    </gml:LineString>";

        let line_string = super::deserialize_line_string(xml_document).expect("should deserialize");

        assert_eq!(line_string.points().len(), 3);
    }

    #[test]
    fn serialize_line_string_writes_gml_tags() {
        let line_string = make_line_string();
        let xml_node = serialize_line_string(&line_string, Formatting::Compact).unwrap();
        let xml = xml_node
            .to_string(Formatting::Compact)
            .expect("should serialize");

        assert!(xml.contains("<gml:LineString"));
        assert!(xml.contains("<gml:posList"));
        assert!(!xml.contains("id="));
    }

    #[test]
    fn round_trip_line_string_preserves_points() {
        let line_string = make_line_string();
        let xml_node = serialize_line_string(&line_string, Formatting::Compact).unwrap();
        let xml = xml_node
            .to_string(Formatting::Compact)
            .expect("should serialize");

        let recovered_line_string =
            deserialize_linear_ring(xml.as_ref()).expect("should deserialize");

        assert_eq!(
            recovered_line_string.points().len(),
            line_string.points().len()
        );
        for (a, b) in recovered_line_string
            .points()
            .iter()
            .zip(line_string.points().iter())
        {
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
        let line_string: LineString = gml.try_into().unwrap();
        let xml_node = serialize_line_string(&line_string, Formatting::Compact).unwrap();
        let output_xml = xml_node
            .to_string(Formatting::Compact)
            .expect("should serialize");

        assert_eq!(input_xml, output_xml);
    }

    #[test]
    fn round_trip_line_string_preserves_float_precision() {
        let points = vec![
            DirectPosition::new(678058.447, 5403817.567, 424.209).unwrap(),
            DirectPosition::new(678058.275, 5403817.484, 424.209).unwrap(),
            DirectPosition::new(678058.689, 5403816.628, 424.209).unwrap(),
        ];
        let line_string = LineString::new(points.clone()).unwrap();

        let xml_node = serialize_line_string(&line_string, Formatting::Compact).unwrap();
        let xml = xml_node
            .to_string(Formatting::Compact)
            .expect("should serialize");

        let recovered_line_string =
            deserialize_linear_ring(xml.as_ref()).expect("should deserialize");

        for (a, b) in recovered_line_string.points().iter().zip(points.iter()) {
            assert_eq!(a.x(), b.x());
            assert_eq!(a.y(), b.y());
            assert_eq!(a.z(), b.z());
        }
    }
}
