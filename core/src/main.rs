use actix_web::{App, HttpResponse, HttpServer, Responder, post, web};
use serde::Serialize;
use serde_json::Value;

use crate::{
    command::process,
    io::Storage,
    json::{input::parser, output::Output},
};

pub mod command;
pub mod error;
pub mod io;
pub mod json;

#[derive(Serialize)]
enum CommandResult {
    Success(Output),
    Error(String),
}

#[post("/execute")]
async fn execute_json(
    payload: web::Json<Value>,
    storage: web::Data<std::sync::Mutex<Storage>>,
) -> impl Responder {
    let packet = match parser(&payload) {
        Ok(p) => p,
        Err(e) => {
            return HttpResponse::BadRequest().json(vec![CommandResult::Error(e.to_string())]);
        }
    };

    // コマンド実行
    let mut s = storage.lock().unwrap(); // Mutexをロック
    let results: Vec<CommandResult> = packet
        .command
        .into_iter()
        .map(|cmd| match process(cmd, &mut s) {
            Ok(output) => CommandResult::Success(output),
            Err(e) => CommandResult::Error(e.to_string()),
        })
        .collect();

    HttpResponse::Ok().json(results)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let storage = Storage::new().unwrap();
    let storage_data = web::Data::new(std::sync::Mutex::new(storage)); // 共有データとして登録

    HttpServer::new(move || {
        App::new()
            .app_data(storage_data.clone()) // ハンドラーで使えるように渡す
            .service(execute_json)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
