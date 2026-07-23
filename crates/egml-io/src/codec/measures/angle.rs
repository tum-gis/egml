use crate::codec::measures::impl_measure_codec;

impl_measure_codec!(GmlAngle, Angle);

#[cfg(test)]
mod tests {
    use super::*;
    use egml_core::model::measures::Angle;
    use quick_xml::de;

    #[test]
    fn deserialize_angle() {
        let xml_document = b"<angle uom=\"deg\">90.0</angle>";

        let gml_angle: GmlAngle = de::from_reader(xml_document.as_ref()).expect("should work");
        let angle = Angle::from(gml_angle);

        assert_eq!(angle.uom(), "deg");
        assert_eq!(angle.value(), 90.0);
    }
}
