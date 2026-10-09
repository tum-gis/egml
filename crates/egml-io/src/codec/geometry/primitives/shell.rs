use crate::Error;
use crate::codec::geometry::primitives::{
    deserialize_abstract_surface, deserialize_abstract_surface_property,
    serialize_abstract_surface, serialize_abstract_surface_attributes,
    serialize_abstract_surface_property,
};
use crate::util::{
    DeserializationConfig, GmlElement, GmlNamespace, XmlDocumentIndex, XmlFragmentWriter,
    collect_children_lenient,
};
use egml_core::model::geometry::primitives::{AsAbstractSurface, Shell};
use std::io::Write;
use tracing::debug;

pub fn deserialize_shell(
    xml_document: &[u8],
    index: &XmlDocumentIndex<GmlElement>,
    config: &DeserializationConfig,
) -> Result<Shell, Error> {
    let index = if index.is_truncated() {
        &XmlDocumentIndex::from_scan(xml_document, config.rescan_depth())?
    } else {
        index
    };

    let abstract_surface = deserialize_abstract_surface(xml_document, index, config)?;

    let (members, skipped) = collect_children_lenient(
        xml_document,
        index,
        GmlElement::SurfaceMemberProperty,
        config,
        deserialize_abstract_surface_property,
    );
    if !skipped.is_empty() {
        debug!(
            count = skipped.len(),
            "Shell: dropped invalid surfaceMember(s)"
        );
    }

    Ok(Shell::from_abstract_surface(abstract_surface, members)?)
}

pub fn serialize_shell<W: Write>(
    shell: &Shell,
    xml_fragment_writer: &mut XmlFragmentWriter<W>,
) -> Result<(), Error> {
    let attributes = serialize_shell_attributes(shell);

    xml_fragment_writer.write_start_event_with_attributes(
        GmlNamespace::Gml,
        GmlElement::Shell,
        attributes,
    )?;

    serialize_abstract_surface(shell.abstract_surface(), xml_fragment_writer)?;

    for member in shell.members() {
        serialize_abstract_surface_property(
            member,
            xml_fragment_writer,
            GmlNamespace::Gml,
            GmlElement::SurfaceMemberProperty,
        )?;
    }

    xml_fragment_writer.write_end_event(GmlNamespace::Gml, GmlElement::Shell)?;

    Ok(())
}

pub fn serialize_shell_attributes(shell: &Shell) -> Vec<(String, String)> {
    serialize_abstract_surface_attributes(shell.abstract_surface())
}

#[cfg(test)]
mod tests {
    // Test-only convenience: builds the index the real function now
    // requires, so existing single-argument call sites below don't all
    // need to construct one by hand.
    fn deserialize(xml_document: &[u8]) -> Result<super::Shell, crate::Error> {
        let index = crate::util::XmlDocumentIndex::from_scan(xml_document, None)?;
        super::deserialize_shell(
            xml_document,
            &index,
            &crate::util::DeserializationConfig::default(),
        )
    }

    use crate::codec::geometry::primitives::shell::serialize_shell;
    use crate::util::{Formatting, XmlFragmentWriter};
    use egml_core::model::geometry::DirectPosition;
    use egml_core::model::geometry::primitives::{
        AbstractRingKind, AbstractSurfaceKind, AbstractSurfaceProperty, LinearRing, Polygon, Shell,
    };

    fn make_shell() -> Shell {
        let ring = LinearRing::new([
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(0.0, 1.0, 0.0).unwrap(),
        ])
        .unwrap();
        let polygon = Polygon::new(Some(AbstractRingKind::LinearRing(ring)), []).unwrap();
        let member = AbstractSurfaceProperty::from_object(AbstractSurfaceKind::Polygon(polygon));
        Shell::new([member]).unwrap()
    }

    #[test]
    fn deserialize_shell_test() {
        let xml_document = b"<gml:Shell>
            <gml:surfaceMember>
                <gml:Polygon>
                    <gml:exterior>
                        <gml:LinearRing>
                            <gml:posList srsDimension=\"3\">0 0 0 1 0 0 0 1 0 0 0 0</gml:posList>
                        </gml:LinearRing>
                    </gml:exterior>
                </gml:Polygon>
            </gml:surfaceMember>
            <gml:surfaceMember>
                <gml:Polygon>
                    <gml:exterior>
                        <gml:LinearRing>
                            <gml:posList srsDimension=\"3\">1 0 0 1 1 0 0 1 0 1 0 0</gml:posList>
                        </gml:LinearRing>
                    </gml:exterior>
                </gml:Polygon>
            </gml:surfaceMember>
        </gml:Shell>";

        let shell = deserialize(xml_document).expect("should deserialize");

        assert_eq!(shell.members().len(), 2);
    }

    #[test]
    fn deserialize_shell_skips_invalid_polygon_member() {
        let xml_document = b"<gml:Shell>
            <gml:surfaceMember>
                <gml:Polygon>
                    <gml:exterior>
                        <gml:LinearRing>
                            <gml:posList>0 0 0 1 0 0</gml:posList>
                        </gml:LinearRing>
                    </gml:exterior>
                </gml:Polygon>
            </gml:surfaceMember>
            <gml:surfaceMember>
                <gml:Polygon>
                    <gml:exterior>
                        <gml:LinearRing>
                            <gml:posList srsDimension=\"3\">0 0 0 1 0 0 0 1 0 0 0 0</gml:posList>
                        </gml:LinearRing>
                    </gml:exterior>
                </gml:Polygon>
            </gml:surfaceMember>
        </gml:Shell>";

        let shell = deserialize(xml_document).expect("should deserialize");

        assert_eq!(shell.members().len(), 1);
    }

    #[test]
    fn serialize_shell_writes_gml_tags() {
        let shell = make_shell();

        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_shell(&shell, &mut xml_fragment_writer).expect("should serialize");
        let xml = String::from_utf8(xml_fragment_writer.into_bytes()).expect("should serialize");

        assert!(xml.contains("<gml:Shell"));
        assert!(xml.contains("<gml:surfaceMember"));
        assert!(xml.contains("<gml:Polygon"));
        assert!(xml.contains("<gml:exterior"));
        assert!(xml.contains("<gml:LinearRing"));
    }

    #[test]
    fn round_trip_shell_preserves_member_count() {
        let shell = make_shell();

        let mut xml_fragment_writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_shell(&shell, &mut xml_fragment_writer).expect("should serialize");
        let xml = String::from_utf8(xml_fragment_writer.into_bytes()).expect("should serialize");

        let recovered = deserialize(xml.as_bytes()).expect("should deserialize");

        assert_eq!(recovered.members().len(), shell.members().len());
    }
}
