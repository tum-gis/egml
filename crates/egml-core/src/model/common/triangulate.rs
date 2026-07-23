use crate::Error;
use crate::model::geometry::primitives::TriangulatedSurface;

pub trait Triangulate {
    fn triangulate(&self) -> Result<Triangulation, Error>;
}

/// The result of triangulating something that tolerates non-fatal member
/// failures: the combined surface, plus the errors from any members that
/// were skipped along the way (e.g. a degenerate ring). Mirrors
/// rust-analyzer's `Parse<T>` / oxc's `ParserReturn`.
#[derive(Debug, Clone, PartialEq)]
pub struct Triangulation {
    surface: TriangulatedSurface,
    skipped: Vec<Error>,
}

impl Triangulation {
    pub fn new(surface: TriangulatedSurface, skipped: Vec<Error>) -> Self {
        Self { surface, skipped }
    }

    pub fn surface(&self) -> &TriangulatedSurface {
        &self.surface
    }

    pub fn into_surface(self) -> TriangulatedSurface {
        self.surface
    }

    pub fn skipped(&self) -> &[Error] {
        &self.skipped
    }

    pub fn has_skipped(&self) -> bool {
        !self.skipped.is_empty()
    }

    pub fn into_parts(self) -> (TriangulatedSurface, Vec<Error>) {
        (self.surface, self.skipped)
    }
}
