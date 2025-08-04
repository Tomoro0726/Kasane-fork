use crate::{
    command::{addspace::addspace, deletespace::deletespace},
    io::Storage,
    parser::Packet,
};
pub mod addkey;
pub mod addspace;
pub mod deletespace;
pub mod error;
pub mod tools;

pub fn process(packet: Packet, s: &mut Storage) {
    for cmd in packet.commands {
        match match cmd {
            crate::parser::Command::AddSpace(v) => addspace(v, s),
            crate::parser::Command::DeleteSpace(v) => deletespace(v, s),
            crate::parser::Command::AddKeys(v) => todo!(),
            crate::parser::Command::DeleteKeys(delete_keys) => todo!(),
            crate::parser::Command::PutValue(put_value) => todo!(),
            crate::parser::Command::SetValue(set_value) => todo!(),
            crate::parser::Command::DeleteValue(delete_value) => todo!(),
            crate::parser::Command::Transaction(transaction) => todo!(),
            crate::parser::Command::Showkeys(showkeys) => todo!(),
            crate::parser::Command::AddUser(add_user) => todo!(),
            crate::parser::Command::DeleteUser(delete_user) => todo!(),
            crate::parser::Command::ShowSpaces => todo!(),
        } {
            Ok(_) => {}
            Err(_) => {}
        }
    }
}
