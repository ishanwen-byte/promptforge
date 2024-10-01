use serde::{Deserialize, Serialize};

use crate::{extract_placeholder_variable, TemplateError};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessagesPlaceholder {
    variable_name: String,
    optional: bool,
    n_messages: usize,
}

impl MessagesPlaceholder {
    pub const DEFAULT_LIMIT: usize = 100;

    pub fn new(variable_name: String) -> Self {
        Self::with_options(variable_name, false, Self::DEFAULT_LIMIT)
    }

    pub fn with_options(variable_name: String, optional: bool, n_messages: usize) -> Self {
        Self {
            variable_name,
            optional,
            n_messages: if n_messages < 1 {
                Self::DEFAULT_LIMIT
            } else {
                n_messages
            },
        }
    }

    pub fn variable_name(&self) -> &str {
        &self.variable_name
    }

    pub fn optional(&self) -> bool {
        self.optional
    }

    pub fn n_messages(&self) -> usize {
        self.n_messages
    }
}

impl TryFrom<&str> for MessagesPlaceholder {
    type Error = TemplateError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let placeholder_variable = extract_placeholder_variable(s)?;
        Ok(MessagesPlaceholder::new(placeholder_variable))
    }
}

impl TryFrom<String> for MessagesPlaceholder {
    type Error = TemplateError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let placeholder_variable = extract_placeholder_variable(&s)?;
        Ok(MessagesPlaceholder::new(placeholder_variable))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_messages_placeholder_new() {
        let placeholder = MessagesPlaceholder::new("history".to_string());

        assert_eq!(placeholder.variable_name, "history");
        assert!(!placeholder.optional);
        assert_eq!(placeholder.n_messages, MessagesPlaceholder::DEFAULT_LIMIT);
    }

    #[test]
    fn test_messages_placeholder_with_options() {
        let placeholder = MessagesPlaceholder::with_options("history".to_string(), true, 50);

        assert_eq!(placeholder.variable_name, "history");
        assert!(placeholder.optional);
        assert_eq!(placeholder.n_messages, 50);
    }

    #[test]
    fn test_messages_placeholder_with_zero_limit() {
        let placeholder = MessagesPlaceholder::with_options("history".to_string(), false, 0);

        assert_eq!(placeholder.variable_name, "history");
        assert!(!placeholder.optional);
        assert_eq!(placeholder.n_messages, MessagesPlaceholder::DEFAULT_LIMIT);
    }

    #[test]
    fn test_messages_placeholder_default_limit_on_zero() {
        let placeholder = MessagesPlaceholder::with_options("history".to_string(), true, 0);

        assert_eq!(placeholder.variable_name, "history");
        assert!(placeholder.optional);
        assert_eq!(placeholder.n_messages, MessagesPlaceholder::DEFAULT_LIMIT);
    }

    #[test]
    fn test_tryfrom_valid_placeholder() {
        let template = "{history}";
        let placeholder = MessagesPlaceholder::try_from(template).unwrap();

        assert_eq!(placeholder.variable_name(), "history");
        assert!(!placeholder.optional());
        assert_eq!(placeholder.n_messages(), MessagesPlaceholder::DEFAULT_LIMIT);
    }

    #[test]
    fn test_tryfrom_multiple_placeholders_should_fail() {
        let template = "{name} {history}";
        let result = MessagesPlaceholder::try_from(template);

        assert!(result.is_err());
        if let Err(e) = result {
            match e {
                TemplateError::MalformedTemplate(msg) => {
                    assert_eq!(
                        msg,
                        "Template must contain exactly one placeholder variable."
                    );
                }
                _ => panic!("Expected MalformedTemplate error."),
            }
        } else {
            panic!("Expected error for multiple placeholders.");
        }
    }

    #[test]
    fn test_tryfrom_no_placeholders_should_fail() {
        let template = "No placeholders here";
        let result = MessagesPlaceholder::try_from(template);

        assert!(result.is_err());
        if let Err(e) = result {
            match e {
                TemplateError::MalformedTemplate(msg) => {
                    assert_eq!(
                        msg,
                        "Template must contain exactly one placeholder variable."
                    );
                }
                _ => panic!("Expected MalformedTemplate error."),
            }
        } else {
            panic!("Expected error for no placeholders.");
        }
    }

    #[test]
    fn test_tryfrom_empty_placeholder_should_fail() {
        let template = "{}";
        let result = MessagesPlaceholder::try_from(template);

        assert!(result.is_err());
        if let Err(e) = result {
            match e {
                TemplateError::MalformedTemplate(msg) => {
                    assert_eq!(
                        msg,
                        "Template must contain exactly one placeholder variable."
                    );
                }
                _ => panic!("Expected MalformedTemplate error."),
            }
        } else {
            panic!("Expected error for empty placeholder.");
        }
    }

    #[test]
    fn test_tryfrom_valid_optional_placeholder() {
        let template = "{history}";
        let mut placeholder = MessagesPlaceholder::try_from(template).unwrap();
        placeholder =
            MessagesPlaceholder::with_options(placeholder.variable_name().to_string(), true, 50);

        assert_eq!(placeholder.variable_name(), "history");
        assert!(placeholder.optional());
        assert_eq!(placeholder.n_messages(), 50);
    }
}
