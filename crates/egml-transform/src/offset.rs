use egml_core::{DirectPosition, LinearRing, MultiSurface, Solid};
use egml_core::{Error, Polygon};
use nalgebra::Vector3;

pub fn offset_position(
    mut position: DirectPosition,
    offset: &Vector3<f64>,
) -> Result<DirectPosition, Error> {
    position.set_x(position.x() + offset.x)?;
    position.set_y(position.y() + offset.y)?;
    position.set_z(position.z() + offset.z)?;
    Ok(position)
}

pub fn offset_linear_ring(
    mut linear_ring: LinearRing,
    offset: &Vector3<f64>,
) -> Result<LinearRing, Error> {
    let points: Vec<DirectPosition> = linear_ring
        .points()
        .iter()
        .map(|p| offset_position(p.to_owned(), offset))
        .collect::<Result<Vec<_>, Error>>()?;

    linear_ring.set_points(points)?;
    Ok(linear_ring)
}

pub fn offset_polygon(mut polygon: Polygon, offset: &Vector3<f64>) -> Result<Polygon, Error> {
    polygon.set_exterior(offset_linear_ring(polygon.exterior().clone(), offset)?);

    let interior = polygon
        .interior()
        .iter()
        .map(|x| offset_linear_ring(x.to_owned(), offset))
        .collect::<Result<Vec<_>, Error>>()?;
    polygon.set_interior(interior);

    Ok(polygon)
}

pub fn offset_multi_surface(
    mut multi_surface: MultiSurface,
    offset: &Vector3<f64>,
) -> Result<MultiSurface, Error> {
    let polygons: Vec<Polygon> = multi_surface
        .members()
        .iter()
        .map(|p| offset_polygon(p.to_owned(), offset))
        .collect::<Result<Vec<_>, Error>>()?;

    multi_surface.set_members(polygons)?;
    Ok(multi_surface)
}

pub fn offset_solid(mut solid: Solid, offset: &Vector3<f64>) -> Result<Solid, Error> {
    let linear_rings: Vec<LinearRing> = solid
        .members()
        .iter()
        .map(|p| offset_linear_ring(p.to_owned(), offset))
        .collect::<Result<Vec<_>, Error>>()?;

    solid.set_members(linear_rings)?;
    Ok(solid)
}
