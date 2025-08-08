use crate::Error;
use crate::model::base::Gml;
use crate::model::geometry::{DirectPosition, Envelope, LinearRing, Triangle, TriangulatedSurface};
use crate::operations::geometry::Geometry;
use crate::operations::surface::Surface;
use crate::operations::triangulate::Triangulate;
use nalgebra::Isometry3;
use rayon::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Polygon {
    pub gml: Gml,
    pub exterior: LinearRing,
    pub interior: Vec<LinearRing>,
}

impl Polygon {
    pub fn new(gml: Gml, exterior: LinearRing, interior: Vec<LinearRing>) -> Result<Self, Error> {
        Ok(Self {
            gml,
            exterior,
            interior,
        })
    }

    pub fn get_envelope(&self) -> Envelope {
        self.exterior.envelope()
    }
}

impl Geometry for Polygon {
    fn points(&self) -> Vec<&DirectPosition> {
        let mut all_points = Vec::new();
        all_points.extend(self.exterior.points());

        for ring in &self.interior {
            all_points.extend(ring.points().iter());
        }

        all_points
    }

    fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.exterior.apply_transform(m);

        self.interior.par_iter_mut().for_each(|p| {
            p.apply_transform(m);
        });
    }
}

impl Surface for Polygon {
    fn outer_boundary_points(&self) -> Vec<&DirectPosition> {
        self.exterior.outer_boundary_points()
    }
}

impl Triangulate for Polygon {
    fn triangulate(&self) -> Result<TriangulatedSurface, Error> {
        let mut exterior_buf = Vec::new();
        let mut all_direct_positions: Vec<&DirectPosition> = self.exterior.points();
        all_direct_positions.extend(self.interior.iter().flat_map(|x| x.points()));

        let linear_ring_lengths: Vec<usize> = {
            let mut vec = vec![self.exterior.points().len()];
            vec.extend(self.interior.iter().map(|x| x.points().len()));
            vec
        };
        let hole_indices: Vec<usize> = linear_ring_lengths
            .iter()
            .scan(0, |sum, e| {
                *sum += e;
                Some(*sum)
            })
            .take(linear_ring_lengths.len() - 1)
            .collect();

        let vertices = all_direct_positions
            .iter()
            .map(|p| p.coords())
            .collect::<Vec<_>>();
        earcut::utils3d::project3d_to_2d(&vertices, vertices.len(), &mut exterior_buf);

        let mut triangle_indices: Vec<usize> = vec![];
        let mut earcut = earcut::Earcut::new();
        earcut.earcut(
            exterior_buf.iter().copied(),
            &hole_indices,
            &mut triangle_indices,
        );

        let triangles: Vec<Triangle> = triangle_indices
            .chunks(3)
            .map(|x| {
                let vertex_a = all_direct_positions[x[0]];
                let vertex_b = all_direct_positions[x[1]];
                let vertex_c = all_direct_positions[x[2]];
                Triangle::new(*vertex_a, *vertex_b, *vertex_c).expect("should work")
            })
            .collect::<Vec<_>>();

        let triangulated_surface = TriangulatedSurface::new(triangles)?;
        Ok(triangulated_surface)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_polygon_triangulation() {
        let gml = Gml::new("exterior_linear_ring_id".to_string().try_into().unwrap());
        let linear_ring_exterior = LinearRing::new(
            gml,
            vec![
                DirectPosition::new(0.0, 0.0, 0.0).expect("should work"),
                DirectPosition::new(1.0, 0.0, 0.0).expect("should work"),
                DirectPosition::new(1.0, 1.0, 2.0).expect("should work"),
                DirectPosition::new(0.0, 1.0, 2.0).expect("should work"),
            ],
        )
        .expect("should work");

        let gml = Gml::new("polygon_id".to_string().try_into().unwrap());
        let polygon = Polygon::new(gml, linear_ring_exterior, vec![]).expect("should work");
        let triangulated_surface = polygon.triangulate().expect("should work");
        assert_eq!(triangulated_surface.number_of_patches(), 2);
    }

    #[test]
    fn test_polygon_with_interior_triangulation() {
        let gml = Gml::new("exterior_linear_ring_id".to_string().try_into().unwrap());
        let linear_ring_exterior = LinearRing::new(
            gml,
            vec![
                DirectPosition::new(0.0, 0.0, 0.0).expect("should work"),
                DirectPosition::new(1.0, 0.0, 0.0).expect("should work"),
                DirectPosition::new(1.0, 1.0, 2.0).expect("should work"),
                DirectPosition::new(0.0, 1.0, 2.0).expect("should work"),
                DirectPosition::new(0.0, 1.0, 3.0).expect("should work"),
                DirectPosition::new(0.0, 1.0, 5.0).expect("should work"),
            ],
        )
        .expect("should work");

        let gml = Gml::new("interior_linear_ring_id".to_string().try_into().unwrap());
        let linear_ring_interior = LinearRing::new(
            gml,
            vec![
                DirectPosition::new(0.5, 0.0, 0.0).expect("should work"),
                DirectPosition::new(1.0, 0.0, 0.0).expect("should work"),
                DirectPosition::new(1.0, 1.0, 2.0).expect("should work"),
                DirectPosition::new(0.5, 1.0, 2.0).expect("should work"),
            ],
        )
        .expect("should work");

        let gml = Gml::new("polygon_id".to_string().try_into().unwrap());
        let polygon = Polygon::new(
            gml,
            linear_ring_exterior,
            vec![linear_ring_interior.clone(), linear_ring_interior.clone()],
        )
        .expect("should work");
        let triangulated_surface = polygon.triangulate().expect("should work");
        // assert_eq!(triangulated_surface.number_of_patches(), 2);
    }
}
