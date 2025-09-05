/// Defines actions that can be performed on a resource.
///
/// Used for authorization purposes.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(any(test, feature = "fake"), derive(fake::Dummy))]
pub enum Action {
    Read,
    List,
    Create,
    Update,
    Delete,
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Action::Read => write!(f, "read"),
            Action::List => write!(f, "list"),
            Action::Create => write!(f, "create"),
            Action::Update => write!(f, "update"),
            Action::Delete => write!(f, "delete"),
        }
    }
}
