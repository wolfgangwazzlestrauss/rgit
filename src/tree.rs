use crate::object;
use crate::object::ObjectType;
use anyhow::Result;
use std::fs;
use std::path::Path;

pub fn ignore(path: &Path) -> bool {
    if let Some(file_name) = path.file_name() {
        file_name == ".rgit"
    } else {
        false
    }
}

pub fn write_tree(repo: &Path, folder: &Path, object_type: &ObjectType) -> Result<String> {
    for entry in fs::read_dir(folder)? {
        let entry = entry?;
        let path = entry.path();

        if ignore(&path) {
            continue;
        } else if path.is_dir() {
            write_tree(repo, &path, object_type)?;
        } else {
            object::hash_object(repo, &path, object_type)?;
        }
    }

    Ok(String::from(""))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    /// Paths are marked as ignore based on their file name.
    #[rstest(
        path_str,
        expected,
        case("file.md", false),
        case(".rgit", true),
        case("parent/.rgit", true)
    )]
    fn ignore_path_name(path_str: &str, expected: bool) {
        let actual = ignore(Path::new(path_str));
        assert_eq!(actual, expected);
    }
}
