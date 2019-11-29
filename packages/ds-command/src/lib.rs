use anyhow::Result;
pub use clap::ArgMatches;
pub use config::Config;

pub trait DsCommand {
    fn execute(self, matches: ArgMatches, config: Config) -> Result<()>;
}
