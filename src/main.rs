use clap::Parser;
use cow_solver::{cli::Cli, logging::init_tracing_subscriber};

#[tokio::main]
async fn main() {
    init_tracing_subscriber();

    let cli = Cli::parse();

    tokio::select! {
        result = cli.run() => tracing::error!(?result, "Main exited"),
    };
}
