/// Creates a newtype with following code automatically generated:
///
/// - Derives `Clone`, `Debug`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`, and
///   `Display`.
/// - Implements `new`, `Deref`, and `From`.
/// - Allows one to implement additional traits and features as needed.
///
/// This was [copied][1] from the `oauth2-rs` crate.
///
/// # Examples
///
/// ```rs
/// new_type! [
///     /// A safe wrapper around a `user_id` string
///     #[derive(Eq, Hash)]
///     UserId(String)
///     impl {
///         fn as_str(&self) -> &str {
///             &self
///         }
///     }
/// ];
/// ```
///
/// [1]: https://github.com/ramosbugs/oauth2-rs/blob/main/src/types.rs
#[macro_export]
macro_rules! new_type {
    // Convenience pattern without an impl.
    (
        $(#[$attr:meta])*
        $name:ident(
            $(#[$type_attr:meta])*
            $type:ty
        )
    ) => {
        new_type![
            @new_type $(#[$attr])*,
            $name(
                $(#[$type_attr])*
                $type
            ),
            concat!(
                "Create a new `",
                stringify!($name),
                "` to wrap the given `",
                stringify!($type),
                "`."
            ),
            impl {}
        ];
    };
    // Main entry point with an impl.
    (
        $(#[$attr:meta])*
        $name:ident(
            $(#[$type_attr:meta])*
            $type:ty
        )
        impl {
            $($item:tt)*
        }
    ) => {
        new_type![
            @new_type $(#[$attr])*,
            $name(
                $(#[$type_attr])*
                $type
            ),
            concat!(
                "Create a new `",
                stringify!($name),
                "` to wrap the given `",
                stringify!($type),
                "`."
            ),
            impl {
                $($item)*
            }
        ];
    };
    // Actual implementation, after stringifying the #[doc] attr.
    (
        @new_type $(#[$attr:meta])*,
        $name:ident(
            $(#[$type_attr:meta])*
            $type:ty
        ),
        $new_doc:expr,
        impl {
            $($item:tt)*
        }
    ) => {
        $(#[$attr])*
        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
        #[cfg_attr(any(test, feature = "fake"), derive(fake::Dummy))]
        pub struct $name(
            $(#[$type_attr])*
            $type
        );
        impl $name {
            $($item)*

            #[doc = $new_doc]
            pub const fn new(s: $type) -> Self {
                $name(s)
            }
        }
        impl std::ops::Deref for $name {
            type Target = $type;
            fn deref(&self) -> &$type {
                &self.0
            }
        }
        impl From<$name> for $type {
            fn from(t: $name) -> $type {
                t.0
            }
        }
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    }
}

pub(crate) use new_type;
