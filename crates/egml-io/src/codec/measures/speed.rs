use crate::codec::measures::impl_measure_codec;

impl_measure_codec!(GmlSpeed, Speed);

#[cfg(test)]
mod tests {
    use super::*;
    use egml_core::model::measures::Speed;
    use quick_xml::de;

    #[test]
    fn deserialize_speed() {
        let xml_document = b"<speed uom=\"m/s\">2.5</speed>";

        let gml_speed: GmlSpeed = de::from_reader(xml_document.as_ref()).expect("should work");
        let speed = Speed::from(gml_speed);

        assert_eq!(speed.uom(), "m/s");
        assert_eq!(speed.value(), 2.5);
    }
}
