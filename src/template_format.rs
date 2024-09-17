use std::collections::HashMap;

use handlebars::RenderError;
use serde::{Deserialize, Serialize};

use crate::{
    braces::{
        count_left_braces, count_right_braces, has_multiple_words_between_braces, has_no_braces,
        has_only_double_braces, has_only_single_braces,
    },
    role::InvalidRoleError,
};

#[derive(Debug)]
pub enum TemplateError {
    MalformedTemplate(String),
    UnsupportedFormat(String),
    MissingVariable(String),
    RenderError(RenderError),
    InvalidRoleError,
}

impl From<InvalidRoleError> for TemplateError {
    fn from(_: InvalidRoleError) -> Self {
        TemplateError::InvalidRoleError
    }
}

impl From<RenderError> for TemplateError {
    fn from(err: RenderError) -> Self {
        TemplateError::RenderError(err)
    }
}

impl std::fmt::Display for TemplateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TemplateError::MalformedTemplate(msg) => write!(f, "Malformed template: {}", msg),
            TemplateError::UnsupportedFormat(msg) => write!(f, "Unsupported format: {}", msg),
            TemplateError::MissingVariable(msg) => write!(f, "Missing variable: {}", msg),
            TemplateError::RenderError(err) => write!(f, "Render error: {}", err),
            TemplateError::InvalidRoleError => write!(f, "Invalid role error"),
        }
    }
}

impl std::error::Error for TemplateError {}

impl TemplateError {
    pub fn matches(&self, other: &TemplateError) -> bool {
        match (self, other) {
            (TemplateError::MissingVariable(a), TemplateError::MissingVariable(b)) => a == b,
            (TemplateError::MalformedTemplate(a), TemplateError::MalformedTemplate(b)) => a == b,
            (TemplateError::UnsupportedFormat(a), TemplateError::UnsupportedFormat(b)) => a == b,
            (TemplateError::RenderError(_), TemplateError::RenderError(_)) => true,
            (TemplateError::InvalidRoleError, TemplateError::InvalidRoleError) => true,
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum TemplateFormat {
    PlainText,
    FmtString,
    Mustache,
}

impl TryFrom<&str> for TemplateFormat {
    type Error = TemplateError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if !is_valid_template(s) {
            return Err(TemplateError::MalformedTemplate(
                "Malformed template".to_string(),
            ));
        }

        if is_fmtstring(s) {
            Ok(TemplateFormat::FmtString)
        } else if is_mustache(s) {
            Ok(TemplateFormat::Mustache)
        } else if is_plain_text(s) {
            Ok(TemplateFormat::PlainText)
        } else {
            Err(TemplateError::UnsupportedFormat(
                "Unsupported template format".to_string(),
            ))
        }
    }
}

pub fn is_plain_text(s: &str) -> bool {
    has_no_braces(s)
}

pub fn is_mustache(s: &str) -> bool {
    has_only_double_braces(s) && !has_multiple_words_between_braces(s)
}

pub fn is_fmtstring(s: &str) -> bool {
    has_only_single_braces(s) && !has_multiple_words_between_braces(s)
}

pub fn is_valid_template(s: &str) -> bool {
    if has_no_braces(s) {
        return true;
    }

    count_left_braces(s) == count_right_braces(s)
        && (has_only_double_braces(s) || has_only_single_braces(s))
}

pub fn validate_template(s: &str) -> Result<(), TemplateError> {
    if !is_valid_template(s) {
        return Err(TemplateError::MalformedTemplate(s.to_string()));
    }

    Ok(())
}

pub fn detect_template(s: &str) -> Result<TemplateFormat, TemplateError> {
    if is_plain_text(s) {
        Ok(TemplateFormat::PlainText)
    } else if is_mustache(s) {
        Ok(TemplateFormat::Mustache)
    } else if is_fmtstring(s) {
        Ok(TemplateFormat::FmtString)
    } else {
        Err(TemplateError::UnsupportedFormat(s.to_string()))
    }
}

pub fn merge_vars<'a>(
    partials: &'a HashMap<String, String>,
    runtime_vars: &HashMap<&'a str, &'a str>,
) -> HashMap<&'a str, &'a str> {
    partials
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .chain(runtime_vars.iter().map(|(&k, &v)| (k, v)))
        .collect()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_is_plain_text() {
        assert!(is_plain_text("No placeholders"));
        assert!(is_plain_text("This has no placeholders"));

        assert!(!is_plain_text("{var}"));
        assert!(!is_plain_text("{{var}}"));
        assert!(!is_plain_text("{var words another}}"));
    }

    #[test]
    fn test_is_mustache() {
        assert!(is_mustache("{{var}}"));
        assert!(is_mustache("{{var}} words {{ another }}"));

        assert!(!is_mustache("{var}"));
        assert!(!is_mustache("This has no placeholders"));
        assert!(!is_mustache("{{var"));
        assert!(!is_mustache("var}}"));
        assert!(!is_mustache("{var} words {{another}}"));
        assert!(!is_mustache("{{ hello world }}"));
    }

    #[test]
    fn test_is_fmtstring() {
        assert!(is_fmtstring("{var}"));
        assert!(is_fmtstring("Here is a {var}"));
        assert!(is_fmtstring("{var} and { another }"));

        assert!(!is_fmtstring("{{var}}"));
        assert!(!is_fmtstring("{{var}"));
        assert!(!is_fmtstring("{var}}"));
        assert!(!is_fmtstring("No placeholders"));
        assert!(!is_fmtstring("{var} words {{another}}"));
        assert!(!is_fmtstring("{ hello world }"));
    }

    #[test]
    fn test_is_valid_template() {
        assert!(is_valid_template("{var}"));
        assert!(is_valid_template("Here is a {var}"));
        assert!(is_valid_template("{var} and {another}"));
        assert!(is_valid_template("{{var}}"));
        assert!(is_valid_template("{{var}} words {{another}}"));

        assert!(!is_valid_template("{{var}"));
        assert!(!is_valid_template("{var}}"));
        assert!(!is_valid_template("{var} words {{another}}"));

        assert!(is_valid_template("No placeholders"));
    }

    #[test]
    fn test_detect_template() {
        assert_eq!(
            detect_template("No placeholders").unwrap(),
            TemplateFormat::PlainText
        );

        assert_eq!(detect_template("{var}").unwrap(), TemplateFormat::FmtString);
        assert_eq!(
            detect_template("Here is a {var}").unwrap(),
            TemplateFormat::FmtString
        );
        assert_eq!(
            detect_template("{var} and {another}").unwrap(),
            TemplateFormat::FmtString
        );
        assert_eq!(
            detect_template("{{var}}").unwrap(),
            TemplateFormat::Mustache
        );
        assert_eq!(
            detect_template("{{var}} and {{another}}").unwrap(),
            TemplateFormat::Mustache
        );

        assert!(detect_template("{var words}")
            .unwrap_err()
            .matches(&TemplateError::UnsupportedFormat("{var words}".to_string())));
    }

    #[test]
    fn test_validate_template() {
        assert!(validate_template("{var}").is_ok());
        assert!(validate_template("Here is a {var}").is_ok());
        assert!(validate_template("{{var}}").is_ok());
        assert!(validate_template("This is a {{valid}} Mustache template").is_ok());
        assert!(validate_template("No placeholders here").is_ok());

        assert!(validate_template("{{var}")
            .unwrap_err()
            .matches(&TemplateError::MalformedTemplate("{{var}".to_string())));

        assert!(validate_template("{var}}")
            .unwrap_err()
            .matches(&TemplateError::MalformedTemplate("{var}}".to_string())));

        assert!(validate_template("{var} words {{another}}")
            .unwrap_err()
            .matches(&TemplateError::MalformedTemplate(
                "{var} words {{another}}".to_string()
            )));
    }

    #[test]
    fn test_try_from_template_format() {
        assert_eq!(
            TemplateFormat::try_from("{name}").unwrap(),
            TemplateFormat::FmtString
        );

        assert_eq!(
            TemplateFormat::try_from("{{name}}").unwrap(),
            TemplateFormat::Mustache
        );

        assert_eq!(
            TemplateFormat::try_from("Hello, world!").unwrap(),
            TemplateFormat::PlainText
        );

        let result = TemplateFormat::try_from("{name {{other}}");
        match result {
            Err(TemplateError::MalformedTemplate(msg)) => {
                assert_eq!(msg, "Malformed template".to_string());
            }
            _ => panic!("Expected MalformedTemplate error"),
        }

        let result = TemplateFormat::try_from("{ name age }");
        match result {
            Err(TemplateError::UnsupportedFormat(msg)) => {
                assert_eq!(msg, "Unsupported template format".to_string());
            }
            e => panic!("Expected UnsupportedFormat error. Got error: {:?}", e),
        }
    }

    #[test]
    fn test_merge_vars_both_non_empty() {
        let mut partials = HashMap::new();
        partials.insert("name".to_string(), "Alice".to_string());
        partials.insert("day".to_string(), "Sunday".to_string());

        let mut runtime_vars = HashMap::new();
        runtime_vars.insert("day", "Monday"); // Should overwrite partial
        runtime_vars.insert("time", "Morning");

        let merged = merge_vars(&partials, &runtime_vars);

        assert_eq!(merged.get("name"), Some(&"Alice"));
        assert_eq!(merged.get("day"), Some(&"Monday")); // Overwritten by runtime var
        assert_eq!(merged.get("time"), Some(&"Morning"));
        assert_eq!(merged.len(), 3);
    }

    #[test]
    fn test_merge_vars_only_partials() {
        let mut partials = HashMap::new();
        partials.insert("name".to_string(), "Alice".to_string());
        partials.insert("day".to_string(), "Sunday".to_string());

        let runtime_vars = HashMap::new(); // Empty runtime vars

        let merged = merge_vars(&partials, &runtime_vars);

        assert_eq!(merged.get("name"), Some(&"Alice"));
        assert_eq!(merged.get("day"), Some(&"Sunday"));
        assert_eq!(merged.len(), 2);
    }

    #[test]
    fn test_merge_vars_only_runtime_vars() {
        let partials = HashMap::new(); // Empty partials

        let mut runtime_vars = HashMap::new();
        runtime_vars.insert("day", "Monday");
        runtime_vars.insert("time", "Morning");

        let merged = merge_vars(&partials, &runtime_vars);

        assert_eq!(merged.get("day"), Some(&"Monday"));
        assert_eq!(merged.get("time"), Some(&"Morning"));
        assert_eq!(merged.len(), 2);
    }

    #[test]
    fn test_merge_vars_both_empty() {
        let partials = HashMap::new(); // Empty partials
        let runtime_vars = HashMap::new(); // Empty runtime vars

        let merged = merge_vars(&partials, &runtime_vars);

        assert!(merged.is_empty());
    }

    #[test]
    fn test_merge_vars_runtime_overwrites_partial() {
        let mut partials = HashMap::new();
        partials.insert("var".to_string(), "PartialValue".to_string());

        let mut runtime_vars = HashMap::new();
        runtime_vars.insert("var", "RuntimeValue"); // Should overwrite partial

        let merged = merge_vars(&partials, &runtime_vars);

        assert_eq!(merged.get("var"), Some(&"RuntimeValue")); // Overwritten
        assert_eq!(merged.len(), 1);
    }

    #[test]
    fn test_merge_vars_runtime_with_no_conflict() {
        let mut partials = HashMap::new();
        partials.insert("name".to_string(), "Alice".to_string());

        let mut runtime_vars = HashMap::new();
        runtime_vars.insert("day", "Monday");

        let merged = merge_vars(&partials, &runtime_vars);

        assert_eq!(merged.get("name"), Some(&"Alice"));
        assert_eq!(merged.get("day"), Some(&"Monday"));
        assert_eq!(merged.len(), 2);
    }

    #[test]
    fn test_merge_vars_handles_empty_strings() {
        let mut partials = HashMap::new();
        partials.insert("name".to_string(), "".to_string()); // Empty partial value
        partials.insert("day".to_string(), "Sunday".to_string());

        let mut runtime_vars = HashMap::new();
        runtime_vars.insert("name", "Bob"); // Should overwrite empty partial
        runtime_vars.insert("time", "Morning");

        let merged = merge_vars(&partials, &runtime_vars);

        assert_eq!(merged.get("name"), Some(&"Bob")); // Overwritten
        assert_eq!(merged.get("day"), Some(&"Sunday")); // From partials
        assert_eq!(merged.get("time"), Some(&"Morning")); // From runtime vars
        assert_eq!(merged.len(), 3);
    }

    #[test]
    fn test_merge_vars_empty_strings_in_runtime() {
        let mut partials = HashMap::new();
        partials.insert("name".to_string(), "Alice".to_string());
        partials.insert("day".to_string(), "Sunday".to_string());

        let mut runtime_vars = HashMap::new();
        runtime_vars.insert("name", ""); // Empty string in runtime vars

        let merged = merge_vars(&partials, &runtime_vars);

        assert_eq!(merged.get("name"), Some(&"")); // Overwritten by empty runtime var
        assert_eq!(merged.get("day"), Some(&"Sunday")); // From partials
        assert_eq!(merged.len(), 2);
    }
}
