use crate::error::Error;
use crate::error::Error::NotEnoughElements;

use crate::Error::{ContainsDuplicateElements, ContainsEqualStartAndLastElement};
use crate::model::base::AbstractGml;
use crate::model::geometry::{DirectPosition, Triangle, TriangulatedSurface};
use crate::operations::geometry::Geometry;
use crate::operations::surface::Surface;
use crate::operations::triangulate::Triangulate;
use nalgebra::Isometry3;
use rayon::prelude::*;

const MINIMUM_NUMBER_OF_POINTS: usize = 3;

#[derive(Debug, Clone, PartialEq)]
pub struct LinearRing {
    pub abstract_gml: AbstractGml,
    points: Vec<DirectPosition>,
}

impl LinearRing {
    pub fn new(abstract_gml: AbstractGml, points: Vec<DirectPosition>) -> Result<Self, Error> {
        let duplicates_count = points.windows(2).filter(|x| x[0] == x[1]).count();
        if duplicates_count >= 1 {
            return Err(ContainsDuplicateElements);
        }
        if points.len() < MINIMUM_NUMBER_OF_POINTS {
            return Err(NotEnoughElements(
                "Linear ring must at least have three unique points",
            ));
        }
        if points.first().expect("") == points.last().expect("") {
            return Err(ContainsEqualStartAndLastElement);
        }

        Ok(Self {
            abstract_gml,
            points,
        })
    }

    pub fn set_points(&mut self, val: Vec<DirectPosition>) -> Result<(), Error> {
        let duplicates_count = val.windows(2).filter(|x| x[0] == x[1]).count();
        if duplicates_count >= 1 {
            return Err(ContainsDuplicateElements);
        }
        if val.len() < MINIMUM_NUMBER_OF_POINTS {
            return Err(NotEnoughElements(
                "Linear ring must at least have three unique points",
            ));
        }
        self.points = val;
        Ok(())
    }
}

impl Geometry for LinearRing {
    fn points(&self) -> Vec<&DirectPosition> {
        self.points.iter().collect()
    }

    fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.points.iter_mut().for_each(|p| {
            p.apply_transform(m);
        });
        // self.points.dedup(); would need error handling
    }
}

impl Surface for LinearRing {
    fn outer_boundary_points(&self) -> Vec<&DirectPosition> {
        self.points.iter().collect()
    }
}

impl Triangulate for LinearRing {
    fn triangulate(&self) -> Result<TriangulatedSurface, Error> {
        let vertices_3d = self.points.iter().map(|p| p.coords()).collect::<Vec<_>>();
        let mut vertices_2d_buf = Vec::new();
        earcut::utils3d::project3d_to_2d(&vertices_3d, vertices_3d.len(), &mut vertices_2d_buf);

        let mut triangle_indices: Vec<usize> = vec![];
        let mut earcut = earcut::Earcut::new();
        earcut.earcut(vertices_2d_buf.iter().copied(), &[], &mut triangle_indices);

        let triangles: Vec<Triangle> = triangle_indices
            .chunks(3)
            .map(|x| {
                let vertex_a = self.points[x[0]];
                let vertex_b = self.points[x[1]];
                let vertex_c = self.points[x[2]];
                Triangle::new(vertex_a, vertex_b, vertex_c).expect("should work")
            })
            .collect::<Vec<_>>();

        let triangulated_surface = TriangulatedSurface::new(triangles)?;
        Ok(triangulated_surface)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::model::base::Id;

    #[test]
    fn triangulate() {
        let abstract_gml = AbstractGml::new(Id::try_from("test_id").expect("must work"));
        let linear_ring = LinearRing::new(
            abstract_gml,
            vec![
                DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
                DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
                DirectPosition::new(1.0, 1.0, 0.0).unwrap(),
                DirectPosition::new(0.0, 1.0, 0.0).unwrap(),
            ],
        )
        .unwrap();

        let result = &linear_ring.triangulate().unwrap();

        assert_eq!(result.number_of_patches(), 2);
        assert!(result.patches()[0].area() > 0.0);
        assert!(result.patches()[1].area() > 0.0);
    }
}
