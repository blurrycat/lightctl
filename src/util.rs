use std::fs;

use anyhow::Result;

pub fn read_u32_from_file(path: &str) -> Result<u32> {
    let content = fs::read_to_string(path)?;
    Ok(content.trim().parse()?)
}
