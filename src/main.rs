use clap::Parser;
use potree_auth::{cli::Cli, initialize_application};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let application = initialize_application(&cli.into());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, application).await.unwrap();
}
