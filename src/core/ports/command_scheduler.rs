use async_trait::async_trait;

#[async_trait]
pub trait CommandScheduler {
    async fn schedule(
        &self,
        bin_path: &str,
        args: &str,
        command_id: u32
    ) -> Result<(), String>;
}