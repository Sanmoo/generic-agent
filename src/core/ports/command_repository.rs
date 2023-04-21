use super::command::Command;

pub trait CommandRepository {
    fn store_new_command(&self, command: &str) -> Result<u32, String>;
    fn fetch_command(&self, command_id: u32) -> Result<Command, String>;
}