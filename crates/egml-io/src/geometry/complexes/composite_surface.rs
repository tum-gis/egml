use crate::Error;
use crate::primitives::GmlSurfaceProperty;
use egml_core::model::geometry::aggregates::AggregationType;
use egml_core::model::geometry::complexes::CompositeSurface;
use egml_core::model::geometry::primitives::{AbstractSurface, SurfaceProperty};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlCompositeSurface {
    #[serde(rename = "@id")]
    id: Option<String>,

    #[serde(rename = "$value")]
    pub surface_members: Vec<GmlSurfaceProperty>,
}

impl TryFrom<GmlCompositeSurface> for CompositeSurface {
    type Error = Error;

    fn try_from(item: GmlCompositeSurface) -> Result<Self, Self::Error> {
        let surface_members: Vec<SurfaceProperty> = item
            .surface_members
            .into_iter()
            .map(|x| x.try_into())
            .collect::<Result<Vec<SurfaceProperty>, Error>>()?;

        let abstract_surface = AbstractSurface::default();

        let composite_surface =
            CompositeSurface::new(abstract_surface, surface_members, AggregationType::Array)?;
        Ok(composite_surface)
    }
}

#[cfg(test)]
mod tests {
    use crate::complexes::GmlCompositeSurface;
    use egml_core::model::geometry::complexes::CompositeSurface;
    use quick_xml::{DeError, de};

    #[test]
    fn parsing_polygon_patch() {
        let xml_document = b"<gml:CompositeSurface>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>314.531005859375 1043.4599609375 7.144559860229492 314.531005859375 1043.4599609375 2.6047298908233643 314.68798828125 1043.22998046875 2.6047298908233643 314.531005859375 1043.4599609375 7.144559860229492</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>314.531005859375 1043.4599609375 7.144559860229492 314.68798828125 1043.22998046875 2.6047298908233643 315.7770080566406 1041.6500244140625 2.6047298908233643 314.531005859375 1043.4599609375 7.144559860229492</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
							<gml:surfaceMember>
								<gml:Polygon>
									<gml:exterior>
										<gml:LinearRing>
											<gml:posList>314.531005859375 1043.4599609375 7.144559860229492 315.7770080566406 1041.6500244140625 2.6047298908233643 316.1080017089844 1041.1700439453125 7.144559860229492 314.531005859375 1043.4599609375 7.144559860229492</gml:posList>
										</gml:LinearRing>
									</gml:exterior>
								</gml:Polygon>
							</gml:surfaceMember>
						</gml:CompositeSurface>";

        let parsed_result: Result<GmlCompositeSurface, DeError> =
            de::from_reader(xml_document.as_ref());
        let parsed_gml = parsed_result.expect("parsing should work");
        let composite_surface: CompositeSurface = parsed_gml.try_into().unwrap();

        assert_eq!(composite_surface.surface_member_count(), 3);
    }
}
