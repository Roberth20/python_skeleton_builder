//! Directory Builder Utility
//!
//! This module provides functionality to scaffold a standardized project directory structure.
//! It is designed to automate the creation of folders for source code, configuration,
//! tests, and documentation.
//!
//! ### Default Structure
//! The following structure is created relative to the provided parent path:
//! ```text
//! <root_name>/
//! ├── config/
//! ├── files/
//! ├── notebooks/
//! ├── test/
//! ├── src/
//! │   └── <package_name>/
//! └── docs/ (optional)
//! ```
use std::fs::DirBuilder;
use std::io;
use std::path::PathBuf;

/// Generates the list of directory paths required for the project structure.
///
/// This is an internal helper function used by [`make_dirs`].
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

/// Creates a standardized python directory tree on the file system.
///
/// This function iterates through the required project directories and creates them
/// using [`DirBuilder`].
///
/// # Arguments
///
/// * `parent_dir` - The base path where the project root will be created.
/// * `root_name` - The name of the project root directory.
/// * `docs` - A boolean flag; if true, a `docs/` folder will be created.
/// * `package_name` - The name of the package inside the `src/` directory.
/// * `verbose` - A boolean flag, if true, print a message of current direcoty build
///
/// # Errors
///
/// This function will return an [`io::Error`] if:
/// * The program lacks permissions to create directories in the `parent_dir`.
/// * A file already exists at one of the paths where a directory is being created.
///
/// # Examples
///
/// ```no_run
/// use std::path::PathBuf;
/// use python_skeleton::dir_builder::make_dirs;
///
/// fn main() -> std::io::Result<()> {
///     let path = PathBuf::from("./projects");
///     make_dirs(&path, "my_new_project", true, "my_package", false)?;
///     Ok(())
/// }
/// ```
pub fn make_dirs(
    parent_dir: &PathBuf,
    root_name: &str,
    docs: bool,
    package_name: &str,
    verbose: bool,
) -> io::Result<()> {
    let dirs_names = get_dirs(root_name, docs, package_name);
    let dir_builder = DirBuilder::new();
    for dir_name in dirs_names {
        // Clone `parent_dir` to not edit the original path
        let mut parent_copy = parent_dir.clone();
        parent_copy.push(&dir_name);
        if verbose {
            println!("Creating directory: {}", parent_copy.display());
        }
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
        assert!(make_dirs(&mut dir, "test-build", false, "test_build", false).is_ok());
        dir.push("test-build");
        let _ = remove_dir_all(dir);
    }
}
