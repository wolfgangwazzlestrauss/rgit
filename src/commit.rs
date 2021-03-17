use crate::object::ObjectType;
use crate::{object, tree};
use anyhow::{anyhow, Result};
use std::convert::TryFrom;
use std::fs;
use std::io;
use std::path::Path;
use std::str;

#[derive(Debug, Eq, PartialEq)]
pub struct Commit {
    message: String,
    parent: Option<String>,
    tree: String,
}

impl TryFrom<&str> for Commit {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        let mut lines = value.lines();

        let mut parts = lines
            .next()
            .ok_or(anyhow!("Missing first line"))?
            .trim()
            .split_ascii_whitespace();
        let tree = match parts.next() {
            Some("tree") => parts
                .next()
                .ok_or(anyhow!("Missing tree hash"))?
                .to_string(),
            Some(_) => Err(anyhow!("Invalid first line"))?,
            None => Err(anyhow!("Missing tree marker"))?,
        };

        let mut parts = lines
            .next()
            .ok_or(anyhow!("Missing second line"))?
            .trim()
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
            Some(_) => Err(anyhow!("Invalid second line"))?,
            None => None,
        };

        let message = lines
            .next()
            .ok_or(anyhow!("Missing message line."))?
            .trim()
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
    Commit::try_from(str::from_utf8(&content)?)
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

mod tests {
    use super::*;

    #[test]
    fn try_from_empty_message() {
        let content = "\
            tree e2037a52b1d22a09e6ca23ef6a16f2ba201a366608a0af06654cbd6e09f2098a\n\
            parent 82220d0a23b54286a6bc32a9f7e882e1174fca57352c80e0a61483a16ffac46f\n\
            \n\
            \n\
        ";

        let expected = Commit {
            message: String::new(),
            parent: Some(String::from("82220d0a23b54286a6bc32a9f7e882e1174fca57352c80e0a61483a16ffac46f")),
            tree: String::from("e2037a52b1d22a09e6ca23ef6a16f2ba201a366608a0af06654cbd6e09f2098a"),
        };
        let actual = Commit::try_from(content).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn try_from_no_parent() {
        let content = "\
            tree e2037a52b1d22a09e6ca23ef6a16f2ba201a366608a0af06654cbd6e09f2098a\n\
            \n\
            Generate project layout\n\
        ";

        let expected = Commit {
            message: String::from("Generate project layout"),
            parent: None,
            tree: String::from("e2037a52b1d22a09e6ca23ef6a16f2ba201a366608a0af06654cbd6e09f2098a"),
        };
        let actual = Commit::try_from(content).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn try_from_with_parent() {
        let content = "\
            tree a56a019e3b1739c35063004884b8cb90012a1f8737eece7b3f7426dcbdf3c84a\n\
            parent 13a4dbddfa611cc36287e2ae9f1fd17e94409de4472251a937abc65114d657be\n\
            \n\
            Add a test with a parent commit\n\
        ";

        let expected = Commit {
            message: String::from("Add a test with a parent commit"),
            parent: Some(String::from(
                "13a4dbddfa611cc36287e2ae9f1fd17e94409de4472251a937abc65114d657be",
            )),
            tree: String::from("a56a019e3b1739c35063004884b8cb90012a1f8737eece7b3f7426dcbdf3c84a"),
        };
        let actual = Commit::try_from(content).unwrap();
        assert_eq!(actual, expected);
    }
}
