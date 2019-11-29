use std::path::PathBuf;
use std::str::FromStr;

use anyhow::{self, Context, Result};
use lazy_static::lazy_static;
use regex::Regex;
use semver::{Version, VersionReq};
use thiserror::Error;
use url::{Host, Url};

#[derive(Debug, Error)]
pub enum PackageArgError {
    #[error("Failed to parse package arg.")]
    ParseError,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackageArg {
    Dir {
        name: Option<String>,
        path: PathBuf,
    },
    Alias {
        name: String,
        package: Box<PackageArg>,
    },
    Tag {
        host: Option<Host>,
        name: String,
        tag: String,
    },
    Version {
        host: Option<Host>,
        name: String,
        version: Version,
    },
    Range {
        host: Option<Host>,
        name: String,
        range: VersionReq,
    },
}

lazy_static! {
    static ref SPLITTER: Regex =
        Regex::new(r"(?i)^(?P<name>(?:[^@]+/legacy/@[^@]+|[^@]+))(?:@(?P<spec>.*))?$").unwrap();
    static ref PKG: Regex = Regex::new(r"^(?P<host>[^/]+/)(?P<name>[^/]+/[^/]+)$").unwrap();
    static ref LEGACY: Regex =
        Regex::new(r"(?i)^(?P<host>[^/]+/)(?P<name>legacy/[^/]+(?:/[^/]+)?)$").unwrap();
    static ref IS_FILE: Regex = Regex::new(r"(?i)^(?:(?:[a-z]:)?[/\\]|\.[/\\]|file:)").unwrap();
}

impl PackageArg {
    pub fn from_string<S: AsRef<str>>(s: S) -> Result<PackageArg> {
        let s: String = s.as_ref().into();
        let matches = SPLITTER
            .captures(&s)
            .ok_or_else(|| PackageArgError::ParseError)
            .with_context(|| format!("Invalid package arg: {}", s))?;
        let name = matches.name("name").map(|name| name.as_str().to_owned());
        let spec = matches.name("spec").map(|name| name.as_str().to_owned());
        if let Some(name) = name {
            if IS_FILE.is_match(&name) {
                Self::resolve(None, Some(name))
            } else {
                Self::resolve(Some(name), spec)
            }
        } else {
            Self::resolve(name, spec)
        }
    }

    pub fn resolve(name: Option<String>, spec: Option<String>) -> Result<PackageArg> {
        if let Some(spec) = spec {
            if IS_FILE.is_match(&spec) {
                from_dir(name, spec)
            } else if name.is_none() {
                Err(PackageArgError::ParseError).with_context(|| {
                    format!(
                        "Tried to resolve a registry spec ({}) without a name.",
                        spec
                    )
                })?
            } else if spec.starts_with("pkg:") {
                from_alias(name.unwrap(), spec)
            } else {
                from_registry(name.unwrap(), Some(spec))
            }
        } else if let Some(name) = name {
            from_registry(name, None)
        } else {
            Err(PackageArgError::ParseError).with_context(|| {
                format!("Neither a name nor a spec were passed in. Failed to resolve.")
            })?
        }
    }

    pub fn validate_name<S: AsRef<str>>(name: S) -> Result<String> {
        let name = name.as_ref();
        Ok(name.into())
    }

    pub fn is_registry(&self) -> bool {
        match self {
            PackageArg::Alias { package, .. } => package.is_registry(),
            PackageArg::Dir { .. } => false,
            PackageArg::Tag { .. } | PackageArg::Version { .. } | PackageArg::Range { .. } => true,
        }
    }
}

impl FromStr for PackageArg {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        PackageArg::from_string(s)
    }
}

fn from_dir(name: Option<String>, spec: String) -> Result<PackageArg> {
    Ok(PackageArg::Dir {
        name,
        path: PathBuf::from(spec),
    })
}

fn from_alias(name: String, spec: String) -> Result<PackageArg> {
    Ok(PackageArg::Alias {
        name,
        package: Box::new(PackageArg::from_string(&spec[4..])?),
    })
}

fn from_registry(name: String, mut spec: Option<String>) -> Result<PackageArg> {
    let caps = PKG
        .captures(&name)
        .or_else(|| LEGACY.captures(&name))
        .ok_or_else(|| PackageArgError::ParseError)
        .with_context(|| format!("Invalid registry arg string: {}", name))?;
    let host = caps.name("host").and_then(|host| {
        let mut string = String::from("https://");
        string.push_str(host.as_str());
        Url::parse(&string)
            .ok()
            .and_then(|x| x.host().map(|x| x.to_owned()))
    });
    let clean_name = caps
        .name("name")
        .map(|x| x.as_str().to_owned())
        .ok_or_else(|| PackageArgError::ParseError)
        .with_context(|| format!("No package name found in registry arg: {}", name))?;
    if spec.is_none() {
        spec = Some("latest".into());
    }
    let spec_str = spec.unwrap();

    let maybe_semver = Version::parse(&spec_str[..]);
    if maybe_semver.is_ok() {
        return Ok(PackageArg::Version {
            name: clean_name,
            version: maybe_semver.unwrap(),
            host,
        });
    }

    let maybe_range = VersionReq::parse(&spec_str[..]);
    if maybe_range.is_ok() {
        return Ok(PackageArg::Range {
            name: clean_name,
            range: maybe_range.unwrap(),
            host,
        });
    }

    Ok(PackageArg::Tag {
        name: clean_name,
        tag: spec_str.to_owned(),
        host,
    })
}

#[cfg(test)]
mod tests {}
