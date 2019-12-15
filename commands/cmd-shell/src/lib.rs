use std::path::PathBuf;
use std::process::{self, Command, Stdio};
use std::{env, fs};

use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use directories::ProjectDirs;
use ds_command::{ArgMatches, Config, DsCommand};
use ssri::Integrity;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct ShellCmd {
    #[structopt(long, default_value = "node")]
    node: String,
    #[structopt(long, hidden = true)]
    data_dir: Option<PathBuf>,
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
        if args.occurrences_of("data_dir") == 0 {
            if let Ok(data_dir) = config.get_str("data_dir") {
                self.data_dir = Some(PathBuf::from(data_dir));
            }
        }
        Ok(())
    }

    async fn execute(self) -> Result<()> {
        let code = Command::new(self.node)
            .env(
                "DS_BIN",
                env::current_exe()
                    .context("Failed to get the location of the current ds binary.")?,
            )
            .arg("-r")
            .arg(ensure_dstopic(self.data_dir)?)
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

fn ensure_dstopic(dir_override: Option<PathBuf>) -> Result<PathBuf> {
    let dir = match dir_override {
        Some(dir) => dir,
        None => ProjectDirs::from("dev", "entropic", "ds")
            .ok_or_else(|| anyhow!("Couldn't find home directory."))
            .context("A home directory is required for ds patch scripts.")?
            .data_dir()
            .to_path_buf(),
    };
    fs::create_dir_all(&dir)
        .with_context(|| format!("Failed to create data directory at {:?}", dir))?;
    let data = include_bytes!("../../../dstopic/dist/dstopic.js").to_vec();
    let hash = Integrity::from(&data).to_hex().1;
    let script = dir.join(format!("dstopic-{}", hash.to_string()));
    if !script.exists() {
        fs::write(&script, &data)
            .with_context(|| format!("Failed to write dstopic data file at {:?}", script))?;
    }
    Ok(script)
}
