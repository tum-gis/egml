use crate::model::base::{AsAbstractGml, Id};
use crate::model::common::IterGeometries;
use crate::model::geometry::refs::AbstractGeometryKindRef;
use std::collections::HashMap;

/// Resolves `gml:id`/`xlink:href` references to the geometry they point at.
///
/// Indexes borrowed [`AbstractGeometryKindRef`]s by [`Id`], so lookups by
/// reference are O(1) instead of re-walking the geometry tree. Geometries
/// without an `id` are not indexed — they cannot be referenced by
/// `xlink:href` in the first place.
#[derive(Debug, Clone, Default)]
pub struct GeometryResolver<'a> {
    by_id: HashMap<Id, AbstractGeometryKindRef<'a>>,
}

impl<'a> GeometryResolver<'a> {
    /// Creates an empty resolver.
    pub fn new() -> Self {
        Self::default()
    }

    /// Builds a resolver by recursively walking `root` and indexing every
    /// descendant geometry (including `root` itself) that carries an `id`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use egml_core::model::geometry::DirectPosition;
    /// use egml_core::model::geometry::primitives::Point;
    /// use egml_core::model::base::{AsAbstractGmlMut, Id};
    /// use egml_core::resolver::GeometryResolver;
    ///
    /// let mut point = Point::new(DirectPosition::new(1.0, 2.0, 3.0).unwrap());
    /// point.set_id(Id::try_from("point-1").expect("valid id"));
    ///
    /// let resolver = GeometryResolver::build(&point);
    /// assert!(resolver.resolve(&Id::try_from("point-1").expect("valid id")).is_some());
    /// ```
    pub fn build<T: IterGeometries>(root: &'a T) -> Self {
        let mut resolver = Self::new();
        resolver.insert_root(root);
        resolver
    }

    /// Recursively walks `root` and indexes every descendant geometry
    /// (including `root` itself) that carries an `id`.
    ///
    /// Unlike [`build`](Self::build), this adds to an existing resolver
    /// instead of creating a new one — call it once per top-level geometry
    /// when a document has several independent roots (e.g. one per feature
    /// in a city model) that should all resolve through the same table.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use egml_core::model::base::{AsAbstractGmlMut, Id};
    /// use egml_core::model::geometry::DirectPosition;
    /// use egml_core::model::geometry::primitives::Point;
    /// use egml_core::resolver::GeometryResolver;
    ///
    /// let mut first = Point::new(DirectPosition::new(1.0, 2.0, 3.0).unwrap());
    /// first.set_id(Id::try_from("point-1").expect("valid id"));
    /// let mut second = Point::new(DirectPosition::new(4.0, 5.0, 6.0).unwrap());
    /// second.set_id(Id::try_from("point-2").expect("valid id"));
    ///
    /// let mut resolver = GeometryResolver::new();
    /// resolver.insert_root(&first);
    /// resolver.insert_root(&second);
    ///
    /// assert_eq!(resolver.len(), 2);
    /// ```
    pub fn insert_root<T: IterGeometries>(&mut self, root: &'a T) {
        self.extend(root.iter_geometries());
    }

    /// Indexes a single geometry by its `id`, if it has one.
    ///
    /// Returns `true` if the geometry had an `id` and was stored. If another
    /// geometry was already indexed under the same `id`, it is silently
    /// replaced — `gml:id` is supposed to be unique within a document, so a
    /// collision indicates malformed input rather than a case this resolver
    /// needs to arbitrate.
    pub fn insert(&mut self, geometry: AbstractGeometryKindRef<'a>) -> bool {
        match geometry.id() {
            Some(id) => {
                self.by_id.insert(id.clone(), geometry);
                true
            }
            None => false,
        }
    }

    /// Indexes every geometry in `geometries` that carries an `id`.
    pub fn extend(&mut self, geometries: impl IntoIterator<Item = AbstractGeometryKindRef<'a>>) {
        for geometry in geometries {
            self.insert(geometry);
        }
    }

    /// Looks up the geometry stored under `id`, if any.
    pub fn resolve(&self, id: &Id) -> Option<AbstractGeometryKindRef<'a>> {
        self.by_id.get(id).copied()
    }

    /// Looks up the geometry stored under `id` and downcasts it to a concrete
    /// leaf type or intermediate `*Ref` enum via the [`TryFrom`] conversions
    /// generated across the geometry ref hierarchy.
    ///
    /// Returns `None` if `id` is unresolved, or if it resolves to a geometry
    /// of a different concrete type than `T`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use egml_core::model::base::{AsAbstractGmlMut, Id};
    /// use egml_core::model::geometry::DirectPosition;
    /// use egml_core::model::geometry::primitives::LinearRing;
    /// use egml_core::resolver::GeometryResolver;
    ///
    /// let mut ring = LinearRing::new([
    ///     DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
    ///     DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
    ///     DirectPosition::new(0.0, 1.0, 0.0).unwrap(),
    /// ])
    /// .unwrap();
    /// ring.set_id(Id::try_from("ring-1").expect("valid id"));
    ///
    /// let resolver = GeometryResolver::build(&ring);
    /// let resolved: Option<&LinearRing> = resolver.resolve_as(&Id::try_from("ring-1").expect("valid id"));
    /// assert!(resolved.is_some());
    /// ```
    pub fn resolve_as<T>(&self, id: &Id) -> Option<T>
    where
        T: TryFrom<AbstractGeometryKindRef<'a>>,
    {
        self.resolve(id)
            .and_then(|geometry| T::try_from(geometry).ok())
    }

    /// Returns `true` if `id` is indexed.
    pub fn contains(&self, id: &Id) -> bool {
        self.by_id.contains_key(id)
    }

    /// Returns an iterator over all indexed `id`s, in arbitrary order.
    pub fn ids(&self) -> impl Iterator<Item = &Id> {
        self.by_id.keys()
    }

    /// Returns an iterator over all indexed `(id, geometry)` pairs, in
    /// arbitrary order.
    pub fn iter(&self) -> impl Iterator<Item = (&Id, &AbstractGeometryKindRef<'a>)> {
        self.by_id.iter()
    }

    pub fn len(&self) -> usize {
        self.by_id.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_id.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::base::AsAbstractGmlMut;
    use crate::model::geometry::DirectPosition;
    use crate::model::geometry::primitives::{
        AbstractRingKind, AbstractRingProperty, LinearRing, Point, Polygon,
    };

    fn ring_with_id(id: &str) -> LinearRing {
        let mut ring = LinearRing::new([
            DirectPosition::new(0.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(1.0, 0.0, 0.0).unwrap(),
            DirectPosition::new(0.0, 1.0, 0.0).unwrap(),
        ])
        .unwrap();
        ring.set_id(Id::try_from(id).expect("valid id"));
        ring
    }

    #[test]
    fn build_indexes_self_and_descendants() {
        let mut polygon = Polygon::new(
            Some(AbstractRingProperty::from_object(
                AbstractRingKind::LinearRing(ring_with_id("ring-1")),
            )),
            [],
        )
        .unwrap();
        polygon.set_id(Id::try_from("polygon-1").expect("valid id"));

        let resolver = GeometryResolver::build(&polygon);

        assert_eq!(resolver.len(), 2);
        assert!(resolver.contains(&Id::try_from("polygon-1").expect("valid id")));
        assert!(resolver.contains(&Id::try_from("ring-1").expect("valid id")));
    }

    #[test]
    fn resolve_unknown_id_yields_none() {
        let polygon = Polygon::new(None, []).unwrap();
        let resolver = GeometryResolver::build(&polygon);

        assert!(
            resolver
                .resolve(&Id::try_from("missing").expect("valid id"))
                .is_none()
        );
    }

    #[test]
    fn resolve_as_downcasts_to_concrete_type() {
        let mut polygon = Polygon::new(
            Some(AbstractRingProperty::from_object(
                AbstractRingKind::LinearRing(ring_with_id("ring-1")),
            )),
            [],
        )
        .unwrap();
        polygon.set_id(Id::try_from("polygon-1").expect("valid id"));

        let resolver = GeometryResolver::build(&polygon);

        let ring: Option<&LinearRing> =
            resolver.resolve_as(&Id::try_from("ring-1").expect("valid id"));
        assert!(ring.is_some());

        // Downcasting to the wrong concrete type fails.
        let wrong: Option<&Point> = resolver.resolve_as(&Id::try_from("ring-1").expect("valid id"));
        assert!(wrong.is_none());
    }

    #[test]
    fn insert_root_adds_a_second_independent_tree_to_the_same_resolver() {
        let mut first_polygon = Polygon::new(
            Some(AbstractRingProperty::from_object(
                AbstractRingKind::LinearRing(ring_with_id("ring-1")),
            )),
            [],
        )
        .unwrap();
        first_polygon.set_id(Id::try_from("polygon-1").expect("valid id"));

        let mut second_polygon = Polygon::new(
            Some(AbstractRingProperty::from_object(
                AbstractRingKind::LinearRing(ring_with_id("ring-2")),
            )),
            [],
        )
        .unwrap();
        second_polygon.set_id(Id::try_from("polygon-2").expect("valid id"));

        let mut resolver = GeometryResolver::new();
        resolver.insert_root(&first_polygon);
        resolver.insert_root(&second_polygon);

        // self (2) + one ring each (2) = 4, across two unrelated root trees.
        assert_eq!(resolver.len(), 4);
        assert!(resolver.contains(&Id::try_from("polygon-1").expect("valid id")));
        assert!(resolver.contains(&Id::try_from("polygon-2").expect("valid id")));
        assert!(resolver.contains(&Id::try_from("ring-1").expect("valid id")));
        assert!(resolver.contains(&Id::try_from("ring-2").expect("valid id")));
    }

    #[test]
    fn insert_skips_geometry_without_id() {
        let point = Point::new(DirectPosition::new(1.0, 2.0, 3.0).unwrap());
        let mut resolver = GeometryResolver::new();

        let inserted = resolver.insert((&point).into());

        assert!(!inserted);
        assert!(resolver.is_empty());
    }

    #[test]
    fn insert_duplicate_id_overwrites_last_wins() {
        let mut first = Point::new(DirectPosition::new(1.0, 2.0, 3.0).unwrap());
        first.set_id(Id::try_from("dup").expect("valid id"));
        let mut second = Point::new(DirectPosition::new(4.0, 5.0, 6.0).unwrap());
        second.set_id(Id::try_from("dup").expect("valid id"));

        let mut resolver = GeometryResolver::new();
        resolver.insert((&first).into());
        resolver.insert((&second).into());

        assert_eq!(resolver.len(), 1);
        let resolved: &Point = resolver
            .resolve_as(&Id::try_from("dup").expect("valid id"))
            .unwrap();
        assert_eq!(resolved.pos().x(), 4.0);
    }

    #[test]
    fn ids_and_iter_expose_all_entries() {
        let mut polygon = Polygon::new(
            Some(AbstractRingProperty::from_object(
                AbstractRingKind::LinearRing(ring_with_id("ring-1")),
            )),
            [],
        )
        .unwrap();
        polygon.set_id(Id::try_from("polygon-1").expect("valid id"));

        let resolver = GeometryResolver::build(&polygon);

        let ids: Vec<&Id> = resolver.ids().collect();
        assert_eq!(ids.len(), 2);
        assert!(ids.contains(&&Id::try_from("polygon-1").expect("valid id")));
        assert!(ids.contains(&&Id::try_from("ring-1").expect("valid id")));

        assert_eq!(resolver.iter().count(), 2);
    }
}
