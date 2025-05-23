use crate::Error;
use crate::model::base::Gml;
use crate::model::geometry::{DirectPosition, Envelope, LinearRing};
use crate::operations::geometry::Geometry;
use crate::operations::surface::Surface;
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
