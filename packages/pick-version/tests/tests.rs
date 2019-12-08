use maplit::hashmap;
use parse_package_arg::PackageArg;
use semver::{Version, VersionReq};
use ssri::Integrity;

use pick_version::{Packument, Picker};

#[test]
fn basic_carat_range() {
    let packument = Packument {
        versions: hashmap! {
            Version::parse("1.0.0").unwrap() => "sha256-100".parse().unwrap(),
            Version::parse("1.0.1").unwrap() => "sha256-101".parse().unwrap(),
            Version::parse("1.0.2").unwrap() => "sha256-102".parse().unwrap(),
            Version::parse("2.0.0").unwrap() => "sha256-200".parse().unwrap(),
        },
        ..Packument::default()
    };
    let sri = Picker::new()
        .pick(
            &packument,
            &PackageArg::Range {
                host: None,
                name: "foo".into(),
                range: VersionReq::parse("^1.0.0").unwrap(),
            },
        )
        .unwrap();
    assert_eq!(sri, "sha256-102".parse::<Integrity>().unwrap());
}

#[test]
fn basic_tilde_range() {
    let packument = Packument {
        versions: hashmap! {
            Version::parse("1.0.0").unwrap() => "sha256-100".parse().unwrap(),
            Version::parse("1.0.1").unwrap() => "sha256-101".parse().unwrap(),
            Version::parse("1.0.2").unwrap() => "sha256-102".parse().unwrap(),
            Version::parse("2.0.0").unwrap() => "sha256-200".parse().unwrap(),
        },
        ..Packument::default()
    };
    let sri = Picker::new()
        .pick(
            &packument,
            &PackageArg::Range {
                host: None,
                name: "foo".into(),
                range: VersionReq::parse("~1.0.0").unwrap(),
            },
        )
        .unwrap();
    assert_eq!(sri, "sha256-102".parse::<Integrity>().unwrap());
}

#[test]
fn basic_math_range() {
    let packument = Packument {
        versions: hashmap! {
            Version::parse("1.0.0").unwrap() => "sha256-100".parse().unwrap(),
            Version::parse("1.0.1").unwrap() => "sha256-101".parse().unwrap(),
            Version::parse("1.0.2").unwrap() => "sha256-102".parse().unwrap(),
            Version::parse("2.0.0").unwrap() => "sha256-200".parse().unwrap(),
        },
        ..Packument::default()
    };
    let sri = Picker::new()
        .pick(
            &packument,
            &PackageArg::Range {
                host: None,
                name: "foo".into(),
                range: VersionReq::parse("<2.0.0").unwrap(),
            },
        )
        .unwrap();
    assert_eq!(sri, "sha256-102".parse::<Integrity>().unwrap());
}

#[test]
fn basic_version_match() {
    let packument = Packument {
        versions: hashmap! {
            Version::parse("1.0.0").unwrap() => "sha256-100".parse().unwrap(),
            Version::parse("1.0.1").unwrap() => "sha256-101".parse().unwrap(),
            Version::parse("1.0.2").unwrap() => "sha256-102".parse().unwrap(),
            Version::parse("2.0.0").unwrap() => "sha256-200".parse().unwrap(),
        },
        ..Packument::default()
    };
    let sri = Picker::new()
        .pick(
            &packument,
            &PackageArg::Version {
                host: None,
                name: "foo".into(),
                version: Version::parse("1.0.1").unwrap(),
            },
        )
        .unwrap();
    assert_eq!(sri, "sha256-101".parse::<Integrity>().unwrap());
}

#[test]
fn basic_tag_match() {
    let packument = Packument {
        tags: hashmap! {
            "latest".into() => Version::parse("1.0.1").unwrap()
        },
        versions: hashmap! {
            Version::parse("1.0.0").unwrap() => "sha256-100".parse().unwrap(),
            Version::parse("1.0.1").unwrap() => "sha256-101".parse().unwrap(),
            Version::parse("1.0.2").unwrap() => "sha256-102".parse().unwrap(),
            Version::parse("2.0.0").unwrap() => "sha256-200".parse().unwrap(),
        },
        ..Packument::default()
    };
    let sri = Picker::new()
        .pick(
            &packument,
            &PackageArg::Tag {
                host: None,
                name: "foo".into(),
                tag: "latest".into(),
            },
        )
        .unwrap();
    assert_eq!(sri, "sha256-101".parse::<Integrity>().unwrap());
}

#[test]
fn tag_match_with_custom_default_tag() {
    let packument = Packument {
        tags: hashmap! {
            "something".into() => Version::parse("1.0.1").unwrap(),
            "latest".into() => Version::parse("1.0.2").unwrap()
        },
        versions: hashmap! {
            Version::parse("1.0.0").unwrap() => "sha256-100".parse().unwrap(),
            Version::parse("1.0.1").unwrap() => "sha256-101".parse().unwrap(),
            Version::parse("1.0.2").unwrap() => "sha256-102".parse().unwrap(),
            Version::parse("2.0.0").unwrap() => "sha256-200".parse().unwrap(),
        },
        ..Packument::default()
    };
    let sri = Picker::new()
        .default_tag("something".into())
        .pick(
            &packument,
            &PackageArg::Tag {
                host: None,
                name: "foo".into(),
                tag: "something".into(),
            },
        )
        .unwrap();
    assert_eq!(sri, "sha256-101".parse::<Integrity>().unwrap());
}

#[test]
fn star_range_uses_default_tag() {
    let packument = Packument {
        tags: hashmap! {
            "latest".into() => Version::parse("1.0.0-pre.0").unwrap(),
            "beta".into() => Version::parse("2.0.0-beta.0").unwrap(),
        },
        versions: hashmap! {
            Version::parse("1.0.0-pre.0").unwrap() => "sha256-100b0".parse().unwrap(),
            Version::parse("1.0.0-pre.1").unwrap() => "sha256-100b1".parse().unwrap(),
            Version::parse("2.0.0-beta.0").unwrap() => "sha256-200b0".parse().unwrap(),
            Version::parse("2.0.0-beta.1").unwrap() => "sha256-200b1".parse().unwrap(),
        },
        ..Packument::default()
    };
    let sri = Picker::new()
        .default_tag("beta".into())
        .pick(
            &packument,
            &PackageArg::Range {
                host: None,
                name: "foo".into(),
                range: VersionReq::any(),
            },
        )
        .unwrap();
    assert_eq!(sri, "sha256-200b0".parse::<Integrity>().unwrap());
    let sri = Picker::new()
        .pick(
            &packument,
            &PackageArg::Range {
                host: None,
                name: "foo".into(),
                range: VersionReq::parse("*").unwrap(),
            },
        )
        .unwrap();
    assert_eq!(sri, "sha256-100b0".parse::<Integrity>().unwrap());
}

#[test]
fn error_if_no_match() {
    let packument = Packument {
        versions: hashmap! {
            Version::parse("1.0.0").unwrap() => "sha256-100".parse().unwrap(),
            Version::parse("1.0.1").unwrap() => "sha256-101".parse().unwrap(),
            Version::parse("1.0.2").unwrap() => "sha256-102".parse().unwrap(),
            Version::parse("2.0.0").unwrap() => "sha256-200".parse().unwrap(),
        },
        ..Packument::default()
    };
    let sri = Picker::new().pick(
        &packument,
        &PackageArg::Range {
            host: None,
            name: "foo".into(),
            range: VersionReq::parse("^2.0.1").unwrap(),
        },
    );
    assert!(sri.is_err());
}

#[test]
fn error_if_no_versions() {
    let packument = Packument::default();
    let sri = Picker::new().pick(
        &packument,
        &PackageArg::Range {
            host: None,
            name: "foo".into(),
            range: VersionReq::parse("^1.0.0").unwrap(),
        },
    );
    assert!(sri.is_err());
}
