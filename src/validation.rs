//! Utilities for validating and normalizing identifier-like names in specific
//! casing styles.
//!
//! This module supports two cases:
//!
//! - **SnakeCase**: lower-case letters with underscores (`_`).
//! - **TrainCase**: hyphen-separated words with each word starting in upper-case
//!   (e.g., `Sk-Learn`).
//!
//! The core entry point is [`check_name`], which validates an input string against
//! the requested [`Case`] and, if valid (or fixable), returns a normalized form.
//!
//! # Rules
//!
//! - **Numbers are not allowed** in any case; encountering a digit yields
//!   [`ErrorCase::NumberNotAllowed`].
//! - **Special characters** are restricted by case:
//!   - For [`Case::SnakeCase`], only alphabetic ASCII letters and `_` are allowed.
//!   - For [`Case::TrainCase`], only alphabetic ASCII letters and `-` are allowed.
//!   Any other character yields [`ErrorCase::SpecialCharNotAllowed`].
//!
//! # Normalization
//!
//! - [`Case::SnakeCase`]: the output is fully lowercased.
//! - [`Case::TrainCase`]: the input is lowercased first, then each segment
//!   (delimited by `-`) is capitalized by making its first character uppercase.
//!
//! # Examples
//!
//! ```rust
//! use python_skeleton::validation::{check_name, Case, ErrorCase};
//!
//! // SnakeCase: valid as-is
//! assert_eq!(check_name("sk_learn".into(), Case::SnakeCase).unwrap(), "sk_learn");
//!
////! // SnakeCase: fixable by lowercasing
//! assert_eq!(check_name("Sk_learn".into(), Case::SnakeCase).unwrap(), "sk_learn");
//!
//! // SnakeCase: invalid (contains '-')
//! assert_eq!(
//!     check_name("sk-learn".into(), Case::SnakeCase).unwrap_err(),
//!     ErrorCase::SpecialCharNotAllowed
//! );
//!
//! // TrainCase: normalized to "Sk-Learn"
//! assert_eq!(check_name("sk-learn".into(), Case::TrainCase).unwrap(), "Sk-Learn");
//!
//! // TrainCase: invalid (contains '_')
//! assert_eq!(
//!     check_name("sk_learn".into(), Case::TrainCase).unwrap_err(),
//!     ErrorCase::SpecialCharNotAllowed
//! );
//!
//! // Numbers are not allowed in either case
//! assert_eq!(
//!     check_name("sk_learn2".into(), Case::SnakeCase).unwrap_err(),
//!     ErrorCase::NumberNotAllowed
//! );
//! assert_eq!(
//!     check_name("sk-learn2".into(), Case::TrainCase).unwrap_err(),
//!     ErrorCase::NumberNotAllowed
//! );
//! ```

use std::fmt;

/// Specifies the target casing and character rules to validate and normalize
/// a name.
///
/// # Variants
///
/// - [`Case::SnakeCase`]: lower-case letters with underscores (`_`).
/// - [`Case::TrainCase`]: hyphen-separated words with each word starting
///   in upper-case (e.g., `Sk-Learn`).
///
/// See [`check_name`] for validation and normalization behavior.
///
/// # Examples
///
/// ```rust
/// use python_skeleton::validation::{Case, check_name};
///
/// assert_eq!(
///     check_name("Sk_learn".into(), Case::SnakeCase).unwrap(),
///     "sk_learn"
/// );
///
/// assert_eq!(
///     check_name("sk-learn".into(), Case::TrainCase).unwrap(),
///     "Sk-Learn"
/// );
/// ```
pub enum Case {
    SnakeCase,
    TrainCase,
}

/// Errors that can occur while validating a name for a given [`Case`].
///
/// # Variants
///
/// - [`ErrorCase::NumberNotAllowed`]: the input contained numeric digits.
/// - [`ErrorCase::SpecialCharNotAllowed`]: the input contained disallowed
///   special characters (anything other than `_` for SnakeCase or `-` for TrainCase).
///
/// # Examples
///
/// ```rust
/// use python_skeleton::validation::{check_name, Case, ErrorCase};
///
/// // Digit causes NumberNotAllowed
/// assert_eq!(
///     check_name("model2".into(), Case::SnakeCase).unwrap_err(),
///     ErrorCase::NumberNotAllowed
/// );
///
/// // Space causes SpecialCharNotAllowed
/// assert_eq!(
///     check_name("sk learn".into(), Case::TrainCase).unwrap_err(),
///     ErrorCase::SpecialCharNotAllowed
/// );
/// ```
#[derive(Debug, PartialEq)]
pub enum ErrorCase {
    NumberNotAllowed,
    SpecialCharNotAllowed,
}

impl fmt::Display for ErrorCase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ErrorCase::NumberNotAllowed => write!(f, "Numbers are not allowed!"),
            ErrorCase::SpecialCharNotAllowed => write!(f, "Only alphabetic characters are allowed"),
        }
    }
}

fn validate_name_snake(name: String) -> Result<String, ErrorCase> {
    for c in name.chars() {
        if c.is_numeric() {
            return Err(ErrorCase::NumberNotAllowed);
        }
        if !c.is_alphabetic() & (c != '_') {
            return Err(ErrorCase::SpecialCharNotAllowed);
        }
    }
    Ok(name.to_lowercase())
}

fn validate_name_train(name: String) -> Result<String, ErrorCase> {
    let mut upper_case = true;
    let mut new_name = String::new();
    for c in name.to_lowercase().chars() {
        if c.is_numeric() {
            return Err(ErrorCase::NumberNotAllowed);
        }
        if !c.is_alphabetic() & (c != '-') {
            return Err(ErrorCase::SpecialCharNotAllowed);
        }
        if upper_case {
            new_name.push(c.to_ascii_uppercase());
            upper_case = false;
            continue;
        }
        if c == '-' {
            upper_case = true;
        }
        new_name.push(c);
    }
    Ok(new_name)
}

/// Validates and normalizes `name` according to the requested [`Case`].
///
/// On success, returns a normalized string:
/// - [`Case::SnakeCase`]: returns the lowercased input if it contains only
///   alphabetic characters and underscores (`_`).
/// - [`Case::TrainCase`]: returns a title-cased, hyphen-separated form:
///   the input is first lowercased; then after each hyphen (`-`), the next
///   alphabetic character is uppercased.
///
/// # Errors
///
/// - Returns [`ErrorCase::NumberNotAllowed`] if `name` contains any numeric digits.
/// - Returns [`ErrorCase::SpecialCharNotAllowed`] if `name` contains disallowed
///   characters for the selected [`Case`].
///
/// # Complexity
///
/// Runs in linear time `O(n)` over the number of Unicode scalar values in `name`.
///
/// # Examples
///
/// ```rust
/// use python_skeleton::validation::{check_name, Case, ErrorCase};
///
/// // SnakeCase normalization
/// assert_eq!(check_name("Sk_learn".into(), Case::SnakeCase).unwrap(), "sk_learn");
/// assert_eq!(
///     check_name("sk-learn".into(), Case::SnakeCase).unwrap_err(),
///     ErrorCase::SpecialCharNotAllowed
/// );
///
/// // TrainCase normalization
/// assert_eq!(check_name("sk-learn".into(), Case::TrainCase).unwrap(), "Sk-Learn");
/// assert_eq!(
///     check_name("sk_learn".into(), Case::TrainCase).unwrap_err(),
///     ErrorCase::SpecialCharNotAllowed
/// );
/// ```
pub fn check_name(name: String, case: Case) -> Result<String, ErrorCase> {
    match case {
        Case::SnakeCase => validate_name_snake(name),
        Case::TrainCase => validate_name_train(name),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_name_snake() {
        let valid_name = String::from("sk_learn");
        let fixable_name = String::from("Sk_learn");

        assert_eq!(
            check_name(valid_name, Case::SnakeCase).ok().unwrap(),
            "sk_learn"
        );
        assert_eq!(
            check_name(fixable_name, Case::SnakeCase).ok().unwrap(),
            "sk_learn"
        );
    }

    #[test]
    fn test_invalid_name_snake() {
        let invalid_name_dash = String::from("sk-learn");
        let invalid_name_space = String::from("sk learn");
        let invalid_name_number = String::from("sk_learn2");

        assert_eq!(
            check_name(invalid_name_dash, Case::SnakeCase)
                .err()
                .unwrap(),
            ErrorCase::SpecialCharNotAllowed
        );
        assert_eq!(
            check_name(invalid_name_space, Case::SnakeCase)
                .err()
                .unwrap(),
            ErrorCase::SpecialCharNotAllowed
        );
        assert_eq!(
            check_name(invalid_name_number, Case::SnakeCase)
                .err()
                .unwrap(),
            ErrorCase::NumberNotAllowed
        );
    }

    #[test]
    fn test_valid_name_train() {
        let valid_name = String::from("Sk-Learn");
        let fixable_name = String::from("sk-learn");

        assert_eq!(
            check_name(valid_name, Case::TrainCase).ok().unwrap(),
            "Sk-Learn"
        );
        assert_eq!(
            check_name(fixable_name, Case::TrainCase).ok().unwrap(),
            "Sk-Learn"
        );
    }

    #[test]
    fn test_invalid_name_train() {
        let invalid_name_dash = String::from("sk_learn");
        let invalid_name_space = String::from("sk learn");
        let invalid_name_number = String::from("sk-learn2");

        assert_eq!(
            check_name(invalid_name_dash, Case::TrainCase)
                .err()
                .unwrap(),
            ErrorCase::SpecialCharNotAllowed
        );
        assert_eq!(
            check_name(invalid_name_space, Case::TrainCase)
                .err()
                .unwrap(),
            ErrorCase::SpecialCharNotAllowed
        );
        assert_eq!(
            check_name(invalid_name_number, Case::TrainCase)
                .err()
                .unwrap(),
            ErrorCase::NumberNotAllowed
        );
    }
}
