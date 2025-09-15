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
use crate::command::transaction::transaction;
use crate::io::Storage;
use crate::{
    command::{
        getvalue::getvalue, keysinfo::keysinfo, putvalue::putvalue, select::select,
        setvalue::setvalue, version::version,
    },
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
pub mod transaction;
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
        Command::InfoKey(info_key) => todo!(),

        //Value操作系
        Command::InsertValue(insert_value) => todo!(),
        Command::UpdateValue(update_value) => todo!(),
        Command::DeleteValue(delete_value) => todo!(),
        Command::SelectValue(select_value) => todo!(),
        Command::ShowValues(show_values) => todo!(),

        //ツール系
        Command::Transaction(commands) => todo!(),
        Command::Range(range) => todo!(),

        //ユーザー操作系
        Command::CreateUser(create_user) => todo!(),
        Command::DropUser(drop_user) => todo!(),
        Command::InfoUser(info_user) => todo!(),
        Command::ShowUsers => todo!(),

        //権限付与系
        Command::GrantDatabase(grant_database) => todo!(),
        Command::GrantSpacePrivilege(grant_space_privilege) => todo!(),
        Command::GrantKeyPrivilege(grant_key_privilege) => todo!(),

        //権限取り上げる系
        Command::RevokeDatabase(revoke_database) => todo!(),
        Command::RevokeSpacePrivilege(revoke_space_privilege) => todo!(),
        Command::RevokeKeyPrivilege(revoke_key_privilege) => todo!(),
    }
}
