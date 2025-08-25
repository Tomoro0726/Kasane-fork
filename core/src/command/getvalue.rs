use crate::{
    command::tools::select::select,
    error::Error,
    io::Storage,
    json::{
        input::GetValue,
        output::{GetValueOutput, Output},
    },
};
pub fn getvalue(v: GetValue, s: &mut Storage) -> Result<Output, Error> {
    let set = select(s, v.range)?;

    let space = s.get_space(&v.spacename)?;
    let key = space.get_key(&v.keyname)?;
    let mut result = key.get_value(set)?;

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

                // vertexオプションがtrueならvertex情報を入れる処理（例）
                if v.vertex {
                    new_item.vertex = Some(new_item.spacetimeid.vertex());
                }
                // centerオプションがtrueなら中心座標を計算してセット
                if v.center {
                    new_item.center = Some(new_item.spacetimeid.center());
                }
                // id_stringオプションがtrueならID文字列化処理
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
