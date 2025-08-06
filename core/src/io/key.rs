use logic::set::SpaceTimeIdSet;

use crate::{
    error::Error,
    io::{Key, ValueEntry, error::IoError, output::IoOutput},
    output::Output,
};

impl Key {
    //データを取得して返す{set:value}の形が帰る
    pub fn get_value(&self, set: SpaceTimeIdSet) -> Vec<(SpaceTimeIdSet, ValueEntry)> {
        let mut result = Vec::new();
        for v in &self.value {
            let and = v.set.clone() & set.clone();

            if !and.is_empty() {
                result.push((and, v.value.clone()));
            }
        }
        return result;
    }
    pub fn set_value(&mut self, set: SpaceTimeIdSet, value: ValueEntry) -> Result<Output, Error> {
        let mut is_push = false;

        //valueが一致した場合はそこに出力
        for v in self.value.iter_mut() {
            if v.value == value {
                v.set = v.set.clone() | set.clone();
                is_push = true
            }
        }

        //一致するvalueがなかった場合はそこに出力
        if !is_push {
            self.value.push(super::Value { value, set });
        }
        Ok(Output::IoResult(IoOutput::Success))
    }
    pub fn put_value(&mut self, set: SpaceTimeIdSet, value: ValueEntry) -> Result<Output, Error> {
        let mut is_push = false;

        //valueが一致してかつ、既存範囲と競合がなければ加える
        //範囲が競合した場合にはエラーを出す
        for v in self.value.iter_mut() {
            if v.value == value {
                if !(v.set.clone() & set.clone()).is_empty() {
                    return Err(Error::IoError(IoError::SpaceTimeIdAlreadyHasValue(
                        "値を上書きするな",
                    )));
                }
                v.set = v.set.clone() | set.clone();
                is_push = true
            }
        }
        if !is_push {
            self.value.push(super::Value { value, set });
        }
        Ok(Output::IoResult(IoOutput::Success))
    }
    pub fn delete_value(&mut self, set: SpaceTimeIdSet) -> Result<Output, Error> {
        self.value.retain_mut(|v| {
            v.set = v.set.clone() | !set.clone();
            !v.set.is_empty()
        });
        Ok(Output::IoResult(IoOutput::Success))
    }
}
