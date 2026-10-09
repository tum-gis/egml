use egml_core::model::xal::enums::ThoroughfareNameType;

pub(crate) fn parse_thoroughfare_name_type(value: &str) -> Option<ThoroughfareNameType> {
    match value {
        "NameOnly" => Some(ThoroughfareNameType::NameOnly),
        "PreDirection" => Some(ThoroughfareNameType::PreDirection),
        "PostDirection" => Some(ThoroughfareNameType::PostDirection),
        "NameAndNumber" => Some(ThoroughfareNameType::NameAndNumber),
        "NameAndType" => Some(ThoroughfareNameType::NameAndType),
        "NameNumberAndType" => Some(ThoroughfareNameType::NameNumberAndType),
        "Unstructured" => Some(ThoroughfareNameType::Unstructured),
        "SubThoroughfareConnector" => Some(ThoroughfareNameType::SubThoroughfareConnector),
        "ReferenceLocation" => Some(ThoroughfareNameType::ReferenceLocation),
        "Type" => Some(ThoroughfareNameType::Type),
        _ => None,
    }
}

pub(crate) fn thoroughfare_name_type_str(name_type: ThoroughfareNameType) -> &'static str {
    match name_type {
        ThoroughfareNameType::NameOnly => "NameOnly",
        ThoroughfareNameType::PreDirection => "PreDirection",
        ThoroughfareNameType::PostDirection => "PostDirection",
        ThoroughfareNameType::NameAndNumber => "NameAndNumber",
        ThoroughfareNameType::NameAndType => "NameAndType",
        ThoroughfareNameType::NameNumberAndType => "NameNumberAndType",
        ThoroughfareNameType::Unstructured => "Unstructured",
        ThoroughfareNameType::SubThoroughfareConnector => "SubThoroughfareConnector",
        ThoroughfareNameType::ReferenceLocation => "ReferenceLocation",
        ThoroughfareNameType::Type => "Type",
    }
}
