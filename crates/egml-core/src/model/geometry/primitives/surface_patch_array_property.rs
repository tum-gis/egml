use crate::model::geometry::Envelope;
use crate::model::geometry::primitives::SurfacePatchKind;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SurfacePatchArrayProperty {
    pub(crate) patches: Vec<SurfacePatchKind>,
}

impl SurfacePatchArrayProperty {
    pub fn new(patches: Vec<SurfacePatchKind>) -> Self {
        Self { patches }
    }

    pub fn patches(&self) -> &Vec<SurfacePatchKind> {
        &self.patches
    }

    pub fn patches_mut(&mut self) -> &mut Vec<SurfacePatchKind> {
        &mut self.patches
    }

    pub fn patches_len(&self) -> usize {
        self.patches.len()
    }

    pub fn compute_envelope(&self) -> Envelope {
        let envelopes: Vec<Envelope> = self
            .patches
            .iter()
            .map(|x| x.compute_envelope())
            .collect::<Vec<_>>();

        Envelope::from_envelopes(&envelopes.iter().collect::<Vec<_>>())
            .expect("MultiSurface must have at least one surface member")
    }
}
