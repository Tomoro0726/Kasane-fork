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

pub fn range(rng: Range) -> Result<Vec<Vec<u8>>, String> {
    let mut result: Vec<Vec<u8>> = Vec::new();
    match rng {
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
            crate::json::input::Function::FilterValue(filter_value) => {}
        },
        Range::Prefix(v) => match v {
            crate::json::input::Prefix::AND(ranges) => {
                let mut and: Vec<Vec<u8>> = vec![];
                for a in ranges {
                    and.extend(dedup_bitmasks(range(a)?));
                }
                return Ok(and_vecs_optimized(and));
            }
            crate::json::input::Prefix::OR(ranges) => {
                let mut or: Vec<Vec<u8>> = vec![];
                for a in ranges {
                    or.extend(dedup_bitmasks(range(a)?));
                }
                return Ok(dedup_bitmasks(or));
            }
            crate::json::input::Prefix::XOR(ranges) => todo!(),
            crate::json::input::Prefix::NOT(ranges) => todo!(),
        },
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
    let mut result = vec![];
    for id in ids.pure() {
        let mut bits = Vec::with_capacity(1 + (id.z as usize) * 3);
        bits.push(if id.f >= 0 { 1 } else { 0 });
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

fn dedup_bitmasks(mut masks: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    // 短い順にソート
    masks.sort_by_key(|m| m.len());

    let mut result: Vec<Vec<u8>> = Vec::new();

    for mask in masks {
        // mask が既存のどれかの先頭を含んでいれば追加不要
        if result
            .iter()
            .any(|existing| existing.len() <= mask.len() && mask[..existing.len()] == existing[..])
        {
            continue; // mask は既存に含まれるのでスキップ
        }

        // mask が既存の先頭に含まれる長いものを削除
        result.retain(|existing| {
            !(existing.len() > mask.len() && existing[..mask.len()] == mask[..])
        });

        result.push(mask);
    }

    result
}

/// ANDルールに従ってVec<Vec<u8>>を処理
pub fn and_vecs_optimized(mut masks: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    // 短い順にソート
    masks.sort_by_key(|m| m.len());
    let mut result: Vec<Vec<u8>> = Vec::new();

    for mask in masks {
        let mut keep_mask = true;

        // 比較対象の既存マスクを残すかどうか決める
        result.retain(|existing| {
            if existing.len() == mask.len() && existing.last() == mask.last() {
                // 同じ長さかつ最後のu8が同じ → 両方残す
                true
            } else if existing.len() < mask.len() {
                // 既存が短い場合
                if existing[..] == mask[..existing.len()] {
                    // 短いものが長い方に部分一致 → 長い方を消す
                    keep_mask = false;
                    true // 既存は残す
                } else {
                    // 部分一致なし → 両方消す
                    keep_mask = false;
                    false // 既存を削除
                }
            } else {
                // 既存が長い場合
                if mask[..] == existing[..mask.len()] {
                    // mask が先頭に一致 → 既存を消す
                    false
                } else {
                    // 部分一致なし → 両方消す
                    keep_mask = false;
                    false
                }
            }
        });

        if keep_mask {
            result.push(mask);
        }
    }

    result
}
