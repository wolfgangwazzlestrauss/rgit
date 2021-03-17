use crate::object::ObjectType;
use crate::{object, tree};
use anyhow::Result;
use std::fs;
use std::path::Path;
use std::str;

pub fn commit(repo: &Path, folder: &Path, message: &str) -> Result<Vec<u8>> {
    let hash_bytes = tree::write_tree(repo, folder)?;
    let hash = str::from_utf8(&hash_bytes)?;
    let content = format!("tree {}\n\n{}\n", hash, message);

    let (hash, data) = object::hash_object(content.as_bytes(), &ObjectType::Blob)?;

    fs::write(object::object_path(repo, &hash)?, data)?;
    Ok(hash)
}
