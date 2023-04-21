use crate::core::ports::command_scheduler::CommandScheduler;
use async_trait::async_trait;
use tokio::sync::mpsc::Sender;

use super::worker::ProcessInBackgroundCommand;

pub struct TokioCommandScheduler {
    pub tx: Sender<ProcessInBackgroundCommand>
}

#[async_trait]
impl CommandScheduler for TokioCommandScheduler {
    async fn schedule(&self, command: &str, args: &str, command_id: u32) -> Result<(), String> {
        self.tx.clone().send(ProcessInBackgroundCommand {
            command_id, command: command.to_string(), args: args.to_string()
        }).await.map_err(|e| e.to_string())
    }
}