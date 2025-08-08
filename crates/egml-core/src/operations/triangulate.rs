use crate::Error;
use crate::model::geometry::TriangulatedSurface;

pub trait Triangulate {
    fn triangulate(&self) -> Result<TriangulatedSurface, Error>;
}
