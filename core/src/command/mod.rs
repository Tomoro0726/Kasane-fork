use crate::{
    command::{
        addkey::addkey, addspace::addspace, deletekey::deletekey, deletespace::deletespace,
        deletevalue::deletevalue, getvalue::getvalue, putvalue::putvalue, setvalue::setvalue,
        showkeys::showkeys, showspaces::showspaces,
    },
    error::Error,
    io::Storage,
    output::Output,
    parser::{Command, GetValue},
};
pub mod addkey;
pub mod addspace;
pub mod deletekey;
pub mod deletespace;
pub mod deletevalue;
pub mod getvalue;
pub mod putvalue;
pub mod setvalue;
pub mod showkeys;
pub mod showspaces;
pub mod tools;

pub fn process(cmd: Command, s: &mut Storage) -> Result<Output, Error> {
    match cmd {
        crate::parser::Command::AddSpace(v) => addspace(v, s),
        crate::parser::Command::DeleteSpace(v) => deletespace(v, s),
        crate::parser::Command::AddKey(v) => addkey(v, s),
        crate::parser::Command::DeleteKey(v) => deletekey(v, s),
        crate::parser::Command::PutValue(v) => putvalue(v, s),
        crate::parser::Command::SetValue(v) => setvalue(v, s),
        crate::parser::Command::DeleteValue(v) => deletevalue(v, s),
        crate::parser::Command::Keys(v) => showkeys(v, s),
        crate::parser::Command::Spaces(_) => showspaces(s),
        crate::parser::Command::GetValue(v) => getvalue(v, s),
    }
}
