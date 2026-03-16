use crate::error::Error;
use crate::error::Error::MissingElements;
use crate::primitives::shell_property::GmlShellProperty;
use egml_core::model::base::{AsAbstractGml, AsAbstractGmlMut};
use egml_core::model::geometry::primitives::{AbstractSolid, Solid, SurfaceProperty};
use quick_xml::{de, se};
use serde::{Deserialize, Serialize};
use std::io::BufRead;
use tracing::debug;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlSolid {
    #[serde(
        rename(serialize = "@gml:id", deserialize = "@id"),
        skip_serializing_if = "Option::is_none"
    )]
    id: Option<String>,

    #[serde(
        rename(serialize = "gml:exterior", deserialize = "exterior"),
        skip_serializing_if = "Option::is_none"
    )]
    exterior: Option<GmlShellProperty>,
}

impl TryFrom<GmlSolid> for Solid {
    type Error = Error;

    fn try_from(value: GmlSolid) -> Result<Self, Self::Error> {
        let id = value.id.map(|id| id.try_into()).transpose()?;
        let mut abstract_solid = AbstractSolid::default();
        abstract_solid.set_id(id);

        let surface_properties: Vec<SurfaceProperty> = value
            .exterior
            .ok_or(MissingElements("".to_string()))?
            .shell
            .ok_or(MissingElements("".to_string()))?
            .members
            .into_iter()
            .flat_map(|x| match x.try_into() {
                Ok(surface_property) => Some(surface_property),
                Err(e) => {
                    debug!("surface property contains invalid geometry: {}", e);
                    None
                }
            })
            .collect();

        let solid = Solid::new(abstract_solid, surface_properties)?;
        Ok(solid)
    }
}

impl From<&Solid> for GmlSolid {
    fn from(solid: &Solid) -> Self {
        Self {
            id: solid.id().map(|id| id.to_string()),
            exterior: Some(GmlShellProperty::from(solid.members())),
        }
    }
}

pub fn deserialize_solid<R: BufRead>(reader: R) -> Result<Solid, Error> {
    let parsed_geometry: GmlSolid = de::from_reader(reader)?;
    parsed_geometry.try_into()
}

/// Serializes a [`Solid`] to a GML XML string.
///
/// # Errors
///
/// Returns [`Error::XmlSe`] if serialization fails.
pub fn serialize_solid(solid: &Solid) -> Result<String, Error> {
    let gml = GmlSolid::from(solid);
    Ok(se::to_string_with_root("gml:Solid", &gml)?)
}

#[cfg(test)]
mod tests {
    use super::GmlSolid;
    use crate::primitives::solid::{deserialize_solid, serialize_solid};
    use egml_core::model::geometry::DirectPosition;
    use egml_core::model::geometry::primitives::{
        AbstractRing, AbstractSurface, LinearRing, Polygon, RingPropertyKind, Solid, SurfaceKind,
        SurfaceProperty,
    };

    fn make_solid() -> Solid {
        let points = vec![
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(0.0, 1.0, 0.0).unwrap(),
        ];
        let ring = LinearRing::new(AbstractRing::default(), points).unwrap();
        let polygon = Polygon::new(
            AbstractSurface::default(),
            Some(RingPropertyKind::LinearRing(ring)),
            vec![],
        )
        .unwrap();
        let surface_prop = SurfaceProperty::new(SurfaceKind::Polygon(polygon));
        Solid::new(Default::default(), vec![surface_prop]).unwrap()
    }

    #[test]
    fn deserialize_solid_with_two_polygon_surfaces() {
        let xml_document = b"\
        <gml:Solid gml:id=\"UUID_9c9c6a8e-4704-4675-b3c0-e8f8c9dc4522\">
          <gml:exterior>
            <gml:Shell>
              <gml:surfaceMember>
                <gml:Polygon gml:id=\"UUID_bdc0d140-fb3a-4f9e-aaaf-90c9d3c4f37e\">
                  <gml:exterior>
                    <gml:LinearRing>
                      <gml:posList srsDimension=\"3\">677996.9921558445 5403654.972995559 414.84686725860973 677996.9730161787 5403654.969217261 414.84686725860973 677996.9538875414 5403654.973051003 414.84686725860973 677996.9376820943 5403654.983913131 414.84686725860973 677996.9268669697 5403655.000149984 414.84686725860973 677996.9230886723 5403655.01928965 414.84686725860973 677996.9269224138 5403655.038418287 414.84686725860973 677996.9377845416 5403655.054623734 414.84686725860973 677996.9540213953 5403655.065438859 414.84686725860973 677996.9731610611 5403655.069217157 414.84686725860973 677996.9922896983 5403655.065383415 414.84686725860973 677997.0084951455 5403655.054521287 414.84686725860973 677997.0193102701 5403655.038284434 414.84686725860973 677997.0230885674 5403655.019144768 414.84686725860973 677997.019254826 5403655.0000161305 414.84686725860973 677997.0083926981 5403654.983810684 414.84686725860973 677996.9921558445 5403654.972995559 414.84686725860973</gml:posList>
                    </gml:LinearRing>
                  </gml:exterior>
                </gml:Polygon>
              </gml:surfaceMember>
              <gml:surfaceMember>
                <gml:Polygon gml:id=\"UUID_b4e59597-63a3-46a0-a91b-91689cf63b7d\">
                  <gml:exterior>
                    <gml:LinearRing>
                      <gml:posList srsDimension=\"3\">677997.0083926981 5403654.983810684 418.08686725860974 677997.019254826 5403655.0000161305 418.08686725860974 677997.0230885674 5403655.019144768 418.08686725860974 677997.0193102701 5403655.038284434 418.08686725860974 677997.0084951455 5403655.054521287 418.08686725860974 677996.9922896983 5403655.065383415 418.08686725860974 677996.9731610611 5403655.069217157 418.08686725860974 677996.9540213953 5403655.065438859 418.08686725860974 677996.9377845416 5403655.054623734 418.08686725860974 677996.9269224138 5403655.038418287 418.08686725860974 677996.9230886723 5403655.01928965 418.08686725860974 677996.9268669697 5403655.000149984 418.08686725860974 677996.9376820943 5403654.983913131 418.08686725860974 677996.9538875414 5403654.973051003 418.08686725860974 677996.9730161787 5403654.969217261 418.08686725860974 677996.9921558445 5403654.972995559 418.08686725860974 677997.0083926981 5403654.983810684 418.08686725860974</gml:posList>
                    </gml:LinearRing>
                  </gml:exterior>
                </gml:Polygon>
              </gml:surfaceMember>
            </gml:Shell>
          </gml:exterior>
        </gml:Solid>";

        let solid_geometry = deserialize_solid(xml_document.as_ref()).unwrap();

        assert_eq!(solid_geometry.members().len(), 2);
    }

    #[test]
    fn deserialize_solid_with_xlink_members_returns_error() {
        let xml_document = b"\
        <gml:Solid srsDimension=\"3\">
          <gml:exterior>
            <gml:Shell>
              <gml:surfaceMember xlink:href=\"#DEBY_LOD2_59772_4becb506-d53b-44ca-a483-e6a3d238b4c2_2_poly\"/>
              <gml:surfaceMember xlink:href=\"#DEBY_LOD2_59772_be3462c3-9865-467b-829d-76e6b9b692e7_2_poly\"/>
              <gml:surfaceMember xlink:href=\"#DEBY_LOD2_59772_c0aae462-3f4b-4062-80bb-8cd04768ab1a_2_poly\"/>
            </gml:Shell>
          </gml:exterior>
        </gml:Solid>";

        let solid_geometry_result = deserialize_solid(xml_document.as_ref());

        assert!(solid_geometry_result.is_err());
    }

    #[test]
    fn deserialize_solid_without_ids() {
        let xml_document = b"\
        <gml:Solid>
          <gml:exterior>
            <gml:Shell>
              <gml:surfaceMember>
                <gml:Polygon>
                  <gml:exterior>
                    <gml:LinearRing>
                      <gml:posList srsDimension=\"3\">677996.9921558445 5403654.972995559 414.84686725860973 677996.9730161787 5403654.969217261 414.84686725860973 677996.9538875414 5403654.973051003 414.84686725860973 677996.9376820943 5403654.983913131 414.84686725860973 677996.9268669697 5403655.000149984 414.84686725860973 677996.9230886723 5403655.01928965 414.84686725860973 677996.9269224138 5403655.038418287 414.84686725860973 677996.9377845416 5403655.054623734 414.84686725860973 677996.9540213953 5403655.065438859 414.84686725860973 677996.9731610611 5403655.069217157 414.84686725860973 677996.9922896983 5403655.065383415 414.84686725860973 677997.0084951455 5403655.054521287 414.84686725860973 677997.0193102701 5403655.038284434 414.84686725860973 677997.0230885674 5403655.019144768 414.84686725860973 677997.019254826 5403655.0000161305 414.84686725860973 677997.0083926981 5403654.983810684 414.84686725860973 677996.9921558445 5403654.972995559 414.84686725860973</gml:posList>
                    </gml:LinearRing>
                  </gml:exterior>
                </gml:Polygon>
              </gml:surfaceMember>
              <gml:surfaceMember>
                <gml:Polygon>
                  <gml:exterior>
                    <gml:LinearRing>
                      <gml:posList srsDimension=\"3\">677997.0083926981 5403654.983810684 418.08686725860974 677997.019254826 5403655.0000161305 418.08686725860974 677997.0230885674 5403655.019144768 418.08686725860974 677997.0193102701 5403655.038284434 418.08686725860974 677997.0084951455 5403655.054521287 418.08686725860974 677996.9922896983 5403655.065383415 418.08686725860974 677996.9731610611 5403655.069217157 418.08686725860974 677996.9540213953 5403655.065438859 418.08686725860974 677996.9377845416 5403655.054623734 418.08686725860974 677996.9269224138 5403655.038418287 418.08686725860974 677996.9230886723 5403655.01928965 418.08686725860974 677996.9268669697 5403655.000149984 418.08686725860974 677996.9376820943 5403654.983913131 418.08686725860974 677996.9538875414 5403654.973051003 418.08686725860974 677996.9730161787 5403654.969217261 418.08686725860974 677996.9921558445 5403654.972995559 418.08686725860974 677997.0083926981 5403654.983810684 418.08686725860974</gml:posList>
                    </gml:LinearRing>
                  </gml:exterior>
                </gml:Polygon>
              </gml:surfaceMember>
            </gml:Shell>
          </gml:exterior>
        </gml:Solid>";

        let solid_geometry = deserialize_solid(xml_document.as_ref()).unwrap();

        assert_eq!(solid_geometry.members().len(), 2);
    }

    #[test]
    fn serialize_solid_writes_gml_tags() {
        let solid = make_solid();
        let xml = serialize_solid(&solid).unwrap();

        assert!(xml.contains("<gml:Solid"));
        assert!(xml.contains("<gml:exterior"));
        assert!(xml.contains("<gml:Shell"));
        assert!(xml.contains("<gml:surfaceMember"));
        assert!(xml.contains("<gml:Polygon"));
        assert!(!xml.contains("id="));
    }

    #[test]
    fn round_trip_solid_preserves_member_count() {
        let solid = make_solid();
        let xml = serialize_solid(&solid).unwrap();
        let recovered = deserialize_solid(xml.as_bytes()).unwrap();

        assert_eq!(recovered.members().len(), solid.members().len());
    }

    #[test]
    fn round_trip_solid_from_xml() {
        let input_xml = "<gml:Solid gml:id=\"test-id\">\
            <gml:exterior><gml:Shell>\
            <gml:surfaceMember><gml:Polygon><gml:exterior><gml:LinearRing><gml:posList srsDimension=\"3\">0 0 0 1 0 0 0 1 0 0 0 0</gml:posList></gml:LinearRing></gml:exterior></gml:Polygon></gml:surfaceMember>\
            </gml:Shell></gml:exterior>\
            </gml:Solid>";

        let gml: GmlSolid = quick_xml::de::from_reader(input_xml.as_bytes()).unwrap();
        let solid: Solid = gml.try_into().unwrap();
        let output_xml = serialize_solid(&solid).unwrap();

        assert_eq!(input_xml, output_xml);
    }
}
