//!  RGit is a personal implementation of [Git](https://git-scm.com/). It is a small project,
//!  desgined for learning the requirements of making a full featured command line appliction in the
//!  Rust programming language. RGit is implemented by following the [DIY Git in
//!  Python](https://www.leshenko.net/p/ugit/) blog posts and translating the Python code to
//!  idiomatic Rust code.

use anyhow::Result;
use std::fs;
use std::path::Path;

pub mod commit;
pub mod object;
pub mod tree;

/// Initialize version control directory.
pub fn init(repo: &Path) -> Result<()> {
    let vc_dir = repo.join(".rgit");
    let obj_dir = vc_dir.join("objects");

    fs::create_dir_all(obj_dir)?;

    Ok(())
}
