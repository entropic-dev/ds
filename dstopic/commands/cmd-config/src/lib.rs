use anyhow::{anyhow, Result};

use ds_command::{ArgMatches, Config, DsCommand};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum ConfigCmd {
    #[structopt(about = "Gets a config value.")]
    Get {
        key: String,
        #[structopt(flatten)]
        opts: ConfigOpts,
    },
    #[structopt(about = "Sets a config value.")]
    Set {
        key: String,
        value: String,
        #[structopt(flatten)]
        opts: ConfigOpts,
    },
    #[structopt(about = "Removes a config value.")]
    Rm {
        key: String,
        #[structopt(flatten)]
        opts: ConfigOpts,
    },
}

#[derive(Debug, StructOpt)]
pub struct ConfigOpts {
    #[structopt(long)]
    local: bool,
    #[structopt(long)]
    global: bool,
}

impl DsCommand for ConfigCmd {
    fn execute(self, _: ArgMatches, config: Config) -> Result<()> {
        match self {
            ConfigCmd::Get { key, .. } => {
                if let Ok(val) = config.get_str(&key) {
                    println!("{}", val);
                } else {
                    return Err(anyhow!("No value set for key: {:?}", key));
                }
            }
            ConfigCmd::Set { .. } => return Err(anyhow!("Command not yet implemented.")),
            ConfigCmd::Rm { .. } => return Err(anyhow!("Command not yet implemented.")),
        }
        Ok(())
    }
}
