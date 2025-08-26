use crate::json::input::KeyType;

pub fn keytype_id(keytype: KeyType) -> u8 {
    match keytype {
        KeyType::INT => 1,
        KeyType::BOOLEAN => 2,
        KeyType::TEXT => 3,
        KeyType::FLOAT => 4,
    }
}
