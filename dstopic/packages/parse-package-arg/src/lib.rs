use anyhow::{Context, Result};
use lazy_static::lazy_static;
use regex::Regex;
use semver::{Version, VersionReq};
use url::{Host, Url};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackageArg {
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
    static ref RE: Regex = Regex::new(r"^(?P<host>[^/]+)/(?P<name>.*/.*)$").unwrap();
}

impl PackageArg {
    pub fn from_string<S: AsRef<str>>(s: S) -> Result<PackageArg> {
        let s: String = s.as_ref().into();
        let split = s.splitn(2, "@").collect::<Vec<&str>>();
        let name: String;
        let spec: Option<String>;
        if split.len() == 2 {
            name = split[0].into();
            spec = Some(split[1].into());
        } else if split.len() == 1 {
            name = split[0].into();
            spec = None
        } else {
            unreachable!()
        }
        Self::resolve(name, spec)
    }

    pub fn resolve(name: String, spec: Option<String>) -> Result<PackageArg> {
        if let Some(spec) = spec {
            if spec.starts_with("pkg:") {
                from_alias(name, spec)
            } else {
                from_registry(name, Some(spec))
            }
        } else {
            from_registry(name, None)
        }
    }

    pub fn validate_name<S: AsRef<str>>(name: S) -> Result<String> {
        let name = name.as_ref();
        Ok(name.into())
    }

    pub fn is_registry(&self) -> bool {
        match self {
            PackageArg::Alias { package, .. } => package.is_registry(),
            PackageArg::Tag { .. } | PackageArg::Version { .. } | PackageArg::Range { .. } => true,
        }
    }
}

fn from_alias(name: String, spec: String) -> Result<PackageArg> {
    Ok(PackageArg::Alias {
        name,
        package: Box::new(PackageArg::from_string(&spec[4..])?),
    })
}

fn from_registry(name: String, mut spec: Option<String>) -> Result<PackageArg> {
    let caps = RE.captures(&name).unwrap_or_else(|| panic!("ENOPARSE"));
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
        .unwrap_or_else(|| panic!("ENOPARSE"));
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
mod tests {
    use super::*;

    #[test]
    fn from_string_tag() {
        let res = PackageArg::from_string("example.com/hey/there@next").unwrap();
        assert_eq!(
            res,
            PackageArg::Tag {
                name: "hey/there".into(),
                tag: "next".into(),
                host: Url::parse("https://example.com")
                    .unwrap()
                    .host()
                    .map(|x| x.to_owned()),
            }
        )
    }

    #[test]
    fn from_string_version() {
        let res = PackageArg::from_string("example.com/hey/there@1.2.3").unwrap();
        assert_eq!(
            res,
            PackageArg::Version {
                name: "hey/there".into(),
                version: Version::parse("1.2.3").unwrap(),
                host: Url::parse("https://example.com")
                    .unwrap()
                    .host()
                    .map(|x| x.to_owned()),
            }
        )
    }

    #[test]
    fn from_string_range() {
        let res = PackageArg::from_string("example.com/hey/there@^1.2.3").unwrap();
        assert_eq!(
            res,
            PackageArg::Range {
                name: "hey/there".into(),
                range: VersionReq::parse("^1.2.3").unwrap(),
                host: Url::parse("https://example.com")
                    .unwrap()
                    .host()
                    .map(|x| x.to_owned()),
            }
        )
    }

    #[test]
    fn from_string_alias() {
        let res = PackageArg::from_string("hi@pkg:example.com/hey/there@^1.2.3").unwrap();
        assert_eq!(
            res,
            PackageArg::Alias {
                name: "hi".into(),
                package: Box::new(PackageArg::Range {
                    name: "hey/there".into(),
                    range: VersionReq::parse("^1.2.3").unwrap(),
                    host: Url::parse("https://example.com")
                        .unwrap()
                        .host()
                        .map(|x| x.to_owned()),
                })
            }
        )
    }
}