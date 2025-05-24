use tracing_subscriber::fmt::format::FmtSpan;

/// Initializes tracing subscribers for the application.
///
/// Currently just logs to the std out.
pub fn init_tracing() {
    tracing_subscriber::fmt()
        .pretty()
        .with_thread_names(true)
        .with_max_level(tracing::Level::DEBUG)
        .with_span_events(FmtSpan::ENTER)
        .init();
}
