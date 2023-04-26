use std::env::{self, VarError};

pub fn check_path() -> Result<String, VarError> {
    if let Ok(path) = env::var("_MAYOR_DB") {
        return Ok(path);
    } 

    Err(VarError::NotPresent)
}
