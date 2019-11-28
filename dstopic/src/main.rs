use anyhow::Result;

use dstopic::Dstopic;
use dstopic_command::Command;

fn main() -> Result<()> {
    Dstopic::load().execute()
}
