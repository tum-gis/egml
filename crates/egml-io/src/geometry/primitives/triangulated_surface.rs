use crate::Error;
use crate::primitives::GmlSurfacePatchArrayProperty;
use egml_core::model::base::{AsAbstractGml, AsAbstractGmlMut};
use egml_core::model::geometry::primitives::{
    AbstractSurface, AsSurface, Surface, SurfacePatchArrayProperty, TriangulatedSurface,
};
use quick_xml::{DeError, de, se};
use serde::{Deserialize, Serialize};
use std::io::BufRead;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlTriangulatedSurface {
    #[serde(
        rename(serialize = "@gml:id", deserialize = "@id"),
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<String>,

    #[serde(rename(serialize = "gml:patches", deserialize = "patches"))]
    pub patches: GmlSurfacePatchArrayProperty,
}

impl TryFrom<GmlTriangulatedSurface> for TriangulatedSurface {
    type Error = Error;

    fn try_from(item: GmlTriangulatedSurface) -> Result<Self, Self::Error> {
        let id = item.id.map(|id| id.try_into()).transpose()?;
        let mut abstract_surface = AbstractSurface::default();
        abstract_surface.set_id(id);

        let patches: SurfacePatchArrayProperty = item.patches.try_into()?;

        let surface = Surface::new(abstract_surface, patches);
        Ok(Self::new(surface)?)
    }
}

impl From<&TriangulatedSurface> for GmlTriangulatedSurface {
    fn from(surface: &TriangulatedSurface) -> Self {
        Self {
            id: surface.id().map(|id| id.to_string()),
            patches: GmlSurfacePatchArrayProperty::from(surface.patches()),
        }
    }
}

pub fn deserialize_triangulated_surface<R: BufRead>(
    reader: R,
) -> Result<TriangulatedSurface, Error> {
    let parsed_geometry: Result<GmlTriangulatedSurface, DeError> = de::from_reader(reader);
    parsed_geometry?.try_into()
}

/// Serializes a [`TriangulatedSurface`] to a GML XML string.
///
/// # Errors
///
/// Returns [`Error::XmlSe`] if serialization fails.
pub fn serialize_triangulated_surface(surface: &TriangulatedSurface) -> Result<String, Error> {
    let gml = GmlTriangulatedSurface::from(surface);
    Ok(se::to_string_with_root("gml:TriangulatedSurface", &gml)?)
}

#[cfg(test)]
mod tests {
    use super::GmlTriangulatedSurface;
    use crate::primitives::triangulated_surface::{
        deserialize_triangulated_surface, serialize_triangulated_surface,
    };
    use egml_core::model::geometry::DirectPosition;
    use egml_core::model::geometry::primitives::{AsSurface, Triangle, TriangulatedSurface};
    use quick_xml::{DeError, de};

    fn make_triangulated_surface() -> TriangulatedSurface {
        let t1 = Triangle::new(
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(0.0, 1.0, 0.0).unwrap(),
        )
        .unwrap();
        let t2 = Triangle::new(
            DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 1.0, 0.0).unwrap(),
            DirectPosition::new(0.0, 1.0, 0.0).unwrap(),
        )
        .unwrap();
        TriangulatedSurface::from_triangles(vec![t1, t2]).unwrap()
    }

    #[test]
    fn deserialize_triangulated_surface_with_three_triangles() {
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

    #[test]
    fn serialize_triangulated_surface_writes_gml_tags() {
        let surface = make_triangulated_surface();
        let xml = serialize_triangulated_surface(&surface).unwrap();

        assert!(xml.contains("<gml:TriangulatedSurface"));
        assert!(xml.contains("<gml:patches"));
        assert!(xml.contains("<gml:Triangle"));
        assert!(xml.contains("<gml:posList"));
        assert!(!xml.contains("id="));
    }

    #[test]
    fn round_trip_triangulated_surface_preserves_patch_count() {
        let surface = make_triangulated_surface();
        let xml = serialize_triangulated_surface(&surface).unwrap();

        let recovered = deserialize_triangulated_surface(xml.as_bytes()).unwrap();

        assert_eq!(recovered.patches_len(), surface.patches_len());
    }

    #[test]
    fn round_trip_triangulated_surface_from_xml() {
        let input_xml = "<gml:TriangulatedSurface gml:id=\"my-id\">\
            <gml:patches>\
            <gml:Triangle><gml:exterior><gml:LinearRing><gml:posList srsDimension=\"3\">0 0 0 1 0 0 0 1 0 0 0 0</gml:posList></gml:LinearRing></gml:exterior></gml:Triangle>\
            <gml:Triangle><gml:exterior><gml:LinearRing><gml:posList srsDimension=\"3\">1 0 0 1 1 0 0 1 0 1 0 0</gml:posList></gml:LinearRing></gml:exterior></gml:Triangle>\
            </gml:patches>\
            </gml:TriangulatedSurface>";

        let gml: GmlTriangulatedSurface = de::from_reader(input_xml.as_bytes()).unwrap();
        let surface: TriangulatedSurface = gml.try_into().unwrap();
        let output_xml = serialize_triangulated_surface(&surface).unwrap();

        assert_eq!(input_xml, output_xml);
    }

    #[test]
    fn round_trip_triangulated_surface_preserves_float_precision() {
        let t1 = Triangle::new(
            DirectPosition::new(-76.10530090332031, 3.262645959854126, -0.023333795368671417)
                .unwrap(),
            DirectPosition::new(-76.19206237792969, 3.5082428455352783, 0.0).unwrap(),
            DirectPosition::new(-77.13458251953125, 3.1704795360565186, 0.0).unwrap(),
        )
        .unwrap();
        let surface = TriangulatedSurface::from_triangles(vec![t1]).unwrap();

        let xml = serialize_triangulated_surface(&surface).unwrap();
        let recovered = deserialize_triangulated_surface(xml.as_bytes()).unwrap();

        let orig_triangles = surface.triangles();
        let rec_triangles = recovered.triangles();
        assert_eq!(orig_triangles.len(), rec_triangles.len());
        assert_eq!(orig_triangles[0].a.x(), rec_triangles[0].a.x());
        assert_eq!(orig_triangles[0].a.y(), rec_triangles[0].a.y());
        assert_eq!(orig_triangles[0].a.z(), rec_triangles[0].a.z());
    }
}
