use crate::{
    error::Error,
    json::{
        input::Select,
        output::{Output, SelectOutput},
    },
};

#[cfg(feature = "full")]
use crate::io::kv::Storage;
#[cfg(feature = "wasm")]
use crate::io::memory::Storage;

pub fn select(v: Select, s: &mut Storage) -> Result<Output, Error> {
    match crate::command::tools::select::select(s, v.range) {
        Ok(a) => {
            let mut result = Vec::new();
            for stid in a.into_iter() {
                let center = if v.center { Some(stid.center()) } else { None };
                let vertex = if v.vertex { Some(stid.vertex()) } else { None };
                let id_string = if v.id_string {
                    Some(stid.to_string())
                } else {
                    None
                };

                if v.id_pure {
                    for mini_id in stid.pure() {
                        result.push(SelectOutput {
                            spacetimeid: mini_id,
                            id_string: id_string.clone(),
                            vertex: vertex,
                            center: center,
                        });
                    }
                } else {
                    result.push(SelectOutput {
                        spacetimeid: stid,
                        id_string,
                        vertex,
                        center,
                    });
                }
            }
            Ok(Output::SelectValue(result))
        }
        Err(e) => Err(Error::ParseError {
            message: e.to_string(),
            location: "command::select::select",
        }),
    }
}
