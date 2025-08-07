// use schemars::JsonSchema;
// use serde::Serialize;
// use wasm_bindgen::prelude::*;

// use crate::command::process;
// use crate::interface::output::json_str::json_from_str;
// use crate::io::Storage;
// use crate::output::Output;

// pub mod command;
// pub mod error;
// pub mod interface;
// pub mod io;
// pub mod output;
// use crate::parser::parser;
// pub mod parser;

// use once_cell::sync::Lazy;
// use std::sync::Mutex;

// #[cfg(not(feature = "BuildJsonSchema"))]
// #[derive(Serialize, JsonSchema)]
// enum PacketState {
//     JsonError(String),
//     UserError(String),
//     CommandError(String),
//     Ok(Vec<Output>),
// }

// static STORAGE: Lazy<Mutex<Option<Storage>>> = Lazy::new(|| Mutex::new(None));

// #[wasm_bindgen]
// pub fn run_packet(json_input: &str) -> String {
//     let mut storage_guard = STORAGE.lock().unwrap();

//     // ストレージが未初期化なら初期化
//     if storage_guard.is_none() {
//         match Storage::new() {
//             Ok(s) => *storage_guard = Some(s),
//             Err(_) => return "エラー".to_string(),
//         }
//     }

//     // ストレージへの可変参照を取得
//     let mut s = storage_guard.as_mut().unwrap();

//     let packet_raw = match json_from_str(json_input) {
//         Ok(v) => v,
//         Err(e) => return to_json(PacketState::JsonError(format!("{:?}", e))),
//     };

//     let packet = match parser(&packet_raw.to_string()) {
//         Ok(v) => v,
//         Err(e) => {
//             return to_json(PacketState::CommandError(format!("Parse error: {:?}", e)));
//         }
//     };

//     if packet.user != "default" {
//         return to_json(PacketState::UserError(format!(
//             "Unknown user: {}",
//             packet.user
//         )));
//     }

//     let mut outputs = Vec::new();
//     for cmd in packet.commands {
//         match process(cmd, &mut s) {
//             Ok(output) => outputs.push(output),
//             Err(e) => {
//                 return to_json(PacketState::CommandError(format!("{:?}", e)));
//             }
//         }
//     }

//     to_json(PacketState::Ok(outputs))
// }

// fn to_json(state: PacketState) -> String {
//     serde_json::to_string_pretty(&state)
//         .unwrap_or_else(|e| format!(r#"{{"status":"SerializeError","data":"{}"}}"#, e))
// }
