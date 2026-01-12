pub mod files_content;

use std::fs::File;
use std::io;
use std::io::prelude::Write;
use std::path::PathBuf;

fn get_files(root_name: &str, package_name: &str) -> Vec<(String, String)> {
    let mut files = Vec::from([
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
        //format!("{root_name}/src/{package_name}/logger.py"),
        (
            format!("{root_name}/test/sample_test.py"),
            files_content::SAMPLE_TEST.to_string(),
        ),
    ]);
    files
}

pub fn make_files(root_name: &str, package_name: &str) -> io::Result<()> {
    let files = get_files(root_name, package_name);
    for (file_name, content) in files.iter() {
        let mut file = File::create(file_name)?;
        file.write_all(content.as_bytes())?;
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
        assert!(make_dirs(&mut dir, "test-build", false, "test_build").is_ok());
        assert!(make_files("test-build", "test_build").is_ok());
        dir.push("test-build");
        remove_dir_all(dir);
    }
}
