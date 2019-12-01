use anyhow::Result;
use async_std;

use ds::Ds;

#[async_std::main]
async fn main() -> Result<()> {
    Ds::load().await
}
