use crate::error::Error;
use crate::impl_abstract_solid_traits;
use crate::model::geometry::primitives::{
    AbstractSolid, AsAbstractSolid, AsAbstractSolidMut, SurfaceProperty, TriangulatedSurface,
};
use crate::model::geometry::{DirectPosition, Envelope};
use nalgebra::Isometry3;
use rayon::prelude::*;

/// A 3-D geometry bounded by one or more surfaces.
///
/// Corresponds to `gml:Solid` in ISO 19136 §10.6.  The bounding surfaces are
/// stored as [`SurfaceProperty`] members and may be of any [`SurfaceKind`](crate::model::geometry::primitives::SurfaceKind).
#[derive(Debug, Clone, PartialEq)]
pub struct Solid {
    pub(crate) abstract_solid: AbstractSolid,
    members: Vec<SurfaceProperty>,
}

impl Solid {
    /// Creates a new `Solid` from its bounding surfaces.
    ///
    /// # Errors
    ///
    /// Returns [`Error::EmptyCollection`] if `members` is empty.
    pub fn new(
        abstract_solid: AbstractSolid,
        members: Vec<SurfaceProperty>,
    ) -> Result<Self, Error> {
        if members.is_empty() {
            return Err(Error::EmptyCollection("solid"));
        }

        Ok(Self {
            abstract_solid,
            members,
        })
    }

    /// Returns the bounding surface members of this solid.
    pub fn members(&self) -> &[SurfaceProperty] {
        &self.members
    }

    /// Replaces the bounding surface members.
    ///
    /// # Errors
    ///
    /// Returns [`Error::EmptyCollection`] if `val` is empty.
    pub fn set_members(&mut self, val: Vec<SurfaceProperty>) -> Result<(), Error> {
        if val.is_empty() {
            return Err(Error::EmptyCollection("solid"));
        }
        self.members = val;
        Ok(())
    }

    /// Triangulates all bounding surfaces and merges them into a single [`TriangulatedSurface`].
    ///
    /// # Errors
    ///
    /// Returns [`Error::TriangulationFailed`] if any bounding surface cannot be triangulated.
    pub fn triangulate(&self) -> Result<TriangulatedSurface, Error> {
        let triangulated_surfaces: Vec<TriangulatedSurface> = self
            .members
            .iter()
            .map(|x| x.content.triangulate())
            .collect::<Result<Vec<TriangulatedSurface>, Error>>()?;

        let combined_triangulated_surface =
            TriangulatedSurface::from_triangulated_surfaces(triangulated_surfaces)?;
        Ok(combined_triangulated_surface)
    }

    pub fn points(&self) -> Vec<&DirectPosition> {
        self.members.iter().fold(Vec::new(), |mut acc, x| {
            acc.extend(x.points().iter());
            acc
        })
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.members.par_iter_mut().for_each(|p| {
            p.apply_transform(m);
        });
    }

    /// Returns the union of the bounding boxes of all surface members.
    pub fn compute_envelope(&self) -> Envelope {
        let envelopes: Vec<Envelope> = self.members.iter().map(|x| x.compute_envelope()).collect();

        Envelope::from_envelopes(&envelopes).expect("Solid must have at least one surface member")
    }
}

impl AsAbstractSolid for Solid {
    fn abstract_solid(&self) -> &AbstractSolid {
        &self.abstract_solid
    }
}

impl AsAbstractSolidMut for Solid {
    fn abstract_solid_mut(&mut self) -> &mut AbstractSolid {
        &mut self.abstract_solid
    }
}

impl_abstract_solid_traits!(Solid);
