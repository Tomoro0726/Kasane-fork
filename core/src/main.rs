use actix_web::{App, HttpResponse, HttpServer, Responder, post, web};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::{Mutex, mpsc, oneshot};
use uuid::Uuid;

use crate::{
    command::process,
    error::Error,
    io::{StorageTrait, full::Storage},
    json::{
        input::{Packet, parser},
        output::Output,
    },
};
pub mod command;
pub mod error;
pub mod io;
pub mod json;

const MAX_SESSIONS: usize = 100;
const SESSION_TIMEOUT: Duration = Duration::from_secs(3600); // 1時間

#[derive(Clone)]
struct AppState {
    sessions: Arc<Mutex<HashMap<String, Session>>>,
}

#[derive(Clone)]
struct Session {
    username: String,
    created_at: Instant,
    last_access: Instant, // 最後アクセス時刻を追加
}

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    session_id: String,
    expires_in_secs: u64,
}

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

    let app_state = AppState {
        sessions: Arc::new(Mutex::new(HashMap::new())),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(job_sender.clone()))
            .app_data(web::Data::new(storage.clone()))
            .app_data(web::Data::new(app_state.clone()))
            .service(execute_json)
            .service(login)
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
    state: web::Data<AppState>,
) -> impl Responder {
    // JSON を Packet にパース
    let packet: Packet = match parser(&payload) {
        Ok(p) => p,
        Err(e) => return HttpResponse::BadRequest().json(vec![e.to_string()]),
    };

    // セッションチェック
    let mut sessions = state.sessions.lock().await;
    let valid_session = if let Some(session) = sessions.get_mut(&packet.session) {
        // タイムアウト確認
        if session.last_access.elapsed() < SESSION_TIMEOUT {
            session.last_access = Instant::now(); // 最後アクセス時刻を更新して延長
            true
        } else {
            sessions.remove(&packet.session);
            false
        }
    } else {
        false
    };

    if !valid_session {
        return HttpResponse::Unauthorized().body("Invalid or expired session");
    }

    // コマンド処理
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

        match resp_rx.await {
            Ok(res) => results.push(res),
            Err(_) => results.push(Err(Error::QueueReceiveError {
                location: "execute_json",
            })),
        }
    }

    HttpResponse::Ok().json(results)
}

#[post("/login")]
async fn login(
    data: web::Data<AppState>,
    req: web::Json<LoginRequest>,
    storage: web::Data<Arc<Storage>>,
) -> impl Responder {
    // 認証チェック（ここでは StorageTrait の verify_user を使用）
    if !storage
        .get_ref()
        .clone()
        .verify_user(&req.username, &req.password)
        .unwrap_or(false)
    {
        return HttpResponse::Unauthorized().body("Invalid credentials");
    }

    let mut sessions = data.sessions.lock().await;

    // 古いセッションを削除
    sessions.retain(|_, session| session.last_access.elapsed() < SESSION_TIMEOUT);

    if sessions.len() >= MAX_SESSIONS {
        return HttpResponse::TooManyRequests().body("Session limit reached");
    }

    // 新しい session_id 発行
    let session_id = Uuid::new_v4().to_string();
    let now = Instant::now();
    let session = Session {
        username: req.username.clone(),
        created_at: now,
        last_access: now, // 初回アクセス時刻を設定
    };

    sessions.insert(session_id.clone(), session);

    HttpResponse::Ok().json(LoginResponse {
        session_id,
        expires_in_secs: SESSION_TIMEOUT.as_secs(),
    })
}
