//! Files sampl contents
//! Here are allocated all the constant to fill the skeleton files.
pub const SAMPLE_README: &'static str = "\
# README's template for projects
A short tagline or description of what your project does.

## Project Structure
```
project-name/
|- src/                # Source code
|- tests/              # Unit tests
|- pyproject.toml      # Python dependencies and setup
|- README.md           # Project documentation
|- config/             # Configuration of environments
|- notebooks/          # Development notebooks
|- files/              # Data related to the project
```

## Installation
Instalation instructions goes here.

## Usage
An explanation of how to use the package.

## Running Tests
How to test the project, environments of package.

## Configuration
How to configure the packages

## Documentation
Where do I find information?

## Contributing
How do we work together?

## Issues
How to report something.

## License
Only if need it.
        ";

pub const SAMPLE_TEST: &'static str = "\
import pytest

def sample_test():
    # Test something
    pass
        ";

pub const SAMPLE_INIT: &'static str = "\
\"\"\"Packages initiator.

Loads the environment variables
\"\"\"

from .env import load_env

load_env()
        ";

pub const SAMPLE_GITIGNORE: &'static str = "\
# Python-generated files
**__pycache__**
*.py[oc]
build/
dist/
wheels/
*.egg-info

# Virtual environments
.venv

# jupyter checkpoints
**ipynb_checkpoints**
            ";

pub const SAMPLE_ENV: &'static str = "\
\"\"\"Load environment variables.\"\"\"

import os
from collections.abc import Iterable
from pathlib import Path
from typing import Optional

import yaml

def find_config_file(
        possible_names: Iterable[str] = (
                \"config.yaml\", 
                \"settings.yaml\", 
                \"DEV.yaml\")
        ) -> Optional[Path]:
    \"\"\"Searcher of configuration file.
    
    Start searching in current directory, then goes to the parents until fail or 
    find the possible file name.
    
    Parameters
    ----------
    possible_names: Iterable[str], default = (\"config.yaml\", \"settings.yaml\", \"DEV.yaml\")
        Possible names of YAML configuration file.

    Returns
    -------
    Optional[Path]
        Path where the files was found.
    \"\"\"
    cwd = Path.cwd()
    for parent in [cwd, *cwd.parents]:
        for name in possible_names:
            candidate = parent / \"config\" / name
            if candidate.exists():
                return candidate
    return None


def load_env(path: Optional[str | Path] = None):
    \"\"\"Load the environment variables from a YAML file.
    
    Parameters
    ----------
    path: Optional[str | Path].
        Path to configuration file. If None, automatically search for it.
    \"\"\"
    # When using as package, add the environment variable
    if path is None:
        path = find_config_file()
        if path is None:
            raise FileNotFoundError(\"It was not possible to find a configuration file.\")
    else:
        path = Path(path)
    # Load yaml file from the project root
    with open(path, \"r\") as f:
        config = yaml.safe_load(f)
        
    # Set environmetn variables
    for mk in config:
        for k in config[mk]:
            os.environ[k] = config[mk][k]
            ";

pub const SAMPLE_DB: &'static str = "\
\"\"\"Databases connections.

This module provides functionalities to build secure conecctions to databases.
Currently, only supports Oracle and configuring the secrets with environment 
variables.

Functions
---------
get_engine
    Function to create the engine to production database.
\"\"\"

import os
import sys

import oracledb
import sqlalchemy

STD_PRD = os.environ[\"DB_USER\"]
STD_PRD_PASS = os.environ[\"DB_PASSWORD\"]
STD_PRD_DSN = f\"{os.environ.get('DB_DATABASE')}/{os.environ.get('DB_HOST')}\"


def get_engine() -> sqlalchemy.Engine:
    \"\"\"Creates the Orecle connection engine.
        
    This functions build the connection to Oracle database using `oracledb` as 
    backend for SQLAlchemy.

    This function must be used as interaction gate with the database with the 
    engine object (`Engine`).

    Returns
    -------
    sqlalchemy.Engine
        Connection engine.

    Raises
    ------
    KeyError
        If a environment variable is missing (`DB_USER`,
        `DB_PASSWORD`, `DB_DATABASE`, `DB_HOST`).

    sqlalchemy.exc.SQLAlchemyError
        Some error from SQLAlchemy when building the engine.

    Notes
    -----
    The function ensure `oracledb` instead of legacy `cx_Oracle` as the `cx_Oracle` to 
    prevent compatibility problems with `oracle+oracledb` dialect with SQLAlchemy.

    Examples
    --------
    >>> engine = get_engine()
    >>> with engine.connect() as conn:
    ...     result = conn.execute(text(\"SELECT * FROM employers\"))
    ...     for row in result:
    ...         print(row)
    \"\"\"
    oracledb.version = \"8.3.0\"
    sys.modules[\"cx_Oracle\"] = oracledb
    engine = sqlalchemy.create_engine(
        \"oracle://:@\",
        connect_args={\"user\": STD_PRD, \"password\": STD_PRD_PASS, \"dsn\": STD_PRD_DSN},
    )
    return engine
              ";

pub static SAMPLE_PYPROJECT: &'static str = "\
[build-system]
requires = [\"setuptools >= 70.0\"]
build-backend = \"setuptools.build_meta\"

[project]
name = \"{}\"
version = \"0.1.0\"
description = \"Some description of the project.\"
readme = \"README.md\"
requires-python = \"==3.14.*\"
dependencies = [
    \"oracledb\",
    \"sqlalchemy\",
    \"numpy\",
    \"polars\",
    \"plotly\",
    \"structlog\"
]

# Scripts here
[project.scripts]

# Uv groups dependencies
[dependency-groups]
dev = [
    \"jupyterlab>=4.4.0\",
    \"pytest\",
    \"ipywidgets\",
]

[tool.ruff]
target-version = \"py314\"

[tool.ruff.lint]
extend-select = [\"SIM\", \"I\", \"D\", \"S\", \"PT\"]

[tool.ruff.lint.pydocstyle]
convention = \"numpy\"

[tool.ruff.lint.per-file-ignores]
\"test/*\" = [\"D\", \"s\"]
                                         ";

pub const SAMPLE_MAIN: &'static str = "\
\"\"\"Example of main file with logs.\"\"\"

import structlog
import polars as pl 

# This must be call in every file to log.
logger = structlog.get_logger()

df = pl.DataFrame({\"A\": [1, 2], \"B\": [3, 4]})
logger.info(\"Hello world!\", more_than_strings=df)
        ";

pub const SAMPLE_CONFIG: &'static str = "\
# Environment variables are splited if categories to make them easier
# to read.
DB:
    DB_USER: \"some_user\"
    DB_PASSWORD: \"some_password\"
    DB_HOST: \"some_host\"
    DB_DATABASE:\"some_service\"
        ";
