use anyhow::{anyhow, Result};
use sha3::{Digest, Sha3_256};
use std::convert::TryFrom;
use std::fmt::{self, Display, Formatter};
use std::fs;
use std::path::{Path, PathBuf};
use std::str;

#[derive(PartialEq)]
pub enum ObjectType {
    Blob,
    Tree,
}

impl ObjectType {
    /// Convert object type to associated bytes value.
    pub fn as_bytes(&self) -> &[u8] {
        match *self {
            ObjectType::Blob => b"blob",
            ObjectType::Tree => b"tree",
        }
    }
}

impl Display for ObjectType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            ObjectType::Blob => write!(f, "blob"),
            ObjectType::Tree => write!(f, "tree"),
        }
    }
}

impl TryFrom<&str> for ObjectType {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "blob" => Ok(ObjectType::Blob),
            "tree" => Ok(ObjectType::Tree),
            _ => Err(anyhow!("ObjectType only accepts values blob and tree.")),
        }
    }
}

impl TryFrom<&[u8]> for ObjectType {
    type Error = &'static str;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        match value {
            b"blob" => Ok(ObjectType::Blob),
            b"tree" => Ok(ObjectType::Tree),
            _ => Err("ObjectType only accepts values blob and tree."),
        }
    }
}

/// Retrieve file text from hashed objects collection.
pub fn cat_file(repo: &Path, hash: &[u8]) -> Result<Vec<u8>> {
    let bytes = fs::read(object_path(repo, hash)?)?;

    let mut parts = bytes.split(|&elem| elem == 0u8);
    let binary = parts
        .nth(1)
        .ok_or_else(|| anyhow!("Missing object type header."))?;

    Ok(binary.to_vec())
}

/// Save file to version control objects directory.
pub fn hash_file(repo: &Path, file: &Path, object_type: &ObjectType) -> Result<Vec<u8>> {
    let file_path = repo.join(file);
    let (hash, data) = hash_object(&fs::read(file_path)?, object_type)?;

    fs::write(object_path(repo, &hash)?, data)?;
    Ok(hash)
}

/// Save file to version control objects directory.
pub fn hash_object(bytes: &[u8], object_type: &ObjectType) -> Result<(Vec<u8>, Vec<u8>)> {
    let data = [object_type.as_bytes(), bytes].join(&0u8);

    let mut hasher = Sha3_256::new();
    hasher.update(&data);
    // TODO: Remove String allocation.
    let hash = format!("{:x}", hasher.finalize()).as_bytes().to_vec();
    Ok((hash, data))
}

pub fn object_path(repo: &Path, hash: &[u8]) -> Result<PathBuf> {
    Ok(repo.join(".rgit/objects").join(str::from_utf8(&hash)?))
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::{prop_assert_eq, proptest};
    use rstest::*;
    use std::path::PathBuf;
    use std::str;
    use tempfile;

    /// Create a repository and initialize it for version control.
    #[fixture]
    fn repository() -> PathBuf {
        let repo = tempfile::tempdir().unwrap().path().to_owned();
        fs::create_dir(&repo).unwrap();
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
        hash_file(&repository, &repository.join("code.txt"), &ObjectType::Blob).unwrap();

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

            let hash = hash_file(&repo, file_path, &ObjectType::Blob).unwrap();
            let bytes = cat_file(&repo, &hash).unwrap();

            let actual = str::from_utf8(&bytes).unwrap();
            prop_assert_eq!(actual, expected);
        }
    }
}
