use crate::command::process;
use crate::io::Storage;
use crate::output::Output;
use crate::parser::parser;
use serde::Serialize;
use wasm_bindgen::prelude::*;

pub mod command;
pub mod error;
pub mod interface;
pub mod io;
pub mod output;
pub mod parser;

#[derive(Serialize)]
enum CommandResult {
    Success(Output),
    Error(String),
}

#[wasm_bindgen]
pub struct KasaneVM {
    storage: Storage,
}

#[wasm_bindgen]
impl KasaneVM {
    /// Rust側の純粋な new（init後に呼ばれる）
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<KasaneVM, JsValue> {
        Storage::new()
            .map(|s| KasaneVM { storage: s })
            .map_err(|e| JsValue::from_str(&format!("Storage init error: {}", e)))
    }

    /// 実行メソッド
    #[wasm_bindgen]
    pub fn execute(&mut self, command_json: &str) -> String {
        match parser(command_json) {
            Ok(cmd) => match process(cmd, &mut self.storage) {
                Ok(output) => to_json(CommandResult::Success(output)),
                Err(e) => to_json(CommandResult::Error(e.to_string())),
            },
            Err(e) => to_json(CommandResult::Error(format!("Parse error: {}", e))),
        }
    }
}

fn to_json(result: CommandResult) -> String {
    serde_json::to_string_pretty(&result)
        .unwrap_or_else(|e| format!("{{\"error\": \"Serialize error: {}\"}}", e))
}
