use serde_json::Value;

pub fn json_from_str(input: &str) -> Result<Value, serde_json::Error> {
    serde_json::from_str(input)
}
