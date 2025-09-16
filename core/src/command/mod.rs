#[cfg(feature = "wasm")]
use std::sync::Arc;
#[cfg(feature = "full")]
use std::sync::{Arc, Mutex};

use crate::command::create_key::create_key;
use crate::command::create_space::create_space;
use crate::command::drop_key::drop_key;
use crate::command::drop_space::drop_space;
use crate::command::show_keys::show_keys;
use crate::command::show_spaces::show_spaces;
use crate::io::Storage;
use crate::{
    command::version::version,
    error::Error,
    json::{input::Command, output::Output},
};
pub mod create_key;
pub mod create_space;
pub mod deletevalue;
pub mod drop_key;
pub mod drop_space;
pub mod getvalue;
pub mod keysinfo;
pub mod line;
pub mod putvalue;
pub mod select;
pub mod setvalue;
pub mod show_keys;
pub mod show_spaces;
pub mod tools;
pub mod triangle;
pub mod version;

//関数のディスパッチ関数
//関数の命令内容とストレージの参照権を関数に入力し、操作を行わせる
pub fn process(cmd: Command, s: Arc<Storage>) -> Result<Output, Error> {
    match cmd {
        //データベース操作系
        Command::CreateSpace(v) => create_space(v, s),
        Command::DropSpace(v) => drop_space(v, s),
        Command::ShowSpaces => show_spaces(s),
        Command::InfoSpace(v) => todo!(),
        Command::Version => version(),

        //Key操作系
        Command::CreateKey(v) => create_key(v, s),
        Command::DropKey(v) => drop_key(v, s),
        Command::ShowKeys(v) => show_keys(v, s),
        Command::InfoKey(v) => todo!(),

        //Value操作系
        Command::InsertValue(v) => todo!(),
        Command::UpdateValue(v) => todo!(),
        Command::DeleteValue(v) => todo!(),
        Command::SelectValue(v) => todo!(),
        Command::ShowValues(v) => todo!(),

        //ツール系
        //Command::Transaction(v) => todo!(),

        //ユーザー操作系
        Command::CreateUser(v) => todo!(),
        Command::DropUser(v) => todo!(),
        Command::InfoUser(v) => todo!(),
        Command::ShowUsers => todo!(),

        //権限付与系
        Command::GrantDatabase(v) => todo!(),
        Command::GrantSpacePrivilege(v) => todo!(),
        Command::GrantKeyPrivilege(v) => todo!(),
        Command::GrantToolPrivilege(v) => todo!(),

        //権限取り上げる系
        Command::RevokeDatabase(v) => todo!(),
        Command::RevokeSpacePrivilege(v) => todo!(),
        Command::RevokeKeyPrivilege(v) => todo!(),
        Command::RevokeToolPrivilege(v) => todo!(),
    }
}
