use crate::codec::measures::impl_measure_codec;

impl_measure_codec!(GmlGridLength, GridLength);

#[cfg(test)]
mod tests {
    use super::*;
    use egml_core::model::measures::GridLength;
    use quick_xml::de;

    #[test]
    fn deserialize_grid_length() {
        let xml_document = b"<gridLength uom=\"gridspacing\">1.0</gridLength>";

        let gml_grid_length: GmlGridLength =
            de::from_reader(xml_document.as_ref()).expect("should work");
        let grid_length = GridLength::from(gml_grid_length);

        assert_eq!(grid_length.uom(), "gridspacing");
        assert_eq!(grid_length.value(), 1.0);
    }
}
