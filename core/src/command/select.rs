use std::{result, vec};

use crate::{
    error::Error,
    io::Storage,
    output::{Output, SelectOutPut},
    parser::DeleteValue,
};

pub fn select(v: crate::parser::Range, s: &mut Storage) -> Result<Output, Error> {
    match crate::command::tools::select::select(s, v) {
        Ok(v) => {
            let mut result = Vec::new();
            for stid in v.into_iter() {
                result.push(SelectOutPut {
                    id: stid.to_string(),
                    spacetimeid: stid,
                    vertex: stid.vertex(),
                });
            }
            Ok(Output::SelectValue(result))
        }
        Err(e) => {
            return Err(Error::ParseError(e.to_string()));
        }
    }
}
