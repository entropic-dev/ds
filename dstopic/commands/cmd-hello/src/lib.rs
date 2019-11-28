use anyhow::Result;

use dstopic_command::Command;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct HelloCmd {
    arg: Option<String>,
}

impl Command for HelloCmd {
    fn execute(self) -> Result<()> {
        println!("Hello, {}", self.arg.unwrap_or("world".into()));
        Ok(())
    }
}
