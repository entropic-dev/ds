use clap::{crate_version, App, Arg, SubCommand};

pub fn parse_args<'a, 'b>() -> App<'a, 'b> {
    App::new("dstopic")
        .version(crate_version!())
        .author("Kat Marchán <kzm@zkat.tech>")
        .about("Manages your Entropic packages.")
        //
        // $ dstopic serve
        //
        .subcommand(SubCommand::with_name("serve").about("Starts up a dstopic server"))
        //
        // $ dstopic resolve FILE [--cwd PATH]
        //
        .subcommand(
            SubCommand::with_name("resolve")
                .about("Resolves a single filepath")
                .arg(
                    Arg::with_name("FILE")
                        .help("File path to resolve")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("cwd")
                        .help("Working directory to resolve from")
                        .takes_value(true),
                ),
        )
}
