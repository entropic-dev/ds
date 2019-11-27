use dstopic_command::Command;
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
    #[structopt(about = "Lists current config values.")]
    List {
        #[structopt(long)]
        json: bool,
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

impl Command for ConfigCmd {
    fn execute(self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        match self {
            ConfigCmd::Get { key, .. } => println!("Getting value for {:?}", key),
            ConfigCmd::Set { key, value, .. } => {
                println!("Setting value for {:?} to {:?}", key, value)
            }
            ConfigCmd::Rm { key, .. } => println!("Getting value for {:?}", key),
            ConfigCmd::List { json, .. } => println!("printing out all configs (json: {:?})", json),
        }
        Ok(())
    }
}
