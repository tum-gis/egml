use crate::error::Error;
use crate::primitives::{GmlSurfaceKind, GmlSurfaceProperty};
use egml_core::model::base::{AsAbstractGml, AsAbstractGmlMut};
use egml_core::model::geometry::aggregates::{AbstractGeometricAggregate, MultiSurface};
use egml_core::model::geometry::primitives::SurfaceKind;
use quick_xml::{DeError, de, se};
use serde::{Deserialize, Serialize};
use std::io::BufRead;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GmlMultiSurface {
    #[serde(
        rename(serialize = "@gml:id", deserialize = "@id"),
        skip_serializing_if = "Option::is_none"
    )]
    id: Option<String>,

    #[serde(
        rename(serialize = "gml:surfaceMember", deserialize = "surfaceMember"),
        default
    )]
    members: Vec<GmlSurfaceProperty>,
}

impl TryFrom<GmlMultiSurface> for MultiSurface {
    type Error = Error;

    fn try_from(item: GmlMultiSurface) -> Result<Self, Self::Error> {
        let mut abstract_aggregate = AbstractGeometricAggregate::default();
        let id = item.id.map(|id| id.try_into()).transpose()?;
        abstract_aggregate.set_id(id);

        let surface_members: Vec<SurfaceKind> = item
            .members
            .into_iter()
            .flat_map(|x| x.content)
            .map(|x| x.try_into())
            .collect::<Result<Vec<SurfaceKind>, Error>>()?;

        let multi_surface = MultiSurface::new(abstract_aggregate, surface_members)?;
        Ok(multi_surface)
    }
}

impl From<&MultiSurface> for GmlMultiSurface {
    fn from(multi_surface: &MultiSurface) -> Self {
        Self {
            id: multi_surface.id().map(|id| id.to_string()),
            members: multi_surface
                .surface_member()
                .iter()
                .map(|kind| GmlSurfaceProperty {
                    href: None,
                    content: Some(GmlSurfaceKind::from(kind)),
                })
                .collect(),
        }
    }
}

pub fn deserialize_multi_surface<R: BufRead>(reader: R) -> Result<MultiSurface, Error> {
    let parsed_geometry: Result<GmlMultiSurface, DeError> = de::from_reader(reader);
    parsed_geometry?.try_into()
}

/// Serializes a [`MultiSurface`] to a GML XML string.
///
/// # Errors
///
/// Returns [`Error::XmlSe`] if serialization fails.
pub fn serialize_multi_surface(multi_surface: &MultiSurface) -> Result<String, Error> {
    let gml = GmlMultiSurface::from(multi_surface);
    Ok(se::to_string_with_root("gml:MultiSurface", &gml)?)
}

#[cfg(test)]
mod tests {
    use super::GmlMultiSurface;
    use crate::aggregates::multi_surface::{deserialize_multi_surface, serialize_multi_surface};
    use egml_core::model::geometry::DirectPosition;
    use egml_core::model::geometry::aggregates::{AbstractGeometricAggregate, MultiSurface};
    use egml_core::model::geometry::primitives::{
        AbstractRing, AbstractSurface, LinearRing, Polygon, RingPropertyKind, SurfaceKind,
    };
    use quick_xml::de;

    fn make_multi_surface() -> MultiSurface {
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
        MultiSurface::new(
            AbstractGeometricAggregate::default(),
            vec![SurfaceKind::Polygon(polygon)],
        )
        .unwrap()
    }

    #[test]
    fn deserialize_multi_surface_with_composite_surface_member() {
        let xml_document = b"
				<gml:MultiSurface srsDimension=\"3\">
					<gml:surfaceMember>
						<gml:CompositeSurface>
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
						</gml:CompositeSurface>
					</gml:surfaceMember>
				</gml:MultiSurface>";

        let _result = deserialize_multi_surface(xml_document.as_ref()).unwrap();
    }

    #[test]
    fn deserialize_multi_surface_with_one_polygon_member() {
        let xml_document = b"<gml:MultiSurface gml:id=\"UUID_6b33ecfa-6e08-4e8e-a4b5-e1d06540faf0\">
              <gml:surfaceMember>
                <gml:Polygon gml:id=\"UUID_efb8f6a5-82fa-4b21-8709-c1d93ed1e595\">
                  <gml:exterior>
                    <gml:LinearRing>
                      <gml:posList srsDimension=\"3\">678009.7116291433 5403638.313338383 417.3480034550211 678012.5609078613 5403634.960884141 417.34658523466385 678013.7892528991 5403636.004867206 417.51938733855997 678010.9399743223 5403639.357321232 417.5208051908512 678009.7116291433 5403638.313338383 417.3480034550211</gml:posList>
                    </gml:LinearRing>
                  </gml:exterior>
                </gml:Polygon>
              </gml:surfaceMember>
            </gml:MultiSurface>";

        let _result = deserialize_multi_surface(xml_document.as_ref()).unwrap();
    }

    #[test]
    fn deserialize_multi_surface_with_duplicate_ring_points() {
        let xml_document = b"<gml:MultiSurface srsName=\"EPSG:25832\" srsDimension=\"3\">
              <gml:surfaceMember>
                <gml:Polygon gml:id=\"4018133_PG.3nRTCd4XPu47PsAAUyNv\">
                  <gml:exterior>
                    <gml:LinearRing gml:id=\"4018133_LR.lHfcvQUrKVl08ifcH6eO\">
                      <gml:posList>678105.792 5403815.554 369.98523 678105.792 5403815.555 367.67323 678106.047 5403815.125 367.67323 678106.047 5403815.125 367.67323 678106.047 5403815.125 367.67323 678106.047 5403815.124 369.98523 678105.792 5403815.554 369.98523</gml:posList>
                    </gml:LinearRing>
                  </gml:exterior>
                </gml:Polygon>
              </gml:surfaceMember>
            </gml:MultiSurface>";

        let _result = deserialize_multi_surface(xml_document.as_ref()).unwrap();
    }

    #[test]
    fn deserialize_multi_surface_with_interior_rings() {
        let xml_document = b"
            <gml:MultiSurface srsName=\"EPSG:25832\" srsDimension=\"3\">
              <gml:surfaceMember>
                <gml:Polygon gml:id=\"4018106_PG.dKY9ug9ol2tsxL5bLAPz\">
                  <gml:exterior>
                    <gml:LinearRing gml:id=\"4018106_LR.Wqmtl1E6Yz3eVJkuGjsK\">
                      <gml:posList>678097.805 5403801.433 367.40123 678092.938 5403810.139 367.40123 678092.938 5403810.139 370.87623 678092.032 5403811.76 370.87623 678092.032 5403811.76 377.09023 678097.805 5403801.433 377.09023 678097.805 5403801.433 367.40123</gml:posList>
                    </gml:LinearRing>
                  </gml:exterior>
                  <gml:interior>
                    <gml:LinearRing gml:id=\"4018106_LR.10JNDsQqif3fouy54mfv\">
                      <gml:posList>678096.88 5403803.088 374.90623 678097.403 5403802.152 374.90623 678097.403 5403802.152 376.19923 678096.88 5403803.088 376.19923 678096.88 5403803.088 374.90623</gml:posList>
                    </gml:LinearRing>
                  </gml:interior>
                  <gml:interior>
                    <gml:LinearRing gml:id=\"4018106_LR.yzLlZkAQX00eXb6Xi0DZ\">
                      <gml:posList>678096.154 5403804.386 376.19923 678096.154 5403804.386 374.90623 678096.677 5403803.45 374.90623 678096.677 5403803.45 376.19923 678096.154 5403804.386 376.19923</gml:posList>
                    </gml:LinearRing>
                  </gml:interior>
                  <gml:interior>
                    <gml:LinearRing gml:id=\"4018106_LR.MIkI0SEPyMQ4yblCNiF2\">
                      <gml:posList>678095.438 5403805.667 376.19923 678095.438 5403805.667 374.90623 678095.961 5403804.731 374.90623 678095.961 5403804.731 376.19923 678095.438 5403805.667 376.19923</gml:posList>
                    </gml:LinearRing>
                  </gml:interior>
                  <gml:interior>
                    <gml:LinearRing gml:id=\"4018106_LR.novU6ZVfhrtxrFFh7eYQ\">
                      <gml:posList>678097.403 5403802.152 372.05223 678097.403 5403802.152 373.34523 678096.88 5403803.088 373.34523 678096.88 5403803.088 372.05223 678097.403 5403802.152 372.05223</gml:posList>
                    </gml:LinearRing>
                  </gml:interior>
                  <gml:interior>
                    <gml:LinearRing gml:id=\"4018106_LR.XdJcfjsruS75wlUmTQdH\">
                      <gml:posList>678096.677 5403803.45 372.05223 678096.677 5403803.45 373.34523 678096.154 5403804.386 373.34523 678096.154 5403804.386 372.05223 678096.677 5403803.45 372.05223</gml:posList>
                    </gml:LinearRing>
                  </gml:interior>
                  <gml:interior>
                    <gml:LinearRing gml:id=\"4018106_LR.wzwxsPr4Ys8dTM1bzH8T\">
                      <gml:posList>678095.961 5403804.731 372.05223 678095.961 5403804.731 373.34523 678095.438 5403805.667 373.34523 678095.438 5403805.667 372.05223 678095.961 5403804.731 372.05223</gml:posList>
                    </gml:LinearRing>
                  </gml:interior>
                  <gml:interior>
                    <gml:LinearRing gml:id=\"4018106_LR.20P6FwXiq4ZJ4EAxdmJ0\">
                      <gml:posList>678093.838 5403808.528 374.89423 678094.361 5403807.593 374.89423 678094.361 5403807.593 376.18723 678093.838 5403808.528 376.18723 678093.838 5403808.528 374.89423</gml:posList>
                    </gml:LinearRing>
                  </gml:interior>
                  <gml:interior>
                    <gml:LinearRing gml:id=\"4018106_LR.saIYdVUNcoK3LJkC2LDw\">
                      <gml:posList>678093.645 5403808.873 374.89423 678093.645 5403808.873 376.18723 678093.122 5403809.809 376.18723 678093.122 5403809.809 374.89423 678093.645 5403808.873 374.89423</gml:posList>
                    </gml:LinearRing>
                  </gml:interior>
                  <gml:interior>
                    <gml:LinearRing gml:id=\"4018106_LR.yPDE98qtqfTYBziBsTpl\">
                      <gml:posList>678093.869 5403808.474 372.04523 678094.392 5403807.538 372.04523 678094.392 5403807.538 373.33823 678093.869 5403808.474 373.33823 678093.869 5403808.474 372.04523</gml:posList>
                    </gml:LinearRing>
                  </gml:interior>
                  <gml:interior>
                    <gml:LinearRing gml:id=\"4018106_LR.XaQt7QEqeVnG2PB8D6ad\">
                      <gml:posList>678093.153 5403809.755 373.33823 678093.153 5403809.755 372.04523 678093.676 5403808.819 372.04523 678093.676 5403808.819 373.33823 678093.153 5403809.755 373.33823</gml:posList>
                    </gml:LinearRing>
                  </gml:interior>
                  <gml:interior>
                    <gml:LinearRing gml:id=\"4018106_LR.kCEyyLA2tigxjpQY9cyU\">
                      <gml:posList>678092.933 5403810.148 372.04523 678092.933 5403810.148 373.32523 678092.126 5403811.591 373.32523 678092.126 5403811.591 372.04523 678092.933 5403810.148 372.04523</gml:posList>
                    </gml:LinearRing>
                  </gml:interior>
                  <gml:interior>
                    <gml:LinearRing gml:id=\"4018106_LR.Wq5AG6YS8zrN5HgtFQD8\">
                      <gml:posList>678092.126 5403811.591 376.18723 678092.126 5403811.591 374.89423 678092.933 5403810.148 374.89423 678092.933 5403810.148 376.18723 678092.126 5403811.591 376.18723</gml:posList>
                    </gml:LinearRing>
                  </gml:interior>
                  <gml:interior>
                    <gml:LinearRing gml:id=\"4018106_LR.aQFMEYkDQkns0ZoJ66pj\">
                      <gml:posList>678095.264 5403805.978 370.34223000000003 678095.264 5403805.978 370.79823 678093.197 5403809.675 370.79823 678093.197 5403809.675 370.34223000000003 678095.264 5403805.978 370.34223000000003</gml:posList>
                    </gml:LinearRing>
                  </gml:interior>
                  <gml:interior>
                    <gml:LinearRing gml:id=\"4018106_LR.tXljCrPP3Efr0mz83aTx\">
                      <gml:posList>678095.254 5403805.996 368.30523 678095.254 5403805.996 370.06923 678093.187 5403809.693 370.06923 678093.187 5403809.693 368.30523 678095.254 5403805.996 368.30523</gml:posList>
                    </gml:LinearRing>
                  </gml:interior>
                  <gml:interior>
                    <gml:LinearRing gml:id=\"4018106_LR.gLnR6siy7dPwvvNX2zz0\">
                      <gml:posList>678095.558 5403805.452 370.06723 678095.558 5403805.452 368.30323 678097.625 5403801.755 368.30323 678097.625 5403801.755 370.06723 678095.558 5403805.452 370.06723</gml:posList>
                    </gml:LinearRing>
                  </gml:interior>
                  <gml:interior>
                    <gml:LinearRing gml:id=\"4018106_LR.Iw6I84mlFFHQEPQCpApK\">
                      <gml:posList>678097.625 5403801.755 370.34223000000003 678097.625 5403801.755 370.79223 678095.558 5403805.452 370.79223 678095.558 5403805.452 370.34223000000003 678097.625 5403801.755 370.34223000000003</gml:posList>
                    </gml:LinearRing>
                  </gml:interior>
                </gml:Polygon>
              </gml:surfaceMember>
            </gml:MultiSurface>";

        let result = deserialize_multi_surface(xml_document.as_ref()).unwrap();
        assert_eq!(result.surface_member().len(), 1);
    }

    #[test]
    fn deserialize_multi_surface_without_id() {
        let xml_document = b"<gml:MultiSurface>
              <gml:surfaceMember>
                <gml:Polygon>
                  <gml:exterior>
                    <gml:LinearRing>
                      <gml:posList srsDimension=\"3\">678009.7116291433 5403638.313338383 417.3480034550211 678012.5609078613 5403634.960884141 417.34658523466385 678013.7892528991 5403636.004867206 417.51938733855997 678010.9399743223 5403639.357321232 417.5208051908512 678009.7116291433 5403638.313338383 417.3480034550211</gml:posList>
                    </gml:LinearRing>
                  </gml:exterior>
                </gml:Polygon>
              </gml:surfaceMember>
            </gml:MultiSurface>";

        let result = deserialize_multi_surface(xml_document.as_ref()).unwrap();
        assert_eq!(result.surface_member().len(), 1);
    }

    #[test]
    fn deserialize_multi_surface_with_polygon_patches() {
        let xml_document = b"<gml:MultiSurface srsDimension=\"3\">
    <gml:surfaceMember>
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
        </gml:Surface>
    </gml:surfaceMember>
    <gml:surfaceMember>
        <gml:Surface>
            <gml:patches>
                <gml:PolygonPatch>
                    <gml:exterior>
                        <gml:LinearRing>
                            <gml:posList>350.54400634765625 972.9130249023438 0.11999999731779099 350.364013671875 968.6048030300961 0.11999999731779099 350.5414201635045 968.6025425887852 0.11999999731779099 350.54400634765625 972.9130249023438 0.11999999731779099</gml:posList>
                        </gml:LinearRing>
                    </gml:exterior>
                </gml:PolygonPatch>
            </gml:patches>
        </gml:Surface>
    </gml:surfaceMember>
</gml:MultiSurface>";

        let result = deserialize_multi_surface(xml_document.as_ref()).unwrap();
        assert_eq!(result.surface_member().len(), 2);
    }

    #[test]
    fn serialize_multi_surface_writes_gml_tags() {
        let multi_surface = make_multi_surface();
        let xml = serialize_multi_surface(&multi_surface).unwrap();

        assert!(xml.contains("<gml:MultiSurface"));
        assert!(xml.contains("<gml:surfaceMember"));
        assert!(xml.contains("<gml:Polygon"));
        assert!(xml.contains("<gml:exterior"));
        assert!(xml.contains("<gml:LinearRing"));
        assert!(!xml.contains("id="));
    }

    #[test]
    fn round_trip_multi_surface_preserves_member_count() {
        let multi_surface = make_multi_surface();
        let xml = serialize_multi_surface(&multi_surface).unwrap();
        let recovered = deserialize_multi_surface(xml.as_bytes()).unwrap();

        assert_eq!(
            recovered.surface_member().len(),
            multi_surface.surface_member().len()
        );
    }

    #[test]
    fn round_trip_multi_surface_from_xml() {
        let input_xml = "<gml:MultiSurface gml:id=\"test-id\">\
            <gml:surfaceMember><gml:Polygon><gml:exterior><gml:LinearRing><gml:posList srsDimension=\"3\">0 0 0 1 0 0 0 1 0 0 0 0</gml:posList></gml:LinearRing></gml:exterior></gml:Polygon></gml:surfaceMember>\
            </gml:MultiSurface>";

        let gml: GmlMultiSurface = de::from_reader(input_xml.as_bytes()).unwrap();
        let multi_surface: MultiSurface = gml.try_into().unwrap();
        let output_xml = serialize_multi_surface(&multi_surface).unwrap();

        assert_eq!(input_xml, output_xml);
    }
}
