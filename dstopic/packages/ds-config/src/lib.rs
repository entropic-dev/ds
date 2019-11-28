use std::env;
use std::path::{Path, PathBuf};

use config::{Config, ConfigError, Environment, File};
use directories::ProjectDirs;

pub fn new_at<P: AsRef<Path>>(working_dir: P) -> Result<Config, ConfigError> {
    new_priv(
        Some(working_dir.as_ref().to_owned()),
        dirs().map(|d| d.config_dir().to_owned()),
    )
}

pub fn new() -> Result<Config, ConfigError> {
    new_priv(
        env::current_dir().ok(),
        dirs().map(|d| d.config_dir().to_owned()),
    )
}

fn new_priv(
    working_dir: Option<PathBuf>,
    config_dir: Option<PathBuf>,
) -> Result<Config, ConfigError> {
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
    Ok(c)
}

fn dirs() -> Option<ProjectDirs> {
    ProjectDirs::from("dev", "entropic", "ds")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn working_dir_config() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("ds.toml");
        fs::write(file, "store = \"hello world\"").unwrap();
        let config = new_priv(Some(dir.path().to_owned()), None).expect("config failed to load?");
        assert_eq!(
            config.get_str("store").unwrap(),
            String::from("hello world")
        )
    }

    #[test]
    fn parent_dir_config() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("ds.toml");
        fs::write(file, "store = \"hello world\"").unwrap();
        let subpath = dir.path().join("foo").join("bar");
        fs::create_dir_all(&subpath).unwrap();
        let config = new_priv(Some(subpath.to_owned()), None).expect("config failed to load?");
        assert_eq!(
            config.get_str("store").unwrap(),
            String::from("hello world")
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
        let config = new_priv(Some(subpath.to_owned()), None).expect("config failed to load?");
        assert_eq!(
            config.get_str("store").unwrap(),
            String::from("hello world")
        )
    }

    #[test]
    fn env_configs() {
        let dir = tempdir().unwrap();
        env::set_var("DS_CONFIG_STORE", dir.path().display().to_string());
        let config = new_priv(None, None).expect("config failed to load?");
        env::remove_var("DS_CONFIG_STORE");
        assert_eq!(
            config.get_str("store").unwrap(),
            dir.path().display().to_string()
        )
    }

    #[test]
    fn file_config() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("ds.toml");
        fs::write(file, "store = \"hello world\"").unwrap();
        let config = new_priv(None, Some(dir.path().to_owned())).expect("config failed to load?");
        assert_eq!(
            config.get_str("store").unwrap(),
            String::from("hello world")
        )
    }

    #[test]
    fn missing_config() {
        let config = new_priv(None, None).expect("config failed to load?");
        assert!(config.get_str("store").is_err())
    }
}
