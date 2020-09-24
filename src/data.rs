use anyhow::Result;
// use sha3::Sha3_256;
use std::fs;
use std::path::Path;

// pub fn hash_object(data: &str) -> Result<()> {
//     let mut hasher = Sha3_256::new();
//     hasher.update(data);
//     let digest = hasher.finalize();

//     let path = "";
//     fs::write(path, data)?;

//     Ok(())
// }

/// Initialize version control directory.
pub fn init(parent: &Path) -> Result<()> {
    let vc_dir = parent.join(".rgit");
    let obj_dir = vc_dir.join("objects");

    fs::create_dir_all(obj_dir)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Internal objects directory is created upon initialization.
    #[test]
    fn init_objects_directory() {
        let dir = tempfile::tempdir().unwrap().path().to_owned();
        init(&dir).unwrap();

        let obj_dir = dir.join(".rgit/objects");
        assert!(obj_dir.exists());
    }
}
