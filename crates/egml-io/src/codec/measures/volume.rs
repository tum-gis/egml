use crate::codec::measures::impl_measure_codec;

impl_measure_codec!(GmlVolume, Volume);

#[cfg(test)]
mod tests {
    use super::*;
    use egml_core::model::measures::Volume;
    use quick_xml::de;

    #[test]
    fn deserialize_volume() {
        let xml_document = b"<volume uom=\"m3\">5.0</volume>";

        let gml_volume: GmlVolume = de::from_reader(xml_document.as_ref()).expect("should work");
        let volume = Volume::from(gml_volume);

        assert_eq!(volume.uom(), "m3");
        assert_eq!(volume.value(), 5.0);
    }
}
