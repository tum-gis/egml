use crate::GmlDirectPosition;
use crate::error::Error;
use crate::geometry::direct_position_list::GmlDirectPositionList;
use egml_core::model::base::{AsAbstractGml, AsAbstractGmlMut};
use egml_core::model::geometry::DirectPosition;
use egml_core::model::geometry::primitives::{AbstractRing, LinearRing};
use quick_xml::se;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlLinearRing {
    #[serde(
        rename(serialize = "@gml:id", deserialize = "@id"),
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<String>,

    #[serde(rename = "$value", skip_serializing_if = "Option::is_none")]
    pub content: Option<GmlLinearRingContent>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum GmlLinearRingContent {
    #[serde(rename(serialize = "gml:posList", deserialize = "posList"))]
    PosList(GmlDirectPositionList),

    #[serde(rename = "pos")]
    Pos(Vec<GmlDirectPosition>),
}

impl TryFrom<GmlLinearRingContent> for Vec<DirectPosition> {
    type Error = Error;

    fn try_from(value: GmlLinearRingContent) -> Result<Self, Self::Error> {
        match value {
            GmlLinearRingContent::PosList(x) => x.try_into(),
            GmlLinearRingContent::Pos(x) => {
                let points: Vec<DirectPosition> = x
                    .into_iter()
                    .map(|p| p.try_into())
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(points)
            }
        }
    }
}

impl TryFrom<GmlLinearRing> for LinearRing {
    type Error = Error;

    fn try_from(value: GmlLinearRing) -> Result<Self, Self::Error> {
        let id = value.id.map(|id| id.try_into()).transpose()?;
        let mut abstract_ring = AbstractRing::default();
        abstract_ring.set_id(id);

        let mut points: Vec<DirectPosition> = value
            .content
            .ok_or(Error::ElementNotFound("No element found".to_string()))?
            .try_into()?;

        points.dedup();
        if points.first().unwrap() == points.last().unwrap() {
            points.pop();
        }

        let linear_ring = LinearRing::new(abstract_ring, points)?;
        Ok(linear_ring)
    }
}

impl From<&LinearRing> for GmlLinearRing {
    fn from(ring: &LinearRing) -> Self {
        // GML requires the ring to be closed: the closing vertex (= first point) must
        // be written explicitly, but LinearRing stores points in open form (no repeat).
        let mut points: Vec<DirectPosition> = ring.points().to_vec();
        if let Some(&first) = ring.points().first() {
            points.push(first);
        }

        Self {
            id: ring.id().map(|id| id.to_string()),
            content: Some(GmlLinearRingContent::PosList(GmlDirectPositionList::from(
                points.as_slice(),
            ))),
        }
    }
}

/// Serializes a [`LinearRing`] to a GML XML string.
///
/// # Errors
///
/// Returns [`Error::XmlSe`] if serialization fails.
pub fn serialize_linear_ring(ring: &LinearRing) -> Result<String, Error> {
    let gml_ring = GmlLinearRing::from(ring);
    Ok(se::to_string_with_root("gml:LinearRing", &gml_ring)?)
}

#[cfg(test)]
mod tests {
    use super::GmlLinearRingContent;
    use crate::Error;
    use crate::primitives::{GmlLinearRing, serialize_linear_ring};
    use egml_core::model::base::Id;
    use egml_core::model::geometry::DirectPosition;
    use egml_core::model::geometry::primitives::{AbstractRing, LinearRing};
    use quick_xml::de;

    fn make_triangle() -> LinearRing {
        let points = vec![
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(0.0, 1.0, 0.0).unwrap(),
        ];
        LinearRing::new(AbstractRing::default(), points).unwrap()
    }

    #[test]
    fn deserialize_linear_ring_with_pos_list_and_id() {
        let xml_document = b"<gml:LinearRing gml:id=\"4018115_LR.d2yyBHNssydL8g3B8MvW\">
<gml:posList srsDimension=\"3\">
678058.447 5403817.567 424.209
678058.275 5403817.484 424.209
678058.689 5403816.628 424.209
678058.871 5403816.718 424.209
678058.447 5403817.567 424.209
</gml:posList>
</gml:LinearRing>";

        let parsed_geometry: GmlLinearRing = de::from_reader(xml_document.as_ref()).expect("");
        let l: LinearRing = parsed_geometry.try_into().unwrap();
        use egml_core::model::base::AsAbstractGml;
        assert!(l.id().is_some());
    }

    #[test]
    fn deserialize_linear_ring_with_pos_list() {
        let xml_document = b"<gml:LinearRing>
                            <gml:posList>350.54400634765625 972.9130249023438 0.11999999731779099 350.5414201635045 968.6025425887852 0.11999999731779099 350.54400634765625 968.6025096366793 0.11999999731779099 350.54400634765625 972.9130249023438 0.11999999731779099</gml:posList>
                        </gml:LinearRing>";

        let parsed_geometry: GmlLinearRing = de::from_reader(xml_document.as_ref()).expect("");
        let result: Result<LinearRing, Error> = parsed_geometry.try_into();

        assert!(result.is_ok());
        let linear_ring = result.unwrap();
        assert_eq!(linear_ring.points().len(), 3);
    }

    #[test]
    fn deserialize_linear_ring_with_duplicate_points_returns_error() {
        let xml_document = b"<gml:LinearRing gml:id=\"DEBY_LOD2_4959457_LR.EEAbfUPItTlOGZGH7VDv\">
                      <gml:posList>691040.851 5336002.449 529.908 691040.741 5336002.172 529.908 691040.851 5336002.449 529.908 691040.851 5336002.449 529.908</gml:posList>
                    </gml:LinearRing>";

        let parsed_geometry: GmlLinearRing = de::from_reader(xml_document.as_ref()).expect("");
        let result: Result<LinearRing, Error> = parsed_geometry.try_into();

        assert!(result.is_err());
    }

    #[test]
    fn serialize_linear_ring_writes_gml_tags() {
        let ring = make_triangle();
        let xml = serialize_linear_ring(&ring).unwrap();

        assert!(xml.contains("<gml:LinearRing"));
        assert!(xml.contains("<gml:posList"));
        assert!(!xml.contains("id="));
    }

    #[test]
    fn serialize_linear_ring_appends_closing_vertex() {
        let ring = make_triangle(); // 3 open points

        // Verify via the intermediate GmlLinearRing that the closing vertex is present
        let gml = GmlLinearRing::from(&ring);
        let positions: Vec<DirectPosition> = match gml.content.unwrap() {
            GmlLinearRingContent::PosList(pos_list) => pos_list.try_into().unwrap(),
            GmlLinearRingContent::Pos(_) => panic!("expected PosList"),
        };
        assert_eq!(positions.len(), 4); // 3 open points + closing vertex
        assert_eq!(positions.first(), positions.last()); // closing vertex equals first point
    }

    #[test]
    fn serialize_linear_ring_with_id() {
        use egml_core::model::base::AsAbstractGmlMut;
        let points = vec![
            DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(0.0, 1.0, 0.0).unwrap(),
            DirectPosition::new(0.0, 0.0, 1.0).unwrap(),
        ];
        let mut abstract_ring = AbstractRing::default();
        abstract_ring.set_id(Some(Id::from_hashed_string("test-ring")));
        let ring = LinearRing::new(abstract_ring, points).unwrap();

        let xml = serialize_linear_ring(&ring).unwrap();
        assert!(xml.contains("id="));
    }

    #[test]
    fn round_trip_linear_ring_preserves_points() {
        let ring = make_triangle();
        let xml = serialize_linear_ring(&ring).unwrap();

        let gml: GmlLinearRing = de::from_reader(xml.as_bytes()).unwrap();
        let recovered: LinearRing = gml.try_into().unwrap();

        assert_eq!(recovered.points().len(), ring.points().len());
        for (a, b) in recovered.points().iter().zip(ring.points().iter()) {
            assert_eq!(a.x(), b.x());
            assert_eq!(a.y(), b.y());
            assert_eq!(a.z(), b.z());
        }
    }

    #[test]
    fn round_trip_linear_ring_preserves_float_precision() {
        let points = vec![
            DirectPosition::new(678058.447, 5403817.567, 424.209).unwrap(),
            DirectPosition::new(678058.275, 5403817.484, 424.209).unwrap(),
            DirectPosition::new(678058.689, 5403816.628, 424.209).unwrap(),
        ];
        let ring = LinearRing::new(AbstractRing::default(), points.clone()).unwrap();

        let xml = serialize_linear_ring(&ring).unwrap();
        let gml: GmlLinearRing = de::from_reader(xml.as_bytes()).unwrap();
        let recovered: LinearRing = gml.try_into().unwrap();

        for (a, b) in recovered.points().iter().zip(points.iter()) {
            assert_eq!(a.x(), b.x());
            assert_eq!(a.y(), b.y());
            assert_eq!(a.z(), b.z());
        }
    }

    #[test]
    fn round_trip_linear_ring_from_xml() {
        let input_xml = "<gml:LinearRing gml:id=\"PolyID7350_878_759628_120742_0\">\
            <gml:posList srsDimension=\"3\">0 0 0 1 0 0 0 1 0 0 0 0</gml:posList>\
            </gml:LinearRing>";

        let gml: GmlLinearRing = de::from_reader(input_xml.as_ref()).unwrap();
        let ring: LinearRing = gml.try_into().unwrap();
        let output_xml = serialize_linear_ring(&ring).unwrap();

        assert_eq!(input_xml, output_xml);
    }

    #[test]
    fn deserialize_linear_ring_with_pos_elements() {
        let xml_document = b"<gml:LinearRing gml:id=\"PolyID7350_878_759628_120742_0\">
                      <gml:pos>457842.0 5439088.0 118.317691453624</gml:pos>
                      <gml:pos>457842.0 5439093.0 115.430940107676</gml:pos>
                      <gml:pos>457842.0 5439093.0 111.8</gml:pos>
                      <gml:pos>457842.0 5439083.0 111.8</gml:pos>
                      <gml:pos>457842.0 5439083.0 115.430940107676</gml:pos>
                      <gml:pos>457842.0 5439088.0 118.317691453624</gml:pos>
                    </gml:LinearRing>";

        let parsed_geometry: GmlLinearRing = de::from_reader(xml_document.as_ref()).expect("");
        let result: Result<LinearRing, Error> = parsed_geometry.try_into();

        assert!(result.is_ok());
        let linear_ring = result.unwrap();
        assert_eq!(linear_ring.points().len(), 5);
    }
}
