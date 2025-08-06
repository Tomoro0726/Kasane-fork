use crate::io::Storage;
use crate::{interface::input::json_file::json_file, parser::parser};

pub mod command;
pub mod error;
pub mod interface;
pub mod io;
pub mod output;
pub mod parser;

#[cfg(not(feature = "BuildJsonSchema"))]

fn main() {
    let s = Storage::new();

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

    //コマンド実行層

    //ストレージ層
    //ストレージはメモリの上に載せておく。
    //ファイルへの書き込みは後で用意する
    //とりあえずメモリ上にきれいな構造を用意する

    //Interface-Output層
    //全ての実行結果を含めてJSONに変換する
    //OK
}

//Json Schemaを出力する
#[cfg(feature = "BuildJsonSchema")]
fn main() {
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
