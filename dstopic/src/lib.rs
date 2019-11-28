use anyhow::Result;
use ds_command::{ArgMatches, Config, DsCommand};
use structopt::StructOpt;

use cmd_config::ConfigCmd;
use cmd_hello::HelloCmd;

#[derive(Debug, StructOpt)]
#[structopt(
    author = "Kat Marchán <kzm@zkat.tech>",
    about = "Manage your Entropic packages."
)]
pub enum Dstopic {
    #[structopt(about = "Say hello to something")]
    Hello(HelloCmd),
    #[structopt(about = "Configuration subcommands.")]
    Config(ConfigCmd),
}

impl DsCommand for Dstopic {
    fn execute(self, args: ArgMatches, conf: Config) -> Result<()> {
        match self {
            Dstopic::Hello(hello) => {
                hello.execute(args.subcommand_matches("hello").unwrap().clone(), conf)
            }
            Dstopic::Config(cfg) => {
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
        let cfg = ds_config::new()?;
        ds.execute(matches, cfg)?;
        Ok(())
    }
}
