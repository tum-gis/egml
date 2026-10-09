use egml_core::model::xal::enums::IdentifierElementType;

pub(crate) fn parse_identifier_element_type(value: &str) -> Option<IdentifierElementType> {
    match value {
        "Name" => Some(IdentifierElementType::Name),
        "RangeFrom" => Some(IdentifierElementType::RangeFrom),
        "Range" => Some(IdentifierElementType::Range),
        "RangeTo" => Some(IdentifierElementType::RangeTo),
        "Prefix" => Some(IdentifierElementType::Prefix),
        "Suffix" => Some(IdentifierElementType::Suffix),
        "Number" => Some(IdentifierElementType::Number),
        "Separator" => Some(IdentifierElementType::Separator),
        "Extension" => Some(IdentifierElementType::Extension),
        _ => None,
    }
}

pub(crate) fn identifier_element_type_str(identifier_type: IdentifierElementType) -> &'static str {
    match identifier_type {
        IdentifierElementType::Name => "Name",
        IdentifierElementType::RangeFrom => "RangeFrom",
        IdentifierElementType::Range => "Range",
        IdentifierElementType::RangeTo => "RangeTo",
        IdentifierElementType::Prefix => "Prefix",
        IdentifierElementType::Suffix => "Suffix",
        IdentifierElementType::Number => "Number",
        IdentifierElementType::Separator => "Separator",
        IdentifierElementType::Extension => "Extension",
    }
}
