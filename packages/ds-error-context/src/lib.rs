use std::path::PathBuf;

use derive_more::Display;

/// Contextual error codes for a variety of `ds` error messages. These codes
/// have an M:N relationship to actual errors and are intended to provide
/// users with additional context that they can easily look up in the `ds`
/// documentation.
#[derive(Display)]
pub enum DsErrContext {
    /// This error occurs due to a failure to get the current executable (see
    /// [std::env::current_exe](https://doc.rust-lang.org/1.39.0/std/env/fn.current_exe.html#)),
    /// and can be for any number of system-related reasons beyond the control
    /// of `ds`.
    #[display(fmt = "DS1000: Failed to get the location of the current ds binary.")]
    DS1000,

    /// `ds shell` tried to execute a given Node.js binary, but the operation
    /// failed for some reason. Is Node installed and available in your $PATH?
    /// Did you pass in an invalid `--node` argument? Are you sure the file is
    /// executable?
    #[display(fmt = "DS1001: Failed to execute node binary at `{}`", _0)]
    DS1001(String),

    #[display(fmt = "DS1002: A home directory is required for ds patch scripts.")]
    DS1002,

    #[display(fmt = "DS1003: Failed to create data directory at `{:?}`", _0)]
    DS1003(PathBuf),

    #[display(fmt = "DS1004: Failed to write dstopic data file at `{:?}`", _0)]
    DS1004(PathBuf),

    /// Invalid Package Arg.
    #[display(fmt = "DS1005: Invalid package arg: `{}`", _0)]
    DS1005(String),

    #[display(
        fmt = "DS1006: Tried to resolve a registry spec ({}) without a name.",
        _0
    )]
    DS1006(String),

    #[display(fmt = "DS1007: Neither a name nor a spec were passed in. Failed to resolve.")]
    DS1007,

    #[display(fmt = "DS1008: Invalid registry arg string: `{}`", _0)]
    DS1008(String),

    #[display(fmt = "DS1009: No package name found in registry arg: `{}`", _0)]
    DS1009(String),

    #[display(fmt = "DS1010: Failed to parse registry URL from `{}`", _0)]
    DS1010(String),

    #[display(fmt = "DS1011: Failed to parse response body")]
    DS1011,

    #[display(fmt = "DS1012: No versions were present in the packument for `{}`", _0)]
    DS1012(String),

    #[display(fmt = "DS1013: Received an unexpected PackageArg type: `{}`", _0)]
    DS1013(String),

    #[display(
        fmt = "DS1014: No versions could be found because `{}` did not match any existing versions",
        _0
    )]
    DS1014(String),

    #[display(fmt = "DS1015: Ping failed for {}: {}", registry, message)]
    DS1015 { registry: String, message: String },

    #[display(fmt = "DS1016: Failed to get response body.")]
    DS1016,

    #[display(fmt = "DS1017: No response from registry at {}", _0)]
    DS1017(String),

    /// Failed to parse a package arg for some reason. The message includes
    /// the actual error.
    #[display(fmt = "DS1018: Package arg `{}` failed to parse:\n{}", input, msg)]
    DS1018 { input: String, msg: String },
}
