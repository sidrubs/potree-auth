/// Initializes tracing subscribers for the application.
///
/// Currently just logs to the std out.
pub fn init_tracing() {
    tracing_subscriber::fmt()
        .pretty()
        .with_thread_names(true)
        .with_max_level(tracing::Level::DEBUG)
        .init();
}
