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
#[derive(Serialize, JsonSchema)]
enum PacketState {
    JsonError(String),
    CommandError(String),
    Ok(Vec<Output>),
}

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
            return_packet(CommandResult::Error(e.to_string()));
            return;
        }
    };

    // JSONパース
    let cmd = match parser(&packet_raw) {
        Ok(v) => v,
        Err(e) => {
            return_packet(CommandResult::Error(e.to_string()));
            return;
        }
    };

    match process(cmd, &mut s) {
        Ok(v) => {
            return_packet(CommandResult::Success(v));
            return;
        }
        Err(e) => {
            return_packet(CommandResult::Error(e.to_string()));
            return;
        }
    }

    // 成功したら出力
}

#[derive(Serialize, JsonSchema)]
enum CommandResult {
    Success(Output),
    Error(String),
}

fn return_packet(result: CommandResult) {
    // JSON に変換
    let json = serde_json::to_string_pretty(&result).expect("Failed to serialize CommandResult");

    // ファイルに書き出す（ファイル名は固定だが変更可能）
    let mut file = File::create("result.json").expect("Failed to create file");

    file.write_all(json.as_bytes())
        .expect("Failed to write to file");
}

//Json Schemaを出力する
#[cfg(feature = "BuildJsonSchema")]
fn main() {
    use super::PacketState;
    use crate::parser::Command;

    use schemars::schema_for;
    //JSON Schemaを出力している
    let schema = schema_for!(Packet);

    // ファイルにも保存（任意）
    std::fs::write(
        "schema.json",
        serde_json::to_string_pretty(&schema).unwrap(),
    )
    .expect("Failed to write schema.json");
}
