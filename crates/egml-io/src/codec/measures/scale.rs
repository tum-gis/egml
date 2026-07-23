use crate::codec::measures::impl_measure_codec;

impl_measure_codec!(GmlScale, Scale);

#[cfg(test)]
mod tests {
    use super::*;
    use egml_core::model::measures::Scale;
    use quick_xml::de;

    #[test]
    fn deserialize_scale() {
        let xml_document = b"<scale uom=\"unity\">0.5</scale>";

        let gml_scale: GmlScale = de::from_reader(xml_document.as_ref()).expect("should work");
        let scale = Scale::from(gml_scale);

        assert_eq!(scale.uom(), "unity");
        assert_eq!(scale.value(), 0.5);
    }
}
