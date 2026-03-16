use crate::model::geometry::Envelope;
use crate::model::geometry::primitives::SurfacePatchKind;
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
    pub fn new(patches: Vec<SurfacePatchKind>) -> Self {
        Self { patches }
    }

    /// Returns the patches as an immutable slice.
    pub fn patches(&self) -> &[SurfacePatchKind] {
        &self.patches
    }

    /// Returns the patches as a mutable slice.
    pub fn patches_mut(&mut self) -> &mut [SurfacePatchKind] {
        &mut self.patches
    }

    /// Returns the number of patches.
    pub fn patches_len(&self) -> usize {
        self.patches.len()
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        self.patches.iter_mut().for_each(|x| x.apply_transform(m));
    }

    /// Returns the union of the bounding boxes of all patches.
    ///
    /// # Panics
    ///
    /// Panics if the patch list is empty.
    pub fn compute_envelope(&self) -> Envelope {
        let envelopes: Vec<Envelope> = self.patches.iter().map(|x| x.compute_envelope()).collect();

        Envelope::from_envelopes(&envelopes).expect("SurfacePatchArrayProperty must not be empty")
    }
}
