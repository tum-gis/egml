use crate::Error;
use crate::model::geometry::Envelope;
use crate::model::geometry::primitives::surface_patch_kind::SurfacePatchKind;
use nalgebra::Isometry3;

/// An ordered collection of [`SurfacePatchKind`] elements.
///
/// Corresponds to `gml:SurfacePatchArrayPropertyType` in ISO 19136.
/// Used inside [`Surface`](crate::model::geometry::primitives::Surface) to
/// hold its constituent patches.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SurfacePatchArrayProperty {
    pub(crate) patches: Vec<SurfacePatchKind>,
}

impl SurfacePatchArrayProperty {
    /// Creates a new property from a list of surface patches.
    pub fn new(patches: impl IntoIterator<Item = SurfacePatchKind>) -> Self {
        Self {
            patches: patches.into_iter().collect(),
        }
    }

    /// Returns the patches as an immutable slice.
    pub fn patches(&self) -> &[SurfacePatchKind] {
        &self.patches
    }

    /// Returns the patches as a mutable slice.
    pub fn patches_mut(&mut self) -> &mut [SurfacePatchKind] {
        &mut self.patches
    }

    pub fn push_patch(&mut self, patch: SurfacePatchKind) {
        self.patches.push(patch);
    }

    pub fn extend_patches(&mut self, patches: impl IntoIterator<Item = SurfacePatchKind>) {
        self.patches.extend(patches);
    }
}

impl SurfacePatchArrayProperty {
    /// Returns the number of patches.
    pub fn patches_len(&self) -> usize {
        self.patches.len()
    }

    pub fn area_3d(&self) -> Result<f64, Error> {
        self.patches
            .iter()
            .map(|p| p.area_3d())
            .collect::<Result<Vec<f64>, Error>>()
            .map(|area_3ds| area_3ds.into_iter().sum())
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.patches.iter_mut().for_each(|x| x.apply_transform(m));
    }

    /// Returns the union of the bounding boxes of all patches.
    ///
    /// # Panics
    ///
    /// Panics if the patch list is empty.
    pub fn compute_envelope(&self) -> Option<Envelope> {
        let envelopes: Vec<Envelope> = self
            .patches
            .iter()
            .flat_map(|x| x.compute_envelope())
            .collect();

        Envelope::from_envelopes(&envelopes)
    }
}
