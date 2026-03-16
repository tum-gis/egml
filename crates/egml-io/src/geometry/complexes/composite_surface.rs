use crate::Error;
use crate::primitives::GmlSurfaceProperty;
use egml_core::model::base::{AsAbstractGml, AsAbstractGmlMut};
use egml_core::model::geometry::aggregates::AggregationType;
use egml_core::model::geometry::complexes::CompositeSurface;
use egml_core::model::geometry::primitives::{AbstractSurface, SurfaceProperty};
use quick_xml::se;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlCompositeSurface {
    #[serde(
        rename(serialize = "@gml:id", deserialize = "@id"),
        skip_serializing_if = "Option::is_none"
    )]
    id: Option<String>,

    #[serde(rename(serialize = "gml:surfaceMember", deserialize = "surfaceMember"))]
    pub surface_members: Vec<GmlSurfaceProperty>,
}

impl TryFrom<GmlCompositeSurface> for CompositeSurface {
    type Error = Error;

    fn try_from(item: GmlCompositeSurface) -> Result<Self, Self::Error> {
        let mut abstract_surface = AbstractSurface::default();
        let id = item.id.map(|id| id.try_into()).transpose()?;
        abstract_surface.set_id(id);

        let surface_members: Vec<SurfaceProperty> = item
            .surface_members
            .into_iter()
            .map(|x| x.try_into())
            .collect::<Result<Vec<SurfaceProperty>, Error>>()?;

        let composite_surface =
            CompositeSurface::new(abstract_surface, surface_members, AggregationType::Array)?;
        Ok(composite_surface)
    }
}

impl From<&CompositeSurface> for GmlCompositeSurface {
    fn from(surface: &CompositeSurface) -> Self {
        Self {
            id: surface.id().map(|id| id.to_string()),
            surface_members: surface
                .surface_member()
                .iter()
                .map(GmlSurfaceProperty::from)
                .collect(),
        }
    }
}

/// Serializes a [`CompositeSurface`] to a GML XML string.
///
/// # Errors
///
/// Returns [`Error::XmlSe`] if serialization fails.
pub fn serialize_composite_surface(surface: &CompositeSurface) -> Result<String, Error> {
    let gml = GmlCompositeSurface::from(surface);
    Ok(se::to_string_with_root("gml:CompositeSurface", &gml)?)
}

#[cfg(test)]
mod tests {
    use super::GmlCompositeSurface;
    use crate::complexes::composite_surface::serialize_composite_surface;
    use egml_core::model::geometry::DirectPosition;
    use egml_core::model::geometry::aggregates::AggregationType;
    use egml_core::model::geometry::complexes::CompositeSurface;
    use egml_core::model::geometry::primitives::{
        AbstractRing, AbstractSurface, LinearRing, Polygon, RingPropertyKind, SurfaceKind,
        SurfaceProperty,
    };
    use quick_xml::{DeError, de};

    fn make_composite_surface() -> CompositeSurface {
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
        CompositeSurface::new(
            AbstractSurface::default(),
            vec![surface_prop],
            AggregationType::Array,
        )
        .unwrap()
    }

    #[test]
    fn deserialize_composite_surface() {
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

    #[test]
    fn serialize_composite_surface_writes_gml_tags() {
        let surface = make_composite_surface();
        let xml = serialize_composite_surface(&surface).unwrap();

        assert!(xml.contains("<gml:CompositeSurface"));
        assert!(xml.contains("<gml:surfaceMember"));
        assert!(xml.contains("<gml:Polygon"));
        assert!(xml.contains("<gml:exterior"));
        assert!(xml.contains("<gml:LinearRing"));
        assert!(!xml.contains("id="));
    }

    #[test]
    fn round_trip_composite_surface_preserves_member_count() {
        let surface = make_composite_surface();
        let xml = serialize_composite_surface(&surface).unwrap();
        let recovered: GmlCompositeSurface = de::from_reader(xml.as_bytes()).unwrap();
        let recovered_surface: CompositeSurface = recovered.try_into().unwrap();

        assert_eq!(
            recovered_surface.surface_member_count(),
            surface.surface_member_count()
        );
    }

    #[test]
    fn round_trip_composite_surface_from_xml() {
        let input_xml = "<gml:CompositeSurface gml:id=\"test-id\">\
            <gml:surfaceMember><gml:Polygon><gml:exterior><gml:LinearRing><gml:posList srsDimension=\"3\">0 0 0 1 0 0 0 1 0 0 0 0</gml:posList></gml:LinearRing></gml:exterior></gml:Polygon></gml:surfaceMember>\
            </gml:CompositeSurface>";

        let gml: GmlCompositeSurface = de::from_reader(input_xml.as_bytes()).unwrap();
        let surface: CompositeSurface = gml.try_into().unwrap();
        let output_xml = serialize_composite_surface(&surface).unwrap();

        assert_eq!(input_xml, output_xml);
    }
}
