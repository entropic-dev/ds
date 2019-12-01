use anyhow::Result;
use async_trait::async_trait;
use ds_command::{ArgMatches, Config, DsCommand};
use structopt::StructOpt;
use surf;
use url::Url;

#[derive(Debug, StructOpt)]
pub struct PingCmd {
    #[structopt(
        long,
        help = "Registry to ping.",
        default_value = "https://registry.entropic.dev"
    )]
    registry: Url,
}

#[async_trait]
impl DsCommand for PingCmd {
    async fn execute(self, arg: ArgMatches<'_>, config: Config) -> Result<()> {
        unimplemented!()
    }
}
