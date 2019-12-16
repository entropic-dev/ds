use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use anyhow::Result;

fn main() -> Result<()> {
    let out_dir = env::var("OUT_DIR")?;
    fs::create_dir_all(&out_dir)?;
    let dest_path = Path::new(&out_dir).join("dssh");
    let mut f = File::create(&dest_path)?;

    f.write_all(
        b"#!/bin/sh
./ds sh -- $@
",
    )?;
    Ok(())
}
