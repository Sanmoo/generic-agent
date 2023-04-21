use crate::core::use_cases::{create_command::CreateCommand, fetch_command::FetchCommand};
use std::fs;
use std::sync::Arc;
use infra::sqlite_command_repository::SqliteCommandRepository;
use infra::tokio_command_scheduler::TokioCommandScheduler;
use infra::{http::commands::{new_command, get_command}, worker::{ProcessInBackgroundCommand, setup_task_manager}};
use r2d2_sqlite::SqliteConnectionManager;
use tokio::sync::mpsc;
use actix_web::{App, HttpServer, web};
use toml::Table;

pub mod infra;
pub mod core;

use crate::infra::db::Pool;

struct UseCases {
    create_command: CreateCommand,
    fetch_command: FetchCommand
}

struct AppState {
    use_cases: UseCases
}

impl AppState {
    fn setup() -> Self {
        let pool = Pool::new(SqliteConnectionManager::file("db.sqlite")).unwrap();
        let (tx, rx) = mpsc::channel::<ProcessInBackgroundCommand>(32);
        let config = fs::read_to_string("config.toml").expect("Could not find config file");
        let config = config.parse::<Table>().unwrap();
        let bin_path = config["processes"]["bin_path"].as_str().unwrap().to_string();
        let command_repository = Arc::new(SqliteCommandRepository { pool: pool.clone() });
        let command_scheduler = TokioCommandScheduler { tx };
    
        setup_task_manager(rx, bin_path.to_string(), pool.clone());

        AppState {
            use_cases: UseCases {
                create_command: CreateCommand {
                    repository: command_repository.clone(),
                    scheduler: Arc::new(command_scheduler),
                    bin_path
                },
                fetch_command: FetchCommand {
                    repository: command_repository.clone(),
                }
            }
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState::setup()))
            .service(new_command)
            .service(get_command)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}