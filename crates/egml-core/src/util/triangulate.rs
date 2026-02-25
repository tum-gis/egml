use crate::Error;
use crate::model::geometry::DirectPosition;
use crate::model::geometry::primitives::{
    LinearRing, RingPropertyKind, Triangle, TriangulatedSurface,
};

pub fn triangulate(
    exterior: Option<RingPropertyKind>,
    interior: Vec<RingPropertyKind>,
) -> Result<TriangulatedSurface, Error> {
    let exterior = match exterior {
        Some(ring) => ring,
        None => {
            todo!("triangulate polygon with no exterior ring needs to be implemented")
        }
    };
    let exterior = match exterior {
        RingPropertyKind::LinearRing(x) => x,
        _ => todo!("triangulate polygon with non-linear exterior ring needs to be implemented"),
    };

    let interior = interior
        .iter()
        .map(|x| match x {
            RingPropertyKind::LinearRing(x) => x.clone(),
            _ => todo!("needs to be implemented"),
        })
        .collect::<Vec<_>>();

    if interior.is_empty() {
        triangulate_without_holes(exterior)
    } else {
        triangulate_with_holes(exterior, interior)
    }
}

fn triangulate_without_holes(exterior: LinearRing) -> Result<TriangulatedSurface, Error> {
    if exterior.points().len() == 3 {
        let triangle = Triangle::new_unchecked(
            exterior.points()[0],
            exterior.points()[1],
            exterior.points()[2],
        )?;
        return TriangulatedSurface::from_triangles(vec![triangle]);
    }

    let vertices_3d = exterior
        .points()
        .iter()
        .map(|p| p.coords())
        .collect::<Vec<_>>();
    let mut vertices_2d_buf = Vec::new();
    earcut::utils3d::project3d_to_2d(&vertices_3d, vertices_3d.len(), &mut vertices_2d_buf);

    let mut triangle_indices: Vec<usize> = vec![];
    let mut earcut = earcut::Earcut::new();
    earcut.earcut(vertices_2d_buf.iter().copied(), &[], &mut triangle_indices);
    if triangle_indices.is_empty() {
        return Err(Error::TriangulationError("earcut failed to triangulate"));
    }

    let triangles: Vec<Triangle> = triangle_indices
        .chunks_exact(3)
        .map(|x| {
            let vertex_a = exterior.points()[x[0]];
            let vertex_b = exterior.points()[x[1]];
            let vertex_c = exterior.points()[x[2]];
            Triangle::new(vertex_a, vertex_b, vertex_c).expect("should work")
        })
        .collect::<Vec<_>>();

    let triangulated_surface = TriangulatedSurface::from_triangles(triangles)?;
    Ok(triangulated_surface)
}

fn triangulate_with_holes(
    exterior: LinearRing,
    interior: Vec<LinearRing>,
) -> Result<TriangulatedSurface, Error> {
    let mut exterior_buf = Vec::new();
    let mut all_direct_positions: Vec<DirectPosition> = exterior.points().clone();
    all_direct_positions.extend(interior.iter().flat_map(|x| x.points()));

    let linear_ring_lengths: Vec<usize> = {
        let mut vec = vec![exterior.points().len()];
        vec.extend(interior.iter().map(|x| x.points().len()));
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
        .chunks_exact(3)
        .map(|x| {
            let vertex_a = all_direct_positions[x[0]];
            let vertex_b = all_direct_positions[x[1]];
            let vertex_c = all_direct_positions[x[2]];
            Triangle::new(vertex_a, vertex_b, vertex_c).expect("should work")
        })
        .collect::<Vec<_>>();

    let triangulated_surface = TriangulatedSurface::from_triangles(triangles)?;
    Ok(triangulated_surface)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::model::base::{AbstractGml, Id};
    use crate::model::geometry::primitives::{AbstractRing, AsAbstractSurfacePatch, AsSurface};
    use nalgebra::{Isometry3, Vector3};

    #[test]
    fn triangulate_test() {
        let linear_ring = LinearRing::new(
            AbstractRing::default(),
            vec![
                DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
                DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
                DirectPosition::new(1.0, 1.0, 0.0).unwrap(),
                DirectPosition::new(0.0, 1.0, 0.0).unwrap(),
            ],
        )
        .unwrap();

        let result = triangulate_without_holes(linear_ring).unwrap();

        assert_eq!(result.patches_len(), 2);
        assert!(result.patches().patches()[0].area() > 0.0);
        assert!(result.patches().patches()[1].area() > 0.0);
    }

    #[test]
    fn linear_ring_test() {
        let linear_ring = LinearRing::new(
            AbstractRing::default(),
            vec![
                DirectPosition::new(478.88403143223741, 1137.6732953797839, 3.813234192323872)
                    .unwrap(),
                DirectPosition::new(478.88403145332472, 1137.6732953253052, 3.8132341922655204)
                    .unwrap(),
                DirectPosition::new(478.88403144458238, 1137.6732953478909, 3.8132341922897117)
                    .unwrap(),
            ],
        )
        .unwrap();

        let result = triangulate_without_holes(linear_ring).unwrap();

        assert_eq!(result.patches_len(), 1);
    }
}
