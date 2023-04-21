use actix_web::{post, get, web, Result, Responder, error};

use crate::{infra::http::types::{PostCommandRequest, PostCommandResponse}, AppState};

use super::types::GetCommandResponse;

#[post("/commands")]
async fn new_command(req: web::Json<PostCommandRequest>, state: web::Data<AppState>) -> Result<impl Responder> {
    let command_id = state.use_cases.create_command.invoke(&req.args).await.map_err(error::ErrorInternalServerError)?;
    Ok(web::Json(PostCommandResponse { id: command_id }))
}

#[get("/commands/{command_id}")]
async fn get_command(path: web::Path<u32>, state: web::Data<AppState>) -> Result<impl Responder> {
    let command_id = path.into_inner();
    let command = state.use_cases.fetch_command.invoke(command_id).await.map_err(error::ErrorInternalServerError)?;

    Ok(web::Json(GetCommandResponse {
        id: command_id,
        created_at: command.created_at,
        started_at: command.started_at,
        finished_at: command.finished_at,
        command: command.command,
        status: command.status,
        stdout: command.stdout,
        stderr: command.stderr
    }))
}
