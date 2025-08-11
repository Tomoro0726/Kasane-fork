use std::fs::File;
use std::io::Write;

use schemars::JsonSchema;
use serde::Serialize;

use crate::io::Storage;
use crate::{interface::input::json_file::json_file, parser::parser};

pub mod command;
pub mod error;
pub mod interface;
pub mod io;
pub mod output;
pub mod parser;
use crate::output::Output;

use crate::command::process;

#[cfg(not(feature = "BuildJsonSchema"))]
fn main() {
    let mut s = match Storage::new() {
        Ok(store) => store,
        Err(e) => {
            panic!("{}", e)
        }
    };

    // JSONファイル読み込み
    let packet_raw = match json_file("sample.json") {
        Ok(v) => v,
        Err(e) => {
            return_packet(vec![CommandResult::Error(e.to_string())]);
            return;
        }
    };

    // JSONパース
    let packet = match parser(&packet_raw) {
        Ok(v) => v,
        Err(e) => {
            return_packet(vec![CommandResult::Error(e.to_string())]);
            return;
        }
    };

    // 複数コマンド実行
    let mut results = Vec::new();
    for cmd in packet.command {
        match process(cmd, &mut s) {
            Ok(v) => results.push(CommandResult::Success(v)),
            Err(e) => results.push(CommandResult::Error(e.to_string())),
        }
    }

    // 実行結果を出力
    return_packet(results);
}

#[derive(Serialize, JsonSchema)]
enum CommandResult {
    Success(Output),
    Error(String),
}

fn return_packet(results: Vec<CommandResult>) {
    // JSON に変換
    let json = serde_json::to_string_pretty(&results).expect("Failed to serialize CommandResult");

    // ファイルに書き出す（ファイル名は固定だが変更可能）
    let mut file = File::create("result.json").expect("Failed to create file");

    file.write_all(json.as_bytes())
        .expect("Failed to write to file");
}

//Json Schemaを出力する
#[cfg(feature = "BuildJsonSchema")]
fn main() {
    use crate::parser::Command;
    use schemars::schema_for;
    //JSON Schemaを出力している
    let input_schema = schema_for!(Command);

    // ファイルにも保存（任意）
    std::fs::write(
        "input_schema.json",
        serde_json::to_string_pretty(&input_schema).unwrap(),
    )
    .expect("Failed to write schema.json");

    let output_schema = schema_for!(Output);
    // ファイルにも保存（任意）
    std::fs::write(
        "output_schema.json",
        serde_json::to_string_pretty(&output_schema).unwrap(),
    )
    .expect("Failed to write schema.json");
}
