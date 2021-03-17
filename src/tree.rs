use crate::object;
use crate::object::ObjectType;
use std::convert::TryInto;
use std::path::Path;
use std::{fs, str};

pub fn ignore(path: &Path) -> bool {
    path.components().any(|comp| comp.as_os_str() == ".rgit")
}

pub fn read_tree(repo: &Path, folder: &Path, hash: &[u8]) -> eyre::Result<()> {
    let bytes = fs::read(object::object_path(repo, &hash)?)?;

    let mut parts = bytes.split(|&elem| elem == 0u8);
    let binary = parts
        .nth(1)
        .ok_or_else(|| eyre::eyre!("Missing object type header."))?;

    for line in str::from_utf8(binary)?.split('\n') {
        let mut parts = line.split(' ');
        let object_type: ObjectType = parts.next().ok_or_else(|| eyre::eyre!(""))?.try_into()?;
        let hash: Vec<u8> = parts.next().ok_or_else(|| eyre::eyre!(""))?.try_into()?;
        let path = folder.join(parts.next().ok_or_else(|| eyre::eyre!(""))?);

        match object_type {
            ObjectType::Blob => {
                let data = object::cat_file(repo, &hash)?;
                fs::write(path, data)?;
            }
            ObjectType::Tree => {
                fs::create_dir(&path)?;
                read_tree(repo, &path, &hash)?;
            }
        }
    }

    Ok(())
}

pub fn write_tree(repo: &Path, folder: &Path) -> eyre::Result<Vec<u8>> {
    let mut objects: Vec<Vec<u8>> = Vec::new();

    for entry in fs::read_dir(folder)? {
        let object_id: Vec<u8>;
        let object_type: ObjectType;
        let path = entry?.path();

        if ignore(&path) {
            continue;
        }

        let file_name = match path.file_name() {
            Some(file_name) => file_name
                .to_str()
                .ok_or_else(|| eyre::eyre!("File path is not valid UTF-8."))?,
            None => continue,
        };

        if path.is_dir() {
            object_type = ObjectType::Tree;
            object_id = write_tree(repo, &path)?;
        } else {
            object_type = ObjectType::Blob;
            object_id = object::hash_file(repo, &path, &object_type)?;
        }

        let data = [object_type.as_bytes(), &object_id, file_name.as_bytes()].join(" ".as_bytes());
        objects.push(data);
    }

    let tree = objects.join("\n".as_bytes());
    let (hash, data) = object::hash_object(&tree, &ObjectType::Tree)?;

    fs::write(object::object_path(repo, &hash)?, data)?;
    Ok(hash)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    /// Paths are ignored if .rgit is in any part.
    #[rstest(
        path_str,
        expected,
        case("file.md", false),
        case(".rgit", true),
        case("parent/.rgit", true),
        case("parent/.rgit/", true),
        case("parent/.rgit/child", true)
    )]
    fn ignore_path_name(path_str: &str, expected: bool) {
        let actual = ignore(Path::new(path_str));
        assert_eq!(actual, expected);
    }
}
