use std::sync::Arc;

use actix_web::web;

use crate::core::ports::{command_repository::CommandRepository, command_scheduler::CommandScheduler};

pub struct CreateCommand {
    pub repository: Arc<dyn CommandRepository + Send + Sync>,
    pub scheduler: Arc<dyn CommandScheduler + Send + Sync>,
    pub bin_path: String,
}

impl CreateCommand {
    pub fn new(
        repository: Arc<dyn CommandRepository + Send + Sync>,
        scheduler: Arc<dyn CommandScheduler + Send + Sync>,
        bin_path: String,
    ) -> Self { Self { repository, scheduler, bin_path } }

    pub async fn invoke(&self, args: &str) -> Result<u32, String> {
        let command_str = format!("{} {}", self.bin_path, args);
        let repository = self.repository.clone();
        let command_id: u32 = web::block(move || {
            repository.store_new_command(&command_str)
        }).await.map_err(|e|e.to_string())??;
        self.scheduler.schedule(self.bin_path.as_str(), args, command_id).await?;
        Ok(command_id)
    }
}