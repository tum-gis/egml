use crate::model::base::Id;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct AbstractGml {
    pub id: Option<Id>,
    pub name: Vec<String>,
}

impl AbstractGml {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_id(id: Id) -> Self {
        Self {
            id: Some(id),
            ..Default::default()
        }
    }

    pub fn with_optional_id(id: Option<Id>) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

pub trait AsAbstractGml {
    fn abstract_gml(&self) -> &AbstractGml;

    fn id(&self) -> Option<&Id> {
        self.abstract_gml().id.as_ref()
    }

    fn name(&self) -> &Vec<String> {
        &self.abstract_gml().name
    }
}

pub trait AsAbstractGmlMut: AsAbstractGml {
    fn abstract_gml_mut(&mut self) -> &mut AbstractGml;

    fn set_id(&mut self, id: Option<Id>) {
        self.abstract_gml_mut().id = id;
    }
}

impl AsAbstractGml for AbstractGml {
    fn abstract_gml(&self) -> &AbstractGml {
        self
    }
}

impl AsAbstractGmlMut for AbstractGml {
    fn abstract_gml_mut(&mut self) -> &mut AbstractGml {
        self
    }
}
