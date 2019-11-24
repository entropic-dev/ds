use parse_package_arg::PackageArg;

#[test]
fn from_string_bad_arg() {
    let res = PackageArg::from_string("foo");
    assert_eq!(res.is_err(), true);
}
