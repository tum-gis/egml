use crate::model::geometry::primitives::shell::Shell;

#[derive(Debug, Clone, PartialEq)]
pub struct ShellProperty {
    pub object: Option<Shell>,
    pub href: Option<String>,
}

impl ShellProperty {
    pub fn new(object: Shell) -> Self {
        Self {
            object: Some(object),
            href: None,
        }
    }

    pub fn new_href(href: impl Into<String>) -> Self {
        Self {
            object: None,
            href: Some(href.into()),
        }
    }
}
