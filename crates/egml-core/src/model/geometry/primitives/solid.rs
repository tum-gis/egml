use crate::error::Error;
use crate::impl_abstract_solid_traits;
use crate::model::geometry::primitives::shell_property::ShellProperty;
use crate::model::geometry::primitives::{AbstractSolid, AsAbstractSolid, AsAbstractSolidMut};
use crate::model::geometry::{DirectPosition, Envelope};
use nalgebra::Isometry3;

/// A 3-D geometry bounded by one or more surfaces.
///
/// Corresponds to `gml:Solid` in [OGC 07-036 §10.6.4](https://docs.ogc.org/is/07-036/07-036.pdf).  The bounding surfaces are
/// stored as [`SurfaceProperty`] members and may be of any [`SurfaceKind`](crate::model::geometry::primitives::surface_kind::SurfaceKind).
#[derive(Debug, Clone, PartialEq)]
pub struct Solid {
    pub(crate) abstract_solid: AbstractSolid,
    exterior: Option<ShellProperty>,
}

impl Solid {
    /// Creates a new `Solid` from its bounding surfaces.
    ///
    /// # Errors
    ///
    /// Returns [`Error::TooFewElements`] if `members` is empty.
    pub fn new(exterior: Option<ShellProperty>) -> Result<Self, Error> {
        Ok(Self {
            abstract_solid: AbstractSolid::default(),
            exterior,
        })
    }

    pub fn exterior(&self) -> Option<&ShellProperty> {
        self.exterior.as_ref()
    }
}

impl Solid {
    pub fn points(&self) -> Vec<&DirectPosition> {
        if let Some(exterior) = &self.exterior
            && let Some(object) = &exterior.object
        {
            object.points()
        } else {
            Vec::new()
        }
    }

    /// Returns the volume of this solid.
    ///
    /// # Errors
    ///
    /// Returns [`Error::MissingExteriorShell`] if the solid has no exterior shell property.
    /// Returns [`Error::UnresolvedShellReference`] if the shell property carries only an
    /// xlink:href that has not been resolved into an inline object.
    /// Propagates any error from triangulating the bounding surfaces.
    pub fn volume_3d(&self) -> Result<f64, Error> {
        let shell_property = self.exterior.as_ref().ok_or(Error::MissingExteriorShell)?;
        let shell =
            shell_property
                .object
                .as_ref()
                .ok_or_else(|| Error::UnresolvedShellReference {
                    href: shell_property.href.clone(),
                })?;
        shell.volume_3d()
    }

    pub fn apply_transform(&mut self, m: &Isometry3<f64>) {
        if let Some(exterior) = self.exterior.as_mut()
            && let Some(object) = exterior.object.as_mut()
        {
            object.apply_transform(m)
        }
    }

    /// Returns the union of the bounding boxes of all surface members.
    pub fn compute_envelope(&self) -> Option<Envelope> {
        if let Some(exterior) = &self.exterior
            && let Some(object) = &exterior.object
        {
            object.compute_envelope()
        } else {
            None
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::geometry::Envelope;
    use crate::model::geometry::primitives::ShellProperty;

    #[test]
    fn volume_3d_unit_cube() {
        let solid = Envelope::new(
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 1.0, 1.0).unwrap(),
        )
        .unwrap()
        .to_solid()
        .unwrap();
        assert!((solid.volume_3d().unwrap() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn volume_3d_2x3x4_box() {
        let solid = Envelope::new(
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(2.0, 3.0, 4.0).unwrap(),
        )
        .unwrap()
        .to_solid()
        .unwrap();
        assert!((solid.volume_3d().unwrap() - 24.0).abs() < 1e-10);
    }

    #[test]
    fn volume_3d_missing_exterior_shell() {
        let solid = Solid::new(None).unwrap();
        assert_eq!(solid.volume_3d(), Err(Error::MissingExteriorShell));
    }

    #[test]
    fn volume_3d_unresolved_shell_reference() {
        let solid = Solid::new(Some(ShellProperty::new_href(
            "urn:example:shell-1".to_string(),
        )))
        .unwrap();
        assert_eq!(
            solid.volume_3d(),
            Err(Error::UnresolvedShellReference {
                href: Some("urn:example:shell-1".to_string())
            })
        );
    }
}
