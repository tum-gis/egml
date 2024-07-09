use crate::error::Error;
use crate::geometry::primitives::triangle::Triangle;

#[derive(Debug, Clone, PartialEq)]
pub struct TriangulatedSurface {
    patches: Vec<Triangle>,
}

impl TriangulatedSurface {
    pub fn new(patches: Vec<Triangle>) -> Result<Self, Error> {
        if patches.is_empty() {
            return Err(Error::MustNotBeEmpty(""));
        }

        Ok(TriangulatedSurface { patches })
    }

    pub fn patches(&self) -> &Vec<Triangle> {
        self.patches.as_ref()
    }

    pub fn append_patches(&mut self, mut patches: Vec<Triangle>) {
        self.patches.append(&mut patches)
    }

    pub fn number_of_patches(&self) -> usize {
        self.patches.len()
    }
}
