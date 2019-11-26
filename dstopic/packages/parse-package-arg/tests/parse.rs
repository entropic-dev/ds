use std::path::PathBuf;

use semver::{Version, VersionReq};
use url::Url;

use parse_package_arg::PackageArg;

#[test]
fn parse_str() {
    let res: PackageArg = "example.com/hey/there@next".parse().unwrap();
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
fn from_string_tag_implicit() {
    let res = PackageArg::from_string("example.com/hey/there").unwrap();
    assert_eq!(
        res,
        PackageArg::Tag {
            name: "hey/there".into(),
            tag: "latest".into(),
            host: Url::parse("https://example.com")
                .unwrap()
                .host()
                .map(|x| x.to_owned()),
        }
    )
}

#[test]
fn from_string_tag_no_long_name() {
    let res = PackageArg::from_string("example.com/hey/you/there@next");
    assert_eq!(res.is_err(), true)
}

#[test]
fn from_string_tag_legacy() {
    let res = PackageArg::from_string("example.com/legacy/there@next").unwrap();
    assert_eq!(
        res,
        PackageArg::Tag {
            name: "legacy/there".into(),
            tag: "next".into(),
            host: Url::parse("https://example.com")
                .unwrap()
                .host()
                .map(|x| x.to_owned()),
        }
    )
}

#[test]
fn from_string_tag_legacy_scoped() {
    let res = PackageArg::from_string("example.com/legacy/@hey/there@next").unwrap();
    assert_eq!(
        res,
        PackageArg::Tag {
            name: "legacy/@hey/there".into(),
            tag: "next".into(),
            host: Url::parse("https://example.com")
                .unwrap()
                .host()
                .map(|x| x.to_owned()),
        }
    )
}

#[test]
fn from_string_implicit_tag_legacy_scoped() {
    let res = PackageArg::from_string("example.com/legacy/@hey/there").unwrap();
    assert_eq!(
        res,
        PackageArg::Tag {
            name: "legacy/@hey/there".into(),
            tag: "latest".into(),
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
fn from_string_dir() {
    let res = PackageArg::from_string("./path/to/dir").unwrap();
    assert_eq!(
        res,
        PackageArg::Dir {
            name: None,
            path: PathBuf::from("./path/to/dir")
        }
    )
}

#[test]
fn from_string_dir_with_name() {
    let res = PackageArg::from_string("dep@./path/to/dir").unwrap();
    assert_eq!(
        res,
        PackageArg::Dir {
            name: Some("dep".into()),
            path: PathBuf::from("./path/to/dir")
        }
    )
}

#[test]
fn from_string_dir_absolute() {
    let res = PackageArg::from_string("/path/to/dir").unwrap();
    assert_eq!(
        res,
        PackageArg::Dir {
            name: None,
            path: PathBuf::from("/path/to/dir")
        }
    )
}

#[test]
fn from_string_dir_with_name_absolute() {
    let res = PackageArg::from_string("dep@/path/to/dir").unwrap();
    assert_eq!(
        res,
        PackageArg::Dir {
            name: Some("dep".into()),
            path: PathBuf::from("/path/to/dir")
        }
    )
}

#[test]
fn from_string_dir_windows() {
    let res = PackageArg::from_string(".\\path\\to\\dir").unwrap();
    assert_eq!(
        res,
        PackageArg::Dir {
            name: None,
            path: PathBuf::from(".\\path\\to\\dir")
        }
    )
}

#[test]
fn from_string_dir_with_name_windows() {
    let res = PackageArg::from_string("dep@.\\path\\to\\dir").unwrap();
    assert_eq!(
        res,
        PackageArg::Dir {
            name: Some("dep".into()),
            path: PathBuf::from(".\\path\\to\\dir")
        }
    )
}

#[test]
fn from_string_dir_absolute_windows() {
    let res = PackageArg::from_string("C:\\path\\to\\dir").unwrap();
    assert_eq!(
        res,
        PackageArg::Dir {
            name: None,
            path: PathBuf::from("C:\\path\\to\\dir")
        }
    )
}

#[test]
fn from_string_dir_with_name_absolute_windows() {
    let res = PackageArg::from_string("dep@C:\\path\\to\\dir").unwrap();
    assert_eq!(
        res,
        PackageArg::Dir {
            name: Some("dep".into()),
            path: PathBuf::from("C:\\path\\to\\dir")
        }
    )
}
