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
    fn layer_config(&mut self, arg: ArgMatches, conf: Config) -> Result<()> {
        if arg.occurrences_of("enthusiastic") == 0 {
            if let Ok(val) = conf.get_bool("hello.enthusiastic") {
                self.enthusiastic = val;
            }
        }
        Ok(())
    }

    async fn execute(self) -> Result<()> {
        print!("Hello, {}", self.arg);
        if self.enthusiastic {
            println!("!");
        } else {
            println!("");
        }
        Ok(())
    }
}
