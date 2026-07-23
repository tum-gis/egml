use crate::error::Error;
use crate::model::AbstractObject;
use crate::model::common::{ApplyTransform, Triangulate, Triangulation};
use crate::model::geometry::DirectPosition;
use crate::model::geometry::primitives::{AbstractRingKind, Shell};
use crate::model::geometry::primitives::{
    AbstractRingProperty, AbstractSurfaceProperty, LinearRing, Polygon, Solid, TriangulatedSurface,
};
use crate::model::geometry::primitives::{AbstractSurfaceKind, ShellProperty};
use nalgebra::{Isometry3, Point3, Rotation3, Scale3, Transform3, Vector3};
use std::fmt;

/// Axis-aligned bounding box in 3-D space.
///
/// An `Envelope` is defined by a lower corner and an upper corner such that
/// each coordinate component of the lower corner is ≤ the corresponding
/// component of the upper corner.
///
/// Corresponds to `gml:EnvelopeType` in ISO 19136.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Envelope {
    abstract_object: AbstractObject,
    lower_corner: DirectPosition,
    upper_corner: DirectPosition,
    srs_name: Option<String>,
    srs_dimension: Option<u8>,
}

impl Envelope {
    /// Creates an envelope from explicit lower and upper corners.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidEnvelopeBounds`] if any coordinate
    /// component of `lower_corner` is strictly greater than the corresponding
    /// component of `upper_corner`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use egml_core::model::geometry::{DirectPosition, Envelope};
    ///
    /// let lo = DirectPosition::new(0.0, 0.0, 0.0).unwrap();
    /// let hi = DirectPosition::new(1.0, 2.0, 3.0).unwrap();
    /// let env = Envelope::new(lo, hi).unwrap();
    /// assert_eq!(env.size_x(), 1.0);
    /// ```
    pub fn new(lower_corner: DirectPosition, upper_corner: DirectPosition) -> Result<Self, Error> {
        if lower_corner.x() > upper_corner.x() {
            return Err(Error::InvalidEnvelopeBounds {
                axis: "x",
                lower: lower_corner.x(),
                upper: upper_corner.x(),
            });
        }
        if lower_corner.y() > upper_corner.y() {
            return Err(Error::InvalidEnvelopeBounds {
                axis: "y",
                lower: lower_corner.y(),
                upper: upper_corner.y(),
            });
        }
        if lower_corner.z() > upper_corner.z() {
            return Err(Error::InvalidEnvelopeBounds {
                axis: "z",
                lower: lower_corner.z(),
                upper: upper_corner.z(),
            });
        }

        Ok(Self {
            abstract_object: AbstractObject::default(),
            lower_corner,
            upper_corner,
            srs_name: None,
            srs_dimension: None,
        })
    }

    /// Creates an `Envelope` without validating that `lower_corner <= upper_corner`.
    ///
    /// # Safety (logical)
    /// The caller must ensure that each component of `lower_corner` is less than
    /// or equal to the corresponding component of `upper_corner`. Violating this
    /// will not cause undefined behavior, but will break the type's invariants
    /// and produce incorrect results from methods like `contains`, `size`, etc.
    pub fn new_unchecked(lower_corner: DirectPosition, upper_corner: DirectPosition) -> Self {
        debug_assert!(
            {
                let lc: Point3<f64> = lower_corner.into();
                let uc: Point3<f64> = upper_corner.into();
                lc <= uc
            },
            "lower_corner must be <= upper_corner"
        );

        Self {
            abstract_object: AbstractObject::default(),
            lower_corner,
            upper_corner,
            srs_name: None,
            srs_dimension: None,
        }
    }

    /// Returns the lower (minimum) corner.
    pub fn lower_corner(&self) -> &DirectPosition {
        &self.lower_corner
    }

    /// Returns the upper (maximum) corner.
    pub fn upper_corner(&self) -> &DirectPosition {
        &self.upper_corner
    }

    pub fn set_lower_corner(&mut self, lower_corner: DirectPosition) {
        self.lower_corner = lower_corner;
    }

    pub fn set_upper_corner(&mut self, upper_corner: DirectPosition) {
        self.upper_corner = upper_corner;
    }

    /// Returns the SRS name identifying the CRS of this envelope's coordinates,
    /// or `None` if unspecified.
    pub fn srs_name(&self) -> Option<&str> {
        self.srs_name.as_deref()
    }

    /// Returns the coordinate dimension of this envelope's positions,
    /// or `None` if unspecified.
    pub fn srs_dimension(&self) -> Option<u8> {
        self.srs_dimension
    }

    /// Sets the SRS name identifying the CRS (e.g. `"urn:ogc:def:crs:EPSG::25832"`).
    pub fn set_srs_name(&mut self, srs_name: impl Into<String>) {
        self.srs_name = Some(srs_name.into());
    }

    /// Sets or clears the SRS name.
    pub fn set_srs_name_opt(&mut self, srs_name: Option<String>) {
        self.srs_name = srs_name;
    }

    /// Clears the SRS name.
    pub fn clear_srs_name(&mut self) {
        self.srs_name = None;
    }

    /// Sets the coordinate dimension of this envelope's positions (typically `2` or `3`).
    pub fn set_srs_dimension(&mut self, srs_dimension: u8) {
        self.srs_dimension = Some(srs_dimension);
    }

    /// Sets or clears the coordinate dimension.
    pub fn set_srs_dimension_opt(&mut self, srs_dimension: Option<u8>) {
        self.srs_dimension = srs_dimension;
    }

    /// Clears the coordinate dimension.
    pub fn clear_srs_dimension(&mut self) {
        self.srs_dimension = None;
    }

    /// Returns the diagonal vector from the lower corner to the upper corner.
    pub fn size(&self) -> Vector3<f64> {
        let lower_corner_point: Point3<f64> = self.lower_corner.into();
        let upper_corner_point: Point3<f64> = self.upper_corner.into();
        upper_corner_point - lower_corner_point
    }

    /// Returns the extent along the X axis (`upper.x - lower.x`).
    pub fn size_x(&self) -> f64 {
        self.upper_corner.x() - self.lower_corner.x()
    }

    /// Returns the extent along the Y axis (`upper.y - lower.y`).
    pub fn size_y(&self) -> f64 {
        self.upper_corner.y() - self.lower_corner.y()
    }

    /// Returns the extent along the Z axis (`upper.z - lower.z`).
    pub fn size_z(&self) -> f64 {
        self.upper_corner.z() - self.lower_corner.z()
    }

    /// Returns the volume of the box (`size_x * size_y * size_z`).
    ///
    /// Returns `0.0` for degenerate envelopes where one or more extents are zero.
    pub fn volume(&self) -> f64 {
        self.size_x() * self.size_y() * self.size_z()
    }

    /// Returns `true` if the lower and upper corners are equal, i.e. the envelope collapses to a point.
    pub fn is_point(&self) -> bool {
        self.lower_corner == self.upper_corner
    }

    /// Returns `true` if exactly one axis has non-zero extent (a line segment).
    #[allow(clippy::nonminimal_bool)]
    pub fn is_linear(&self) -> bool {
        let nx = self.size_x() > 0.0;
        let ny = self.size_y() > 0.0;
        let nz = self.size_z() > 0.0;
        (nx && !ny && !nz) || (!nx && ny && !nz) || (!nx && !ny && nz)
    }

    /// Returns `true` if exactly two axes have non-zero extent (a flat rectangle).
    #[allow(clippy::nonminimal_bool)]
    pub fn is_surface(&self) -> bool {
        let nx = self.size_x() > 0.0;
        let ny = self.size_y() > 0.0;
        let nz = self.size_z() > 0.0;
        (nx && ny && !nz) || (nx && !ny && nz) || (!nx && ny && nz)
    }

    /// Returns `true` if all three axes have non-zero extent.
    pub fn is_volume(&self) -> bool {
        self.size_x() > 0.0 && self.size_y() > 0.0 && self.size_z() > 0.0
    }

    fn non_zero_extents(&self) -> u8 {
        [self.size_x(), self.size_y(), self.size_z()]
            .iter()
            .filter(|&&s| s > 0.0)
            .count() as u8
    }

    /// Returns the center point of the envelope.
    ///
    /// Computed as `lower + size / 2` to avoid overflow with large coordinates.
    pub fn center(&self) -> DirectPosition {
        DirectPosition::new(
            self.lower_corner.x() + self.size_x() / 2.0,
            self.lower_corner.y() + self.size_y() / 2.0,
            self.lower_corner.z() + self.size_z() / 2.0,
        )
        .expect("envelope corners are finite")
    }

    /// Returns `true` if `point` lies inside or on the boundary of this envelope.
    pub fn contains(&self, point: &DirectPosition) -> bool {
        let lower_corner: Point3<f64> = self.lower_corner.into();
        let upper_corner: Point3<f64> = self.upper_corner.into();
        let point: Point3<f64> = (*point).into();

        lower_corner <= point && point <= upper_corner
    }

    /// Returns `true` if `envelope` is fully contained within (or touches the boundary of) `self`.
    pub fn contains_envelope(&self, envelope: &Envelope) -> bool {
        self.contains(&envelope.lower_corner) && self.contains(&envelope.upper_corner)
    }

    /// Returns `true` if `self` and `envelope` overlap at all (including touching).
    ///
    /// Uses the standard per-axis AABB overlap test rather than checking corners:
    /// two boxes intersect iff they overlap on every axis independently. A
    /// corner-only check would miss cases like a thin slab passing straight
    /// through the other box, where neither box's own min/max corner lies
    /// inside the other.
    pub fn contains_envelope_partially(&self, envelope: &Envelope) -> bool {
        self.lower_corner.x() <= envelope.upper_corner.x()
            && self.upper_corner.x() >= envelope.lower_corner.x()
            && self.lower_corner.y() <= envelope.upper_corner.y()
            && self.upper_corner.y() >= envelope.lower_corner.y()
            && self.lower_corner.z() <= envelope.upper_corner.z()
            && self.upper_corner.z() >= envelope.lower_corner.z()
    }

    /// Returns a new envelope expanded by `distance` in every direction.
    ///
    /// Each lower-corner coordinate is decreased by `distance` and each
    /// upper-corner coordinate is increased by `distance`.
    ///
    /// # Errors
    ///
    /// Returns [`Error::NonFiniteCoordinate`] if `distance` is NaN or infinite,
    /// or if the resulting coordinates would overflow `f64::MAX`.
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

        Envelope::new(lower_corner, upper_corner)
    }
}

impl Envelope {
    /// Computes the union of a slice of envelopes.
    ///
    /// Returns `None` if `envelopes` is empty; otherwise returns the smallest
    /// envelope that contains all envelopes in the slice.
    pub fn from_envelopes(envelopes: &[Self]) -> Option<Self> {
        let first = envelopes.first()?;

        let (lower, upper) = envelopes.iter().skip(1).fold(
            (first.lower_corner, first.upper_corner),
            |(lo, hi), e| {
                let new_lo = DirectPosition::new(
                    lo.x().min(e.lower_corner.x()),
                    lo.y().min(e.lower_corner.y()),
                    lo.z().min(e.lower_corner.z()),
                )
                .unwrap();
                let new_hi = DirectPosition::new(
                    hi.x().max(e.upper_corner.x()),
                    hi.y().max(e.upper_corner.y()),
                    hi.z().max(e.upper_corner.z()),
                )
                .unwrap();
                (new_lo, new_hi)
            },
        );

        Some(Envelope::new_unchecked(lower, upper))
    }

    /// Computes the smallest envelope that contains all `points`.
    ///
    /// # Errors
    ///
    /// Returns [`Error::TooFewElements`] if `points` is empty.
    pub fn from_points(points: &[DirectPosition]) -> Result<Self, Error> {
        if points.is_empty() {
            return Err(Error::TooFewElements {
                geometry: "Envelope::from_points",
                minimum: 1,
                spec: None,
                id: None,
                detail: None,
            });
        }

        let first = &points[0];
        let (mut min_x, mut min_y, mut min_z) = (first.x(), first.y(), first.z());
        let (mut max_x, mut max_y, mut max_z) = (first.x(), first.y(), first.z());

        for point in points.iter().skip(1) {
            min_x = min_x.min(point.x());
            min_y = min_y.min(point.y());
            min_z = min_z.min(point.z());
            max_x = max_x.max(point.x());
            max_y = max_y.max(point.y());
            max_z = max_z.max(point.z());
        }

        let lower_corner = DirectPosition::new(min_x, min_y, min_z)?;
        let upper_corner = DirectPosition::new(max_x, max_y, max_z)?;

        Ok(Self::new_unchecked(lower_corner, upper_corner))
    }
}

impl Envelope {
    /// Constructs a [`Solid`] whose boundary is the six faces of the bounding box.
    ///
    /// Each face is a [`Polygon`] with an outward-facing [`LinearRing`] exterior.
    /// Faces are ordered: bottom (−z), top (+z), front (−y), back (+y), left (−x), right (+x).
    ///
    /// # Errors
    ///
    /// Returns [`Error::NotAVolume`] if the envelope does not have all three extents non-zero.
    pub fn to_solid(&self) -> Result<Solid, Error> {
        if !self.is_volume() {
            return Err(Error::NotAVolume {
                non_zero_extents: self.non_zero_extents(),
            });
        }

        let (lx, ly, lz) = (
            self.lower_corner.x(),
            self.lower_corner.y(),
            self.lower_corner.z(),
        );
        let (hx, hy, hz) = (
            self.upper_corner.x(),
            self.upper_corner.y(),
            self.upper_corner.z(),
        );

        let p000 = DirectPosition::new(lx, ly, lz).expect("envelope corners are finite");
        let p100 = DirectPosition::new(hx, ly, lz).expect("envelope corners are finite");
        let p110 = DirectPosition::new(hx, hy, lz).expect("envelope corners are finite");
        let p010 = DirectPosition::new(lx, hy, lz).expect("envelope corners are finite");
        let p001 = DirectPosition::new(lx, ly, hz).expect("envelope corners are finite");
        let p101 = DirectPosition::new(hx, ly, hz).expect("envelope corners are finite");
        let p111 = DirectPosition::new(hx, hy, hz).expect("envelope corners are finite");
        let p011 = DirectPosition::new(lx, hy, hz).expect("envelope corners are finite");

        let face_rings: [Vec<DirectPosition>; 6] = [
            vec![p000, p010, p110, p100], // bottom (−z)
            vec![p001, p101, p111, p011], // top    (+z)
            vec![p000, p100, p101, p001], // front  (−y)
            vec![p010, p011, p111, p110], // back   (+y)
            vec![p000, p001, p011, p010], // left   (−x)
            vec![p100, p110, p111, p101], // right  (+x)
        ];

        let members: Vec<AbstractSurfaceProperty> = face_rings
            .into_iter()
            .map(|points| {
                let ring = LinearRing::new(points).ok()?;
                let polygon = Polygon::new(
                    Some(AbstractRingProperty::from_object(
                        AbstractRingKind::LinearRing(ring),
                    )),
                    vec![],
                )
                .ok()?;
                Some(AbstractSurfaceProperty::from_object(
                    AbstractSurfaceKind::Polygon(polygon),
                ))
            })
            .collect::<Option<_>>()
            .expect("envelope corners are finite and valid");
        let shell = Shell::new(members).expect("envelope is valid");
        let shell_property = ShellProperty::from_object(shell);

        let solid = Solid::new(Some(shell_property)).expect("envelope is valid");
        Ok(solid)
    }

    /// Constructs a [`Polygon`] from the flat rectangle of this envelope.
    ///
    /// The four corners are wound counter-clockwise when viewed from the
    /// positive side of the collapsed axis (i.e. outward-facing normal).
    ///
    /// # Errors
    ///
    /// Returns [`Error::NotASurface`] if the envelope does not have exactly two non-zero extents.
    pub fn to_polygon(&self) -> Result<Polygon, Error> {
        if !self.is_surface() {
            return Err(Error::NotASurface {
                non_zero_extents: self.non_zero_extents(),
            });
        }

        let (lx, ly, lz) = (
            self.lower_corner.x(),
            self.lower_corner.y(),
            self.lower_corner.z(),
        );
        let (hx, hy, hz) = (
            self.upper_corner.x(),
            self.upper_corner.y(),
            self.upper_corner.z(),
        );

        let points = if self.size_z() == 0.0 {
            // XY plane — normal along +Z
            vec![
                DirectPosition::new(lx, ly, lz).expect("envelope corners are finite"),
                DirectPosition::new(hx, ly, lz).expect("envelope corners are finite"),
                DirectPosition::new(hx, hy, lz).expect("envelope corners are finite"),
                DirectPosition::new(lx, hy, lz).expect("envelope corners are finite"),
            ]
        } else if self.size_y() == 0.0 {
            // XZ plane — normal along +Y
            vec![
                DirectPosition::new(lx, ly, lz).expect("envelope corners are finite"),
                DirectPosition::new(lx, ly, hz).expect("envelope corners are finite"),
                DirectPosition::new(hx, ly, hz).expect("envelope corners are finite"),
                DirectPosition::new(hx, ly, lz).expect("envelope corners are finite"),
            ]
        } else {
            // YZ plane — normal along +X
            vec![
                DirectPosition::new(lx, ly, lz).expect("envelope corners are finite"),
                DirectPosition::new(lx, hy, lz).expect("envelope corners are finite"),
                DirectPosition::new(lx, hy, hz).expect("envelope corners are finite"),
                DirectPosition::new(lx, ly, hz).expect("envelope corners are finite"),
            ]
        };

        let ring = LinearRing::new(points).expect("envelope corners are finite and valid");
        Polygon::new(
            Some(AbstractRingProperty::from_object(
                AbstractRingKind::LinearRing(ring),
            )),
            vec![],
        )
        .map_err(|_| Error::NotASurface {
            non_zero_extents: self.non_zero_extents(),
        })
    }

    /// Triangulates the envelope into a [`TriangulatedSurface`].
    ///
    /// - For a surface envelope (`is_surface()`): triangulates the single rectangular face.
    /// - For a volume envelope (`is_volume()`): triangulates all six bounding faces and merges them.
    ///
    /// # Errors
    ///
    /// Returns [`Error::NotSurfaceOrVolume`] if the envelope is a point or line segment.
    pub fn to_triangulated_surface(&self) -> Result<TriangulatedSurface, Error> {
        if self.is_surface() {
            self.to_polygon()?
                .triangulate()
                .map(Triangulation::into_surface)
        } else if self.is_volume() {
            self.to_solid()?
                .exterior()
                .as_ref()
                .expect("must be created")
                .object()
                .expect("must be created")
                .triangulate()
                .map(Triangulation::into_surface)
        } else {
            Err(Error::NotSurfaceOrVolume {
                non_zero_extents: self.non_zero_extents(),
            })
        }
    }
}

impl ApplyTransform for Envelope {
    fn apply_transform(&mut self, transform: Transform3<f64>) {
        let transformed_lower_corner: Point3<f64> = transform * Point3::from(self.lower_corner);
        let transformed_upper_corner: Point3<f64> = transform * Point3::from(self.upper_corner);

        self.lower_corner = DirectPosition::new(
            transformed_lower_corner.x.min(transformed_upper_corner.x),
            transformed_lower_corner.y.min(transformed_upper_corner.y),
            transformed_lower_corner.z.min(transformed_upper_corner.z),
        )
        .expect("envelope corners are finite");
        self.upper_corner = DirectPosition::new(
            transformed_lower_corner.x.max(transformed_upper_corner.x),
            transformed_lower_corner.y.max(transformed_upper_corner.y),
            transformed_lower_corner.z.max(transformed_upper_corner.z),
        )
        .expect("envelope corners are finite");
    }

    /// Applies a rigid-body isometry (rotation + translation) to this envelope in place.
    ///
    /// Both corners are transformed and the result is re-fitted as an axis-aligned bounding
    /// box by taking per-axis minima/maxima. This keeps the AABB invariant valid after
    /// rotation, at the cost of a potentially larger box for non-axis-aligned rotations.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use egml_core::model::geometry::{DirectPosition, Envelope};
    /// use nalgebra::{Isometry3, Vector3};
    /// use crate::egml_core::model::common::ApplyTransform;
    ///
    /// let lo = DirectPosition::new(0.0, 0.0, 0.0).unwrap();
    /// let hi = DirectPosition::new(1.0, 2.0, 3.0).unwrap();
    /// let mut env = Envelope::new(lo, hi).unwrap();
    ///
    /// let translation = Isometry3::translation(10.0, 0.0, 0.0);
    /// env.apply_isometry(translation);
    ///
    /// assert_eq!(env.lower_corner().x(), 10.0);
    /// assert_eq!(env.upper_corner().x(), 11.0);
    /// ```
    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        let transformed_lower_corner: Point3<f64> = isometry * Point3::from(self.lower_corner);
        let transformed_upper_corner: Point3<f64> = isometry * Point3::from(self.upper_corner);

        self.lower_corner = DirectPosition::new(
            transformed_lower_corner.x.min(transformed_upper_corner.x),
            transformed_lower_corner.y.min(transformed_upper_corner.y),
            transformed_lower_corner.z.min(transformed_upper_corner.z),
        )
        .expect("envelope corners are finite");
        self.upper_corner = DirectPosition::new(
            transformed_lower_corner.x.max(transformed_upper_corner.x),
            transformed_lower_corner.y.max(transformed_upper_corner.y),
            transformed_lower_corner.z.max(transformed_upper_corner.z),
        )
        .expect("envelope corners are finite");
    }

    /// Applies a pure translation to this envelope in place.
    ///
    /// Fast path: translation preserves per-axis ordering, so both corners can be shifted
    /// directly with no re-fit via min/max — unlike rotation or scale, which can invalidate
    /// the axis-aligned invariant and require re-deriving the corners.
    fn apply_translation(&mut self, vector: Vector3<f64>) {
        self.lower_corner = DirectPosition::new(
            self.lower_corner.x() + vector.x,
            self.lower_corner.y() + vector.y,
            self.lower_corner.z() + vector.z,
        )
        .expect("envelope corners are finite");
        self.upper_corner = DirectPosition::new(
            self.upper_corner.x() + vector.x,
            self.upper_corner.y() + vector.y,
            self.upper_corner.z() + vector.z,
        )
        .expect("envelope corners are finite");
    }

    /// Applies a pure rotation (about the origin) to this envelope in place.
    ///
    /// Both corners are rotated and the result is re-fitted as an axis-aligned bounding box
    /// by taking per-axis minima/maxima, same as [`apply_isometry`](Self::apply_isometry).
    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        let transformed_lower_corner: Point3<f64> = rotation * Point3::from(self.lower_corner);
        let transformed_upper_corner: Point3<f64> = rotation * Point3::from(self.upper_corner);

        self.lower_corner = DirectPosition::new(
            transformed_lower_corner.x.min(transformed_upper_corner.x),
            transformed_lower_corner.y.min(transformed_upper_corner.y),
            transformed_lower_corner.z.min(transformed_upper_corner.z),
        )
        .expect("envelope corners are finite");
        self.upper_corner = DirectPosition::new(
            transformed_lower_corner.x.max(transformed_upper_corner.x),
            transformed_lower_corner.y.max(transformed_upper_corner.y),
            transformed_lower_corner.z.max(transformed_upper_corner.z),
        )
        .expect("envelope corners are finite");
    }

    /// Applies a per-axis scale to this envelope in place.
    ///
    /// Both corners are scaled and the result is re-fitted via min/max, same as
    /// [`apply_isometry`](Self::apply_isometry) — a negative scale factor mirrors an axis and
    /// would otherwise leave the lower corner greater than the upper corner on that axis.
    fn apply_scale(&mut self, scale: Scale3<f64>) {
        let transformed_lower_corner = Point3::new(
            self.lower_corner.x() * scale.vector.x,
            self.lower_corner.y() * scale.vector.y,
            self.lower_corner.z() * scale.vector.z,
        );
        let transformed_upper_corner = Point3::new(
            self.upper_corner.x() * scale.vector.x,
            self.upper_corner.y() * scale.vector.y,
            self.upper_corner.z() * scale.vector.z,
        );

        self.lower_corner = DirectPosition::new(
            transformed_lower_corner.x.min(transformed_upper_corner.x),
            transformed_lower_corner.y.min(transformed_upper_corner.y),
            transformed_lower_corner.z.min(transformed_upper_corner.z),
        )
        .expect("envelope corners are finite");
        self.upper_corner = DirectPosition::new(
            transformed_lower_corner.x.max(transformed_upper_corner.x),
            transformed_lower_corner.y.max(transformed_upper_corner.y),
            transformed_lower_corner.z.max(transformed_upper_corner.z),
        )
        .expect("envelope corners are finite");
    }
}

impl fmt::Display for Envelope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Envelope[{}, {}, {} -> {}, {}, {}]",
            self.lower_corner.x(),
            self.lower_corner.y(),
            self.lower_corner.z(),
            self.upper_corner.x(),
            self.upper_corner.y(),
            self.upper_corner.z()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pos(x: f64, y: f64, z: f64) -> DirectPosition {
        DirectPosition::new(x, y, z).unwrap()
    }

    fn env(lx: f64, ly: f64, lz: f64, ux: f64, uy: f64, uz: f64) -> Envelope {
        Envelope::new(pos(lx, ly, lz), pos(ux, uy, uz)).unwrap()
    }

    #[test]
    fn apply_translation_shifts_both_corners() {
        let mut envelope = env(0.0, 0.0, 0.0, 1.0, 2.0, 3.0);

        envelope.apply_translation(Vector3::new(10.0, -5.0, 1.0));

        assert_eq!(envelope, env(10.0, -5.0, 1.0, 11.0, -3.0, 4.0));
    }

    #[test]
    fn apply_rotation_refits_axis_aligned_box() {
        use std::f64::consts::FRAC_PI_2;

        let mut envelope = env(0.0, 0.0, 0.0, 1.0, 2.0, 3.0);

        // 90-degree rotation about Z swaps the X and Y extents.
        envelope.apply_rotation(Rotation3::from_euler_angles(0.0, 0.0, FRAC_PI_2));

        assert!((envelope.size_x() - 2.0).abs() < 1e-10);
        assert!((envelope.size_y() - 1.0).abs() < 1e-10);
        assert!((envelope.size_z() - 3.0).abs() < 1e-10);
    }

    #[test]
    fn apply_scale_refits_on_mirroring_negative_scale() {
        let mut envelope = env(1.0, 1.0, 1.0, 2.0, 3.0, 4.0);

        // Negative X scale mirrors the box, which would otherwise leave lower.x > upper.x.
        envelope.apply_scale(Scale3::new(-1.0, 2.0, 1.0));

        assert_eq!(envelope, env(-2.0, 2.0, 1.0, -1.0, 6.0, 4.0));
    }

    #[test]
    fn from_envelopes_empty_returns_none() {
        let result = Envelope::from_envelopes(&[]);
        assert!(result.is_none());
    }

    #[test]
    fn from_envelopes_single_returns_same_envelope() {
        let e = env(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
        let result = Envelope::from_envelopes(&[e.clone()]).unwrap();

        assert_eq!(result, e);
    }

    #[test]
    fn from_envelopes_two_disjoint() {
        let a = env(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);
        let b = env(5.0, 6.0, 7.0, 8.0, 9.0, 10.0);
        let result = Envelope::from_envelopes(&[a, b]).unwrap();

        assert_eq!(result, env(0.0, 0.0, 0.0, 8.0, 9.0, 10.0));
    }

    #[test]
    fn from_envelopes_overlapping() {
        let a = env(0.0, 0.0, 0.0, 5.0, 5.0, 5.0);
        let b = env(3.0, 3.0, 3.0, 7.0, 7.0, 7.0);
        let result = Envelope::from_envelopes(&[a, b]).unwrap();

        assert_eq!(result, env(0.0, 0.0, 0.0, 7.0, 7.0, 7.0));
    }

    #[test]
    fn from_envelopes_one_contains_the_other() {
        let outer = env(0.0, 0.0, 0.0, 10.0, 10.0, 10.0);
        let inner = env(2.0, 3.0, 4.0, 5.0, 6.0, 7.0);
        let result = Envelope::from_envelopes(&[outer.clone(), inner]).unwrap();

        assert_eq!(result, outer);
    }

    #[test]
    fn from_envelopes_multiple() {
        let a = env(0.0, 10.0, 20.0, 1.0, 11.0, 21.0);
        let b = env(-5.0, 8.0, 25.0, 2.0, 12.0, 30.0);
        let c = env(1.0, 9.0, 18.0, 3.0, 15.0, 22.0);
        let result = Envelope::from_envelopes(&[a, b, c]).unwrap();

        assert_eq!(result, env(-5.0, 8.0, 18.0, 3.0, 15.0, 30.0));
    }

    #[test]
    fn from_envelopes_with_negative_coords() {
        let a = env(-10.0, -20.0, -30.0, -1.0, -2.0, -3.0);
        let b = env(-5.0, -25.0, -15.0, 0.0, -1.0, 0.0);
        let result = Envelope::from_envelopes(&[a, b]).unwrap();

        assert_eq!(result, env(-10.0, -25.0, -30.0, 0.0, -1.0, 0.0));
    }

    #[test]
    fn from_envelopes_zero_volume_envelopes() {
        let a = env(1.0, 1.0, 1.0, 1.0, 1.0, 1.0); // point
        let b = env(3.0, 3.0, 3.0, 3.0, 3.0, 3.0); // point
        let result = Envelope::from_envelopes(&[a, b]).unwrap();

        assert_eq!(result, env(1.0, 1.0, 1.0, 3.0, 3.0, 3.0));
    }

    #[test]
    fn is_point_when_corners_equal() {
        let e = env(1.0, 2.0, 3.0, 1.0, 2.0, 3.0);
        assert!(e.is_point());
        assert!(!e.is_linear());
        assert!(!e.is_surface());
        assert!(!e.is_volume());
    }

    #[test]
    fn is_linear_along_x() {
        let e = env(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        assert!(!e.is_point());
        assert!(e.is_linear());
        assert!(!e.is_surface());
        assert!(!e.is_volume());
    }

    #[test]
    fn is_linear_along_y() {
        let e = env(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        assert!(e.is_linear());
    }

    #[test]
    fn is_linear_along_z() {
        let e = env(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        assert!(e.is_linear());
    }

    #[test]
    fn is_surface_xy_plane() {
        let e = env(0.0, 0.0, 0.0, 1.0, 1.0, 0.0);
        assert!(!e.is_point());
        assert!(!e.is_linear());
        assert!(e.is_surface());
        assert!(!e.is_volume());
    }

    #[test]
    fn is_surface_xz_plane() {
        let e = env(0.0, 0.0, 0.0, 1.0, 0.0, 1.0);
        assert!(e.is_surface());
    }

    #[test]
    fn is_surface_yz_plane() {
        let e = env(0.0, 0.0, 0.0, 0.0, 1.0, 1.0);
        assert!(e.is_surface());
    }

    #[test]
    fn is_volume_all_extents_nonzero() {
        let e = env(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);
        assert!(!e.is_point());
        assert!(!e.is_linear());
        assert!(!e.is_surface());
        assert!(e.is_volume());
    }

    #[test]
    fn to_polygon_returns_err_for_point() {
        assert_eq!(
            env(1.0, 1.0, 1.0, 1.0, 1.0, 1.0).to_polygon(),
            Err(Error::NotASurface {
                non_zero_extents: 0
            })
        );
    }

    #[test]
    fn to_polygon_returns_err_for_linear() {
        assert_eq!(
            env(0.0, 0.0, 0.0, 1.0, 0.0, 0.0).to_polygon(),
            Err(Error::NotASurface {
                non_zero_extents: 1
            })
        );
    }

    #[test]
    fn to_polygon_returns_err_for_volume() {
        assert_eq!(
            env(0.0, 0.0, 0.0, 1.0, 1.0, 1.0).to_polygon(),
            Err(Error::NotASurface {
                non_zero_extents: 3
            })
        );
    }

    #[test]
    fn to_polygon_xy_plane() {
        assert!(env(0.0, 0.0, 0.0, 2.0, 3.0, 0.0).to_polygon().is_ok());
    }

    #[test]
    fn to_polygon_xz_plane() {
        assert!(env(0.0, 0.0, 0.0, 2.0, 0.0, 3.0).to_polygon().is_ok());
    }

    #[test]
    fn to_polygon_yz_plane() {
        assert!(env(0.0, 0.0, 0.0, 0.0, 2.0, 3.0).to_polygon().is_ok());
    }

    #[test]
    fn to_triangulated_surface_returns_err_for_point() {
        assert_eq!(
            env(0.0, 0.0, 0.0, 0.0, 0.0, 0.0).to_triangulated_surface(),
            Err(Error::NotSurfaceOrVolume {
                non_zero_extents: 0
            })
        );
    }

    #[test]
    fn to_triangulated_surface_returns_err_for_linear() {
        assert_eq!(
            env(0.0, 0.0, 0.0, 1.0, 0.0, 0.0).to_triangulated_surface(),
            Err(Error::NotSurfaceOrVolume {
                non_zero_extents: 1
            })
        );
    }

    #[test]
    fn to_triangulated_surface_surface_has_two_triangles() {
        let result = env(0.0, 0.0, 0.0, 2.0, 3.0, 0.0)
            .to_triangulated_surface()
            .unwrap();
        assert_eq!(result.triangles().len(), 2);
    }

    #[test]
    fn to_triangulated_surface_volume_has_twelve_triangles() {
        let result = env(0.0, 0.0, 0.0, 1.0, 1.0, 1.0)
            .to_triangulated_surface()
            .unwrap();
        assert_eq!(result.triangles().len(), 12);
    }

    #[test]
    fn envelope_contains() {
        let lower_corner = DirectPosition::new(1.0, 2.0, 3.0).unwrap();
        let upper_corner = DirectPosition::new(2.0, 3.0, 4.0).unwrap();
        let envelope = Envelope::new(lower_corner, upper_corner).unwrap();
        let point_a = DirectPosition::new(1.5, 2.5, 3.5).unwrap();
        let point_b = DirectPosition::new(2.5, 3.5, 4.5).unwrap();

        assert!(envelope.contains(&point_a));
        assert!(!envelope.contains(&point_b));
    }

    #[test]
    fn contains_envelope_partially_true_for_overlapping_boxes() {
        let a = env(0.0, 0.0, 0.0, 10.0, 10.0, 10.0);
        let b = env(5.0, 5.0, 5.0, 15.0, 15.0, 15.0);
        assert!(a.contains_envelope_partially(&b));
        assert!(b.contains_envelope_partially(&a));
    }

    #[test]
    fn contains_envelope_partially_true_for_disjoint_boxes() {
        let a = env(0.0, 0.0, 0.0, 10.0, 10.0, 10.0);
        let b = env(20.0, 20.0, 20.0, 30.0, 30.0, 30.0);
        assert!(!a.contains_envelope_partially(&b));
        assert!(!b.contains_envelope_partially(&a));
    }

    #[test]
    fn contains_envelope_partially_true_for_slab_crossing_with_no_corner_contained() {
        // A thin slab passing straight through the middle of `a` along the z axis.
        // Neither box's own min/max corner lies inside the other, so a corner-only
        // check would wrongly report no overlap.
        let a = env(0.0, 0.0, 0.0, 10.0, 10.0, 10.0);
        let slab = env(-5.0, -5.0, 4.0, 15.0, 15.0, 6.0);
        assert!(a.contains_envelope_partially(&slab));
        assert!(slab.contains_envelope_partially(&a));
    }

    #[test]
    fn contains_envelope_partially_true_when_touching_at_boundary() {
        let a = env(0.0, 0.0, 0.0, 10.0, 10.0, 10.0);
        let b = env(10.0, 10.0, 10.0, 20.0, 20.0, 20.0);
        assert!(a.contains_envelope_partially(&b));
    }
}
