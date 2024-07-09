use crate::Error;
use egml_core::{
    DirectPosition, LinearRing, MultiSurface, Polygon, Solid, Triangle, TriangulatedSurface,
};
use itertools::Itertools;

pub fn triangulate_polygon(polygon: &Polygon) -> Result<TriangulatedSurface, Error> {
    let mut triangulated_surface = triangulate_linear_ring(polygon.exterior())?;

    // TODO: no cloning
    let mut interior_triangulated_surface: Vec<TriangulatedSurface> = polygon
        .interior()
        .iter()
        .map(triangulate_linear_ring)
        .collect::<Result<Vec<_>, _>>()?;
    interior_triangulated_surface
        .iter_mut()
        .for_each(|x| triangulated_surface.append_patches(x.patches().clone()));

    Ok(triangulated_surface)
}

pub fn triangulate_linear_ring(linear_ring: &LinearRing) -> Result<TriangulatedSurface, Error> {
    //let points: Vec<DirectPosition> = linear_ring.points().iter().collect();

    let first_point = *linear_ring.points().first().unwrap();
    let mut triangles: Vec<Triangle> = Vec::new();

    for (prev, next) in linear_ring
        .points()
        .iter()
        .skip(1)
        .collect::<Vec<&DirectPosition>>()
        .iter()
        .tuple_windows()
    {
        let new_triangle = Triangle::new(first_point, **prev, **next)?;
        triangles.push(new_triangle);
    }

    let triangulated_surface = TriangulatedSurface::new(triangles)?;
    Ok(triangulated_surface)
}

pub fn triangulate_multi_surface(
    multi_surface: &MultiSurface,
) -> Result<Vec<TriangulatedSurface>, Error> {
    let triangulated_surfaces: Vec<TriangulatedSurface> = multi_surface
        .members()
        .iter()
        .map(triangulate_polygon)
        .collect::<Result<Vec<TriangulatedSurface>, Error>>()?;

    Ok(triangulated_surfaces)
}

pub fn triangulate_solid(solid: &Solid) -> Result<Vec<TriangulatedSurface>, Error> {
    let triangulated_surfaces: Vec<TriangulatedSurface> = solid
        .members()
        .iter()
        .map(triangulate_linear_ring)
        .collect::<Result<Vec<TriangulatedSurface>, Error>>()?;

    Ok(triangulated_surfaces)
}
