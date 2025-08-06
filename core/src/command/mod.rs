use crate::{
    command::{
        addkey::addkey, addspace::addspace, deletekey::deletekey, deletespace::deletespace,
        showkeys::showkeys, showspaces::showspaces,
    },
    io::Storage,
    parser::Packet,
};
pub mod addkey;
pub mod addspace;
pub mod deletekey;
pub mod deletespace;
pub mod error;
pub mod showkeys;
pub mod showspaces;
pub mod tools;

pub fn process(packet: Packet, s: &mut Storage) {
    for cmd in packet.commands {
        match match cmd {
            //未定義の関数については追加が必要
            crate::parser::Command::AddSpace(v) => addspace(v, s),
            crate::parser::Command::DeleteSpace(v) => deletespace(v, s),
            crate::parser::Command::AddKey(v) => addkey(v, s),
            crate::parser::Command::DeleteKey(v) => deletekey(v, s),
            crate::parser::Command::PutValue(put_value) => todo!(),
            crate::parser::Command::SetValue(set_value) => todo!(),
            crate::parser::Command::DeleteValue(delete_value) => todo!(),
            crate::parser::Command::Showkeys(v) => showkeys(v, s),
            crate::parser::Command::ShowSpaces => showspaces(s),
        } {
            Ok(_) => {}
            Err(_) => {}
        }
    }
}
