use crate::Error;
use crate::primitives::GmlCurveProperty;
use egml_core::model::geometry::aggregates::{AbstractGeometricAggregate, MultiCurve};
use egml_core::model::geometry::primitives::CurveKind;
use quick_xml::{DeError, de};
use serde::{Deserialize, Serialize};
use std::io::BufRead;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlMultiCurve {
    #[serde(rename = "@id")]
    id: Option<String>,

    #[serde(rename = "curveMember", default)]
    members: Vec<GmlCurveProperty>,
}

impl TryFrom<GmlMultiCurve> for MultiCurve {
    type Error = Error;

    fn try_from(item: GmlMultiCurve) -> Result<Self, Self::Error> {
        /*let id = value.id.map(|id| id.try_into()).transpose()?;
        let abstract_gml = AbstractGml::with_optional_id(id);
        let abstract_geometry = AbstractGeometry::new(abstract_gml);*/

        let surface_members: Vec<CurveKind> = item
            .members
            .into_iter()
            .flat_map(|x| x.content)
            .map(|x| x.try_into())
            .collect::<Result<Vec<CurveKind>, Error>>()?;

        let multi_curve = MultiCurve::new(AbstractGeometricAggregate::default(), surface_members)?;
        Ok(multi_curve)
    }
}

pub fn parse_multi_curve<R: BufRead>(reader: R) -> Result<MultiCurve, Error> {
    let parsed_geometry: Result<GmlMultiCurve, DeError> = de::from_reader(reader);
    parsed_geometry?.try_into()
}

#[cfg(test)]
mod tests {
    use crate::aggregates::GmlMultiCurve;
    use egml_core::model::geometry::aggregates::MultiCurve;
    use quick_xml::de;

    #[test]
    fn parsing_multi_curve() {
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
}
