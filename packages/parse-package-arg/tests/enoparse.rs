use parse_package_arg::PackageArg;

#[test]
fn from_string_bad_arg() {
    let res = PackageArg::from_string("foo");
    assert!(res
        .err()
        .unwrap()
        .to_string()
        .contains("Invalid registry arg string: `foo`"));
}

#[test]
fn from_string_tag_no_host() {
    let res = PackageArg::from_string("foo/bar");
    assert!(res.is_err());
}

#[test]
fn from_string_alias_no_host() {
    let res = PackageArg::from_string("hi@pkg:hey/there@^1.2.3");
    assert!(res.is_err())
}
