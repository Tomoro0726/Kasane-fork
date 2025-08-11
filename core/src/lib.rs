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
pub struct Kasane {
    storage: Storage,
}
#[wasm_bindgen]
impl Kasane {
    /// 実行メソッド（複数コマンド対応）
    #[wasm_bindgen]
    pub fn execute(&mut self, command_json: &str) -> String {
        match parser(command_json) {
            Ok(packet) => {
                let mut results = Vec::new();

                for cmd in packet.command {
                    match process(cmd, &mut self.storage) {
                        Ok(output) => results.push(CommandResult::Success(output)),
                        Err(e) => results.push(CommandResult::Error(e.to_string())),
                    }
                }

                to_json(results)
            }
            Err(e) => to_json(vec![CommandResult::Error(format!("Parse error: {}", e))]),
        }
    }
}

fn to_json(result: Vec<CommandResult>) -> String {
    serde_json::to_string_pretty(&result)
        .unwrap_or_else(|e| format!("{{\"error\": \"Serialize error: {}\"}}", e))
}
