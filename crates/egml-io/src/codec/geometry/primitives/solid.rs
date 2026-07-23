use crate::Error;
use crate::codec::geometry::primitives::abstract_solid::{
    deserialize_abstract_solid, serialize_abstract_solid,
};
use crate::codec::geometry::primitives::{deserialize_shell_property, serialize_shell_property};
use crate::util::{
    Formatting, GmlElement, XmlNode, XmlNodeContent, collect_child, collect_children,
    extract_xml_element_spans,
};
use egml_core::model::geometry::primitives::{AsAbstractSolid, ShellProperty, Solid};

pub fn deserialize_solid(xml_document: &[u8]) -> Result<Solid, Error> {
    let spans = extract_xml_element_spans(xml_document)?;
    let abstract_solid = deserialize_abstract_solid(xml_document, &spans)?;

    let exterior = collect_child(
        xml_document,
        &spans,
        GmlElement::ExteriorProperty,
        deserialize_shell_property,
    )?;
    let interior: Vec<ShellProperty> = collect_children(
        xml_document,
        &spans,
        GmlElement::InteriorProperty,
        deserialize_shell_property,
    )?;

    let mut solid = Solid::from_abstract_solid(abstract_solid, exterior);
    solid.set_interior(interior);
    Ok(solid)
}

pub fn serialize_solid(solid: &Solid, formatting: Formatting) -> Result<XmlNode, Error> {
    let mut xml_node_parts = serialize_abstract_solid(solid.abstract_solid(), formatting)?;

    if let Some(object) = &solid.exterior() {
        xml_node_parts
            .content
            .push(XmlNodeContent::Child(serialize_shell_property(
                object,
                formatting,
                GmlElement::ExteriorProperty.into(),
            )?));
    }
    for prop in solid.interior() {
        xml_node_parts
            .content
            .push(XmlNodeContent::Child(serialize_shell_property(
                prop,
                formatting,
                GmlElement::InteriorProperty.into(),
            )?));
    }

    Ok(XmlNode::new(GmlElement::Solid.into(), xml_node_parts))
}

#[cfg(test)]
mod tests {
    use crate::codec::geometry::primitives::solid::{deserialize_solid, serialize_solid};
    use crate::util::Formatting;
    use egml_core::model::base::HasAssociationAttributes;
    use egml_core::model::base::{AsAbstractGml, AsAbstractGmlMut};
    use egml_core::model::geometry::DirectPosition;
    use egml_core::model::geometry::primitives::{
        AbstractRingKind, AbstractRingProperty, AbstractSurfaceKind, AbstractSurfaceProperty,
        LinearRing, Polygon, Shell, ShellProperty, Solid,
    };

    fn make_solid() -> Solid {
        let points = vec![
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(0.0, 1.0, 0.0).unwrap(),
        ];
        let ring_kind = AbstractRingKind::LinearRing(LinearRing::new(points).unwrap());
        let polygon = Polygon::new(Some(AbstractRingProperty::from_object(ring_kind)), []).unwrap();
        let surface_prop =
            AbstractSurfaceProperty::from_object(AbstractSurfaceKind::Polygon(polygon));
        let shell = Shell::new([surface_prop]).expect("should create shell");
        let exterior = Some(ShellProperty::from_object(shell));

        Solid::new(exterior).unwrap()
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

        let solid_geometry = deserialize_solid(xml_document).unwrap();
        let exterior_shell = solid_geometry.exterior().unwrap().object().unwrap();

        assert_eq!(exterior_shell.members().len(), 2);
    }

    #[test]
    fn deserialize_solid_with_xlink_members() {
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

        let solid_geometry =
            deserialize_solid(xml_document).expect("should deserialize solid geometry");
        assert!(solid_geometry.exterior().is_some());

        let shell = solid_geometry
            .exterior()
            .as_ref()
            .unwrap()
            .object()
            .expect("should have exterior");

        assert_eq!(shell.members().len(), 3);
        assert!(shell.members().iter().all(|x| x.href().is_some()));
        assert!(shell.members().iter().all(|x| x.object().is_none()));
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

        let solid_geometry = deserialize_solid(xml_document).unwrap();
        let exterior_shell = solid_geometry.exterior().unwrap().object().unwrap();

        assert_eq!(exterior_shell.members().len(), 2);
    }

    #[test]
    fn serialize_solid_writes_gml_tags() {
        let solid = make_solid();

        let xml_node = serialize_solid(&solid, Formatting::Compact).expect("should serialize");
        let xml = xml_node
            .to_string(Formatting::Compact)
            .expect("to string should work");

        assert!(xml.contains("<gml:Solid"));
        assert!(xml.contains("<gml:exterior"));
        assert!(xml.contains("<gml:Shell"));
        assert!(xml.contains("<gml:surfaceMember"));
        assert!(xml.contains("<gml:Polygon"));
        assert!(!xml.contains("id="));
    }

    #[test]
    fn serialize_solid_with_id_writes_id() {
        use egml_core::model::base::Id;

        let mut solid = make_solid();
        solid.set_id(Id::try_from("test-id").unwrap());

        let xml_node = serialize_solid(&solid, Formatting::Compact).expect("should serialize");
        let xml = xml_node
            .to_string(Formatting::Compact)
            .expect("to string should work");

        assert!(xml.contains("gml:id=\"test-id\""));
    }

    #[test]
    fn round_trip_solid_preserves_member_count() {
        let solid = make_solid();

        let xml_node = serialize_solid(&solid, Formatting::Compact).expect("should serialize");
        let xml = xml_node.to_string(Formatting::Compact).unwrap();
        let recovered = deserialize_solid(xml.as_bytes()).unwrap();

        assert_eq!(
            solid.exterior().unwrap().object().unwrap().members().len(),
            recovered
                .exterior()
                .unwrap()
                .object()
                .unwrap()
                .members()
                .len(),
        );
    }

    #[test]
    fn deserialize_solid_with_interior() {
        let xml_document = b"\
        <gml:Solid>
          <gml:exterior>
            <gml:Shell>
              <gml:surfaceMember xlink:href=\"#ext\"/>
            </gml:Shell>
          </gml:exterior>
          <gml:interior>
            <gml:Shell>
              <gml:surfaceMember xlink:href=\"#int1\"/>
              <gml:surfaceMember xlink:href=\"#int2\"/>
            </gml:Shell>
          </gml:interior>
        </gml:Solid>";

        let solid = deserialize_solid(xml_document).expect("should deserialize");

        assert!(solid.exterior().is_some());
        assert_eq!(solid.interior().len(), 1);
        let interior_shell = solid.interior()[0]
            .object()
            .expect("should have interior shell");
        assert_eq!(interior_shell.members().len(), 2);
    }

    #[test]
    fn round_trip_solid_with_interior() {
        let xml_document = b"<gml:Solid>\
            <gml:exterior><gml:Shell>\
            <gml:surfaceMember><gml:Polygon><gml:exterior><gml:LinearRing>\
            <gml:posList srsDimension=\"3\">0 0 0 1 0 0 0 1 0 0 0 0</gml:posList>\
            </gml:LinearRing></gml:exterior></gml:Polygon></gml:surfaceMember>\
            </gml:Shell></gml:exterior>\
            <gml:interior><gml:Shell>\
            <gml:surfaceMember><gml:Polygon><gml:exterior><gml:LinearRing>\
            <gml:posList srsDimension=\"3\">0 0 1 1 0 1 0 1 1 0 0 1</gml:posList>\
            </gml:LinearRing></gml:exterior></gml:Polygon></gml:surfaceMember>\
            </gml:Shell></gml:interior>\
            </gml:Solid>";

        let solid = deserialize_solid(xml_document).expect("should deserialize");
        let xml_node = serialize_solid(&solid, Formatting::Compact).expect("should serialize");
        let output = xml_node.to_string(Formatting::Compact).unwrap();
        let recovered = deserialize_solid(output.as_bytes()).expect("should deserialize recovered");

        assert_eq!(solid.interior().len(), recovered.interior().len());
        assert_eq!(
            solid.interior()[0].object().unwrap().members().len(),
            recovered.interior()[0].object().unwrap().members().len(),
        );
    }

    #[test]
    fn round_trip_solid_from_xml() {
        let xml_document = b"<gml:Solid gml:id=\"test-id\">\
            <gml:exterior><gml:Shell>\
            <gml:surfaceMember><gml:Polygon><gml:exterior><gml:LinearRing>\
            <gml:posList srsDimension=\"3\">0 0 0 1 0 0 0 1 0 0 0 0</gml:posList>\
            </gml:LinearRing></gml:exterior></gml:Polygon></gml:surfaceMember>\
            </gml:Shell></gml:exterior>\
            </gml:Solid>";

        let solid = deserialize_solid(xml_document).expect("should deserialize");
        let xml_node = serialize_solid(&solid, Formatting::Compact).expect("should serialize");
        let output = xml_node.to_string(Formatting::Compact).unwrap();

        let recovered = deserialize_solid(output.as_bytes()).expect("should deserialize recovered");

        assert_eq!(
            recovered
                .exterior()
                .unwrap()
                .object()
                .unwrap()
                .members()
                .len(),
            solid.exterior().unwrap().object().unwrap().members().len(),
        );
        assert_eq!(recovered.id(), solid.id());
    }
}
