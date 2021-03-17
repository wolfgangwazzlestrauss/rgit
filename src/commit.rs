use crate::object::ObjectType;
use crate::{object, tree};
use std::convert::TryFrom;
use std::path::Path;
use std::{fs, io, str};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CommitInvalid {
    #[error("Content does not have enough lines")]
    NotEnoughLines,
    #[error("{0}: Line does not have an associated hash")]
    MissingHash(String),
    #[error("Line {0} should not be empty")]
    EmptyLine(usize),
    #[error("{0}: Incorrect line marker")]
    WrongMarker(String),
}

#[derive(Debug, Eq, PartialEq)]
pub struct Commit {
    message: String,
    parent: Option<String>,
    tree: String,
}

impl TryFrom<&str> for Commit {
    type Error = CommitInvalid;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let mut lines = value.lines();

        let line = lines.next().ok_or(CommitInvalid::NotEnoughLines)?.trim();
        let mut parts = line.split_ascii_whitespace();
        let tree = match parts.next() {
            Some("tree") => parts
                .next()
                .ok_or_else(|| CommitInvalid::MissingHash(line.into()))?
                .to_string(),
            Some(marker) => return Err(CommitInvalid::WrongMarker(marker.to_string())),
            None => return Err(CommitInvalid::EmptyLine(0)),
        };

        let line = lines.next().ok_or(CommitInvalid::NotEnoughLines)?.trim();
        let mut parts = line.split_ascii_whitespace();
        let parent = match parts.next() {
            Some("parent") => {
                lines.next();

                Some(
                    parts
                        .next()
                        .ok_or_else(|| CommitInvalid::MissingHash(line.into()))?
                        .to_string(),
                )
            }
            Some(marker) => return Err(CommitInvalid::WrongMarker(marker.into())),
            None => None,
        };

        let message = lines
            .next()
            .ok_or(CommitInvalid::NotEnoughLines)?
            .trim()
            .to_string();

        Ok(Commit {
            message,
            parent,
            tree,
        })
    }
}

/// Find commit in repository and parse content.
pub fn get_commit(repo: &Path, hash: &[u8]) -> eyre::Result<Commit> {
    let content = object::cat_file(repo, hash)?;
    Ok(Commit::try_from(str::from_utf8(&content)?)?)
}

/// Get hash of repository head if it exists.
fn get_head(repo: &Path) -> io::Result<Option<Vec<u8>>> {
    match fs::read(repo.join(".rgit/HEAD")) {
        Ok(head) => Ok(Some(head)),
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => Ok(None),
            _ => Err(error),
        },
    }
}

/// Get log repository commits in most recent order.
fn _log(repo: &Path) -> io::Result<String> {
    let _hash = match get_head(repo)? {
        Some(hash) => hash,
        None => return Ok(String::new()),
    };

    Ok(String::new())
}

/// Set hash for repository head.
fn set_head(repo: &Path, hash: &[u8]) -> io::Result<()> {
    Ok(fs::write(repo.join(".rgit/HEAD"), hash)?)
}

/// Save commit to version control as an object.
pub fn write_commit(repo: &Path, folder: &Path, message: &str) -> eyre::Result<Vec<u8>> {
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

#[cfg(test)]
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
            parent: Some(String::from(
                "82220d0a23b54286a6bc32a9f7e882e1174fca57352c80e0a61483a16ffac46f",
            )),
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
