pub trait Command {
    fn execute(self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}
