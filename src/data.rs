use anyhow::Result;
use std::fs;

pub fn init() -> Result<()> {
    fs::create_dir(".rgit")?;
    Ok(())
}
