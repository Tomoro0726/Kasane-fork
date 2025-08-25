use crate::{
    command::tools::select::select,
    error::Error,
    json::{
        input::GetValue,
        output::{GetValueOutput, Output},
    },
};

#[cfg(feature = "default")]
use crate::io::kv::Storage;
#[cfg(feature = "wasm")]
use crate::io::memory::Storage;

pub fn getvalue(v: GetValue, s: &mut Storage) -> Result<Output, Error> {
    let set = select(s, v.range)?;

    let mut result = s.get_value(&v.spacename, &v.keyname, set)?;

    if let Output::GetValue(outputs) = result {
        let mut new_results = Vec::new();

        for item in outputs {
            // id_pure==trueなら純粋IDに展開、それ以外は元のIDのみ
            let id_list = if v.id_pure {
                item.spacetimeid.pure()
            } else {
                vec![item.spacetimeid.clone()]
            };

            for id in id_list {
                let mut new_item = GetValueOutput {
                    spacetimeid: id,
                    vertex: None,
                    value: item.value.clone(),
                    id_string: None,
                    center: None,
                };

                if v.vertex {
                    new_item.vertex = Some(new_item.spacetimeid.vertex());
                }
                if v.center {
                    new_item.center = Some(new_item.spacetimeid.center());
                }
                if v.id_string {
                    new_item.id_string = Some(new_item.spacetimeid.to_string());
                }

                new_results.push(new_item);
            }
        }

        result = Output::GetValue(new_results);
    }

    Ok(result)
}
