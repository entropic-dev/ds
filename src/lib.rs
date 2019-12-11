use std::path::PathBuf;

use anyhow::Result;
use async_trait::async_trait;
use ds_command::{ArgMatches, Config, DsCommand};
use ds_config::ConfigOptions;
use structopt::StructOpt;

use cmd_config::ConfigCmd;
use cmd_hello::HelloCmd;
use cmd_ping::PingCmd;

#[derive(Debug, StructOpt)]
#[structopt(
    author = "Kat Marchán <kzm@zkat.tech>",
    about = "Manage your Entropic packages."
)]
pub struct Ds {
    #[structopt(
        help = "Directory to look for the config file in.",
        long,
        global = true
    )]
    config: Option<PathBuf>,
    #[structopt(subcommand)]
    subcommand: DsCmd,
}

impl Ds {
    pub async fn load() -> Result<()> {
        let clp = Ds::clap();
        let matches = clp.get_matches();
        let ds = Ds::from_clap(&matches);
        let cfg = if let Some(file) = &ds.config {
            ConfigOptions::new()
                .local(false)
                .global_config_file(Some(file.clone()))
                .load()?
        } else {
            ConfigOptions::new().load()?
        };
        ds.execute(matches, cfg).await?;
        Ok(())
    }
}

#[derive(Debug, StructOpt)]
pub enum DsCmd {
    #[structopt(about = "Say hello to something", alias = "hi", alias = "yo")]
    Hello(HelloCmd),
    #[structopt(about = "Configuration subcommands.", alias = "c")]
    Config(ConfigCmd),
    #[structopt(about = "Ping an entropic server")]
    Ping(PingCmd),
}

#[async_trait]
impl DsCommand for Ds {
    async fn execute(self, args: ArgMatches<'_>, conf: Config) -> Result<()> {
        match self.subcommand {
            DsCmd::Hello(hello) => {
                hello
                    .execute(args.subcommand_matches("hello").unwrap().clone(), conf)
                    .await
            }
            DsCmd::Config(cfg) => {
                cfg.execute(args.subcommand_matches("config").unwrap().clone(), conf)
                    .await
            }
            DsCmd::Ping(ping) => {
                ping.execute(args.subcommand_matches("ping").unwrap().clone(), conf)
                    .await
            }
        }
    }
}
