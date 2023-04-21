use tokio::sync::mpsc::Receiver;
use tokio::process::Command;

use super::db::Pool;

#[derive(Debug)]
pub struct ProcessInBackgroundCommand {
    pub command_id: u32,
    pub command: String,
    pub args: String
}

pub fn setup_task_manager(mut rx: Receiver<ProcessInBackgroundCommand>, bin_path: String, db: Pool) {
    tokio::spawn(async move {
        while let Some(arg) = rx.recv().await {
            let mut command = Command::new(bin_path.clone());
            let output = command.arg(arg.args).output().await.unwrap();

            let pool = db.clone();
            let conn = pool.get().unwrap();
            conn.execute(
                "update commands set finished_at = datetime('now'), status = ?, stdout = ?, stderr = ? where id = ?",
                [
                    output.status.code().unwrap().to_string(),
                    String::from_utf8(output.stdout).unwrap(),
                    String::from_utf8(output.stderr).unwrap(),
                    arg.command_id.to_string()
                ]
            ).unwrap();
        }
    });
}
