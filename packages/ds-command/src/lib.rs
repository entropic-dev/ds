use anyhow::Result;
use async_trait::async_trait;
pub use clap::ArgMatches;
pub use config::Config;

#[async_trait]
pub trait DsCommand {
    async fn execute(self, matches: ArgMatches<'_>, config: Config) -> Result<()>;
}
