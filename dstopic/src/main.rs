use dstopic::Dstopic;
use dstopic_command::Command;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    Dstopic::load().execute()
}
