#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct AbstractObject {}

impl AbstractObject {
    pub fn new() -> Self {
        Self {}
    }
}

pub trait AsAbstractObject {
    fn abstract_object(&self) -> &AbstractObject;
}

pub trait AsAbstractObjectMut: AsAbstractObject {
    fn abstract_object_mut(&mut self) -> &mut AbstractObject;
}

impl AsAbstractObject for AbstractObject {
    fn abstract_object(&self) -> &AbstractObject {
        self
    }
}

impl AsAbstractObjectMut for AbstractObject {
    fn abstract_object_mut(&mut self) -> &mut AbstractObject {
        self
    }
}
