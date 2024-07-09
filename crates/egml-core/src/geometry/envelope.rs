use crate::error::Error;
use crate::error::Error::LowerCornerMustBeBelowUpperCorner;
use crate::DirectPosition;
use nalgebra::{Point3, Vector3};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Envelope {
    lower_corner: DirectPosition,
    upper_corner: DirectPosition,
}

impl Envelope {
    pub fn new(lower_corner: DirectPosition, upper_corner: DirectPosition) -> Result<Self, Error> {
        let lower_corner_point: Point3<f64> = lower_corner.into();
        let upper_corner_point: Point3<f64> = upper_corner.into();
        if lower_corner_point >= upper_corner_point {
            return Err(LowerCornerMustBeBelowUpperCorner(""));
        }

        Ok(Self {
            lower_corner,
            upper_corner,
        })
    }

    pub fn lower_corner(&self) -> &DirectPosition {
        &self.lower_corner
    }

    pub fn upper_corner(&self) -> &DirectPosition {
        &self.upper_corner
    }

    pub fn size(&self) -> Vector3<f64> {
        let lower_corner_point: Point3<f64> = self.lower_corner.into();
        let upper_corner_point: Point3<f64> = self.upper_corner.into();
        upper_corner_point - lower_corner_point
    }

    pub fn contains(&self, point: &DirectPosition) -> bool {
        let lower_corner: Point3<f64> = self.lower_corner.into();
        let upper_corner: Point3<f64> = self.upper_corner.into();
        let point: Point3<f64> = (*point).into();

        lower_corner <= point && point <= upper_corner
    }

    /// Returns `true` if envelope is fully contained.
    pub fn contains_envelope(&self, envelope: &Envelope) -> bool {
        self.contains(&envelope.lower_corner) && self.contains(&envelope.upper_corner)
    }

    /// Returns `true` if envelope is fully contained.
    pub fn contains_envelope_partially(&self, envelope: &Envelope) -> bool {
        self.contains(&envelope.lower_corner) || self.contains(&envelope.upper_corner)
    }

    pub fn enlarge(&self, distance: f64) -> Result<Envelope, Error> {
        let lower_corner = DirectPosition::new(
            self.lower_corner.x() - distance,
            self.lower_corner.y() - distance,
            self.lower_corner.z() - distance,
        )?;
        let upper_corner = DirectPosition::new(
            self.upper_corner.x() + distance,
            self.upper_corner.y() + distance,
            self.upper_corner.z() + distance,
        )?;

        let envelope = Envelope::new(lower_corner, upper_corner)?;
        Ok(envelope)
    }
}

pub fn enlarge_envelopes(envelopes: &Vec<Envelope>) -> Result<Envelope, Error> {
    if envelopes.is_empty() {
        return Err(Error::MustNotBeEmpty("envelopes"));
    }

    let x_min: f64 = envelopes
        .iter()
        .map(|e| e.lower_corner)
        .min_by(|a, b| a.x().partial_cmp(&b.x()).unwrap())
        .unwrap()
        .x();
    let y_min = envelopes
        .iter()
        .map(|e| e.lower_corner)
        .min_by(|a, b| a.y().partial_cmp(&b.y()).unwrap())
        .unwrap()
        .y();
    let z_min = envelopes
        .iter()
        .map(|e| e.lower_corner)
        .min_by(|a, b| a.z().partial_cmp(&b.z()).unwrap())
        .unwrap()
        .z();
    let lower_corner = DirectPosition::new(x_min, y_min, z_min).unwrap();

    let x_max: f64 = envelopes
        .iter()
        .map(|e| e.upper_corner)
        .max_by(|a, b| a.x().partial_cmp(&b.x()).unwrap())
        .unwrap()
        .x();
    let y_max = envelopes
        .iter()
        .map(|e| e.upper_corner)
        .max_by(|a, b| a.y().partial_cmp(&b.y()).unwrap())
        .unwrap()
        .y();
    let z_max = envelopes
        .iter()
        .map(|e| e.upper_corner)
        .max_by(|a, b| a.z().partial_cmp(&b.z()).unwrap())
        .unwrap()
        .z();
    let upper_corner = DirectPosition::new(x_max, y_max, z_max).unwrap();

    let envelope = Envelope::new(lower_corner, upper_corner)?;
    Ok(envelope)
}
