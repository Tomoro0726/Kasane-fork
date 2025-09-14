#![cfg(feature = "full")]
use actix_web::{App, HttpResponse, HttpServer, Responder, post, web};
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::{mpsc, oneshot};

#[derive(Clone)]
struct JobSender {
    tx: mpsc::Sender<Job>,
}

struct Job {
    cmd: crate::json::input::Command,
    storage: Arc<Storage>,
    resp: oneshot::Sender<Result<Output, Error>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let storage = Arc::new(Storage::new(None).unwrap());
    let (tx, rx) = mpsc::channel::<Job>(1000);
    let job_sender = JobSender { tx };

    let rx = Arc::new(tokio::sync::Mutex::new(rx));
    let cpu_cores = num_cpus::get();

    for _ in 0..cpu_cores {
        let rx = Arc::clone(&rx);
        tokio::spawn(async move {
            loop {
                let job_opt = {
                    let mut guard = rx.lock().await;
                    guard.recv().await
                };
                if let Some(job) = job_opt {
                    let storage = job.storage.clone();
                    let cmd = job.cmd.clone();

                    let resp = tokio::task::spawn_blocking(move || process(cmd, storage)).await;

                    let _ = match resp {
                        Ok(r) => job.resp.send(r),
                        Err(_) => job.resp.send(Err(Error::QueueReceiveError {
                            location: "spawn_blocking",
                        })),
                    };
                } else {
                    break;
                }
            }
        });
    }

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(job_sender.clone()))
            .app_data(web::Data::new(storage.clone()))
            .service(execute_json)
    })
    .workers(cpu_cores)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[post("/execute")]
async fn execute_json(
    payload: web::Json<Value>,
    job_sender: web::Data<JobSender>,
    storage: web::Data<Arc<Storage>>,
) -> impl Responder {
    let packet = match parser(&payload) {
        Ok(p) => p,
        Err(e) => return HttpResponse::BadRequest().json(vec![e.to_string()]),
    };

    let mut results = Vec::new();

    for cmd in packet.command {
        let (resp_tx, resp_rx) = oneshot::channel();
        let job = Job {
            cmd,
            storage: storage.get_ref().clone(),
            resp: resp_tx,
        };

        if let Err(_) = job_sender.tx.send(job).await {
            results.push(Err(Error::QueueSendError {
                location: "execute_json",
            }));
            continue;
        }

        // 順序を保持して逐次 await
        match resp_rx.await {
            Ok(res) => results.push(res),
            Err(_) => results.push(Err(Error::QueueReceiveError {
                location: "execute_json",
            })),
        }
    }

    HttpResponse::Ok().json(results)
}
