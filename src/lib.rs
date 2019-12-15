use std::path::PathBuf;

use anyhow::Result;
use async_trait::async_trait;
use clap;
use ds_command::{ArgMatches, Config, DsCommand};
use ds_config::ConfigOptions;
use structopt::StructOpt;

use cmd_config::ConfigCmd;
use cmd_ping::PingCmd;
use cmd_shell::ShellCmd;

#[derive(Debug, StructOpt)]
#[structopt(
    author = "Kat Marchán <kzm@zkat.tech>",
    about = "Manage your Entropic packages.",
    setting = clap::AppSettings::ColoredHelp,
    setting = clap::AppSettings::DisableHelpSubcommand,
    setting = clap::AppSettings::DeriveDisplayOrder,
)]
pub struct Ds {
    #[structopt(help = "File to read configuration values from.", long, global = true)]
    config: Option<PathBuf>,
    #[structopt(subcommand)]
    subcommand: DsCmd,
}

impl Ds {
    pub async fn load() -> Result<()> {
        let clp = Ds::clap();
        let matches = clp.get_matches();
        let mut ds = Ds::from_clap(&matches);
        let cfg = if let Some(file) = &ds.config {
            ConfigOptions::new()
                .local(false)
                .global_config_file(Some(file.clone()))
                .load()?
        } else {
            ConfigOptions::new().load()?
        };
        ds.layer_config(matches, cfg)?;
        ds.execute().await?;
        Ok(())
    }
}

#[derive(Debug, StructOpt)]
pub enum DsCmd {
    #[structopt(about = "Configuration subcommands.", alias = "c")]
    Config(ConfigCmd),
    #[structopt(about = "Ping an entropic server")]
    Ping(PingCmd),
    #[structopt(
        about = "Execute a new wrapped `node` shell.",
        alias = "sh",
        setting = clap::AppSettings::TrailingVarArg
    )]
    Shell(ShellCmd),
}

#[async_trait]
impl DsCommand for Ds {
    fn layer_config(&mut self, args: ArgMatches<'_>, conf: Config) -> Result<()> {
        match self.subcommand {
            DsCmd::Config(ref mut cfg) => {
                cfg.layer_config(args.subcommand_matches("config").unwrap().clone(), conf)
            }
            DsCmd::Ping(ref mut ping) => {
                ping.layer_config(args.subcommand_matches("ping").unwrap().clone(), conf)
            }
            DsCmd::Shell(ref mut shell) => {
                shell.layer_config(args.subcommand_matches("shell").unwrap().clone(), conf)
            }
        }
    }

    async fn execute(self) -> Result<()> {
        match self.subcommand {
            DsCmd::Config(cfg) => cfg.execute().await,
            DsCmd::Ping(ping) => ping.execute().await,
            DsCmd::Shell(shell) => shell.execute().await,
        }
    }
}
