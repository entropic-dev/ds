use semver::{Version, VersionReq};
use url::Url;

use parse_package_arg::PackageArg;

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

#[test]
fn from_string_bad_arg() {
    let res = PackageArg::from_string("foo");
    assert_eq!(res.is_err(), true);
}
