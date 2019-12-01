use anyhow::Result;
use async_std::task;

use ds::Ds;

fn main() -> Result<()> {
    task::block_on(Ds::load())
}
