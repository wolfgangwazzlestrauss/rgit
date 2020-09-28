use anyhow::Result;
use sha3::{Digest, Sha3_256};
use std::fs;
use std::path::Path;

pub fn cat_file(repo: &Path, hash: &str) -> Result<()> {
    let object = get_object(repo, hash)?;
    println!("{}", object);

    Ok(())
}

fn get_object(repo: &Path, hash: &str) -> Result<String> {
    let path = repo.join(format!(".rgit/objects/{}", hash));
    let text = fs::read_to_string(path)?;

    Ok(text)
}

/// Save file to version control and print generated hash.
pub fn hash_object(repo: &Path, file: &Path) -> Result<()> {
    let text = fs::read(file)?;

    let object_id = hash_text(repo, &text)?;
    println!("{}", object_id);

    Ok(())
}

/// Save text to version control objects directory.
fn hash_text(repo: &Path, data: impl AsRef<[u8]>) -> Result<String> {
    let mut hasher = Sha3_256::new();
    hasher.update(&data);
    let obj_id = format!("{:x}", hasher.finalize());

    let path = repo.join(format!(".rgit/objects/{}", obj_id));
    fs::write(path, data)?;

    Ok(obj_id)
}

/// Initialize version control directory.
pub fn init(repo: &Path) -> Result<()> {
    let vc_dir = repo.join(".rgit");
    let obj_dir = vc_dir.join("objects");

    fs::create_dir_all(obj_dir)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    /// Create a repository and initialize it for version control.
    fn repository() -> Result<PathBuf> {
        let repo = tempfile::tempdir()?.path().to_owned();
        init(&repo)?;

        let text = "I am some mock text for a file.";
        let file_path = repo.join("code.txt");
        fs::write(file_path, text)?;

        Ok(repo)
    }

    /// Internal objects directory is created upon initialization.
    #[test]
    fn init_objects_directory() {
        let repo = repository().unwrap();

        let obj_dir = repo.join(".rgit/objects");
        assert!(obj_dir.exists());
    }

    /// File is saved at location in version control directory based on known SHA-3 hash.
    #[test]
    fn hash_file_known_id() {
        let repo = repository().unwrap();
        hash_object(&repo, &repo.join("code.txt")).unwrap();

        let object_id = "4173a5fc172c843e938d93bee53624eec976de67557832bbb5f3a03b7da6a7c2";
        let object_path = repo.join(".rgit/objects").join(object_id);

        assert!(object_path.exists());
    }
}
