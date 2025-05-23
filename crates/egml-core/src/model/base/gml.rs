use crate::model::base::Id;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Gml {
    pub id: Id,
    pub name: Vec<String>,
}

impl Gml {
    pub fn new(id: Id) -> Self {
        Self { id, name: vec![] }
    }
}
