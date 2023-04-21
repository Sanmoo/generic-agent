pub struct Command {
    pub id: u32,
    pub created_at: String,
    pub started_at: Option<String>,
    pub finished_at: Option<String>,
    pub command: String,
    pub status: Option<u32>,
    pub stdout: Option<String>,
    pub stderr: Option<String>
}