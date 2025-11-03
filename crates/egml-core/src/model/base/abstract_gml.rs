use crate::model::base::Id;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AbstractGml {
    pub id: Id,
    pub name: Vec<String>,
}

impl AbstractGml {
    pub fn new(id: Id) -> Self {
        Self { id, name: vec![] }
    }
}
