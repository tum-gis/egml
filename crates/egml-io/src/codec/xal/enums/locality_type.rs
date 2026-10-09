use egml_core::model::xal::enums::LocalityType;

pub(crate) fn parse_locality_type(value: &str) -> Option<LocalityType> {
    match value {
        "Municipality" => Some(LocalityType::Municipality),
        "PostTown" => Some(LocalityType::PostTown),
        "Place" => Some(LocalityType::Place),
        "Suburb" => Some(LocalityType::Suburb),
        "Town" => Some(LocalityType::Town),
        "Village" => Some(LocalityType::Village),
        "Area" => Some(LocalityType::Area),
        "Zone" => Some(LocalityType::Zone),
        _ => None,
    }
}

pub(crate) fn locality_type_str(locality_type: LocalityType) -> &'static str {
    match locality_type {
        LocalityType::Municipality => "Municipality",
        LocalityType::PostTown => "PostTown",
        LocalityType::Place => "Place",
        LocalityType::Suburb => "Suburb",
        LocalityType::Town => "Town",
        LocalityType::Village => "Village",
        LocalityType::Area => "Area",
        LocalityType::Zone => "Zone",
    }
}
