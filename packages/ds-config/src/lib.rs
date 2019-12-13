use std::env;
use std::path::PathBuf;

use anyhow::Result;
pub use config::Config;
use config::{ConfigError, Environment, File};
use directories::ProjectDirs;

pub struct ConfigOptions {
    global: bool,
    local: bool,
    env: bool,
    local_config_dir: Option<PathBuf>,
    global_config_file: Option<PathBuf>,
}

impl Default for ConfigOptions {
    fn default() -> Self {
        ConfigOptions {
            global: true,
            local: true,
            env: true,
            local_config_dir: env::current_dir().ok().map(|d| d.to_owned()),
            global_config_file: ProjectDirs::from("dev", "entropic", "ds")
                .map(|d| d.config_dir().to_owned().join("dsrc.toml")),
        }
    }
}

impl ConfigOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn local(mut self, local: bool) -> Self {
        self.local = local;
        self
    }

    pub fn global(mut self, global: bool) -> Self {
        self.global = global;
        self
    }

    pub fn env(mut self, env: bool) -> Self {
        self.env = env;
        self
    }

    pub fn local_config_dir(mut self, dir: Option<PathBuf>) -> Self {
        self.local_config_dir = dir;
        self
    }

    pub fn global_config_file(mut self, file: Option<PathBuf>) -> Self {
        self.global_config_file = file;
        self
    }

    pub fn load(self) -> Result<Config, ConfigError> {
        let mut c = Config::new();
        if self.global {
            if let Some(config_file) = self.global_config_file {
                let path = config_file.display().to_string();
                c.merge(File::with_name(&path[..]).required(false))?;
            }
        }
        if self.local {
            if let Some(dir) = self.local_config_dir {
                for path in dir.ancestors().collect::<Vec<_>>().iter().rev() {
                    let p = path.join("dsrc").display().to_string();
                    c.merge(File::with_name(&p[..]).required(false))?;
                    let p = path.join(".dsrc").display().to_string();
                    c.merge(File::with_name(&p[..]).required(false))?;
                }
            }
        }
        if self.env {
            c.merge(Environment::with_prefix("ds_config"))?;
        }
        Ok(c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::env;
    use std::fs;

    use anyhow::Result;
    use tempfile::tempdir;

    #[test]
    fn working_dir_config() -> Result<()> {
        let dir = tempdir()?;
        let file = dir.path().join("dsrc.toml");
        fs::write(file, "store = \"hello world\"")?;
        let config = ConfigOptions::new()
            .env(false)
            .global(false)
            .local_config_dir(Some(dir.path().to_owned()))
            .load()?;
        assert_eq!(config.get_str("store")?, String::from("hello world"));
        Ok(())
    }

    #[test]
    fn parent_dir_config() -> Result<()> {
        let dir = tempdir()?;
        let file = dir.path().join("dsrc.toml");
        fs::write(file, "store = \"hello world\"")?;
        let subpath = dir.path().join("foo").join("bar");
        fs::create_dir_all(&subpath)?;
        let config = ConfigOptions::new()
            .env(false)
            .global(false)
            .local_config_dir(Some(subpath.to_owned()))
            .load()?;
        assert_eq!(config.get_str("store")?, String::from("hello world"));
        Ok(())
    }

    #[test]
    fn working_dir_shadowing_config() -> Result<()> {
        let dir = tempdir()?;
        let file = dir.path().join("dsrc.toml");
        fs::write(file, "store = \"goodbye world\"")?;
        let subpath = dir.path().join("foo").join("bar");
        fs::create_dir_all(&subpath)?;
        let file = dir.path().join("foo").join("dsrc.toml");
        fs::write(file, "store = \"hello world\"")?;
        let config = ConfigOptions::new()
            .env(false)
            .global(false)
            .local_config_dir(Some(subpath.to_owned()))
            .load()?;
        assert_eq!(config.get_str("store")?, String::from("hello world"));
        Ok(())
    }

    #[test]
    fn working_dir_shadowing_config_dotfile() -> Result<()> {
        let dir = tempdir()?;
        let file = dir.path().join(".dsrc.toml");
        fs::write(file, "store = \"goodbye world\"")?;
        let subpath = dir.path().join("foo").join("bar");
        fs::create_dir_all(&subpath)?;
        let file = dir.path().join("foo").join(".dsrc.toml");
        fs::write(file, "store = \"hello world\"")?;
        let config = ConfigOptions::new()
            .env(false)
            .global(false)
            .local_config_dir(Some(subpath.to_owned()))
            .load()?;
        assert_eq!(config.get_str("store")?, String::from("hello world"));
        Ok(())
    }

    #[test]
    fn env_configs() -> Result<()> {
        let dir = tempdir()?;
        env::set_var("DS_CONFIG_STORE", dir.path().display().to_string());
        let config = ConfigOptions::new().local(false).global(false).load()?;
        env::remove_var("DS_CONFIG_STORE");
        assert_eq!(config.get_str("store")?, dir.path().display().to_string());
        Ok(())
    }

    #[test]
    fn file_config() -> Result<()> {
        let dir = tempdir()?;
        let file = dir.path().join("dsrc.toml");
        fs::write(&file, "store = \"hello world\"")?;
        let config = ConfigOptions::new()
            .local(false)
            .env(false)
            .global_config_file(Some(file.to_owned()))
            .load()?;
        assert_eq!(config.get_str("store")?, String::from("hello world"));
        Ok(())
    }

    #[test]
    fn missing_config() -> Result<()> {
        let config = ConfigOptions::new()
            .local(false)
            .global(false)
            .env(false)
            .load()?;
        assert!(config.get_str("store").is_err());
        Ok(())
    }
}
