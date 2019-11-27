use dstopic_command::Command;
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

impl Command for Dstopic {
    fn execute(self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        match self {
            Dstopic::Hello(hello) => hello.execute(),
            Dstopic::Config(cfg) => cfg.execute(),
        }
    }
}

impl Dstopic {
    pub fn load() -> Dstopic {
        Dstopic::from_args()
    }
}
