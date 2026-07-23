use crate::codec::measures::impl_measure_codec;

impl_measure_codec!(GmlLength, Length);

#[cfg(test)]
mod tests {
    use super::*;
    use egml_core::model::measures::Length;
    use quick_xml::de;

    #[test]
    fn deserialize_length() {
        let xml_document = b"<length uom=\"m\">12.5</length>";

        let gml_length: GmlLength = de::from_reader(xml_document.as_ref()).expect("should work");
        let length = Length::from(gml_length);

        assert_eq!(length.uom(), "m");
        assert_eq!(length.value(), 12.5);
    }
}
