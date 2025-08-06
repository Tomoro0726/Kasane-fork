use schemars::JsonSchema;
use serde::Serialize;

#[derive(Serialize, JsonSchema)]
pub enum IoOutput {
    Success,
    SpaceNames(Vec<String>),
    KeyNames(Vec<String>),
}
