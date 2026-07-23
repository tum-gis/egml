use crate::codec::measures::impl_measure_codec;

impl_measure_codec!(GmlArea, Area);

#[cfg(test)]
mod tests {
    use super::*;
    use egml_core::model::measures::Area;
    use quick_xml::de;

    #[test]
    fn deserialize_area() {
        let xml_document = b"<area uom=\"m2\">120.0</area>";

        let gml_area: GmlArea = de::from_reader(xml_document.as_ref()).expect("should work");
        let area = Area::from(gml_area);

        assert_eq!(area.uom(), "m2");
        assert_eq!(area.value(), 120.0);
    }
}
