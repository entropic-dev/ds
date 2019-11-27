use std::env;
use std::path::{Path, PathBuf};

use config::{Config, ConfigError, Environment, File};
use directories::ProjectDirs;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(default)]
pub struct DsConfig {
    store: Option<PathBuf>,
}

impl Default for DsConfig {
    fn default() -> Self {
        DsConfig {
            store: Self::dirs().map(|d| d.data_dir().join("v1")),
        }
    }
}

impl DsConfig {
    pub fn new_at<P: AsRef<Path>>(working_dir: P) -> Result<Self, ConfigError> {
        Self::new_priv(
            Some(working_dir.as_ref().to_owned()),
            Self::dirs().map(|d| d.config_dir().to_owned()),
        )
    }

    pub fn new() -> Result<Self, ConfigError> {
        Self::new_priv(
            env::current_dir().ok(),
            Self::dirs().map(|d| d.config_dir().to_owned()),
        )
    }

    fn new_priv(
        working_dir: Option<PathBuf>,
        config_dir: Option<PathBuf>,
    ) -> Result<Self, ConfigError> {
        let mut c = Config::new();
        if let Some(config_dir) = config_dir {
            let path = config_dir.join("ds").display().to_string();
            c.merge(File::with_name(&path[..]).required(false))?;
        }
        if let Some(dir) = working_dir {
            for path in dir.ancestors().collect::<Vec<_>>().iter().rev() {
                let path = path.join("ds").display().to_string();
                c.merge(File::with_name(&path[..]).required(false))?;
            }
        }
        c.merge(Environment::with_prefix("ds_config"))?;
        c.try_into()
    }

    fn dirs() -> Option<ProjectDirs> {
        ProjectDirs::from("dev", "entropic", "ds")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[test]
    fn working_dir_config() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("ds.toml");
        fs::write(file, "store = \"hello world\"").unwrap();
        let config =
            DsConfig::new_priv(Some(dir.path().to_owned()), None).expect("config failed to load?");
        assert_eq!(
            config,
            DsConfig {
                store: Some(PathBuf::from("hello world")),
                ..DsConfig::default()
            }
        )
    }

    #[test]
    fn parent_dir_config() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("ds.toml");
        fs::write(file, "store = \"hello world\"").unwrap();
        let subpath = dir.path().join("foo").join("bar");
        fs::create_dir_all(&subpath).unwrap();
        let config =
            DsConfig::new_priv(Some(subpath.to_owned()), None).expect("config failed to load?");
        assert_eq!(
            config,
            DsConfig {
                store: Some(PathBuf::from("hello world")),
                ..DsConfig::default()
            }
        )
    }

    #[test]
    fn working_dir_shadowing_config() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("ds.toml");
        fs::write(file, "store = \"goodbye world\"").unwrap();
        let subpath = dir.path().join("foo").join("bar");
        fs::create_dir_all(&subpath).unwrap();
        let file = dir.path().join("foo").join("ds.toml");
        fs::write(file, "store = \"hello world\"").unwrap();
        let config =
            DsConfig::new_priv(Some(subpath.to_owned()), None).expect("config failed to load?");
        assert_eq!(
            config,
            DsConfig {
                store: Some(PathBuf::from("hello world")),
                ..DsConfig::default()
            }
        )
    }

    #[test]
    fn env_configs() {
        let dir = tempdir().unwrap();
        env::set_var("DS_CONFIG_STORE", dir.path().display().to_string());
        let config = DsConfig::new_priv(None, None).expect("config failed to load?");
        env::remove_var("DS_CONFIG_STORE");
        assert_eq!(
            config,
            DsConfig {
                store: Some(dir.path().to_owned()),
                ..DsConfig::default()
            }
        )
    }

    #[test]
    fn file_config() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("ds.toml");
        fs::write(file, "store = \"hello world\"").unwrap();
        let config =
            DsConfig::new_priv(None, Some(dir.path().to_owned())).expect("config failed to load?");
        assert_eq!(
            config,
            DsConfig {
                store: Some(PathBuf::from("hello world")),
                ..DsConfig::default()
            }
        )
    }

    #[test]
    fn default_config() {
        let config = DsConfig::new_priv(None, None).expect("config failed to load?");
        assert_eq!(config, DsConfig::default());
        if let Some(store) = config.store {
            assert_eq!(
                store,
                ProjectDirs::from("dev", "entropic", "ds")
                    .unwrap()
                    .data_dir()
                    .join("v1")
            );
        }
    }
}
