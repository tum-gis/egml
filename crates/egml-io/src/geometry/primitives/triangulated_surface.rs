use crate::Error;
use crate::primitives::GmlSurfacePatchArrayProperty;
use egml_core::model::geometry::primitives::{
    AbstractSurface, Surface, SurfacePatchArrayProperty, TriangulatedSurface,
};
use quick_xml::{DeError, de};
use serde::{Deserialize, Serialize};
use std::io::BufRead;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlTriangulatedSurface {
    #[serde(rename = "@id")]
    pub id: Option<String>,

    pub patches: GmlSurfacePatchArrayProperty,
}

impl TryFrom<GmlTriangulatedSurface> for TriangulatedSurface {
    type Error = Error;

    fn try_from(item: GmlTriangulatedSurface) -> Result<Self, Self::Error> {
        let patches: SurfacePatchArrayProperty = item.patches.try_into()?;

        let abstract_surface = AbstractSurface::default();
        let surface = Surface::new(abstract_surface, patches);
        Ok(Self::new(surface)?)
    }
}

pub fn parse_triangulated_surface<R: BufRead>(reader: R) -> Result<TriangulatedSurface, Error> {
    let parsed_geometry: Result<GmlTriangulatedSurface, DeError> = de::from_reader(reader);
    parsed_geometry?.try_into()
}

#[cfg(test)]
mod tests {
    use crate::primitives::triangulated_surface::GmlTriangulatedSurface;
    use egml_core::model::geometry::primitives::{AsSurface, TriangulatedSurface};
    use quick_xml::{DeError, de};

    #[test]
    fn parsing_triangulated_surface_with_triangles() {
        let xml_document = b"
        <gml:TriangulatedSurface srsDimension=\"3\">
          <gml:patches>
            <gml:Triangle>
              <gml:exterior>
                <gml:LinearRing>
                  <gml:posList>-76.10530090332031 3.262645959854126 -0.023333795368671417 -76.19206237792969 3.5082428455352783 0.0 -77.13458251953125 3.1704795360565186 0.0 -76.10530090332031 3.262645959854126 -0.023333795368671417</gml:posList>
                </gml:LinearRing>
              </gml:exterior>
            </gml:Triangle>
            <gml:Triangle>
              <gml:exterior>
                <gml:LinearRing>
                  <gml:posList>-76.10530090332031 3.262645959854126 -0.023333795368671417 -77.13458251953125 3.1704795360565186 0.0 -77.04559326171875 2.925680637359619 -0.023333795368671417 -76.10530090332031 3.262645959854126 -0.023333795368671417</gml:posList>
                </gml:LinearRing>
              </gml:exterior>
            </gml:Triangle>
            <gml:Triangle>
              <gml:exterior>
                <gml:LinearRing>
                  <gml:posList>-66.17540740966797 6.251502513885498 -0.023333795368671417 -66.23863220214844 6.504178047180176 0.0 -71.25069427490234 5.123663902282715 0.0 -66.17540740966797 6.251502513885498 -0.023333795368671417</gml:posList>
                </gml:LinearRing>
              </gml:exterior>
            </gml:Triangle>
          </gml:patches>
        </gml:TriangulatedSurface>";

        let parsed_result: Result<GmlTriangulatedSurface, DeError> =
            de::from_reader(xml_document.as_ref());
        let parsed_gml = parsed_result.expect("parsing should work");
        let triangulated_surface: TriangulatedSurface = parsed_gml.try_into().unwrap();

        assert_eq!(triangulated_surface.patches_len(), 3);
    }
}
