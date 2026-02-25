use crate::geometry::direct_position_list::GmlDirectPositionList;
use crate::{Error, GmlDirectPosition};
use egml_core::model::base::AsAbstractGmlMut;
use egml_core::model::geometry::DirectPosition;
use egml_core::model::geometry::primitives::{AbstractCurve, LineString};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlLineString {
    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "$value")]
    pub content: Option<GmlLineStringContent>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum GmlLineStringContent {
    #[serde(rename = "posList")]
    PosList(GmlDirectPositionList),

    #[serde(rename = "pos")]
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

#[cfg(test)]
mod tests {
    use crate::primitives::line_string::GmlLineString;
    use egml_core::model::geometry::primitives::{LineString, LinearRing};
    use quick_xml::de;

    #[test]
    fn parsing_line_string() {
        let xml_document = b"<gml:LineString>
                      <gml:posList srsDimension=\"3\">0.0 0.0 0.0 1.0 1.0 1.0 2.0 2.0 2.0</gml:posList>
                    </gml:LineString>";

        let parsed_geometry: GmlLineString = de::from_reader(xml_document.as_ref()).expect("");
        let line_string: LineString = parsed_geometry.try_into().unwrap();
        assert_eq!(line_string.points().len(), 3);
    }
}
