use egml_core::model::xal::enums::LocalityNameType;

pub(crate) fn parse_locality_name_type(value: &str) -> Option<LocalityNameType> {
    match value {
        "Name" => Some(LocalityNameType::Name),
        "Number" => Some(LocalityNameType::Number),
        "ReferenceLocation" => Some(LocalityNameType::ReferenceLocation),
        "Type" => Some(LocalityNameType::Type),
        _ => None,
    }
}

pub(crate) fn locality_name_type_str(name_type: LocalityNameType) -> &'static str {
    match name_type {
        LocalityNameType::Name => "Name",
        LocalityNameType::Number => "Number",
        LocalityNameType::ReferenceLocation => "ReferenceLocation",
        LocalityNameType::Type => "Type",
    }
}
