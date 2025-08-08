use logic::{
    id::{SpaceTimeId, points::Point},
    set::SpaceTimeIdSet,
};

use crate::{
    error::Error,
    io::{self, Key, ValueEntry, key},
    output::{GetValueOutput, Output},
    parser::{
        FilterINT::{self, Equal, NotEqual},
        FilterTEXT, FilterType, FilterValue, KeyType,
    },
};

use crate::parser::FilterBOOLEAN::{Equals, IsFalse, IsTrue, NotEquals};

impl Key {
    pub fn has_value(&self) -> SpaceTimeIdSet {
        let mut result = SpaceTimeIdSet::new();
        for v in &self.value {
            result = result | v.set.clone();
        }
        return result;
    }

    pub fn filter_value(&self, filter: FilterType) -> Result<SpaceTimeIdSet, Error> {
        let mut result = SpaceTimeIdSet::new();
        match filter {
            FilterType::FilterBOOLEAN(v) => {
                if self.r#type != KeyType::BOOLEAN {
                    return Err(Error::FilterTypeMismatch("BOOLEAN".to_string()));
                }

                for k in &self.value {
                    match v {
                        IsTrue if k.value == io::ValueEntry::BOOLEAN(true) => {
                            result = result | k.set.clone();
                        }
                        IsFalse if k.value == io::ValueEntry::BOOLEAN(false) => {
                            result = result | k.set.clone();
                        }
                        Equals(val) if k.value == io::ValueEntry::BOOLEAN(val) => {
                            result = result | k.set.clone();
                        }
                        NotEquals(val) if k.value != io::ValueEntry::BOOLEAN(val) => {
                            result = result | k.set.clone();
                        }
                        _ => {}
                    }
                }

                Ok(result)
            }
            FilterType::FilterINT(v) => {
                if self.r#type != KeyType::INT {
                    return Err(Error::FilterTypeMismatch("INT".to_string()));
                };

                let mut result = SpaceTimeIdSet::new();

                for k in &self.value {
                    match v {
                        FilterINT::Equal(val) if k.value == io::ValueEntry::INT(val) => {
                            result = result | k.set.clone();
                        }
                        FilterINT::NotEqual(val) if k.value != io::ValueEntry::INT(val) => {
                            result = result | k.set.clone();
                        }
                        FilterINT::GreaterThan(val) => {
                            if let io::ValueEntry::INT(v) = k.value {
                                if v > val {
                                    result = result | k.set.clone();
                                }
                            }
                        }
                        FilterINT::GreaterEqual(val) => {
                            if let io::ValueEntry::INT(v) = k.value {
                                if v >= val {
                                    result = result | k.set.clone();
                                }
                            }
                        }
                        FilterINT::LessThan(val) => {
                            if let io::ValueEntry::INT(v) = k.value {
                                if v < val {
                                    result = result | k.set.clone();
                                }
                            }
                        }
                        FilterINT::LessEqual(val) => {
                            if let io::ValueEntry::INT(v) = k.value {
                                if v <= val {
                                    result = result | k.set.clone();
                                }
                            }
                        }
                        FilterINT::Between(start, end) => {
                            if let io::ValueEntry::INT(v) = k.value {
                                if v >= start && v <= end {
                                    result = result | k.set.clone();
                                }
                            }
                        }
                        FilterINT::In(ref items) => {
                            if let io::ValueEntry::INT(v) = k.value {
                                if items.contains(&v) {
                                    result = result | k.set.clone();
                                }
                            }
                        }
                        FilterINT::NotIn(ref items) => {
                            if let io::ValueEntry::INT(v) = k.value {
                                if !items.contains(&v) {
                                    result = result | k.set.clone();
                                }
                            }
                        }
                        _ => {}
                    }
                }

                Ok(result)
            }

            FilterType::FilterTEXT(v) => {
                if self.r#type != KeyType::TEXT {
                    return Err(Error::FilterTypeMismatch("TEXT".to_string()));
                }

                let mut result = SpaceTimeIdSet::new();

                for k in &self.value {
                    if let io::ValueEntry::TEXT(ref text_value) = k.value {
                        match v {
                            FilterTEXT::Equal(ref val) if text_value == val => {
                                result = result | k.set.clone();
                            }
                            FilterTEXT::NotEqual(ref val) if text_value != val => {
                                result = result | k.set.clone();
                            }
                            FilterTEXT::Contains(ref val) if text_value.contains(val) => {
                                result = result | k.set.clone();
                            }
                            FilterTEXT::NotContains(ref val) if !text_value.contains(val) => {
                                result = result | k.set.clone();
                            }
                            FilterTEXT::StartsWith(ref val) if text_value.starts_with(val) => {
                                result = result | k.set.clone();
                            }
                            FilterTEXT::EndsWith(ref val) if text_value.ends_with(val) => {
                                result = result | k.set.clone();
                            }
                            FilterTEXT::CaseInsensitiveEqual(ref val)
                                if text_value.eq_ignore_ascii_case(val) =>
                            {
                                result = result | k.set.clone();
                            }
                            _ => {}
                        }
                    }
                }

                Ok(result)
            }
        }
    }

    pub fn get_value(&self, set: SpaceTimeIdSet) -> Result<Output, Error> {
        let mut result = Vec::new();
        for v in &self.value {
            let and = v.set.clone() & set.clone();
            if !and.is_empty() {
                for stid in and.into_iter() {
                    result.push(GetValueOutput {
                        id: stid.to_string(),
                        spacetimeid: stid,
                        vertex: stid.vertex(),
                        value: v.value.clone(),
                    });
                }
            }
        }
        return Ok(Output::GetValue(result));
    }
    pub fn set_value(&mut self, set: SpaceTimeIdSet, value: ValueEntry) -> Result<Output, Error> {
        //入力された型のチェック

        match value {
            ValueEntry::INT(_) => {
                if self.r#type != KeyType::INT {
                    return Err(Error::ValueTypeMismatch("INTをよこせ".to_string()));
                }
            }
            ValueEntry::TEXT(_) => {
                if self.r#type != KeyType::TEXT {
                    return Err(Error::ValueTypeMismatch("TEXTをよこせ".to_string()));
                }
            }
            ValueEntry::BOOLEAN(_) => {
                if self.r#type != KeyType::BOOLEAN {
                    return Err(Error::ValueTypeMismatch("BOOLEANをよこせ".to_string()));
                }
            }
        }

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
        Ok(Output::Success)
    }
    pub fn put_value(&mut self, set: SpaceTimeIdSet, value: ValueEntry) -> Result<Output, Error> {
        match value {
            ValueEntry::INT(_) => {
                if self.r#type != KeyType::INT {
                    return Err(Error::ValueTypeMismatch("INTをよこせ".to_string()));
                }
            }
            ValueEntry::TEXT(_) => {
                if self.r#type != KeyType::TEXT {
                    return Err(Error::ValueTypeMismatch("TEXTをよこせ".to_string()));
                }
            }
            ValueEntry::BOOLEAN(_) => {
                if self.r#type != KeyType::BOOLEAN {
                    return Err(Error::ValueTypeMismatch("BOOLEANをよこせ".to_string()));
                }
            }
        }
        let mut is_push = false;

        //valueが一致してかつ、既存範囲と競合がなければ加える
        //範囲が競合した場合にはエラーを出す
        for v in self.value.iter_mut() {
            if v.value == value {
                if !(v.set.clone() & set.clone()).is_empty() {
                    return Err(Error::SpaceTimeIdAlreadyHasValue("値を上書きするな"));
                }
                v.set = v.set.clone() | set.clone();
                is_push = true
            }
        }
        if !is_push {
            self.value.push(super::Value { value, set });
        }
        Ok(Output::Success)
    }
    pub fn delete_value(&mut self, set: SpaceTimeIdSet) -> Result<Output, Error> {
        self.value.retain_mut(|v| {
            v.set = v.set.clone() | !set.clone();
            !v.set.is_empty()
        });
        Ok(Output::Success)
    }
}
