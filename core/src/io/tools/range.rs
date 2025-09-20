use std::{collections::HashSet, result};

use kasane_logic::{
    function::{
        line::line,
        point::{self, point},
        triangle::triangle,
    },
    id::SpaceTimeId,
};

use crate::json::input::Range;

pub fn range(range: Range) -> Result<Vec<Vec<u8>>, String> {
    let mut result: Vec<Vec<u8>> = Vec::new();
    match range {
        Range::Function(v) => match v {
            crate::json::input::Function::Spot(spot) => {
                result.extend(id_to_bitmask(point(spot.zoom, spot.point1)))
            }
            crate::json::input::Function::Line(k) => {
                result.extend(ids_to_bitmask(line(k.zoom, k.point1, k.point2)))
            }
            crate::json::input::Function::Triangle(k) => result.extend(ids_to_bitmask(triangle(
                k.zoom, k.point1, k.point2, k.point3,
            ))),
            crate::json::input::Function::FilterValue(filter_value) => todo!(),
        },
        Range::Prefix(v) => {}
        Range::IdSet(v) => {
            let mut ids: HashSet<SpaceTimeId> = HashSet::new();
            for id in v {
                ids.insert(SpaceTimeId::new(id.z, id.f, id.x, id.y, id.i, id.t)?);
            }
            result.extend(ids_to_bitmask(ids))
        }
    };

    Ok(result)
}

fn ids_to_bitmask(ids: HashSet<SpaceTimeId>) -> Vec<Vec<u8>> {
    let mut result: Vec<Vec<u8>> = Vec::new();
    for ele in ids {
        result.extend(id_to_bitmask(ele))
    }

    result
}

fn id_to_bitmask(ids: SpaceTimeId) -> Vec<Vec<u8>> {
    let mut result = Vec::new();

    for id in ids.pure() {
        let mut bytes = Vec::with_capacity(1 + (id.z as usize + 1) / 2);

        // 先頭バイトは F軸符号
        bytes.push(if id.f >= 0 { 1 } else { 0 });

        let mut level = 0;
        while level < id.z {
            // 1レベル目の octant
            let x0 = if is_even_after(id.x as i32, level.into()) {
                0
            } else {
                1
            };
            let y0 = if is_even_after(id.y as i32, level.into()) {
                0
            } else {
                1
            };
            let f0 = if is_even_after(id.f.abs() as i32, level.into()) {
                0
            } else {
                1
            };
            let oct0 = (x0 << 2) | (y0 << 1) | f0; // 0..7

            // 2レベル目の octant（存在する場合のみ）
            let oct1 = if level + 1 < id.z {
                let x1 = if is_even_after(id.x as i32, (level + 1).into()) {
                    0
                } else {
                    1
                };
                let y1 = if is_even_after(id.y as i32, (level + 1).into()) {
                    0
                } else {
                    1
                };
                let f1 = if is_even_after(id.f.abs() as i32, (level + 1).into()) {
                    0
                } else {
                    1
                };
                (x1 << 2) | (y1 << 1) | f1 // 0..7
            } else {
                0 // 2レベル目がなければ0で埋める
            };

            // 1バイトに詰める（上位3bit: oct0, 次の3bit: oct1, 下位2bitは0）
            let byte = (oct0 << 5) | (oct1 << 2);
            bytes.push(byte);

            level += 2; // 2レベル進める
        }

        result.push(bytes);
    }

    result
}

/// zビット目以降を右シフトして偶奇判定
fn is_even_after(dim: i32, z: u32) -> bool {
    ((dim >> z) & 1) == 0
}
