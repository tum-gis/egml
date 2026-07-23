use crate::model::base::AssociationAttributes;
use crate::model::basic_types::NilReason;
use crate::model::xlink::{ActuateType, HRef, ShowType};

/// Read-only access to the `gml:AssociationAttributeGroup` on a property element.
pub trait HasAssociationAttributes {
    fn association(&self) -> &AssociationAttributes;

    /// `xlink:href` — reference to the geometry object, if not inline.
    fn href(&self) -> Option<&HRef> {
        self.association().href.as_ref()
    }

    /// Bare GML `gml:id` when `href` is a local fragment reference.
    fn local_id(&self) -> Option<&str> {
        self.association().local_id()
    }

    /// `gml:nilReason` — explanation for a missing value.
    fn nil_reason(&self) -> Option<&NilReason> {
        self.association().nil_reason.as_ref()
    }

    /// `xlink:title` — human-readable label for the linked resource.
    fn title(&self) -> Option<&str> {
        self.association().title.as_deref()
    }

    /// `xlink:role` — URI identifying the semantic role of the linked resource.
    fn role(&self) -> Option<&str> {
        self.association().role.as_deref()
    }

    /// `xlink:arcrole` — URI identifying the role of the link arc.
    fn arcrole(&self) -> Option<&str> {
        self.association().arcrole.as_deref()
    }

    /// `xlink:show` — how the linked resource should be presented on traversal.
    fn show(&self) -> Option<&ShowType> {
        self.association().show.as_ref()
    }

    /// `xlink:actuate` — when link traversal should be triggered.
    fn actuate(&self) -> Option<&ActuateType> {
        self.association().actuate.as_ref()
    }
}

/// Mutable access to the `gml:AssociationAttributeGroup` on a property element.
pub trait HasAssociationAttributesMut: HasAssociationAttributes {
    fn association_mut(&mut self) -> &mut AssociationAttributes;

    /// Sets `xlink:href`.
    fn set_href(&mut self, href: HRef) {
        self.association_mut().href = Some(href);
    }

    /// Sets or clears `xlink:href`.
    fn set_href_opt(&mut self, href: Option<HRef>) {
        self.association_mut().href = href;
    }

    /// Clears `xlink:href`.
    fn clear_href(&mut self) {
        self.association_mut().href = None;
    }

    /// Sets `gml:nilReason`.
    fn set_nil_reason(&mut self, nil_reason: NilReason) {
        self.association_mut().nil_reason = Some(nil_reason);
    }

    /// Sets or clears `gml:nilReason`.
    fn set_nil_reason_opt(&mut self, nil_reason: Option<NilReason>) {
        self.association_mut().nil_reason = nil_reason;
    }

    /// Clears `gml:nilReason`.
    fn clear_nil_reason(&mut self) {
        self.association_mut().nil_reason = None;
    }

    /// Sets `xlink:title`.
    fn set_title(&mut self, title: impl Into<String>) {
        self.association_mut().title = Some(title.into());
    }

    /// Sets or clears `xlink:title`.
    fn set_title_opt(&mut self, title: Option<String>) {
        self.association_mut().title = title;
    }

    /// Clears `xlink:title`.
    fn clear_title(&mut self) {
        self.association_mut().title = None;
    }

    /// Sets `xlink:role`.
    fn set_role(&mut self, role: impl Into<String>) {
        self.association_mut().role = Some(role.into());
    }

    /// Sets or clears `xlink:role`.
    fn set_role_opt(&mut self, role: Option<String>) {
        self.association_mut().role = role;
    }

    /// Clears `xlink:role`.
    fn clear_role(&mut self) {
        self.association_mut().role = None;
    }

    /// Sets `xlink:arcrole`.
    fn set_arcrole(&mut self, arcrole: impl Into<String>) {
        self.association_mut().arcrole = Some(arcrole.into());
    }

    /// Sets or clears `xlink:arcrole`.
    fn set_arcrole_opt(&mut self, arcrole: Option<String>) {
        self.association_mut().arcrole = arcrole;
    }

    /// Clears `xlink:arcrole`.
    fn clear_arcrole(&mut self) {
        self.association_mut().arcrole = None;
    }

    /// Sets `xlink:show`.
    fn set_show(&mut self, show: ShowType) {
        self.association_mut().show = Some(show);
    }

    /// Sets or clears `xlink:show`.
    fn set_show_opt(&mut self, show: Option<ShowType>) {
        self.association_mut().show = show;
    }

    /// Clears `xlink:show`.
    fn clear_show(&mut self) {
        self.association_mut().show = None;
    }

    /// Sets `xlink:actuate`.
    fn set_actuate(&mut self, actuate: ActuateType) {
        self.association_mut().actuate = Some(actuate);
    }

    /// Sets or clears `xlink:actuate`.
    fn set_actuate_opt(&mut self, actuate: Option<ActuateType>) {
        self.association_mut().actuate = actuate;
    }

    /// Clears `xlink:actuate`.
    fn clear_actuate(&mut self) {
        self.association_mut().actuate = None;
    }
}
