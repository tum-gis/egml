use crate::Error;
use crate::codec::base::{
    deserialize_abstract_gml, serialize_abstract_gml, serialize_abstract_gml_attributes,
};
use crate::codec::feature::bounding_shape::deserialize_bounding_shape;
use crate::codec::feature::serialize_bounding_shape;
use crate::util::{DeserializationConfig, GmlElement, XmlDocumentIndex, XmlFragmentWriter};
use egml_core::model::base::AsAbstractGml;
use egml_core::model::feature::{
    AbstractFeature, AsAbstractFeature, AsAbstractFeatureMut, BoundingShape,
};
use std::io::Write;

pub fn deserialize_abstract_feature(
    xml_document: &[u8],
    index: &XmlDocumentIndex<GmlElement>,
    config: &DeserializationConfig,
) -> Result<AbstractFeature, Error> {
    debug_assert!(
        !index.is_truncated(),
        "deserialize_abstract_feature received a truncated index — caller must scan to sufficient depth"
    );

    let abstract_gml = deserialize_abstract_gml(xml_document, index, config)?;
    let mut abstract_feature = AbstractFeature::from_abstract_gml(abstract_gml);

    let bounded_by: Option<BoundingShape> = index
        .first(GmlElement::BoundedByProperty)
        .map(|x| deserialize_bounding_shape(&xml_document[x.range()], x, config))
        .transpose()?;
    abstract_feature.set_bounded_by(bounded_by);

    Ok(abstract_feature)
}

pub fn serialize_abstract_feature<W: Write>(
    abstract_feature: &AbstractFeature,
    xml_fragment_writer: &mut XmlFragmentWriter<W>,
) -> Result<(), Error> {
    serialize_abstract_gml(abstract_feature.abstract_gml(), xml_fragment_writer)?;

    if let Some(bounded_by) = abstract_feature.bounded_by() {
        serialize_bounding_shape(bounded_by, xml_fragment_writer)?;
    }

    Ok(())
}

pub fn serialize_abstract_feature_attributes(
    abstract_feature: &AbstractFeature,
) -> Vec<(String, String)> {
    serialize_abstract_gml_attributes(abstract_feature.abstract_gml())
}

#[cfg(test)]
mod tests {
    use crate::codec::feature::abstract_feature::deserialize_abstract_feature;
    use crate::util::{GmlElement, XmlDocumentIndex};
    use egml_core::model::feature::{AbstractFeature, AsAbstractFeature};
    use egml_core::model::geometry::Envelope;

    #[test]
    fn deserialize_simple_abstract_feature() {
        let xml_document = b"<ExampleFeature gml:id=\"UUID_7580dd4b-0f98-3428-a3ab-dfbc85853d86\">
    <gml:boundedBy>
        <gml:Envelope srsDimension=\"3\" srsName=\"urn:ogc:def:crs:EPSG::25832\">
            <gml:lowerCorner>1.0 2.0 3</gml:lowerCorner>
            <gml:upperCorner>11.0 12.0 13.0</gml:upperCorner>
        </gml:Envelope>
    </gml:boundedBy>
</ExampleFeature>";

        let index: XmlDocumentIndex<GmlElement> =
            XmlDocumentIndex::from_scan(xml_document, None).expect("should work");
        let abstract_feature: AbstractFeature = deserialize_abstract_feature(
            xml_document.as_ref(),
            &index,
            &crate::util::DeserializationConfig::default(),
        )
        .unwrap();

        let envelope: Envelope = abstract_feature
            .bounded_by()
            .unwrap()
            .envelope()
            .unwrap()
            .clone();
        assert_eq!(envelope.lower_corner().x(), 1.0);
        assert_eq!(envelope.lower_corner().y(), 2.0);
        assert_eq!(envelope.lower_corner().z(), 3.0);
        assert_eq!(envelope.upper_corner().x(), 11.0);
        assert_eq!(envelope.upper_corner().y(), 12.0);
        assert_eq!(envelope.upper_corner().z(), 13.0);
    }
}
