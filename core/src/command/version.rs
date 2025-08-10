use crate::{error::Error, output::Output};

pub fn version() -> Result<Output, Error> {
    return Ok(Output::Version(env!("CARGO_PKG_VERSION").to_string()));
}
