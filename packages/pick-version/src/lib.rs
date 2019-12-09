use anyhow::{Context, Result};
use ds_api_types::EntropicPackument;
use parse_package_arg::PackageArg;
use semver::{Version, VersionReq};
use ssri::Integrity;
use thiserror::Error;

pub struct Picker {
    pub default_tag: String,
}

impl Default for Picker {
    fn default() -> Self {
        Picker {
            default_tag: "latest".into(),
        }
    }
}

#[derive(Debug, Error)]
pub enum PickerError {
    #[error("No matching version.")]
    NoVersion,
    #[error("Only Version, Tag, Range, and Alias package args are supported.")]
    InvalidPackageArg,
}

impl Picker {
    pub fn new() -> Self {
        Picker::default()
    }

    pub fn default_tag(mut self, tag: String) -> Self {
        self.default_tag = tag;
        self
    }

    pub fn pick(&self, packument: &EntropicPackument, wanted: &PackageArg) -> Result<Integrity> {
        if packument.versions.len() == 0 {
            Err(PickerError::NoVersion).with_context(|| {
                format!(
                    "No versions were present in the packument for {}",
                    packument.name
                )
            })?;
        }
        match wanted {
            PackageArg::Alias { package, .. } => return self.pick(&packument, &package),
            _ => (),
        }

        let mut target: Option<Version> = match wanted {
            PackageArg::Version { version, .. } => Some(version.clone()),
            PackageArg::Tag { tag, .. } => packument.tags.get(tag.as_str()).map(|v| v.clone()),
            PackageArg::Range { .. } => None,
            _ => Err(PickerError::InvalidPackageArg)
                .with_context(|| format!("Received an unexpected PackageArg type: {:?}", wanted))?,
        };

        let tag_version = packument.tags.get(&self.default_tag).map(|v| v.clone());

        if target.is_none()
            && tag_version.is_some()
            && packument
                .versions
                .get(&tag_version.clone().unwrap())
                .is_some()
            && match wanted {
                PackageArg::Range { range, .. } => range.matches(&tag_version.clone().unwrap()),
                _ => false,
            }
        {
            target = tag_version.clone();
        }

        if target.is_none() {
            match wanted {
                PackageArg::Range { range, .. } => {
                    target = max_satisfying(packument.versions.keys(), range)
                }
                _ => (),
            }
        }

        if target.is_none() {
            match wanted {
                PackageArg::Range { range, .. } => {
                    if range == &VersionReq::any() || range == &VersionReq::parse("*").unwrap() {
                        target = tag_version;
                    }
                }
                _ => (),
            }
        }

        target
            .and_then(|v| packument.versions.get(&v))
            .map(|i| i.clone())
            .ok_or_else(|| PickerError::NoVersion)
            .with_context(|| {
                format!(
                    "No versions could be found because {:?} did not match any existing versions",
                    wanted
                )
            })
    }
}

fn max_satisfying<'a>(
    versions: impl Iterator<Item = &'a Version>,
    range: &VersionReq,
) -> Option<Version> {
    versions
        .filter(|v| range.matches(v))
        .max()
        .map(|v| v.clone())
}
