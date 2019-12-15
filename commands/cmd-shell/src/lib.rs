use std::process::{self, Command, Stdio};

use anyhow::{Context, Result};
use async_trait::async_trait;
use ds_command::{ArgMatches, Config, DsCommand};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct ShellCmd {
    #[structopt(long, default_value = "node")]
    node: String,
    #[structopt(multiple = true)]
    args: Vec<String>,
}

#[async_trait]
impl DsCommand for ShellCmd {
    fn layer_config(&mut self, args: ArgMatches, config: Config) -> Result<()> {
        if args.occurrences_of("node") == 0 {
            if let Ok(node) = config.get_str("node") {
                self.node = node;
            }
        }
        Ok(())
    }

    async fn execute(self) -> Result<()> {
        let code = Command::new(self.node)
            .args(self.args)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .stdin(Stdio::inherit())
            .status()
            .context("Failed to execute node binary.")?
            .code()
            .unwrap_or(1);
        if code > 0 {
            process::exit(code);
        }
        Ok(())
    }
}
