use egml_core::error::Error;
use egml_core::{DirectPosition, LinearRing, MultiSurface};
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
        // TODO: handle gracefully
        .map(|p| offset_position(p.to_owned(), offset).unwrap())
        .collect();

    linear_ring.set_points(points)?;
    Ok(linear_ring)
}

pub fn offset_multi_surface(
    mut multi_surface: MultiSurface,
    offset: &Vector3<f64>,
) -> Result<MultiSurface, Error> {
    let linear_rings: Vec<LinearRing> = multi_surface
        .members()
        .iter()
        // TODO: handle gracefully
        .map(|p| offset_linear_ring(p.to_owned(), offset).unwrap())
        .collect();

    multi_surface.set_members(linear_rings)?;
    Ok(multi_surface)
}
