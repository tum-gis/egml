use egml_core::model::xal::enums::CountryNameType;

pub(crate) fn parse_country_name_type(value: &str) -> Option<CountryNameType> {
    match value {
        "Name" => Some(CountryNameType::Name),
        "Type" => Some(CountryNameType::Type),
        _ => None,
    }
}

pub(crate) fn country_name_type_str(name_type: CountryNameType) -> &'static str {
    match name_type {
        CountryNameType::Name => "Name",
        CountryNameType::Type => "Type",
    }
}
