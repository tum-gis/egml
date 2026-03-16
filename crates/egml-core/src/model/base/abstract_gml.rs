use crate::model::base::Id;

/// Base data shared by every GML object (ISO 19136 §7.2.2, `gml:AbstractGMLType`).
///
/// Every GML object carries an optional stable [`Id`] and zero-or-more human-readable
/// name strings.  Concrete geometry and feature types embed `AbstractGml` and
/// expose it through the [`AsAbstractGml`] trait.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct AbstractGml {
    /// Optional stable identifier for this GML object.
    pub id: Option<Id>,
    /// Human-readable names associated with this GML object.
    pub name: Vec<String>,
}

impl AbstractGml {
    /// Creates a new `AbstractGml` with no id and no names.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use egml_core::model::base::AbstractGml;
    ///
    /// let gml = AbstractGml::new();
    /// assert!(gml.id.is_none());
    /// assert!(gml.name.is_empty());
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new `AbstractGml` pre-populated with the given `id`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use egml_core::model::base::{AbstractGml, Id};
    ///
    /// let id = Id::from_hashed_string("my-object");
    /// let gml = AbstractGml::with_id(id);
    /// assert!(gml.id.is_some());
    /// ```
    pub fn with_id(id: Id) -> Self {
        Self {
            id: Some(id),
            ..Default::default()
        }
    }

    /// Creates a new `AbstractGml` with an optional id.
    ///
    /// Equivalent to [`with_id`](Self::with_id) when `id` is `Some`, and to
    /// [`new`](Self::new) when `id` is `None`.
    pub fn with_optional_id(id: Option<Id>) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

/// Object-safe read accessor for [`AbstractGml`] fields.
///
/// Implemented by all GML object types.  The default methods delegate to
/// [`abstract_gml()`](Self::abstract_gml), so implementors only need to
/// provide that single method.
pub trait AsAbstractGml {
    /// Returns a reference to the embedded [`AbstractGml`] base data.
    fn abstract_gml(&self) -> &AbstractGml;

    /// Returns the optional identifier of this GML object.
    fn id(&self) -> Option<&Id> {
        self.abstract_gml().id.as_ref()
    }

    /// Returns the names of this GML object.
    fn name(&self) -> &[String] {
        &self.abstract_gml().name
    }
}

/// Mutable companion to [`AsAbstractGml`].
///
/// Implemented by all GML object types that expose mutable access to their
/// base data.
pub trait AsAbstractGmlMut: AsAbstractGml {
    /// Returns a mutable reference to the embedded [`AbstractGml`] base data.
    fn abstract_gml_mut(&mut self) -> &mut AbstractGml;

    /// Sets or clears the identifier of this GML object.
    fn set_id(&mut self, id: Option<Id>) {
        self.abstract_gml_mut().id = id;
    }
}

impl AsAbstractGml for AbstractGml {
    fn abstract_gml(&self) -> &AbstractGml {
        self
    }
}

impl AsAbstractGmlMut for AbstractGml {
    fn abstract_gml_mut(&mut self) -> &mut AbstractGml {
        self
    }
}
