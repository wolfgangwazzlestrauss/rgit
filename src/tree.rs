use crate::object;
use crate::object::ObjectType;
use anyhow::Result;
use std::fs;
use std::path::Path;

pub fn ignore(path: &Path) -> bool {
    if let Some(name) = path.to_str() {
        name == ".rgit"
    } else {
        false
    }
}

pub fn write_tree(repo: &Path, folder: &Path, object_type: &ObjectType) -> Result<()> {
    for entry in fs::read_dir(folder)? {
        let entry = entry?;
        let path = entry.path();

        if ignore(&path) {
            continue
        } else if path.is_dir() {
            write_tree(repo, &path, object_type)?;
        } else {
            object::hash_object(repo, &path, object_type)?;
        }
    }

    Ok(())
}
