use crate::Error;
use crate::codec::xal::{
    deserialize_country, deserialize_locality, deserialize_thoroughfare, serialize_country,
    serialize_locality, serialize_thoroughfare,
};
use crate::util::{
    DeserializationConfig, XalElement, XalNamespace, XmlDocumentIndex, XmlFragmentWriter,
    collect_child,
};
use egml_core::model::xal::Address;
use std::io::Write;

pub fn deserialize_address(
    xml_document: &[u8],
    index: &XmlDocumentIndex<XalElement>,
    config: &DeserializationConfig,
) -> Result<Address, Error> {
    let index = if index.is_truncated() {
        &XmlDocumentIndex::from_scan(xml_document, config.rescan_depth())?
    } else {
        index
    };

    let mut address = Address::new();

    let country = collect_child(
        xml_document,
        index,
        XalElement::Country,
        config,
        deserialize_country,
    )?;
    address.set_country_opt(country);

    let locality = collect_child(
        xml_document,
        index,
        XalElement::Locality,
        config,
        deserialize_locality,
    )?;
    address.set_locality_opt(locality);

    let thoroughfare = collect_child(
        xml_document,
        index,
        XalElement::Thoroughfare,
        config,
        deserialize_thoroughfare,
    )?;
    address.set_thoroughfare_opt(thoroughfare);

    Ok(address)
}

pub fn serialize_address<W: Write>(
    address: &Address,
    xml_fragment_writer: &mut XmlFragmentWriter<W>,
) -> Result<(), Error> {
    xml_fragment_writer.write_start_event(XalNamespace::Xal, XalElement::Address)?;

    if let Some(country) = address.country() {
        serialize_country(country, xml_fragment_writer)?;
    }

    if let Some(locality) = address.locality() {
        serialize_locality(locality, xml_fragment_writer)?;
    }

    if let Some(thoroughfare) = address.thoroughfare() {
        serialize_thoroughfare(thoroughfare, xml_fragment_writer)?;
    }

    xml_fragment_writer.write_end_event(XalNamespace::Xal, XalElement::Address)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{deserialize_address, serialize_address};
    use crate::util::{
        DeserializationConfig, Formatting, XalElement, XmlDocumentIndex, XmlFragmentWriter,
    };
    use egml_core::model::xal::enums::{
        CountryNameType, LocalityNameType, LocalityType, ThoroughfareNameType,
    };
    use egml_core::model::xal::{
        Address, Country, CountryName, Identifier, Locality, LocalityName, Thoroughfare,
        ThoroughfareName, ThoroughfareNameOrNumber,
    };

    fn germany() -> CountryName {
        CountryName::with_name_type("Germany".to_string(), CountryNameType::Name)
    }

    fn bhavani() -> Locality {
        let mut locality = Locality::new(vec![LocalityName::with_name_type(
            "Bhavani".to_string(),
            LocalityNameType::Name,
        )]);
        locality.set_locality_type(LocalityType::Town);
        locality
    }

    fn baker_street() -> Thoroughfare {
        let mut thoroughfare = Thoroughfare::new(vec![
            ThoroughfareNameOrNumber::Number(Identifier::new("39".to_string())),
            ThoroughfareNameOrNumber::Name(ThoroughfareName::with_name_type(
                "Baker".to_string(),
                ThoroughfareNameType::NameOnly,
            )),
        ]);
        thoroughfare.set_thoroughfare_type("Street");
        thoroughfare
    }

    #[test]
    fn deserialize_address_reads_country() {
        let xml_document = br#"<xAL:Address>
<xAL:Country>
<xAL:NameElement xAL:NameType="Name">Germany</xAL:NameElement>
</xAL:Country>
</xAL:Address>"#;

        let index = XmlDocumentIndex::<XalElement>::from_scan(xml_document, None).unwrap();
        let address = deserialize_address(xml_document, &index, &DeserializationConfig::default())
            .expect("should deserialize");

        assert_eq!(address.country().unwrap().name_elements(), [germany()]);
    }

    #[test]
    fn deserialize_address_reads_locality() {
        let xml_document = br#"<xAL:Address>
<xAL:Locality xAL:Type="Town">
<xAL:NameElement xAL:NameType="Name">Bhavani</xAL:NameElement>
</xAL:Locality>
</xAL:Address>"#;

        let index = XmlDocumentIndex::<XalElement>::from_scan(xml_document, None).unwrap();
        let address = deserialize_address(xml_document, &index, &DeserializationConfig::default())
            .expect("should deserialize");

        assert_eq!(address.locality().unwrap(), &bhavani());
    }

    #[test]
    fn deserialize_address_reads_thoroughfare() {
        let xml_document = br#"<xAL:Address>
<xAL:Thoroughfare xAL:Type="Street">
<xAL:Number>39</xAL:Number>
<xAL:NameElement xAL:NameType="NameOnly">Baker</xAL:NameElement>
</xAL:Thoroughfare>
</xAL:Address>"#;

        let index = XmlDocumentIndex::<XalElement>::from_scan(xml_document, None).unwrap();
        let address = deserialize_address(xml_document, &index, &DeserializationConfig::default())
            .expect("should deserialize");

        assert_eq!(address.thoroughfare().unwrap(), &baker_street());
    }

    #[test]
    fn serialize_address_writes_xal_tags() {
        let mut address = Address::new();
        address.set_country(Country::new(vec![germany()]));

        let mut writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_address(&address, &mut writer).expect("should serialize");
        let xml = String::from_utf8(writer.into_bytes()).unwrap();

        assert_eq!(
            xml,
            r#"<xAL:Address><xAL:Country><xAL:NameElement xAL:NameType="Name">Germany</xAL:NameElement></xAL:Country></xAL:Address>"#
        );
    }

    #[test]
    fn serialize_address_writes_locality() {
        let mut address = Address::new();
        address.set_locality(bhavani());

        let mut writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_address(&address, &mut writer).expect("should serialize");
        let xml = String::from_utf8(writer.into_bytes()).unwrap();

        assert_eq!(
            xml,
            r#"<xAL:Address><xAL:Locality xAL:Type="Town"><xAL:NameElement xAL:NameType="Name">Bhavani</xAL:NameElement></xAL:Locality></xAL:Address>"#
        );
    }

    #[test]
    fn serialize_address_writes_thoroughfare() {
        let mut address = Address::new();
        address.set_thoroughfare(baker_street());

        let mut writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_address(&address, &mut writer).expect("should serialize");
        let xml = String::from_utf8(writer.into_bytes()).unwrap();

        assert_eq!(
            xml,
            r#"<xAL:Address><xAL:Thoroughfare xAL:Type="Street"><xAL:Number>39</xAL:Number><xAL:NameElement xAL:NameType="NameOnly">Baker</xAL:NameElement></xAL:Thoroughfare></xAL:Address>"#
        );
    }

    #[test]
    fn round_trip_address() {
        let mut original = Address::new();
        original.set_country(Country::new(vec![germany()]));
        original.set_locality(bhavani());
        original.set_thoroughfare(baker_street());

        let mut writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_address(&original, &mut writer).unwrap();
        let xml = writer.into_bytes();

        let index = XmlDocumentIndex::<XalElement>::from_scan(&xml, None).unwrap();
        let recovered =
            deserialize_address(&xml, &index, &DeserializationConfig::default()).unwrap();

        assert_eq!(recovered, original);
    }

    #[test]
    fn round_trip_address_country_locality_and_thoroughfare_example() {
        let xml_document = r#"<xAL:Address>
<xAL:Country>
<xAL:NameElement xAL:NameType="Name">Germany</xAL:NameElement>
</xAL:Country>
<xAL:Locality xAL:Type="Town">
<xAL:NameElement xAL:NameType="Name">München</xAL:NameElement>
</xAL:Locality>
<xAL:Thoroughfare xAL:Type="Street">
<xAL:NameElement>Pacellistraße 8</xAL:NameElement>
</xAL:Thoroughfare>
</xAL:Address>"#
            .as_bytes();

        let index = XmlDocumentIndex::<XalElement>::from_scan(xml_document, None).unwrap();
        let address = deserialize_address(xml_document, &index, &DeserializationConfig::default())
            .expect("should deserialize");

        assert_eq!(address.country().unwrap().name_elements(), [germany()]);
        assert_eq!(
            address.locality().unwrap().name_elements(),
            [LocalityName::with_name_type(
                "München".to_string(),
                LocalityNameType::Name
            )]
        );
        assert_eq!(
            address.locality().unwrap().locality_type(),
            Some(LocalityType::Town)
        );
        assert_eq!(
            address.thoroughfare().unwrap().elements(),
            [ThoroughfareNameOrNumber::Name(ThoroughfareName::new(
                "Pacellistraße 8".to_string()
            ))]
        );
        assert_eq!(
            address.thoroughfare().unwrap().thoroughfare_type(),
            Some("Street")
        );

        let mut writer = XmlFragmentWriter::new_in_memory(Formatting::Compact);
        serialize_address(&address, &mut writer).expect("should serialize");
        let xml = writer.into_bytes();

        let index = XmlDocumentIndex::<XalElement>::from_scan(&xml, None).unwrap();
        let recovered =
            deserialize_address(&xml, &index, &DeserializationConfig::default()).unwrap();

        assert_eq!(recovered, address);
    }
}
