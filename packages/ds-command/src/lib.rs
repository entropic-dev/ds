use anyhow::Result;
use async_trait::async_trait;
pub use clap::ArgMatches;
pub use config::Config;

#[async_trait]
pub trait DsCommand {
    fn layer_config(&mut self, _matches: ArgMatches, _config: Config) -> Result<()> {
        Ok(())
    }
    async fn execute(self) -> Result<()>;
}
