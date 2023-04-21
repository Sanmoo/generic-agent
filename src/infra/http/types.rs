use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct PostCommandResponse {
    pub id: u32,
}

#[derive(Deserialize)]
pub struct PostCommandRequest {
    pub args: String
}

#[derive(Serialize)]
pub struct GetCommandResponse {
    pub id: u32,
    pub created_at: String,
    pub started_at: Option<String>,
    pub finished_at: Option<String>,
    pub command: String,
    pub status: Option<u32>,
    pub stdout: Option<String>,
    pub stderr: Option<String>
}