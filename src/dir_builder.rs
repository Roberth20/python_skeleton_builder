use std::fs::DirBuilder;
use std::io;
use std::path::PathBuf;

fn get_dirs(root_name: &str, docs: bool, package_name: &str) -> Vec<String> {
    let mut dirs = Vec::from([
        root_name.to_string(),
        format!("{root_name}/config"),
        format!("{root_name}/files"),
        format!("{root_name}/notebooks"),
        format!("{root_name}/test"),
        format!("{root_name}/src"),
        format!("{root_name}/src/{package_name}"),
    ]);
    if docs {
        dirs.push(format!("{root_name}/docs"));
    }
    dirs
}

pub fn make_dirs(
    parent_dir: &PathBuf,
    root_name: &str,
    docs: bool,
    package_name: &str,
) -> io::Result<()> {
    let dirs_names = get_dirs(root_name, docs, package_name);
    let dir_builder = DirBuilder::new();
    for dir_name in dirs_names {
        let mut parent_copy = parent_dir.clone();
        parent_copy.push(&dir_name);
        println!("Creating dir: {}", parent_copy.display());
        let result = dir_builder.create(parent_copy);
        if result.is_err() {
            return result;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::current_dir;
    use std::fs::remove_dir_all;

    #[test]
    fn test_make_directories() {
        let mut dir = current_dir().unwrap();
        assert!(make_dirs(&mut dir, "test-build", false, "test_build").is_ok());
        dir.push("test-build");
        remove_dir_all(dir);
    }
}
