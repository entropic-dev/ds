use anyhow::Result;
use url::Url;

use ds_package_arg::{PackageArg, VersionReq};

fn ppa(input: &str) -> Result<PackageArg> {
    input.parse()
}

#[test]
fn ent_pkg_basic() -> Result<()> {
    let res = ppa("example.com/hello/world")?;
    assert_eq!(
        res,
        PackageArg::Entropic {
            name: "hello/world".into(),
            requested: None,
            host: Url::parse("https://example.com")?
                .host()
                .map(|x| x.to_owned())
                .unwrap(),
        }
    );
    Ok(())
}

#[test]
fn ent_pkg_prefixed() -> Result<()> {
    let res = ppa("ent:example.com/hello/world")?;
    assert_eq!(
        res,
        PackageArg::Entropic {
            name: "hello/world".into(),
            requested: None,
            host: Url::parse("https://example.com")?
                .host()
                .map(|x| x.to_owned())
                .unwrap(),
        }
    );
    Ok(())
}

#[test]
fn ent_pkg_with_req() -> Result<()> {
    let res = ppa("example.com/hello/world@^1.2.3")?;
    assert_eq!(
        res,
        PackageArg::Entropic {
            name: "hello/world".into(),
            requested: Some(VersionReq::Range(semver::VersionReq::parse("^1.2.3")?)),
            host: Url::parse("https://example.com")?
                .host()
                .map(|x| x.to_owned())
                .unwrap(),
        }
    );
    Ok(())
}
