use crate::model::xal::enums::CountryNameType;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CountryName {
    content: String,
    name_type: Option<CountryNameType>,
}

impl CountryName {
    pub fn new(content: String) -> Self {
        Self {
            content,
            name_type: None,
        }
    }

    pub fn with_name_type(content: String, name_type: CountryNameType) -> Self {
        Self {
            content,
            name_type: Some(name_type),
        }
    }

    /// Equivalent to [`with_name_type`](Self::with_name_type) when `name_type`
    /// is `Some`, and to [`new`](Self::new) when it's `None`.
    pub fn with_optional_name_type(content: String, name_type: Option<CountryNameType>) -> Self {
        match name_type {
            Some(name_type) => Self::with_name_type(content, name_type),
            None => Self::new(content),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn name_type(&self) -> Option<CountryNameType> {
        self.name_type
    }
}
