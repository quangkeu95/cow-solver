use clap::Parser;
use cow_solver::{cli::Cli, logging::init_tracing_subscriber};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    init_tracing_subscriber();

    let cli = Cli::parse();

    cli.run()?;

    Ok(())
}
