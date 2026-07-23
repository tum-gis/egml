use nalgebra::{Isometry3, Rotation3, Scale3, Transform3, Translation3, Vector3};

/// Applies a transform to `self` in place.
///
/// Only [`apply_transform`](ApplyTransform::apply_transform) is required. The other four
/// methods have default implementations that convert to `Transform3<f64>` and delegate —
/// correct everywhere, but not the fastest path: `apply_translation`, for instance, ends up
/// doing a full homogeneous matrix multiply per point instead of a plain component-wise add.
///
/// A default can only reach `self` through other trait methods, so it can never be faster
/// than `apply_transform` on its own. Implementors that care about that cost (in particular
/// leaf position types, and container types that recurse into many of them) should override
/// the specific methods with dedicated math all the way down the recursion — falling back to
/// the default at any layer converts to a general transform there, and every layer below pays
/// the full matrix cost regardless of what leaves further down are capable of.
pub trait ApplyTransform {
    fn apply_transform(&mut self, transform: Transform3<f64>);

    fn apply_isometry(&mut self, isometry: Isometry3<f64>) {
        self.apply_transform(nalgebra::convert(isometry));
    }

    fn apply_translation(&mut self, vector: Vector3<f64>) {
        self.apply_transform(nalgebra::convert(Translation3::from(vector)));
    }

    fn apply_rotation(&mut self, rotation: Rotation3<f64>) {
        self.apply_transform(nalgebra::convert(rotation));
    }

    fn apply_scale(&mut self, scale: Scale3<f64>) {
        self.apply_transform(nalgebra::convert(scale));
    }
}
