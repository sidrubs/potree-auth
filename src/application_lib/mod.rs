//! The top level `potree-auth` layer. Composes functionality from all the
//! various domains.
//!
//! See [`router`] for the bulk of the logic.

mod cli;
mod config;
pub mod error;
pub mod http;
mod observability;

pub use cli::Cli;
pub use http::router::init_application;
pub use observability::init_tracing;
