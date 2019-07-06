use dstopic;
use std::fs;

fn main() {
    dstopic::parse_args::parse_args().get_matches();
    println!("{}", fs::read_to_string("./Cargo.toml").unwrap().len());
}
