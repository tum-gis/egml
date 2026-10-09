use crate::model::xal::enums::LocalityNameType;

/// One `xAL:NameElement` of a [`Locality`](crate::model::xal::Locality) — a
/// name fragment plus what it represents (e.g. the locality's name vs. a
/// reference location used to describe it).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LocalityName {
    content: String,
    name_type: Option<LocalityNameType>,
}

impl LocalityName {
    pub fn new(content: String) -> Self {
        Self {
            content,
            name_type: None,
        }
    }

    pub fn with_name_type(content: String, name_type: LocalityNameType) -> Self {
        Self {
            content,
            name_type: Some(name_type),
        }
    }

    /// Equivalent to [`with_name_type`](Self::with_name_type) when `name_type`
    /// is `Some`, and to [`new`](Self::new) when it's `None`.
    pub fn with_optional_name_type(content: String, name_type: Option<LocalityNameType>) -> Self {
        match name_type {
            Some(name_type) => Self::with_name_type(content, name_type),
            None => Self::new(content),
        }
    }

    /// The name text itself, e.g. `"Bhavani"`.
    pub fn content(&self) -> &str {
        &self.content
    }

    /// What this name represents, e.g. `Name` vs. `ReferenceLocation`.
    pub fn name_type(&self) -> Option<LocalityNameType> {
        self.name_type
    }
}
