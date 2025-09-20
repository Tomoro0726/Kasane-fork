use std::result;

use kasane_logic::{function::line::line, id::SpaceTimeId};

use crate::json::input::Range;

pub fn range(range: Range) -> Result<Vec<Vec<u8>>, String> {
    match range {
        Range::Function(v) => todo!(),
        Range::Prefix(v) => todo!(),
        Range::IdSet(v) => todo!(),
    }
}

fn id_to_bitmask(ids: SpaceTimeId) -> Vec<Vec<u8>> {
    let mut result = vec![];

    for id in ids.pure() {
        let mut bits = Vec::with_capacity(1 + (id.z as usize) * 3);

        // F軸の符号（0:負, 1:非負）
        bits.push(if id.f >= 0 { 1 } else { 0 });

        // X, Y, F軸の各ビット（下位ビット → 上位ビット）
        for shift in 0..id.z {
            bits.push(if is_even_after(id.x as i32, shift.into()) {
                0
            } else {
                1
            });
            bits.push(if is_even_after(id.y as i32, shift.into()) {
                0
            } else {
                1
            });
            bits.push(if is_even_after(id.f.abs() as i32, shift.into()) {
                0
            } else {
                1
            });
        }

        result.push(bits);
    }

    result
}

/// zビット目以降を右シフトして偶奇判定
fn is_even_after(dim: i32, z: u32) -> bool {
    ((dim >> z) & 1) == 0
}
