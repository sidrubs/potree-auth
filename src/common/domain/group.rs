use serde::Deserialize;
use serde::Serialize;

/// Represents groups that users and projects can be part of.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[cfg_attr(any(test, feature = "fake"), derive(fake::Dummy))]
pub struct Group(String);

impl Group {
    pub fn new(name: &str) -> Self {
        Self(name.to_owned())
    }
}
