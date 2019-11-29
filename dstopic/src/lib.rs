use std::path::PathBuf;

use anyhow::Result;
use ds_command::{ArgMatches, Config, DsCommand};
use ds_config::ConfigOptions;
use structopt::StructOpt;

use cmd_config::ConfigCmd;
use cmd_hello::HelloCmd;

#[derive(Debug, StructOpt)]
#[structopt(
    author = "Kat Marchán <kzm@zkat.tech>",
    about = "Manage your Entropic packages."
)]
pub struct Dstopic {
    #[structopt(help = "Directory to look for the config file in.", long)]
    config: Option<PathBuf>,
    #[structopt(subcommand)]
    subcommand: DstopicCmd,
}

#[derive(Debug, StructOpt)]
pub enum DstopicCmd {
    #[structopt(about = "Say hello to something")]
    Hello(HelloCmd),
    #[structopt(about = "Configuration subcommands.")]
    Config(ConfigCmd),
}

impl DsCommand for Dstopic {
    fn execute(self, args: ArgMatches, conf: Config) -> Result<()> {
        match self.subcommand {
            DstopicCmd::Hello(hello) => {
                hello.execute(args.subcommand_matches("hello").unwrap().clone(), conf)
            }
            DstopicCmd::Config(cfg) => {
                cfg.execute(args.subcommand_matches("config").unwrap().clone(), conf)
            }
        }
    }
}

impl Dstopic {
    pub fn load() -> Result<()> {
        let clp = Dstopic::clap();
        let matches = clp.get_matches();
        let ds = Dstopic::from_clap(&matches);
        let cfg = if let Some(file) = &ds.config {
            ConfigOptions::new()
                .local(false)
                .global_config_file(Some(file.clone()))
                .load()?
        } else {
            ConfigOptions::new().load()?
        };
        ds.execute(matches, cfg)?;
        Ok(())
    }
}
