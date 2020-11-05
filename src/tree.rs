use crate::object;
use crate::object::ObjectType;
use anyhow::Result;
use std::fs;
use std::path::Path;

pub fn ignore(path: &Path) -> bool {
    path.components().any(|comp| comp.as_os_str() == ".rgit")
}

pub fn write_tree(repo: &Path, folder: &Path) -> Result<String> {
    for entry in fs::read_dir(folder)? {
        let entry = entry?;
        let path = entry.path();

        if ignore(&path) {
            continue;
        } else if path.is_dir() {
            object::hash_object(repo, &path, &ObjectType::Tree)?;
            write_tree(repo, &path)?;
        } else {
            object::hash_object(repo, &path, &ObjectType::Blob)?;
        }
    }

    Ok(String::from(""))
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
