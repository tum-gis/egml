use crate::Error;
use crate::primitives::ring_property::GmlRingProperty;
use egml_core::model::base::{AsAbstractGml, AsAbstractGmlMut};
use egml_core::model::geometry::primitives::{Polygon, RingProperty};
use quick_xml::se;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlPolygon {
    #[serde(
        rename(serialize = "@gml:id", deserialize = "@id"),
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<String>,

    #[serde(
        rename(serialize = "gml:exterior", deserialize = "exterior",),
        skip_serializing_if = "Option::is_none"
    )]
    pub exterior: Option<GmlRingProperty>,

    #[serde(
        rename(serialize = "gml:interior", deserialize = "interior"),
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub interior: Vec<GmlRingProperty>,
}

impl TryFrom<GmlPolygon> for Polygon {
    type Error = Error;

    fn try_from(item: GmlPolygon) -> Result<Self, Self::Error> {
        let id = item.id.map(|id| id.try_into()).transpose()?;

        let exterior: Option<RingProperty> = item.exterior.map(|x| x.try_into()).transpose()?;
        let interior: Vec<RingProperty> = item
            .interior
            .into_iter()
            .map(|x| x.try_into())
            .collect::<Result<Vec<_>, _>>()?;

        let mut polygon = Polygon::new(exterior, interior)?;
        polygon.set_id(id);
        Ok(polygon)
    }
}

impl From<&Polygon> for GmlPolygon {
    fn from(polygon: &Polygon) -> Self {
        let exterior: Option<GmlRingProperty> = polygon.exterior().map(|x| x.into());

        let interior: Vec<GmlRingProperty> = polygon.interior().iter().map(|x| x.into()).collect();

        Self {
            id: polygon.id().map(|id| id.to_string()),
            exterior,
            interior,
        }
    }
}

/// Serializes a [`Polygon`] to a GML XML string.
///
/// # Errors
///
/// Returns [`Error::XmlSe`] if serialization fails.
pub fn serialize_polygon(polygon: &Polygon) -> Result<String, Error> {
    let gml = GmlPolygon::from(polygon);
    Ok(se::to_string_with_root("gml:Polygon", &gml)?)
}

#[cfg(test)]
mod tests {
    use super::GmlPolygon;
    use crate::primitives::polygon::serialize_polygon;
    use egml_core::model::geometry::DirectPosition;
    use egml_core::model::geometry::primitives::{LinearRing, Polygon, RingKind, RingProperty};
    use quick_xml::de;

    fn make_polygon(points: Vec<DirectPosition>) -> Polygon {
        let ring_kind = RingKind::LinearRing(LinearRing::new(points).unwrap());
        Polygon::new(Some(RingProperty::new(ring_kind)), vec![]).unwrap()
    }

    fn make_square() -> Polygon {
        make_polygon(vec![
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 1.0, 0.0).unwrap(),
            DirectPosition::new(0.0, 1.0, 0.0).unwrap(),
        ])
    }

    #[test]
    fn deserialize_polygon_with_interior_rings() {
        let xml_document = "<gml:Polygon gml:id=\"DEBY_LOD2_4959457_f5d787b1-1fee-441a-898d-0d1bab1fc83f_poly\">
                  <gml:exterior>
                    <gml:LinearRing gml:id=\"DEBY_LOD2_4959457_f5d787b1-1fee-441a-898d-0d1bab1fc83f_poly_0_\">
                      <gml:posList>690985.156 5336010.964 530.92 691004.477 5336059.877 530.92 690987.939 5336066.373 530.92 690968.654 5336017.45 530.92 690985.156 5336010.964 530.92</gml:posList>
                    </gml:LinearRing>
                  </gml:exterior>
                  <gml:interior>
                    <gml:LinearRing gml:id=\"DEBY_LOD2_4959457_f5d787b1-1fee-441a-898d-0d1bab1fc83f_poly_0_.p848qtslOqDyVvoEINYt\">
                      <gml:posList>690997.492 5336051.391 530.92 690996.582 5336049.058 530.92 690987.295 5336052.68 530.92 690988.205 5336055.013 530.92 690997.492 5336051.391 530.92</gml:posList>
                    </gml:LinearRing>
                  </gml:interior>
                  <gml:interior>
                    <gml:LinearRing gml:id=\"DEBY_LOD2_4959457_f5d787b1-1fee-441a-898d-0d1bab1fc83f_poly_0_.kRRfuBGJHBBSenuutsor\">
                      <gml:posList>690976.045 5336024.363 530.92 690976.805 5336026.31 530.92 690986.204 5336022.644 530.92 690985.444 5336020.697 530.92 690976.045 5336024.363 530.92</gml:posList>
                    </gml:LinearRing>
                  </gml:interior>
                </gml:Polygon>";

        let parsed_geometry: GmlPolygon = de::from_str(xml_document).expect("");
        let p: Polygon = parsed_geometry.try_into().unwrap();

        assert_eq!(p.interior().len(), 2)
    }

    #[test]
    fn serialize_polygon_writes_gml_tags() {
        let polygon = make_square();
        let xml = serialize_polygon(&polygon).unwrap();

        assert!(xml.contains("<gml:Polygon"));
        assert!(xml.contains("<gml:exterior"));
        assert!(xml.contains("<gml:LinearRing"));
        assert!(xml.contains("<gml:posList"));
        assert!(!xml.contains("id="));
    }

    #[test]
    fn round_trip_polygon_preserves_points() {
        let polygon = make_square();
        let xml = serialize_polygon(&polygon).unwrap();

        let gml: GmlPolygon = de::from_reader(xml.as_bytes()).unwrap();
        let recovered: Polygon = gml.try_into().unwrap();

        let orig = polygon
            .exterior()
            .unwrap()
            .object
            .as_ref()
            .expect("ring missing");
        let recov = recovered
            .exterior()
            .unwrap()
            .object
            .as_ref()
            .expect("ring missing");
        assert_eq!(orig.points().len(), recov.points().len());
    }

    #[test]
    fn round_trip_polygon_from_xml() {
        let input_xml = "<gml:Polygon>\
            <gml:exterior><gml:LinearRing><gml:posList srsDimension=\"3\">0 0 0 1 0 0 1 1 0 0 1 0 0 0 0</gml:posList></gml:LinearRing></gml:exterior>\
            </gml:Polygon>";

        let gml: GmlPolygon = de::from_reader(input_xml.as_bytes()).unwrap();
        let polygon: Polygon = gml.try_into().unwrap();
        let output_xml = serialize_polygon(&polygon).unwrap();

        assert_eq!(input_xml, output_xml);
    }

    #[test]
    fn serialize_polygon_with_interior_rings() {
        let exterior_pts = vec![
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(4.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(4.0, 4.0, 0.0).unwrap(),
            DirectPosition::new(0.0, 4.0, 0.0).unwrap(),
        ];
        let interior_pts = vec![
            DirectPosition::new(1.0, 1.0, 0.0).unwrap(),
            DirectPosition::new(3.0, 1.0, 0.0).unwrap(),
            DirectPosition::new(3.0, 3.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 3.0, 0.0).unwrap(),
        ];
        let exterior = RingKind::LinearRing(LinearRing::new(exterior_pts).unwrap());
        let interior = RingKind::LinearRing(LinearRing::new(interior_pts).unwrap());
        let polygon = Polygon::new(
            Some(RingProperty::new(exterior)),
            vec![RingProperty::new(interior)],
        )
        .unwrap();

        let xml = serialize_polygon(&polygon).unwrap();

        assert!(xml.contains("<gml:interior"));
        let gml: GmlPolygon = de::from_reader(xml.as_bytes()).unwrap();
        let recovered: Polygon = gml.try_into().unwrap();
        assert_eq!(recovered.interior().len(), 1);
    }
}
