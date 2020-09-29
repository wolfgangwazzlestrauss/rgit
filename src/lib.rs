use anyhow::Result;
use std::fs;
use std::path::Path;

pub mod object;
pub mod tree;

/// Initialize version control directory.
pub fn init(repo: &Path) -> Result<()> {
    let vc_dir = repo.join(".rgit");
    let obj_dir = vc_dir.join("objects");

    fs::create_dir_all(obj_dir)?;

    Ok(())
}
