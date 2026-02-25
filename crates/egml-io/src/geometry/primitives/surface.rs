use crate::Error;
use crate::primitives::GmlSurfacePatchArrayProperty;
use egml_core::model::geometry::primitives::{AbstractSurface, Surface, SurfacePatchArrayProperty};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlSurface {
    #[serde(rename = "@id")]
    id: Option<String>,

    pub patches: GmlSurfacePatchArrayProperty,
}

impl TryFrom<GmlSurface> for Surface {
    type Error = Error;

    fn try_from(item: GmlSurface) -> Result<Self, Self::Error> {
        let patches: SurfacePatchArrayProperty = item.patches.try_into()?;

        let abstract_surface = AbstractSurface::default();
        Ok(Self::new(abstract_surface, patches))
    }
}

#[cfg(test)]
mod tests {
    use crate::primitives::GmlSurface;
    use quick_xml::{DeError, de};

    #[test]
    fn parsing_surface_with_patches() {
        let xml_document = b"
        <gml:Surface>
            <gml:patches>
                <gml:PolygonPatch>
                    <gml:exterior>
                        <gml:LinearRing>
                            <gml:posList>350.54400634765625 972.9130249023438 0.11999999731779099 350.5414201635045 968.6025425887852 0.11999999731779099 350.54400634765625 968.6025096366793 0.11999999731779099 350.54400634765625 972.9130249023438 0.11999999731779099</gml:posList>
                        </gml:LinearRing>
                    </gml:exterior>
                </gml:PolygonPatch>
            </gml:patches>
        </gml:Surface>";

        let parsed_result: Result<GmlSurface, DeError> = de::from_reader(xml_document.as_ref());
        let parsed_gml = parsed_result.expect("parsing should work");

        assert_eq!(parsed_gml.patches.patches.len(), 1);

        println!("{:?}", parsed_gml);
    }
}
