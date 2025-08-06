use crate::io::Storage;
use crate::{interface::input::json_file::json_file, parser::parser};

pub mod command;
pub mod error;
pub mod interface;
pub mod io;
pub mod output;
pub mod parser;
use crate::command::process;

#[cfg(not(feature = "BuildJsonSchema"))]

fn main() {
    let mut s = Storage::new().unwrap();

    //パケットを入力している
    let packet_raw = match json_file("sample.json") {
        Ok(v) => v,
        Err(e) => {
            println!("{:?}", e);
            return;
        }
    };

    //PacketのJSONをパースしてみる
    let packet = match parser(&packet_raw) {
        Ok(v) => v,
        Err(e) => {
            println!("{:?}", e);
            return;
        }
    };

    //ユーザー情報確認レイヤー
    //存在しないユーザーははじく
    //Wasm用の段階ではdefault以外に受け付けない
    if packet.user != "default" {
        println!("ユーザーが違うな...")
    }

    for cmd in packet.commands {
        match process(cmd, &mut s) {
            Ok(o) => {}
            Err(v) => {}
        }
    }
}

//Json Schemaを出力する
#[cfg(feature = "BuildJsonSchema")]
fn main() {
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
