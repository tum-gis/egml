use crate::model::geometry::{DirectPosition, Envelope};
use nalgebra::Isometry3;

pub trait Geometry {
    fn points(&self) -> Vec<&DirectPosition>;

    fn apply_transform(&mut self, m: &Isometry3<f64>);

    fn get_lower_corner(&self) -> DirectPosition {
        let x_min = self
            .points()
            .iter()
            .min_by(|a, b| a.x().partial_cmp(&b.x()).unwrap())
            .unwrap()
            .x();
        let y_min = self
            .points()
            .iter()
            .min_by(|a, b| a.y().partial_cmp(&b.y()).unwrap())
            .unwrap()
            .y();
        let z_min = self
            .points()
            .iter()
            .min_by(|a, b| a.z().partial_cmp(&b.z()).unwrap())
            .unwrap()
            .z();

        DirectPosition::new(x_min, y_min, z_min).unwrap()
    }

    fn get_upper_corner(&self) -> DirectPosition {
        let x_max = self
            .points()
            .iter()
            .max_by(|a, b| a.x().partial_cmp(&b.x()).unwrap())
            .unwrap()
            .x();
        let y_max = self
            .points()
            .iter()
            .max_by(|a, b| a.y().partial_cmp(&b.y()).unwrap())
            .unwrap()
            .y();
        let z_max = self
            .points()
            .iter()
            .max_by(|a, b| a.z().partial_cmp(&b.z()).unwrap())
            .unwrap()
            .z();

        DirectPosition::new(x_max, y_max, z_max).unwrap()
    }

    fn envelope(&self) -> Envelope {
        let lower = self.get_lower_corner();
        let upper = self.get_upper_corner();

        Envelope::new(lower, upper).expect("Must be constructable with a valid geometry.")
    }
}
