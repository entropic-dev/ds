use std::path::PathBuf;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use ds_command::{ArgMatches, Config, DsCommand};
use ds_config::ConfigOptions;
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
    #[structopt(long, short)]
    local: bool,
    #[structopt(long, short)]
    global: bool,
    #[structopt(skip)]
    config: Option<PathBuf>,
}

#[async_trait]
impl DsCommand for ConfigCmd {
    fn layer_config(&mut self, args: ArgMatches<'_>, _: Config) -> Result<()> {
        match self {
            ConfigCmd::Get { ref mut opts, .. } => {
                opts.config = if args.is_present("config") {
                    args.value_of("config").map(PathBuf::from)
                } else {
                    None
                };
                Ok(())
            }
            _ => Ok(()),
        }
    }

    async fn execute(mut self) -> Result<()> {
        match self {
            ConfigCmd::Get { key, opts } => config_read(key, opts)?,
            ConfigCmd::Set { .. } => return Err(anyhow!("Command not yet implemented.")),
            ConfigCmd::Rm { .. } => return Err(anyhow!("Command not yet implemented.")),
        }
        Ok(())
    }
}

fn config_read(key: String, opts: ConfigOpts) -> Result<()> {
    let config = if opts.config.is_some() {
        ConfigOptions::new()
            .env(false)
            .local(false)
            .global_config_file(opts.config.clone())
            .load()?
    } else if opts.global {
        ConfigOptions::new()
            .env(false)
            .local(false)
            .global(true)
            .load()?
    } else if opts.local {
        ConfigOptions::new()
            .env(false)
            .local(true)
            .global(false)
            .load()?
    } else {
        ConfigOptions::new()
            .env(true)
            .local(true)
            .global(true)
            .load()?
    };
    println!("{}", config.get_str(&key)?);
    Ok(())
}
