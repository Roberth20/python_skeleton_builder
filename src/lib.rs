//! # Project skeleton Library
//!
//! This crate provides a robust engine for generating personalized python
//! project structures. It handles name validation, directory tree generation,
//! and boilerplate file creation in a single coordinated workflow.
//!
//! ## Core Workflow
//! 1. **Validation**: Ensures project names follow `Train-Case` and package names follow `snake_case`.
//! 2. **Directory Creation**: Scaffolds the folder hierarchy.
//! 3. **File Creation**: Populates the folders with boilerplate (README, TOML, etc.).
//! 4. **Rollback**: If any step fails after the root directory is created, the library
//!    attempts to clean up the partial build to leave the filesystem in a clean state.
use std::env::current_dir;
use std::fs::{remove_dir, remove_dir_all};

pub mod dir_builder;
pub mod files_builder;
pub mod validation;

use validation::Case;

/// Errors that can occur during the project building process.
#[derive(Debug, PartialEq)]
pub enum BuildError {
    /// Encountered when a filesystem operation fails (permissions, missing paths, etc.).
    IOError,
    /// Encountered when a provided name does not match the required naming convention.
    NameError,
}

/// Orchestrates the creation of a new project skeleton.
///
/// This is the primary function of the library. It validates the inputs and coordinates
/// the `dir_builder` and `files_builder` modules.
///
/// # Arguments
///
/// * `project_name` - The name of the root directory (must be `Train-Case`).
/// * `pkg_name` - The name of the internal package (must be `snake_case`).
/// * `verbose` - If true, logs progress and validation steps to the console.
/// * `include_doc_dir` - Whether to include a `docs/` directory in the structure.
///
/// # Errors
///
/// Returns [`BuildError::NameError`] if:
/// * `project_name` is not valid Train-Case.
/// * `pkg_name` is not valid snake_case.
///
/// Returns [`BuildError::IOError`] if:
/// * The current working directory cannot be accessed.
/// * Directory or file creation fails.
///
/// # Examples
///
/// ```no_run
/// use python_skeleton::{build_skeleton, BuildError};
///
/// fn main() -> Result<(), BuildError> {
///     build_skeleton(
///         "my-awesome-project".to_string(),
///         "my_package".to_string(),
///         true,
///         true
///     )?;
///     Ok(())
/// }
/// ```
pub fn build_skeleton(
    project_name: String,
    pkg_name: String,
    verbose: bool,
    include_doc_dir: bool,
) -> Result<(), BuildError> {
    // Check project name.
    if verbose {
        println!("Validating `{}` as Train-Case", project_name);
    }
    let project_name = match validation::check_name(project_name, Case::TrainCase) {
        Ok(project_name) => project_name,
        Err(error) => {
            eprintln!("The name have an error: {error}");
            return Err(BuildError::NameError);
        }
    };
    // Check package name.
    if verbose {
        println!("Validating `{}` as snake_case", pkg_name);
    }
    let pkg_name = match validation::check_name(pkg_name, Case::SnakeCase) {
        Ok(pkg_name) => pkg_name,
        Err(error) => {
            eprintln!("The name have an error: {error}");
            return Err(BuildError::NameError);
        }
    };
    // Get safely current directory.
    let mut dir = match current_dir() {
        Ok(path) => path,
        Err(error) => {
            eprintln!("Can not get current directory: {error}");
            return Err(BuildError::IOError);
        }
    };
    // Make directories safely, delete all the created is error.
    if let Err(error) =
        dir_builder::make_dirs(&dir, &project_name, include_doc_dir, &pkg_name, verbose)
    {
        eprintln!("There was a prblem creating the directories: {error}");
        if verbose {
            println!("Falling back from directories creation");
        }
        dir.push(&project_name);
        let _ = remove_dir(dir);
        return Err(BuildError::IOError);
    }
    // Make the files safele, remove directories and files if an error.
    if let Err(error) = files_builder::make_files(&project_name, &pkg_name, verbose) {
        eprintln!("There was a problem creating the files. {error}");
        if verbose {
            println!("Falling back from files creation");
        }
        dir.push(&project_name);
        let _ = remove_dir_all(dir);
        return Err(BuildError::IOError);
    }

    Ok(())
}

#[cfg(test)]
pub mod tests {
    use super::build_skeleton;

    #[test]
    fn test_fail_name_build() {
        assert!(build_skeleton("01".to_string(), "test".to_string(), true, false).is_err());
        assert!(build_skeleton("test".to_string(), "test$".to_string(), true, false).is_err());
    }
}
