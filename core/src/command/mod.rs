use crate::{
    command::{
        addkey::addkey, addspace::addspace, deletekey::deletekey, deletespace::deletespace,
        deletevalue::deletevalue, getvalue::getvalue, keysinfo::keysinfo, putvalue::putvalue,
        select::select, setvalue::setvalue, showkeys::showkeys, showspaces::showspaces,
        version::version,
    },
    error::Error,
    io::Storage,
    json::{input::Command, output::Output},
};
pub mod addkey;
pub mod addspace;
pub mod deletekey;
pub mod deletespace;
pub mod deletevalue;
pub mod getvalue;
pub mod keysinfo;
pub mod line;
pub mod putvalue;
pub mod select;
pub mod setvalue;
pub mod showkeys;
pub mod showspaces;
pub mod tools;
pub mod triangle;
pub mod version;

pub fn process(cmd: Command, s: &mut Storage) -> Result<Output, Error> {
    match cmd {
        Command::AddSpace(v) => addspace(v, s),
        Command::DeleteSpace(v) => deletespace(v, s),
        Command::AddKey(v) => addkey(v, s),
        Command::DeleteKey(v) => deletekey(v, s),
        Command::PutValue(v) => putvalue(v, s),
        Command::SetValue(v) => setvalue(v, s),
        Command::DeleteValue(v) => deletevalue(v, s),
        Command::Keys(v) => showkeys(v, s),
        Command::Spaces => showspaces(s),
        Command::GetValue(v) => getvalue(v, s),
        Command::Select(v) => select(v, s),
        Command::Version => version(),
        Command::KeysInfo(v) => keysinfo(v, s),
    }
}
