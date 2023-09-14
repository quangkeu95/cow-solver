use clap::Parser;
use url::Url;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(long, required = true)]
    pub orderbook_url: Url,
    #[clap(long, required = true)]
    pub node_url: Url,
}

impl Cli {
    pub fn run(self) -> eyre::Result<()> {
        tracing::info!(%self.orderbook_url, %self.node_url);
        Ok(())
    }
}
