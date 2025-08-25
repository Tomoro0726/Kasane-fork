#![cfg(feature = "wasm")]

use serde::Serialize;
use wasm_bindgen::prelude::*;

use crate::{
    command::process,
    io::memory::Storage,
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

#[wasm_bindgen]
pub struct Kasane {
    storage: Storage,
}
#[wasm_bindgen]
impl Kasane {
    /// Rust側の純粋な new（init後に呼ばれる）
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<Kasane, JsValue> {
        Storage::new()
            .map(|s| Kasane { storage: s })
            .map_err(|e| JsValue::from_str(&format!("Storage init error: {}", e)))
    }

    /// 実行メソッド（複数コマンド対応）
    #[wasm_bindgen]
    pub fn execute(&mut self, command_json: &str) -> String {
        // まず serde_json::from_str で JSON をパース
        let value: serde_json::Value = match serde_json::from_str(command_json) {
            Ok(v) => v,
            Err(e) => {
                return to_json(vec![CommandResult::Error(format!(
                    "JSON parse error: {}",
                    e
                ))]);
            }
        };

        // parser でさらに独自の構文解析
        let packet = match parser(&value) {
            Ok(p) => p,
            Err(e) => {
                return to_json(vec![CommandResult::Error(format!(
                    "Command parse error: {}",
                    e
                ))]);
            }
        };

        // 複数コマンドの処理
        let results: Vec<CommandResult> = packet
            .command
            .into_iter()
            .map(|cmd| match process(cmd, &mut self.storage) {
                Ok(output) => CommandResult::Success(output),
                Err(e) => CommandResult::Error(e.to_string()),
            })
            .collect();

        to_json(results)
    }
}

fn to_json(result: Vec<CommandResult>) -> String {
    serde_json::to_string_pretty(&result)
        .unwrap_or_else(|e| format!("{{\"error\": \"Serialize error: {}\"}}", e))
}
