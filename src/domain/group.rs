/// Represents groups that are allowed to access a specific project.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize)]
#[cfg_attr(test, derive(fake::Dummy))]
pub struct Group(String);

impl Group {
    pub fn new(name: &str) -> Self {
        Self(name.to_owned())
    }
}
