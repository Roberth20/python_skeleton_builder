# Python skeleton builder

This CLI helps you quickly bootstrap a personalized Python project structure, with a focus on data science and machine learning workflows.

## Features
- Generates a standardized Python project structure
- Enforces naming conventions (Train-Case project, snake_case package)
- Optional documentation folder
- Fast, single-binary CLI written in Rust

## Project Structure
```
python-skeleton-builder/
|- README.md                           # Repo doc
|- CHANGELOG.md                        # Change registry between versions
|- Cargo.toml                          # Configuration of Rust package
|- src/                                # Source code
```

## Installation

### Prerequisites
- Rust (stable) – [https://rustup.rs](https://rustup.rs)

### Build from source
```bash
git clone https://github.com/Roberth20/python_skeleton_builder.git
cd python-skeleton-builder
cargo build --release
```

## Usage
Create a new project with documentation folder:
```bash
python-skeleton My-Project my_package --doc
```
This will generate
```
My-Project/
|-- docs/
|-- README.md
|-- pyproject.toml
|-- .gitignore
|-- test/
|---- sample_test.py 
|-- config/
|---- DEV.yaml
|-- notebooks/
|-- files/
|-- src/my_package/
|---- __init__.py
|---- env.py
|---- db.py 
|---- main.py
```

## Documentation
- CLI usage: see this README
- Developer documentation: `cargo doc --open`


## Testing
Run all tests:
```bash
cargo test
```
Run a specific test
```bash
cargo test test_name
```

## Contributing
- Use `cargo fmt` before committing
- Ensure `cargo clippy` passes without warnings
- Follow Conventional Commits

## Issues or TODO
- Add executable to repo and releases.
- Improve a little more the CLI messages and help menu.
- Add `--dry-run` option for testing, without creating files or directory
- Add `--force` to rewrite directory.
