use crate::object::ObjectType;
use crate::{object, tree};
use anyhow::Result;
use std::io;
use std::fs;
use std::path::Path;
use std::str;

pub fn commit(repo: &Path, folder: &Path, message: &str) -> Result<Vec<u8>> {
    let hash = tree::write_tree(repo, folder)?;
    let hash = str::from_utf8(&hash)?;

    let content = match get_head(repo) {
        Ok(parent) => {
            let parent = str::from_utf8(&parent)?;
            format!("tree {}\nparent {}\n\n{}\n", hash, parent, message)
        },
        Err(error) => {
            match error.kind() {
                io::ErrorKind::NotFound => format!("tree {}\n\n{}\n", hash, message),
                _ => Err(error)?,
            }
        },
    };

    let (hash, data) = object::hash_object(content.as_bytes(), &ObjectType::Blob)?;
    set_head(repo, &hash)?;

    fs::write(object::object_path(repo, &hash)?, data)?;
    Ok(hash)
}

fn get_head(repo: &Path) -> io::Result<Vec<u8>> {
    fs::read(repo.join(".rgit/HEAD"))
}

fn set_head(repo: &Path, hash: &[u8]) -> Result<()> {
    Ok(fs::write(repo.join(".rgit/HEAD"), hash)?)
}
