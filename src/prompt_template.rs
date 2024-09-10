use std::collections::HashMap;

use handlebars::Handlebars;

use crate::placeholder::extract_variables;
use crate::template::Template;
use crate::template_format::{detect_template, validate_template, TemplateError, TemplateFormat};

#[derive(Debug)]
pub struct PromptTemplate {
    template: String,
    template_format: TemplateFormat,
    input_variables: Vec<String>,
    handlebars: Option<Handlebars<'static>>,
}

impl PromptTemplate {
    pub const MUSTACHE_TEMPLATE: &'static str = "mustache_template";

    pub fn new(tmpl: &str) -> Result<Self, TemplateError> {
        validate_template(tmpl)?;

        let template_format = detect_template(tmpl)?;
        let input_variables = extract_variables(tmpl);

        let handlebars = if template_format == TemplateFormat::Mustache {
            let handle = Self::initialize_handlebars(tmpl)?;
            Some(handle)
        } else {
            None
        };

        Ok(PromptTemplate {
            template: tmpl.to_string(),
            template_format,
            input_variables,
            handlebars,
        })
    }

    pub fn from_template(tmpl: &str) -> Result<Self, TemplateError> {
        Self::new(tmpl)
    }

    fn initialize_handlebars(tmpl: &str) -> Result<Handlebars<'static>, TemplateError> {
        let mut handlebars = Handlebars::new();
        handlebars
            .register_template_string(Self::MUSTACHE_TEMPLATE, tmpl)
            .map_err(|e| {
                TemplateError::MalformedTemplate(format!("Failed to register template: {}", e))
            })?;
        Ok(handlebars)
    }

    fn validate_variables(
        &self,
        variables: &std::collections::HashMap<&str, &str>,
    ) -> Result<(), TemplateError> {
        for var in &self.input_variables {
            if !variables.contains_key(var.as_str()) {
                return Err(TemplateError::MissingVariable(var.clone()));
            }
        }
        Ok(())
    }

    fn format_fmtstring(&self, variables: &HashMap<&str, &str>) -> Result<String, TemplateError> {
        let mut result = self.template.clone();

        for var in &self.input_variables {
            if let Some(value) = variables.get(var.as_str()) {
                let placeholder = format!("{{{}}}", var);
                result = result.replace(&placeholder, value);
            } else {
                return Err(TemplateError::MissingVariable(var.clone()));
            }
        }

        Ok(result)
    }

    fn format_mustache(&self, variables: &HashMap<&str, &str>) -> Result<String, TemplateError> {
        match &self.handlebars {
            None => Err(TemplateError::UnsupportedFormat(
                "Handlebars not initialized".to_string(),
            )),
            Some(handlebars) => handlebars
                .render(Self::MUSTACHE_TEMPLATE, variables)
                .map_err(|err| TemplateError::MissingVariable(err.to_string())),
        }
    }
}

impl Template for PromptTemplate {
    fn format(
        &self,
        variables: std::collections::HashMap<&str, &str>,
    ) -> Result<String, TemplateError> {
        self.validate_variables(&variables)?;

        match self.template_format {
            TemplateFormat::FmtString => self.format_fmtstring(&variables),
            TemplateFormat::Mustache => self.format_mustache(&variables),
            TemplateFormat::PlainText => Ok(self.template.clone()),
        }
    }

    fn template_format(&self) -> TemplateFormat {
        self.template_format.clone()
    }

    fn input_variables(&self) -> Vec<String> {
        self.input_variables.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prompt_vars;

    #[test]
    fn test_prompt_template_new_success() {
        let valid_template = "Tell me a {adjective} joke about {content}.";
        let tmpl = PromptTemplate::new(valid_template);
        assert!(tmpl.is_ok());
        let tmpl = tmpl.unwrap();
        assert_eq!(tmpl.template, valid_template);
        assert_eq!(tmpl.template_format, TemplateFormat::FmtString);
        assert_eq!(tmpl.input_variables, vec!["adjective", "content"]);

        let valid_mustache_template = "Tell me a {{adjective}} joke about {{content}}.";
        let tmpl = PromptTemplate::new(valid_mustache_template);
        assert!(tmpl.is_ok());
        let tmpl = tmpl.unwrap();
        assert_eq!(tmpl.template, valid_mustache_template);
        assert_eq!(tmpl.template_format, TemplateFormat::Mustache);
        assert_eq!(tmpl.input_variables, vec!["adjective", "content"]);

        let no_placeholder_template = "Tell me a joke.";
        let tmpl = PromptTemplate::new(no_placeholder_template);
        assert!(tmpl.is_ok());
        let tmpl = tmpl.unwrap();
        assert_eq!(tmpl.template, no_placeholder_template);
        assert_eq!(tmpl.template_format, TemplateFormat::PlainText);
        assert_eq!(tmpl.input_variables.len(), 0);
    }

    #[test]
    fn test_prompt_template_new_error() {
        let mixed_template = "Tell me a {adjective} joke about {{content}}.";
        let tmpl_err = PromptTemplate::new(mixed_template).unwrap_err();
        assert!(matches!(tmpl_err, TemplateError::MalformedTemplate(_)));

        let malformed_fmtstring = "Tell me a {adjective joke about {content}.";
        let tmpl_err = PromptTemplate::new(malformed_fmtstring).unwrap_err();
        assert!(matches!(tmpl_err, TemplateError::MalformedTemplate(_)));

        let malformed_mustache = "Tell me a {{adjective joke about {{content}}.";
        let tmpl_err = PromptTemplate::new(malformed_mustache).unwrap_err();
        assert!(matches!(tmpl_err, TemplateError::MalformedTemplate(_)));
    }

    #[test]
    fn test_fmtstring_formatting() {
        let tmpl = PromptTemplate::new("Hello, {name}!").unwrap();
        let variables = prompt_vars!(name = "John");
        let formatted = tmpl.format(variables).unwrap();
        assert_eq!(formatted, "Hello, John!");

        let tmpl = PromptTemplate::new("Hi {name}, you are {age} years old!").unwrap();
        let variables = prompt_vars!(name = "Alice", age = "30");
        let formatted = tmpl.format(variables).unwrap();
        assert_eq!(formatted, "Hi Alice, you are 30 years old!");

        let tmpl = PromptTemplate::new("Hello World!").unwrap();
        let variables = prompt_vars!();
        let formatted = tmpl.format(variables).unwrap();
        assert_eq!(formatted, "Hello World!");

        let tmpl = PromptTemplate::new("Goodbye, {name}!").unwrap();
        let variables = prompt_vars!(name = "John", extra = "data");
        let formatted = tmpl.format(variables).unwrap();
        assert_eq!(formatted, "Goodbye, John!");

        let tmpl = PromptTemplate::new("Goodbye, {name}!").unwrap();
        let variables = prompt_vars!(wrong_name = "John");
        let result = tmpl.format(variables);
        assert!(result.is_err());

        let tmpl = PromptTemplate::new("Hi {name}, you are {age} years old!").unwrap();
        let variables = prompt_vars!(name = "Alice");
        let result = tmpl.format(variables).unwrap_err();
        assert!(matches!(result, TemplateError::MissingVariable(_)));
    }

    #[test]
    fn test_format_mustache_success() {
        let tmpl = PromptTemplate::new("Hello, {{name}}!").unwrap();
        let variables = prompt_vars!(name = "John");
        let result = tmpl.format(variables).unwrap();
        assert_eq!(result, "Hello, John!");

        let variables = prompt_vars!(name = "John", extra = "data");
        let result = tmpl.format(variables).unwrap();
        assert_eq!(result, "Hello, John!");

        let tmpl_multiple_vars =
            PromptTemplate::new("Hello, {{name}}! You are {{adjective}}.").unwrap();
        let variables = prompt_vars!(name = "John", adjective = "awesome");
        let result = tmpl_multiple_vars.format(variables).unwrap();
        assert_eq!(result, "Hello, John! You are awesome.");

        let tmpl_multiple_instances =
            PromptTemplate::new("{{greeting}}, {{name}}! {{greeting}}, again!").unwrap();
        let variables = prompt_vars!(greeting = "Hello", name = "John");
        let result = tmpl_multiple_instances.format(variables).unwrap();
        assert_eq!(result, "Hello, John! Hello, again!");
    }

    #[test]
    fn test_format_mustache_error() {
        let tmpl_missing_var = PromptTemplate::new("Hello, {{name}}!").unwrap();
        let variables = prompt_vars!(adjective = "cool");
        let err = tmpl_missing_var.format(variables).unwrap_err();
        assert!(matches!(err, TemplateError::MissingVariable(_)));
    }

    #[test]
    fn test_format_plaintext() {
        let tmpl = PromptTemplate::new("Hello, world!").unwrap();
        let variables = prompt_vars!();
        let result = tmpl.format(variables).unwrap();
        assert_eq!(result, "Hello, world!");

        let tmpl = PromptTemplate::new("Welcome to the Rust world!").unwrap();
        let variables = prompt_vars!(name = "John", adjective = "awesome");
        let result = tmpl.format(variables).unwrap();
        assert_eq!(result, "Welcome to the Rust world!");

        let tmpl_no_placeholders = PromptTemplate::new("No placeholders here").unwrap();
        let variables = prompt_vars!(name = "ignored");
        let result = tmpl_no_placeholders.format(variables).unwrap();
        assert_eq!(result, "No placeholders here");

        let tmpl_extra_spaces = PromptTemplate::new("  Just some text   ").unwrap();
        let variables = prompt_vars!();
        let result = tmpl_extra_spaces.format(variables).unwrap();
        assert_eq!(result, "  Just some text   ");

        let tmpl_with_newlines = PromptTemplate::new("Text with\nmultiple lines\n").unwrap();
        let result = tmpl_with_newlines.format(prompt_vars!()).unwrap();
        assert_eq!(result, "Text with\nmultiple lines\n");
    }
}
