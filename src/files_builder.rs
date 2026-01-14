//! File Scaffolding Utility
//!
//! This module handles the creation of default boilerplate files for a new project.
//! It works in tandem with the directory builder to ensure that once folders are
//! created, they are populated with the necessary configuration and source files.
//!
//! ### Generated Files
//! | File Path | Description |
//! |-----------|-------------|
//! | `README.md` | Basic project documentation. |
//! | `pyproject.toml` | Build system requirements and metadata. |
//! | `.gitignore` | Standard patterns for Python and IDEs. |
//! | `src/<package>/__init__.py` | Init file for python package. |
//! | `src/<package>/main.py` | The main entry point for the application. |
//! | `src/<package>/env.py` | Environment loading boilerplate.
//! | `src/<package>/db.py` | Database connection boilerplate. |
//! | `config/DEV.yaml` | Development environment configuration. |
//! | `test/sample_test.py` | Placeholder for unittest. |
pub mod files_content;

use std::fs::File;
use std::io;
use std::io::prelude::Write;

/// Maps project file paths to their respective boilerplate content.
///
/// This internal function retrieves strings from [`files_content`] and performs
/// necessary string replacements (like inserting the `package_name` into the TOML).
///
/// Returns a [`Vec`] of tuples containing `(file_path, file_content)`.
fn get_files(root_name: &str, package_name: &str) -> Vec<(String, String)> {
    Vec::from([
        (
            format!("{root_name}/README.md"),
            files_content::SAMPLE_README.to_string(),
        ),
        (
            format!("{root_name}/pyproject.toml"),
            files_content::SAMPLE_PYPROJECT.replace("{}", package_name),
        ),
        (
            format!("{root_name}/.gitignore"),
            files_content::SAMPLE_GITIGNORE.to_string(),
        ),
        (
            format!("{root_name}/src/{package_name}/__init__.py"),
            files_content::SAMPLE_INIT.to_string(),
        ),
        (
            format!("{root_name}/src/{package_name}/env.py"),
            files_content::SAMPLE_ENV.to_string(),
        ),
        (
            format!("{root_name}/src/{package_name}/db.py"),
            files_content::SAMPLE_DB.to_string(),
        ),
        (
            format!("{root_name}/test/sample_test.py"),
            files_content::SAMPLE_TEST.to_string(),
        ),
        (
            format!("{root_name}/src/{package_name}/main.py"),
            files_content::SAMPLE_MAIN.to_string(),
        ),
        (
            format!("{root_name}/config/DEV.yaml"),
            files_content::SAMPLE_CONFIG.to_string(),
        ),
    ])
}

/// Populates the project structure with boilerplate files.
///
/// This function iterates through a predefined list of files and writes them
/// to the disk. It assumes the directory structure already exists.
///
/// # Arguments
///
/// * `root_name` - The name of the project root directory.
/// * `package_name` - The internal package name (used for the `src` subfolder).
/// * `verbose` - If true, prints a confirmation message to stdout for every file created.
///
/// # Errors
///
/// Returns an [`io::Error`] if:
/// * The target directory does not exist.
/// * The program lacks write permissions for the target paths.
/// * The disk is full or another I/O failure occurs during writing.
///
/// # Examples
///
/// ```no_run
/// use python_skeleton::files_builder::make_files;
///
/// fn main() -> std::io::Result<()> {
///     make_files("my_project", "my_app", true)?;
///     Ok(())
/// }
/// ```
pub fn make_files(root_name: &str, package_name: &str, verbose: bool) -> io::Result<()> {
    let files = get_files(root_name, package_name);
    for (file_name, content) in files.iter() {
        let mut file = File::create(file_name)?;
        file.write_all(content.as_bytes())?;
        if verbose {
            println!("Created file {}", file_name);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::make_files;
    use crate::dir_builder::make_dirs;
    use std::env::current_dir;
    use std::fs::remove_dir_all;

    #[test]
    fn test_file_creation() {
        let mut dir = current_dir().unwrap();
        assert!(make_dirs(&mut dir, "test-build", false, "test_build", false).is_ok());
        assert!(make_files("test-build", "test_build", false).is_ok());
        dir.push("test-build");
        let _ = remove_dir_all(dir);
    }
}
