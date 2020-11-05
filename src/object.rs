use anyhow::{anyhow, Result};
use sha3::{Digest, Sha3_256};
use std::fs;
use std::path::Path;

pub enum ObjectType {
    Blob,
    Tree,
}

impl ObjectType {
    /// Convert object type to associated string value.
    fn value(&self) -> &[u8] {
        match *self {
            ObjectType::Blob => b"blob",
            ObjectType::Tree => b"tree",
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
    use proptest::prelude::{prop_assert_eq, proptest};
    use rstest::*;
    use std::path::PathBuf;
    use std::str;

    /// Create a repository and initialize it for version control.
    #[fixture]
    fn repository() -> PathBuf {
        let repo = tempfile::tempdir().unwrap().path().to_owned();
        crate::init(&repo).unwrap();

        let text = "I am some mock text for a file.";
        let file_path = repo.join("code.txt");
        fs::write(file_path, text).unwrap();

        repo
    }

    /// Internal objects directory is created upon initialization.
    #[rstest]
    fn init_objects_directory(repository: PathBuf) {
        let obj_dir = repository.join(".rgit/objects");
        assert!(obj_dir.exists());
    }

    /// File is saved at location in version control directory based on known SHA-3 hash.
    #[rstest]
    fn hash_file_known_id(repository: PathBuf) {
        hash_object(&repository, &repository.join("code.txt"), &ObjectType::Blob).unwrap();

        let object_id = "7986d944ad3819fbd5431df6704a6aa1a24291b4f19b158b4ba127161ceacc24";
        let object_path = repository.join(".rgit/objects").join(object_id);

        assert!(object_path.exists());
    }

    proptest! {

        /// Original text and hashed object are the same.
        #[test]
        fn hash_invariant(expected in "\\PC*") {
            let repo = repository();

            let file_path = Path::new("hash_invariant.txt");
            fs::write(repo.join(file_path), &expected).unwrap();

            let hash = hash_object(&repo, file_path, &ObjectType::Blob).unwrap();
            let bytes = cat_file(&repo, &hash).unwrap();

            let actual = str::from_utf8(&bytes).unwrap();
            prop_assert_eq!(actual, expected);
        }
    }
}
