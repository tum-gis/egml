use crate::codec::measures::impl_measure_codec;

impl_measure_codec!(GmlTime, Time);

#[cfg(test)]
mod tests {
    use super::*;
    use egml_core::model::measures::Time;
    use quick_xml::de;

    #[test]
    fn deserialize_time() {
        let xml_document = b"<time uom=\"s\">3.0</time>";

        let gml_time: GmlTime = de::from_reader(xml_document.as_ref()).expect("should work");
        let time = Time::from(gml_time);

        assert_eq!(time.uom(), "s");
        assert_eq!(time.value(), 3.0);
    }
}
