use crate::model::abstract_object::{AbstractObject, AsAbstractObject, AsAbstractObjectMut};
use crate::model::base::Id;
use crate::model::basic_types::Code;

/// Base data shared by every GML object ([OGC 07-036 §7.2.2.2](https://docs.ogc.org/is/07-036/07-036.pdf), `gml:AbstractGMLType`).
///
/// Every GML object carries an optional stable [`Id`] and zero-or-more human-readable
/// name strings.  Concrete geometry and feature types embed `AbstractGml` and
/// expose it through the [`AsAbstractGml`] trait.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct AbstractGml {
    pub abstract_object: AbstractObject,
    /// Optional stable identifier for this GML object.
    id: Option<Id>,
    /// Human-readable names associated with this GML object.
    names: Vec<Code>,
}

impl AbstractGml {
    /// Creates a new `AbstractGml` with no id and no names.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use egml_core::model::base::AbstractGml;
    /// use crate::egml_core::model::base::AsAbstractGml;
    ///
    /// let gml = AbstractGml::new();
    /// assert!(gml.id().is_none());
    /// assert!(gml.names().is_empty());
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_abstract_object(abstract_object: AbstractObject) -> Self {
        Self {
            abstract_object,
            id: None,
            names: Vec::new(),
        }
    }

    /// Creates a new `AbstractGml` pre-populated with the given `id`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use egml_core::model::base::{AbstractGml, Id};
    /// use crate::egml_core::model::base::AsAbstractGml;
    ///
    /// let id = Id::from_hashed_string("my-object");
    /// let gml = AbstractGml::with_id(id);
    /// assert!(gml.id().is_some());
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
pub trait AsAbstractGml: AsAbstractObject {
    /// Returns a reference to the embedded [`AbstractGml`] base data.
    fn abstract_gml(&self) -> &AbstractGml;

    /// Returns the optional identifier of this GML object.
    fn id(&self) -> Option<&Id> {
        self.abstract_gml().id.as_ref()
    }

    /// Returns the names of this GML object.
    fn names(&self) -> &[Code] {
        &self.abstract_gml().names
    }
}

/// Mutable companion to [`AsAbstractGml`].
///
/// Implemented by all GML object types that expose mutable access to their
/// base data.
pub trait AsAbstractGmlMut: AsAbstractObjectMut + AsAbstractGml {
    /// Returns a mutable reference to the embedded [`AbstractGml`] base data.
    fn abstract_gml_mut(&mut self) -> &mut AbstractGml;

    /// Sets the identifier of this GML object.
    fn set_id(&mut self, id: Id) {
        self.abstract_gml_mut().id = Some(id);
    }

    /// Sets or clears the identifier of this GML object.
    fn set_id_opt(&mut self, id: Option<Id>) {
        self.abstract_gml_mut().id = id;
    }

    /// Clears the identifier of this GML object.
    fn clear_id(&mut self) {
        self.abstract_gml_mut().id = None;
    }

    fn set_names(&mut self, names: Vec<Code>) {
        self.abstract_gml_mut().names = names;
    }

    fn push_name(&mut self, name: Code) {
        self.abstract_gml_mut().names.push(name);
    }

    fn extend_names(&mut self, names: impl IntoIterator<Item = Code>) {
        self.abstract_gml_mut().names.extend(names);
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

#[macro_export]
macro_rules! impl_abstract_gml_traits {
    ($type:ty) => {
        impl $crate::model::AsAbstractObject for $type {
            fn abstract_object(&self) -> &$crate::model::AbstractObject {
                &<$type as $crate::model::base::AsAbstractGml>::abstract_gml(self).abstract_object
            }
        }
    };
}

#[macro_export]
macro_rules! impl_abstract_gml_mut_traits {
    ($type:ty) => {
        impl $crate::model::AsAbstractObjectMut for $type {
            fn abstract_object_mut(&mut self) -> &mut $crate::model::AbstractObject {
                &mut <$type as $crate::model::base::AsAbstractGmlMut>::abstract_gml_mut(self)
                    .abstract_object
            }
        }
    };
}

impl_abstract_gml_traits!(AbstractGml);
impl_abstract_gml_mut_traits!(AbstractGml);
