use crate::{
    interface::input::json_file::json_file,
    parser::parser::Command,
    parser::parser::{parser, Packet},
};

pub mod command;
pub mod interface;
pub mod io;
pub mod parser;

#[cfg(not(feature = "BuildJsonSchema"))]

fn main() {
    use crate::io::Storage;

    let s = Storage::new();

    //パケットを入力している
    let packet_raw = match json_file("sample.json") {
        Ok(v) => v,
        Err(e) => {
            println!("{:?}", e);
            return;
        }
    };

    let packet = match parser(&packet_raw) {
        Ok(v) => v,
        Err(e) => {
            println!("{:?}", e);
            return;
        }
    };

    //ユーザー情報確認レイヤー
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
}

//Json Schemaを出力する
#[cfg(feature = "BuildJsonSchema")]
fn main() {
    //JSON Schemaを出力している
    let schema = schema_for!(Packet);

    // ファイルにも保存（任意）
    std::fs::write(
        "schema.json",
        serde_json::to_string_pretty(&schema).unwrap(),
    )
    .expect("Failed to write schema.json");
}
