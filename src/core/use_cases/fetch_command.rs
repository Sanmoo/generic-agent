use std::sync::Arc;

use actix_web::web;

use crate::core::ports::{command_repository::CommandRepository, command::Command};

pub struct FetchCommand {
    pub repository: Arc<dyn CommandRepository + Send + Sync>
}

impl FetchCommand {
    pub fn new(repository: Arc<dyn CommandRepository + Send + Sync>) -> Self { Self { repository } }

    pub async fn invoke(&self, command_id: u32) -> Result<Command, String> {
        let repository = self.repository.clone();
        web::block(move || {
            repository.fetch_command(command_id)
        }).await.map_err(|e|e.to_string())?
    }
}
