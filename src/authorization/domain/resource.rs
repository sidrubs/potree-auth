use std::fmt::Debug;

use crate::common::domain::Group;
use crate::common::domain::utils::new_type::new_type;

/// Defines a resource type that can be authorized against.
pub trait Resource: Debug {
    /// The groups that the resource belongs to. Currently, if a user is part of
    /// the same group they are allowed do action the resource.
    fn resource_type(&self) -> ResourceType;
}

/// Defines the functionality required for fine grained access to a particular
/// resource (instance-level authZ).
pub trait ResourceInstance: Resource {
    /// Identifies the specific resource instance.
    fn resource_identifier(&self) -> ResourceIdentifier;

    /// The groups that the resource belongs to. Currently, if a user is part of
    /// the same group they are allowed do action the resource.
    fn groups(&self) -> Vec<Group>;
}

new_type![
    /// The identifier of a resource type (type-level).
    #[cfg_attr(test, derive(fake::Dummy))]
    ResourceType(String)
];

new_type![
    /// The identifier of a specific resource instance (instance-level).
    #[cfg_attr(test, derive(fake::Dummy))]
    ResourceIdentifier(String)
];

/// A super basic struct implementing the [`Resource`] and [`ResourceInstance`]
/// traits for testing purposes.
///
/// I was struggling to get mockall to deal with the supertrait mocking.
#[cfg(test)]
pub mod mocked_resource {
    use super::Group;
    use super::Resource;
    use super::ResourceIdentifier;
    use super::ResourceInstance;
    use super::ResourceType;

    #[derive(Debug, fake::Dummy)]
    pub struct MockedResource {
        pub resource_type: ResourceType,
        pub resource_identifier: ResourceIdentifier,
        pub groups: Vec<Group>,
    }

    impl Resource for MockedResource {
        fn resource_type(&self) -> ResourceType {
            self.resource_type.clone()
        }
    }

    impl ResourceInstance for MockedResource {
        fn resource_identifier(&self) -> ResourceIdentifier {
            self.resource_identifier.clone()
        }

        fn groups(&self) -> Vec<Group> {
            self.groups.clone()
        }
    }
}
