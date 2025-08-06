use schemars::JsonSchema;
use serde::Serialize;

use crate::io::Storage;
use crate::{interface::input::json_file::json_file, parser::parser};

use std::fs::write;

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
    UserError(String),
    CommandError(String),
    Ok(Vec<Output>),
}

fn main() {
    let mut s = match Storage::new() {
        Ok(store) => store,
        Err(e) => {
            write_packet_state(PacketState::CommandError(format!(
                "Storage init error: {:?}",
                e
            )));
            return;
        }
    };

    // JSONファイル読み込み
    let packet_raw = match json_file("sample.json") {
        Ok(v) => v,
        Err(e) => {
            write_packet_state(PacketState::JsonError(format!("{:?}", e)));
            return;
        }
    };

    // JSONパース
    let packet = match parser(&packet_raw) {
        Ok(v) => v,
        Err(e) => {
            write_packet_state(PacketState::CommandError(format!("Parse error: {:?}", e)));
            return;
        }
    };

    // ユーザー確認
    if packet.user != "default" {
        write_packet_state(PacketState::UserError(format!(
            "Unknown user: {}",
            packet.user
        )));
        return;
    }

    let mut outputs = Vec::new();
    for cmd in packet.commands {
        match process(cmd, &mut s) {
            Ok(output) => outputs.push(output),
            Err(e) => {
                write_packet_state(PacketState::CommandError(format!("{:?}", e)));
                return;
            }
        }
    }

    // 成功したら出力
    write_packet_state(PacketState::Ok(outputs));
}

/// PacketState を JSON にしてファイルに保存
fn write_packet_state(state: PacketState) {
    match serde_json::to_string_pretty(&state) {
        Ok(json) => {
            if let Err(e) = write("packet_state.json", json) {
                eprintln!("Failed to write packet_state.json: {:?}", e);
            }
        }
        Err(e) => {
            eprintln!("Failed to serialize PacketState: {:?}", e);
        }
    }
}
//Json Schemaを出力する
#[cfg(feature = "BuildJsonSchema")]
fn main() {
    use super::PacketState;
    use crate::parser::Packet;

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
