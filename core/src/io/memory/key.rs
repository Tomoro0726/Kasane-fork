use kasane_logic::set::SpaceTimeIdSet;

use crate::io::ValueEntry;
use crate::io::memory::Storage;
use crate::json::input::FilterBOOLEAN::{Equals, IsFalse, IsTrue, NotEquals};
use crate::json::input::{FilterValue, HasValue};
use crate::{
    error::Error,
    json::{
        input::{FilterINT, FilterTEXT, FilterType, KeyType},
        output::{GetValueOutput, Output},
    },
};

impl Storage {
    pub fn has_value(&mut self, v: HasValue) -> Result<SpaceTimeIdSet, Error> {
        let key = Self::get_key(self, &v.spacename, &v.keyname)?;

        let mut result = SpaceTimeIdSet::new();
        for v in &key.value {
            result = result | v.set.clone();
        }
        return Ok(result);
    }

    pub fn filter_value(&mut self, v: FilterValue) -> Result<SpaceTimeIdSet, Error> {
        let key = self.get_key(&v.spacename, &v.keyname)?;

        let mut result = SpaceTimeIdSet::new();
        match v.filter {
            FilterType::FilterBOOLEAN(v) => {
                if key.r#type != KeyType::BOOLEAN {
                    return Err(Error::TypeMismatchFilter {
                        expected_type: "BOOLEAN".to_string(),
                        operation: "filter".to_string(),
                        location: "io::key::filter_value",
                    });
                }

                for k in key.value.iter() {
                    match v {
                        IsTrue if k.value == ValueEntry::BOOLEAN(true) => {
                            result = result | k.set.clone();
                        }
                        IsFalse if k.value == ValueEntry::BOOLEAN(false) => {
                            result = result | k.set.clone();
                        }
                        Equals(val) if k.value == ValueEntry::BOOLEAN(val) => {
                            result = result | k.set.clone();
                        }
                        NotEquals(val) if k.value != ValueEntry::BOOLEAN(val) => {
                            result = result | k.set.clone();
                        }
                        _ => {}
                    }
                }

                Ok(result)
            }
            FilterType::FilterINT(v) => {
                if key.r#type != KeyType::INT {
                    return Err(Error::TypeMismatchFilter {
                        expected_type: "INT".to_string(),
                        operation: "filter".to_string(),
                        location: "io::key::filter_value",
                    });
                };

                let mut result = SpaceTimeIdSet::new();

                for k in key.value.iter() {
                    match v {
                        FilterINT::Equal(val) if k.value == ValueEntry::INT(val) => {
                            result = result | k.set.clone();
                        }
                        FilterINT::NotEqual(val) if k.value != ValueEntry::INT(val) => {
                            result = result | k.set.clone();
                        }
                        FilterINT::GreaterThan(val) => {
                            if let ValueEntry::INT(v) = k.value {
                                if v > val {
                                    result = result | k.set.clone();
                                }
                            }
                        }
                        FilterINT::GreaterEqual(val) => {
                            if let ValueEntry::INT(v) = k.value {
                                if v >= val {
                                    result = result | k.set.clone();
                                }
                            }
                        }
                        FilterINT::LessThan(val) => {
                            if let ValueEntry::INT(v) = k.value {
                                if v < val {
                                    result = result | k.set.clone();
                                }
                            }
                        }
                        FilterINT::LessEqual(val) => {
                            if let ValueEntry::INT(v) = k.value {
                                if v <= val {
                                    result = result | k.set.clone();
                                }
                            }
                        }
                        FilterINT::Between(start, end) => {
                            if let ValueEntry::INT(v) = k.value {
                                if v >= start && v <= end {
                                    result = result | k.set.clone();
                                }
                            }
                        }
                        FilterINT::In(ref items) => {
                            if let ValueEntry::INT(v) = k.value {
                                if items.contains(&v) {
                                    result = result | k.set.clone();
                                }
                            }
                        }
                        FilterINT::NotIn(ref items) => {
                            if let ValueEntry::INT(v) = k.value {
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
                if key.r#type != KeyType::TEXT {
                    return Err(Error::TypeMismatchFilter {
                        expected_type: "TEXT".to_string(),
                        operation: "filter".to_string(),
                        location: "io::key::filter_value",
                    });
                }

                let mut result = SpaceTimeIdSet::new();

                for k in key.value.iter() {
                    if let ValueEntry::TEXT(ref text_value) = k.value {
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

    pub fn get_value(
        &mut self,
        spacename: &str,
        keyname: &str,
        set: SpaceTimeIdSet,
    ) -> Result<Output, Error> {
        let key = self.get_key(spacename, keyname)?;
        let mut result = Vec::new();
        for v in &key.value {
            let and = v.set.clone() & set.clone();
            if !and.is_empty() {
                for stid in and.into_iter() {
                    result.push(GetValueOutput {
                        spacetimeid: stid,
                        vertex: None,
                        value: v.value.clone(),
                        id_string: None,
                        center: None,
                    });
                }
            }
        }
        return Ok(Output::GetValue(result));
    }
    pub fn set_value(
        &mut self,
        spacename: &str,
        keyname: &str,
        value: ValueEntry,
        set: SpaceTimeIdSet,
    ) -> Result<Output, Error> {
        let key = self.get_key(spacename, keyname)?;

        // 型チェック
        match value {
            ValueEntry::INT(_) if key.r#type != KeyType::INT => {
                return Err(Error::TypeMismatchValue {
                    expected_type: "INT".to_string(),
                    received_type: format!("{:?}", value),
                    location: "io::key::set_value",
                });
            }
            ValueEntry::TEXT(_) if key.r#type != KeyType::TEXT => {
                return Err(Error::TypeMismatchValue {
                    expected_type: "TEXT".to_string(),
                    received_type: format!("{:?}", value),
                    location: "io::key::set_value",
                });
            }
            ValueEntry::BOOLEAN(_) if key.r#type != KeyType::BOOLEAN => {
                return Err(Error::TypeMismatchValue {
                    expected_type: "BOOLEAN".to_string(),
                    received_type: format!("{:?}", value),
                    location: "io::key::set_value",
                });
            }
            ValueEntry::FLOAT(_) if key.r#type != KeyType::FLOAT => {
                return Err(Error::TypeMismatchValue {
                    expected_type: "FLOAT".to_string(),
                    received_type: format!("{:?}", value),
                    location: "io::key::set_value",
                });
            }
            _ => {}
        }

        let mut is_push = false;

        for a in key.value.iter_mut() {
            if a.value == value {
                a.set = a.set.clone() | set.clone();
                is_push = true;
            }
        }

        if !is_push {
            key.value.push(super::Value { value, set });
        }

        Ok(Output::Success)
    }

    pub fn put_value(
        &mut self,
        spacename: &str,
        keyname: &str,
        value: ValueEntry,
        set: SpaceTimeIdSet,
    ) -> Result<Output, Error> {
        // キー取得
        let key = Self::get_key(self, spacename, keyname)?;

        // 型チェック
        match value {
            ValueEntry::INT(_) if key.r#type != KeyType::INT => {
                return Err(Error::TypeMismatchValue {
                    expected_type: "INT".to_string(),
                    received_type: format!("{:?}", value),
                    location: "io::key::put_value",
                });
            }
            ValueEntry::TEXT(_) if key.r#type != KeyType::TEXT => {
                return Err(Error::TypeMismatchValue {
                    expected_type: "TEXT".to_string(),
                    received_type: format!("{:?}", value),
                    location: "io::key::put_value",
                });
            }
            ValueEntry::BOOLEAN(_) if key.r#type != KeyType::BOOLEAN => {
                return Err(Error::TypeMismatchValue {
                    expected_type: "BOOLEAN".to_string(),
                    received_type: format!("{:?}", value),
                    location: "io::key::put_value",
                });
            }
            ValueEntry::FLOAT(_) if key.r#type != KeyType::FLOAT => {
                return Err(Error::TypeMismatchValue {
                    expected_type: "FLOAT".to_string(),
                    received_type: format!("{:?}", value),
                    location: "io::key::put_value",
                });
            }
            _ => {}
        }

        let mut is_push = false;

        // 既存値と競合チェック
        for v in key.value.iter_mut() {
            if v.value == value {
                if !(v.set.clone() & set.clone()).is_empty() {
                    return Err(Error::ValueAlreadyExists {
                        space_time_id: format!("{:?}", v.set.clone() & set.clone()),
                        location: "io::key::put_value",
                    });
                }
                v.set = v.set.clone() | set.clone();
                is_push = true;
            }
        }

        // 一致する値がなければ新規追加
        if !is_push {
            key.value.push(super::Value { value, set });
        }

        Ok(Output::Success)
    }
    pub fn delete_value(
        &mut self,
        spacename: &str,
        keyname: &str,
        set: SpaceTimeIdSet,
    ) -> Result<Output, Error> {
        // キー取得
        let key = Self::get_key(self, spacename, keyname)?;

        // 対象の set を削除
        key.value.retain_mut(|v| {
            v.set = v.set.clone() & !set.clone();
            !v.set.is_empty()
        });

        Ok(Output::Success)
    }
}
