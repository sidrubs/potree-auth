use dotenvy::dotenv;
// Using `jemalloc` as opposed to the standard system allocator to reduce memory
// fragmentation.
#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

use clap::Parser;
use potree_auth::{cli::Cli, init_tracing, initialize_application};

#[tokio::main]
async fn main() {
    // Load environment variables from a `.env` file if it exists.
    let _ = dotenv();

    // Set up tracing subscribers
    init_tracing();

    let cli = Cli::parse();

    let application = initialize_application(&cli.into()).await.unwrap();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, application).await.unwrap();
}
