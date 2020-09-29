use anyhow::{anyhow, Result};
use sha3::{Digest, Sha3_256};
use std::fs;
use std::path::Path;

pub enum ObjectType {
    Blob,
}

impl ObjectType {
    /// Convert object type to associated string value.
    fn value(&self) -> &[u8] {
        match *self {
            ObjectType::Blob => b"blob",
        }
    }
}

/// Retrieve file text from hashed objects collection.
pub fn cat_file(repo: &Path, hash: &str) -> Result<Vec<u8>> {
    let path = repo.join(format!(".rgit/objects/{}", hash));
    let bytes = fs::read(path)?;

    let mut parts = bytes.split(|&elem| elem == 0u8);
    let binary = parts.nth(1).ok_or(anyhow!("Missing object type header."))?;

    Ok(binary.to_vec())
}

/// Save file to version control objects directory.
pub fn hash_object(repo: &Path, file: &Path, object_type: &ObjectType) -> Result<String> {
    let file_path = repo.join(file);
    let data = [object_type.value(), &fs::read(file_path)?].join(&0u8);

    let mut hasher = Sha3_256::new();
    hasher.update(&data);
    let hash = format!("{:x}", hasher.finalize());

    let object_path = repo.join(format!(".rgit/objects/{}", hash));
    fs::write(object_path, data)?;

    Ok(hash)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::str;

    /// Create a repository and initialize it for version control.
    fn repository() -> Result<PathBuf> {
        let repo = tempfile::tempdir()?.path().to_owned();
        crate::init(&repo)?;

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
        hash_object(&repo, &repo.join("code.txt"), &ObjectType::Blob).unwrap();

        let object_id = "7986d944ad3819fbd5431df6704a6aa1a24291b4f19b158b4ba127161ceacc24";
        let object_path = repo.join(".rgit/objects").join(object_id);

        assert!(object_path.exists());
    }

    /// Original text and hashed object are the same.
    #[test]
    fn hash_invariant() {
        let repo = repository().unwrap();

        let expected = "I am fake text for the hash invariant test.";
        let file_path = Path::new("hash_invariant.txt");
        fs::write(repo.join(file_path), expected).unwrap();

        let hash = hash_object(&repo, file_path, &ObjectType::Blob).unwrap();
        let bytes = cat_file(&repo, &hash).unwrap();

        let actual = str::from_utf8(&bytes).unwrap();
        assert_eq!(actual, expected);
    }
}
