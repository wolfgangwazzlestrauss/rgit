use crate::object::ObjectType;
use crate::{object, tree};
use anyhow::{anyhow, Result};
use std::fs;
use std::io;
use std::path::Path;
use std::str;

pub struct Commit {
    message: String,
    parent: Option<String>,
    tree: String,
}

impl Commit {
    pub fn from_str(content: &str) -> Result<Commit> {
        let mut lines = content.lines();

        let mut parts = lines
            .next()
            .ok_or(anyhow!("Missing first line."))?
            .split_ascii_whitespace();
        let tree = match parts.next() {
            Some("tree") => parts
                .next()
                .ok_or(anyhow!("Missing tree hash"))?
                .to_string(),
            Some(_) => Err(anyhow!("Incorrect first line."))?,
            None => Err(anyhow!("Missing tree hash."))?,
        };

        let mut parts = lines
            .next()
            .ok_or(anyhow!("Missing second line."))?
            .split_ascii_whitespace();
        let parent = match parts.next() {
            Some("parent") => {
                lines.next();

                Some(
                    parts
                        .next()
                        .ok_or(anyhow!("Missing parent hash"))?
                        .to_string(),
                )
            }
            Some(_) => None,
            None => Err(anyhow!("Missing tree hash."))?,
        };

        let message = lines
            .next()
            .ok_or(anyhow!("Missing message line."))?
            .to_string();

        Ok(Commit {
            message: message,
            parent: parent,
            tree: tree,
        })
    }
}

pub fn get_commit(repo: &Path, hash: &[u8]) -> Result<Commit> {
    let content = object::cat_file(repo, hash)?;
    Commit::from_str(str::from_utf8(&content)?)
}

fn get_head(repo: &Path) -> io::Result<Option<Vec<u8>>> {
    match fs::read(repo.join(".rgit/HEAD")) {
        Ok(head) => Ok(Some(head)),
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => Ok(None),
            _ => Err(error),
        },
    }
}

fn log(repo: &Path) -> Result<String> {
    let hash = match get_head(repo)? {
        Some(hash) => hash,
        None => return Ok(String::new()),
    };

    Ok(String::new())
}

fn set_head(repo: &Path, hash: &[u8]) -> Result<()> {
    Ok(fs::write(repo.join(".rgit/HEAD"), hash)?)
}

pub fn write_commit(repo: &Path, folder: &Path, message: &str) -> Result<Vec<u8>> {
    let hash = tree::write_tree(repo, folder)?;
    let hash = str::from_utf8(&hash)?;

    let content = match get_head(repo)? {
        Some(parent) => {
            let parent = str::from_utf8(&parent)?;
            format!("tree {}\nparent {}\n\n{}\n", hash, parent, message)
        }
        None => format!("tree {}\n\n{}\n", hash, message),
    };

    let (hash, data) = object::hash_object(content.as_bytes(), &ObjectType::Blob)?;
    set_head(repo, &hash)?;

    fs::write(object::object_path(repo, &hash)?, data)?;
    Ok(hash)
}
