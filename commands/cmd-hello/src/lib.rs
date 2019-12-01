use anyhow::Result;
use async_trait::async_trait;
use ds_command::{ArgMatches, Config, DsCommand};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct HelloCmd {
    #[structopt(help = "who to say hello to", default_value = "world")]
    arg: String,
    #[structopt(help = "whether to greet enthusiastically", short, long)]
    enthusiastic: bool,
}

#[async_trait]
impl DsCommand for HelloCmd {
    async fn execute(mut self, arg: ArgMatches<'_>, conf: Config) -> Result<()> {
        if !arg.is_present("enthusiastic") {
            self.enthusiastic = conf.get_bool("hello.enthusiastic").unwrap_or(false);
        }
        print!("Hello, {}", self.arg);
        if self.enthusiastic {
            println!("!");
        } else {
            println!("");
        }
        Ok(())
    }
}
