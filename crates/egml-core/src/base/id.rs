use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Id(String);

impl From<Id> for String {
    fn from(item: Id) -> Self {
        item.0
    }
}

impl From<String> for Id {
    fn from(item: String) -> Self {
        Self(item)
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
