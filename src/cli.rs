use std::net::SocketAddr;

use clap::Parser;
use url::Url;

use crate::api;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    // #[clap(long, required = true)]
    // pub orderbook_url: Url,
    // #[clap(long, required = true)]
    // pub node_url: Url,
    #[clap(long, default_value = "0.0.0.0:8000")]
    pub bind_address: SocketAddr,
}

impl Cli {
    pub async fn run(self) -> eyre::Result<()> {
        // tracing::info!(%self.orderbook_url, %self.node_url);
        api::serve(&self.bind_address).await?;

        Ok(())
    }
}
