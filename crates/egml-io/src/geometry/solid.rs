use quick_xml::de;
use std::hash::{DefaultHasher, Hash, Hasher};

use crate::GmlSurfaceMember;
use crate::error::Error;
use crate::error::Error::MissingElements;
use egml_core::model::base::{Gml, Id};
use egml_core::model::geometry::{LinearRing, Solid};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename = "gml:Solid")]
struct GmlSolid {
    #[serde(rename = "@id", default)]
    id: String,
    // #[serde(rename = "$value")]
    // members: Vec<SurfaceMemberElement>,
    #[serde(rename = "$value")]
    exterior: Option<GmlShellProperty>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename = "gml:exterior")]
struct GmlShellProperty {
    #[serde(rename = "$value")]
    shell: Option<GmlShell>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename = "gml:Shell")]
struct GmlShell {
    #[serde(rename = "$value")]
    members: Vec<GmlSurfaceMember>,
}

impl TryFrom<GmlSolid> for Solid {
    type Error = Error;

    fn try_from(value: GmlSolid) -> Result<Self, Self::Error> {
        let id: Id = value.id.clone().try_into().ok().unwrap_or_else(|| {
            let mut hasher = DefaultHasher::new();
            value.hash(&mut hasher);
            Id::from_hashed_u64(hasher.finish())
        });
        let gml = Gml::new(id);

        let linear_rings: Vec<LinearRing> = value
            .exterior
            .ok_or(MissingElements())?
            .shell
            .ok_or(MissingElements())?
            .members
            .into_iter()
            .flat_map(|x| x.polygon)
            .map(|x| x.exterior)
            .flat_map(|x| x.linear_ring)
            .map(|x| x.try_into().unwrap())
            .collect();

        let solid = Solid::new(gml, linear_rings)?;
        Ok(solid)
    }
}

pub fn parse_solid(source_text: &str) -> Result<Solid, Error> {
    let parsed_geometry: GmlSolid = de::from_str(source_text)?;
    parsed_geometry.try_into()
}

#[cfg(test)]
mod tests {
    use crate::geometry::solid::parse_solid;

    #[test]
    fn parsing_solid() {
        let source_text = "\
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

        let solid_geometry = parse_solid(source_text).unwrap();

        assert_eq!(solid_geometry.members().len(), 2);
    }

    #[test]
    fn parsing_solid_without_ids() {
        let source_text = "\
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

        let solid_geometry = parse_solid(source_text).unwrap();

        assert_eq!(solid_geometry.members().len(), 2);
    }

    #[test]
    fn parsing_solid_with_xlinks_only() {
        // TODO
        let source_text = "\
        <gml:Solid srsDimension=\"3\">
          <gml:exterior>
            <gml:Shell>
              <gml:surfaceMember xlink:href=\"#DEBY_LOD2_4905373_a8f507d0-7f89-4966-8f8d-0ffa02508922_poly\"/>
              <gml:surfaceMember xlink:href=\"#DEBY_LOD2_4905373_d2691fca-4e89-404c-80f8-fc90794581d8_poly\"/>
              <gml:surfaceMember xlink:href=\"#DEBY_LOD2_4905373_f3d48f8e-6ae2-49ea-9dd8-4f08af06a738_poly\"/>
              <gml:surfaceMember xlink:href=\"#DEBY_LOD2_4905373_1dea43d2-0e28-4010-9297-606eb7abcc42_poly\"/>
            </gml:Shell>
          </gml:exterior>
        </gml:Solid>";

        //let solid_geometry = parse_solid(source_text).unwrap();

        //assert_eq!(solid_geometry.members().len(), 4);
    }
}
