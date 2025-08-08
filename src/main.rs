use axum::ServiceExt;
use axum::extract::Request;
use clap::Parser;
use dotenvy::dotenv;
use potree_auth::application_lib::Cli;
use potree_auth::application_lib::init_application;
use potree_auth::application_lib::init_tracing;
use potree_auth::application_lib::shutdown_signal;
// Using `jemalloc` as opposed to the standard system allocator to reduce memory
// fragmentation.
#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Load environment variables from a `.env` file if it exists.
    let _ = dotenv();

    // Set up tracing subscribers
    init_tracing();

    // Parse arguments from the CLI.
    let cli = Cli::parse();

    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", &cli.server.host, &cli.server.port)).await?;

    let application = init_application(cli.into()).await?;

    tracing::info!("listening on {}", listener.local_addr()?);
    axum::serve(
        listener,
        ServiceExt::<Request>::into_make_service(application),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await?;

    Ok(())
}
