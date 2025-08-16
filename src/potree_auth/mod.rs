//! The top level `potree-auth` layer. Composes functionality from all the
//! various domains.
//!
//! See [`router`] for the bulk of the logic.

mod cli;
pub mod config;
mod http;
mod observability;
mod shutdown_signal;

pub use cli::Cli;
pub use http::init_application;
pub use observability::init_tracing;
pub use shutdown_signal::shutdown_signal;
