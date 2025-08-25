#![cfg(feature = "full")]

use actix_web::{App, HttpResponse, HttpServer, Responder, post, web};
use serde::Serialize;
use serde_json::Value;

use crate::{
    command::process,
    io::kv::Storage,
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let storage = Storage::new(None).unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(storage.clone()))
            .service(execute_json)
    })
    //.workers(num_cpus::get())
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[post("/execute")]
async fn execute_json(payload: web::Json<Value>, storage: web::Data<Storage>) -> impl Responder {
    let packet = match parser(&payload) {
        Ok(p) => p,
        Err(e) => {
            return HttpResponse::BadRequest().json(vec![CommandResult::Error(e.to_string())]);
        }
    };

    let results: Vec<CommandResult> = packet
        .command
        .into_iter()
        .map(|cmd| match process(cmd, &storage) {
            Ok(output) => CommandResult::Success(output),
            Err(e) => CommandResult::Error(e.to_string()),
        })
        .collect();

    HttpResponse::Ok().json(results)
}
