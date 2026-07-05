use crate::impl_abstract_curve_traits;
use crate::model::geometry::Envelope;
use crate::model::geometry::primitives::{
    AbstractCurve, AsAbstractCurve, AsAbstractCurveMut, LineString,
};
use nalgebra::Isometry3;

/// Discriminated union of all concrete curve implementations.
#[derive(Debug, Clone, PartialEq)]
pub enum CurveKind {
    /// A [`LineString`] curve.
    LineString(LineString),
}

impl AsAbstractCurve for CurveKind {
    fn abstract_curve(&self) -> &AbstractCurve {
        match self {
            Self::LineString(x) => x.abstract_curve(),
        }
    }
}

impl AsAbstractCurveMut for CurveKind {
    fn abstract_curve_mut(&mut self) -> &mut AbstractCurve {
        match self {
            Self::LineString(x) => x.abstract_curve_mut(),
        }
    }
}

impl_abstract_curve_traits!(CurveKind);

impl CurveKind {
    pub fn compute_envelope(&self) -> Envelope {
        match self {
            Self::LineString(x) => x.compute_envelope(),
        }
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        match self {
            Self::LineString(x) => x.apply_transform(m),
        }
    }

    pub fn length_3d(&self) -> f64 {
        match self {
            Self::LineString(x) => x.length_3d(),
        }
    }
}
