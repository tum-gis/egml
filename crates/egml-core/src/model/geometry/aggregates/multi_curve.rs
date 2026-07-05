use crate::model::geometry::Envelope;
use crate::model::geometry::aggregates::{
    AbstractGeometricAggregate, AsAbstractGeometricAggregate, AsAbstractGeometricAggregateMut,
};
use crate::model::geometry::primitives::CurveProperty;
use crate::{Error, impl_abstract_geometric_aggregate_traits};
use nalgebra::Isometry3;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;

/// An unordered collection of [`CurveKind`] members.
///
/// Corresponds to `gml:MultiCurve` in [OGC 07-036 §11.3.3.1](https://docs.ogc.org/is/07-036/07-036.pdf).
#[derive(Debug, Clone, PartialEq)]
pub struct MultiCurve {
    pub(crate) abstract_geometric_aggregate: AbstractGeometricAggregate,
    curve_member: Vec<CurveProperty>,
}

impl MultiCurve {
    /// Creates a new `MultiCurve` from an ordered list of curve members.
    ///
    /// # Errors
    ///
    /// Returns [`Error::TooFewElements`] if `members` is empty.
    pub fn new(members: impl IntoIterator<Item = CurveProperty>) -> Result<Self, Error> {
        let members: Vec<CurveProperty> = members.into_iter().collect();
        if members.is_empty() {
            return Err(Error::TooFewElements {
                geometry: "gml:MultiCurve",
                minimum: 1,
                spec: Some("OGC 07-036 §11.3.3.1"),
                id: None,
                detail: None,
            });
        }

        Ok(Self {
            abstract_geometric_aggregate: AbstractGeometricAggregate::default(),
            curve_member: members,
        })
    }

    /// Returns the curve members as a slice.
    pub fn curve_member(&self) -> &[CurveProperty] {
        &self.curve_member
    }

    /// Replaces the curve members.
    ///
    /// # Errors
    ///
    /// Returns [`Error::TooFewElements`] if `val` is empty.
    pub fn set_curve_member(&mut self, val: Vec<CurveProperty>) -> Result<(), Error> {
        if val.is_empty() {
            return Err(Error::TooFewElements {
                geometry: "gml:MultiCurve",
                minimum: 1,
                spec: Some("OGC 07-036 §11.3.3.1"),
                id: None,
                detail: None,
            });
        }
        self.curve_member = val;
        Ok(())
    }

    pub fn push_curve_member(&mut self, member: CurveProperty) {
        self.curve_member.push(member);
    }

    pub fn extend_curve_members(&mut self, members: impl IntoIterator<Item = CurveProperty>) {
        self.curve_member.extend(members);
    }
}

impl MultiCurve {
    /// Returns the total 3D length of all curve members.
    ///
    /// # Errors
    ///
    /// Returns [`Error::UnresolvedCurveReference`] if any member carries only an
    /// xlink:href that has not been resolved into an inline object.
    pub fn length_3d(&self) -> Result<f64, Error> {
        self.curve_member
            .iter()
            .map(|c| {
                c.object
                    .as_ref()
                    .ok_or_else(|| Error::UnresolvedCurveReference {
                        href: c.href.clone(),
                    })
                    .map(|curve| curve.length_3d())
            })
            .collect::<Result<Vec<f64>, Error>>()
            .map(|lengths| lengths.into_iter().sum())
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.curve_member.par_iter_mut().for_each(|p| {
            if let Some(object) = &mut p.object {
                object.apply_transform(m);
            }
        });
    }

    /// Returns the union of the bounding boxes of all curve members.
    pub fn compute_envelope(&self) -> Option<Envelope> {
        let envelopes: Vec<Envelope> = self
            .curve_member
            .iter()
            .flat_map(|x| x.object.as_ref())
            .map(|x| x.compute_envelope())
            .collect();

        Envelope::from_envelopes(&envelopes)
    }
}

impl AsAbstractGeometricAggregate for MultiCurve {
    fn abstract_geometric_aggregate(&self) -> &AbstractGeometricAggregate {
        &self.abstract_geometric_aggregate
    }
}

impl AsAbstractGeometricAggregateMut for MultiCurve {
    fn abstract_geometric_aggregate_mut(&mut self) -> &mut AbstractGeometricAggregate {
        &mut self.abstract_geometric_aggregate
    }
}

impl_abstract_geometric_aggregate_traits!(MultiCurve);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;
    use crate::model::geometry::DirectPosition;
    use crate::model::geometry::primitives::{CurveKind, LineString};

    fn line_string(points: Vec<DirectPosition>) -> CurveProperty {
        CurveProperty::new(CurveKind::LineString(LineString::new(points).unwrap()))
    }

    #[test]
    fn length_3d_two_segments() {
        // Two unit segments along X — total length 2.
        let multi_curve = MultiCurve::new([
            line_string(vec![
                DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
                DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
            ]),
            line_string(vec![
                DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
                DirectPosition::new(2.0, 0.0, 0.0).unwrap(),
            ]),
        ])
        .unwrap();
        assert!((multi_curve.length_3d().unwrap() - 2.0).abs() < 1e-10);
    }

    #[test]
    fn length_3d_unresolved_curve_reference() {
        let multi_curve =
            MultiCurve::new([CurveProperty::new_href("urn:example:curve-1")]).unwrap();
        assert_eq!(
            multi_curve.length_3d(),
            Err(Error::UnresolvedCurveReference {
                href: Some("urn:example:curve-1".to_string())
            })
        );
    }
}
