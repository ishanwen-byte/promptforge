use crate::{braces::has_multiple_words_between_braces, TemplateError};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

lazy_static! {
    static ref IDENTIFIER_RE: Regex = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap();
}

pub fn is_valid_identifier(s: &str) -> bool {
    IDENTIFIER_RE.is_match(s)
}

pub fn extract_variables(template: &str) -> Vec<String> {
    let re = Regex::new(r"\{{1,2}([^}]+)\}{1,2}").unwrap();
    let mut unique_vars = HashSet::new();
    let mut result = Vec::new();

    for cap in re.captures_iter(template) {
        let var = cap[1].trim();
        if is_valid_identifier(var)
            && !has_multiple_words_between_braces(var)
            && unique_vars.insert(var.to_string())
        {
            result.push(var.to_string());
        }
    }

    result
}

pub fn extract_placeholder_variable(template: &str) -> Result<String, TemplateError> {
    let variables = extract_variables(template);

    if variables.len() == 1 {
        Ok(variables[0].clone())
    } else {
        Err(TemplateError::MalformedTemplate(
            "Template must contain exactly one placeholder variable.".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_identifier() {
        assert!(is_valid_identifier("variable"));
        assert!(is_valid_identifier("_variable"));
        assert!(is_valid_identifier("variable123"));
        assert!(is_valid_identifier("var_123"));
        assert!(is_valid_identifier("_var_123"));

        assert!(!is_valid_identifier("123variable"));
        assert!(!is_valid_identifier("var-123"));
        assert!(!is_valid_identifier("var!"));
        assert!(!is_valid_identifier("var 123"));
        assert!(!is_valid_identifier(""));

        assert!(!is_valid_identifier("var@name"));
        assert!(!is_valid_identifier("var#name"));
        assert!(!is_valid_identifier("1variable"));
    }

    fn check_variables(template: &str, expected_vars: Vec<&str>) {
        let extracted_vars = extract_variables(template);
        assert_eq!(extracted_vars, expected_vars);
    }

    #[test]
    fn test_extract_fmtstring_variables() {
        check_variables("{var}", vec!["var"]);
        check_variables("Hello {name}", vec!["name"]);
        check_variables("{var1} and { var2 }", vec!["var1", "var2"]);
        check_variables("{var} and {var}", vec!["var"]);

        check_variables("{{ var }}", vec!["var"]);
        check_variables("Hello {{name}}", vec!["name"]);
        check_variables("{{var1}} and {{ var2 }}", vec!["var1", "var2"]);
        check_variables("{{var}} and {{ var }}", vec!["var"]);

        check_variables("No variables here", vec![]);
        check_variables("{}", vec![]);
        check_variables("{{}}", vec![]);

        check_variables("{123invalid}", vec![]);
        check_variables("{var with spaces}", vec![]);
        check_variables("{{var!invalid}}", vec![]);
        check_variables("{!@#}", vec![]);

        check_variables("{var_with_underscores}", vec!["var_with_underscores"]);
        check_variables("{_leading_underscore}", vec!["_leading_underscore"]);
        check_variables("{}", vec![]);
        check_variables("{", vec![]);
        check_variables("}", vec![]);
        check_variables("{var} end {other_var}", vec!["var", "other_var"]);

        check_variables("{var_123}", vec!["var_123"]);
        check_variables("{var123}", vec!["var123"]);
    }
}
